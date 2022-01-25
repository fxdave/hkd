use xcb::{
    cast_event, ButtonPressEvent, ButtonReleaseEvent, KeyPressEvent, KeyReleaseEvent, ReplyError,
};
use xcb_util::keysyms::KeySymbols;

use crate::display::{
    client::{DisplayServerEvent as Event, GrabError},
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
}

impl<'a> DisplayServerClient for X11Client {
    fn wait_for_event(&mut self) -> Option<Event> {
        self.conn.wait_for_event().and_then(|evt| {
            let event_type: u8 = evt.response_type();
            match event_type {
                xcb::KEY_PRESS => {
                    let event: &KeyPressEvent = unsafe { cast_event(&evt) };
                    Some(Event::KeyPress(event.detail()))
                }
                xcb::KEY_RELEASE => {
                    let event: &KeyReleaseEvent = unsafe { cast_event(&evt) };
                    Some(Event::KeyRelease(event.detail()))
                }
                xcb::BUTTON_PRESS => {
                    let event: &ButtonPressEvent = unsafe { cast_event(&evt) };
                    Some(Event::ButtonPress(event.detail()))
                }
                xcb::BUTTON_RELEASE => {
                    let event: &ButtonReleaseEvent = unsafe { cast_event(&evt) };
                    Some(Event::ButtonRelease(event.detail()))
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

    fn release_event(&mut self, event: Event, handling: EventHandling) {
        use EventHandling::{Hide, Replay};
        let mode = match (event, handling) {
            (Event::KeyPress(_) | Event::KeyRelease(_), Replay) => xcb::ALLOW_REPLAY_KEYBOARD,
            (Event::KeyPress(_) | Event::KeyRelease(_), Hide) => xcb::ALLOW_SYNC_KEYBOARD,
            (Event::ButtonPress(_) | Event::ButtonRelease(_), Replay) => xcb::ALLOW_REPLAY_POINTER,
            (Event::ButtonPress(_) | Event::ButtonRelease(_), Hide) => xcb::ALLOW_SYNC_POINTER,
            _ => {
                // only keyboard and pointer events could be allowed
                return;
            }
        } as u8;
        xcb::allow_events(&self.conn, mode, xcb::CURRENT_TIME);
        self.conn.flush();
    }

    /// subscribe to key events
    fn grab_keycode_checked(
        &mut self,
        keycode: xcb::Keycode,
        modifiers: u16,
    ) -> Result<xcb::Keycode, GrabError<xcb::Keycode>> {
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
            keycode
        })
        .map_err(|reply_error| GrabError {
            details: fetch_error(reply_error),
            event_type: "key",
            value: keycode,
            modifiers: modifiers,
        })
    }

    fn grab_keysym_checked(
        &mut self,
        keysym: Keysym,
        modifiers: u16,
    ) -> (Vec<Keycode>, Vec<GrabError<Keycode>>) {
        let keycodes: Vec<_> = self.key_symbol_tool().get_keycode(keysym).collect();
        let (oks, errs): (Vec<_>, Vec<_>) = keycodes
            .iter()
            .map(|keycode| self.grab_keycode_checked(*keycode, modifiers))
            .partition(|r| r.is_ok());
        (
            oks.into_iter().filter_map(|r| r.ok()).collect(),
            errs.into_iter().filter_map(|r| r.err()).collect()
        )
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
