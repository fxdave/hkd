mod binding;
mod config;
use std::fmt::Debug;

use xcb::{
    cast_event, ButtonPressEvent, ButtonReleaseEvent, KeyPressEvent, KeyReleaseEvent, ReplyError,
};
use xcb_util::keysyms::KeySymbols;

struct App {
    // display: the collection of monitors that share common keyboards and pointers
    // screen: a single monitor with common keyboards and pointers
    conn: xcb::Connection,
    root: xcb::Window,
}

impl App {
    fn new() -> Self {
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

    fn key_symbol_tool(&self) -> KeySymbols<'_> {
        KeySymbols::new(&self.conn)
    }

    // For e.g: US layout to HU layout
    fn handle_keymap_change(&mut self, evt: xcb::GenericEvent) {
        let e: &xcb::MappingNotifyEvent = unsafe { cast_event(&evt) };
        if self.key_symbol_tool().refresh_keyboard_mapping(e) == 1 {
            println!("mapping notify {:?} {:?}", e.request(), e.count());
        }
    }

    /// let other clients get the event
    /// this must be called after every event
    fn allow_event(&mut self, event_type: u8, replay_event: bool) {
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

    /// subscribe to key events
    fn grab_keycode_checked(
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
            reply_error,
            event_type: "key",
            value: keycode,
            modifiers: modifiers,
        })
    }

    /// subscribe to pointer events
    fn grab_button_checked(
        &mut self,
        button: xcb::Button,
        modifiers: u16,
    ) -> Result<(), GrabError<xcb::Button>> {
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
            reply_error,
            event_type: "button",
            value: button,
            modifiers: modifiers,
        })
    }

    /// subscribe to key events by keysym like x11::keysym::XK_4 for the key 4
    fn grab_keysym(&mut self, keysym: xcb::Keysym, modifiers: u16) {
        let keycodes: Vec<_> = self.key_symbol_tool().get_keycode(keysym).collect();
        keycodes.into_iter().for_each(|keycode| {
            match self.grab_keycode_checked(keycode, modifiers) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        });
    }

    // subscribe to mouse button events like xcb::BUTTON_INDEX_2 for middle mouse button
    fn grab_button(&mut self, button: xcb::Button, modifiers: u16) {
        match self.grab_button_checked(button, modifiers) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
}

impl Drop for App {
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

#[derive(Debug)]
struct GrabError<T> {
    reply_error: ReplyError,
    event_type: &'static str, // key | pointer button
    value: T,
    modifiers: u16,
}

impl<T: Debug> std::error::Error for GrabError<T> {}
impl<T: Debug> std::fmt::Display for GrabError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WARN: Could not grab {:?} {:?} with modifiers {:?}: {details}",
            self.event_type,
            self.value,
            self.modifiers,
            details = match self.reply_error {
                ReplyError::GenericError(ref err) if err.error_code() == xcb::ACCESS => {
                    "WARN: the combination is already grabbed.".into()
                }
                ReplyError::GenericError(ref err) => {
                    format!(
                        "WARN: xcb request error number {} encountered.",
                        err.error_code()
                    )
                }
                _ => "".into(),
            }
        )
    }
}

fn main() {
    // TODO:
    // signal(SIGINT, hold); // Ctrl + C
    // signal(SIGHUP, hold); // controlling terminal closed
    // signal(SIGTERM, hold); // request to close
    // signal(SIGUSR1, hold); // custom
    // signal(SIGUSR2, hold); // custom
    // signal(SIGALRM, hold); // low power

    let mut app = App::new();

    // Grab keys
    app.grab_keysym(x11::keysym::XK_4, 0);
    app.grab_keysym(x11::keysym::XF86XK_Calculator, 0);
    app.grab_keysym(x11::keysym::XK_5, 0);
    app.grab_button(xcb::BUTTON_INDEX_2 as u8, 0);
    app.conn.flush();

    // Get events
    while let Some(evt) = app.conn.wait_for_event() {
        let event_type: u8 = evt.response_type();
        match event_type {
            xcb::KEY_PRESS => {
                let event: &KeyPressEvent = unsafe { cast_event(&evt) };
                println!("A key pressed: {:?}", event.detail());
            }
            xcb::KEY_RELEASE => {
                let event: &KeyReleaseEvent = unsafe { cast_event(&evt) };
                println!("A key released: {:?}", event.detail());
            }
            xcb::BUTTON_PRESS => {
                let event: &ButtonPressEvent = unsafe { cast_event(&evt) };
                println!("A button pressed: {:?}", event.detail());
            }
            xcb::BUTTON_RELEASE => {
                let event: &ButtonReleaseEvent = unsafe { cast_event(&evt) };
                println!("A button released: {:?}", event.detail());
            }
            // when the user changes keyboard layout
            xcb::MAPPING_NOTIFY => {
                println!("Mapping");
                app.handle_keymap_change(evt);
            }
            e => {
                println!("received event, {:?}", e);
            }
        }
        app.allow_event(event_type, true);
    }
}
