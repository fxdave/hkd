use xcb::{
    cast_event, ButtonPressEvent, ButtonReleaseEvent, KeyPressEvent, KeyReleaseEvent, ReplyError,
};
use xcb_util::keysyms::KeySymbols;

use crate::display::{
    client::{DisplayServerEvent, GrabError},
    Button, DisplayServerClient, EventHandling, Keycode, Keysym,
};

// Definitions:
// display: the collection of monitors that share common keyboards and pointers
// screen: a single monitor with common keyboards and pointers
// window: a node in the X server's window tree.
//   So windows can contain other windows.

pub struct X11Client {
    /// The connection with the X server
    conn: xcb::Connection,
    /// The most outer window in the window tree.
    root: xcb::Window,
}

impl X11Client {
    pub fn new() -> Self {
        let (conn, screen_idx) = xcb::Connection::connect(None).expect("Can't open display");
        Self {
            root: conn
                .get_setup()
                .roots()
                .nth(screen_idx as usize)
                .expect("Can't acquire screen")
                .root(),
            conn,
        }
    }

    pub fn key_symbol_tool(&self) -> KeySymbols<'_> {
        KeySymbols::new(&self.conn)
    }

    // For e.g: US layout to HU layout
    pub fn handle_keymap_change(&mut self, evt: xcb::GenericEvent) {
        let e: &xcb::MappingNotifyEvent = unsafe { cast_event(&evt) };
        if self.key_symbol_tool().refresh_keyboard_mapping(e) == 1 {
            println!("mapping notify {:?} {:?}", e.request(), e.count());
        }
    }

    /// subscribe to key events
    pub fn grab_keycode_checked(
        &mut self,
        keycode: xcb::Keycode,
        modifiers: u16,
    ) -> Result<(), GrabError<xcb::Keycode>> {
        xcb::xproto::grab_key(
            &self.conn,
            true,
            self.root,
            modifiers,
            keycode,
            xcb::GRAB_MODE_ASYNC as u8,
            xcb::GRAB_MODE_SYNC as u8,
        )
        .request_check()
        .map(|_| {
            println!("grab key {:?} {:?} ", keycode, modifiers);
        })
        .map_err(|reply_error| GrabError {
            details: fetch_error(reply_error),
            event_type: "key",
            value: keycode,
            modifiers: modifiers,
        })
    }

    fn wait_for_event(&mut self) -> Option<(DisplayServerEvent, u8)> {
        self.conn.wait_for_event().and_then(|evt| {
            let event_type: u8 = evt.response_type();
            match event_type {
                xcb::KEY_PRESS => {
                    let event: &KeyPressEvent = unsafe { cast_event(&evt) };
                    Some((DisplayServerEvent::KeyPress(event.detail()), event_type))
                }
                xcb::KEY_RELEASE => {
                    let event: &KeyReleaseEvent = unsafe { cast_event(&evt) };
                    Some((DisplayServerEvent::KeyRelease(event.detail()), event_type))
                }
                xcb::BUTTON_PRESS => {
                    let event: &ButtonPressEvent = unsafe { cast_event(&evt) };
                    Some((DisplayServerEvent::ButtonPress(event.detail()), event_type))
                }
                xcb::BUTTON_RELEASE => {
                    let event: &ButtonReleaseEvent = unsafe { cast_event(&evt) };
                    Some((
                        DisplayServerEvent::ButtonRelease(event.detail()),
                        event_type,
                    ))
                }
                // when the user changes keyboard layout
                xcb::MAPPING_NOTIFY => {
                    println!("Mapping");
                    self.handle_keymap_change(evt);
                    None
                }
                e => {
                    println!("received event, {:?}", e);
                    None
                }
            }
        })
    }
}

impl DisplayServerClient for X11Client {
    fn release_event(&mut self, event_type: u8, replay_event: bool) {
        let mode = match (event_type, replay_event) {
            (xcb::KEY_PRESS | xcb::KEY_RELEASE, true) => xcb::ALLOW_REPLAY_KEYBOARD,
            (xcb::KEY_PRESS | xcb::KEY_RELEASE, false) => xcb::ALLOW_SYNC_KEYBOARD,
            (xcb::BUTTON_PRESS | xcb::BUTTON_RELEASE, true) => xcb::ALLOW_REPLAY_POINTER,
            (xcb::BUTTON_PRESS | xcb::BUTTON_RELEASE, false) => xcb::ALLOW_SYNC_POINTER,
            _ => {
                // only keyboard and pointer events could be allowed
                return;
            }
        } as u8;
        xcb::allow_events(&self.conn, mode, xcb::CURRENT_TIME);
        self.conn.flush();
    }

    fn grab_keysym_checked(
        &mut self,
        keysym: Keysym,
        modifiers: u16,
    ) -> Result<(), Vec<GrabError<Keycode>>> {
        let keycodes: Vec<_> = self.key_symbol_tool().get_keycode(keysym).collect();
        let errors: Option<Vec<GrabError<xcb::Keycode>>> = keycodes
            .into_iter()
            .map(|keycode| self.grab_keycode_checked(keycode, modifiers).err())
            .collect();
        if let Some(errors) = errors {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn grab_button_checked(
        &mut self,
        button: Button,
        modifiers: u16,
    ) -> Result<(), GrabError<Button>> {
        xcb::xproto::grab_button(
            &self.conn,
            true,
            self.root,
            xcb::EVENT_MASK_BUTTON_PRESS as u16 | xcb::EVENT_MASK_BUTTON_RELEASE as u16,
            xcb::GRAB_MODE_SYNC as u8,
            xcb::GRAB_MODE_ASYNC as u8,
            0,
            0,
            button,
            modifiers,
        )
        .request_check()
        .map(|_| {
            println!("grab button {:?} {:?} ", button, modifiers);
        })
        .map_err(|reply_error| GrabError {
            details: fetch_error(reply_error),
            event_type: "button",
            value: button,
            modifiers: modifiers,
        })
    }

    fn flush(&mut self) {
        self.conn.flush();
    }

    fn subscribe(&mut self, mut callback: Box<dyn FnMut(DisplayServerEvent) -> EventHandling>) {
        loop {
            if let Some((event, event_type)) = self.wait_for_event() {
                let replay_event = callback(event).into();
                self.release_event(event_type, replay_event)
            }
        }
    }
}

impl Drop for X11Client {
    fn drop(&mut self) {
        println!("UNGRAB");
        xcb::xproto::ungrab_key(
            &self.conn,
            xcb::GRAB_ANY as u8,
            self.root,
            xcb::BUTTON_MASK_ANY as u16,
        );
        xcb::xproto::ungrab_button(
            &self.conn,
            xcb::BUTTON_INDEX_ANY as u8,
            self.root,
            xcb::MOD_MASK_ANY as u16,
        );
        self.conn.flush();
    }
}

fn fetch_error(reply_error: ReplyError) -> String {
    match reply_error {
        xcb::ReplyError::GenericError(ref err) if err.error_code() == xcb::ACCESS => {
            "WARN: the combination is already grabbed.".into()
        }
        xcb::ReplyError::GenericError(ref err) => {
            format!(
                "WARN: xcb request error number {} encountered.",
                err.error_code()
            )
        }
        _ => "".into(),
    }
}
