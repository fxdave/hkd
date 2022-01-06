use super::{Keycode, Button, Keysym};

/// Generic display server.
///
/// Note that this trait is not 100% independent from X11,
/// but xcb::Keycode and Button could be part of the interface.
///
/// In case of other display servers we will see how will this fit.
pub trait DisplayServerClient {
    /// let other clients get the event
    /// this must be called after every event
    fn release_event(&mut self, event_type: u8, replay_event: bool);

    /// subscribe to key events
    fn grab_keysym_checked(
        &mut self,
        keysym: Keysym,
        modifiers: u16,
    ) -> Result<(), Vec<GrabError<Keycode>>>;

    /// subscribe to pointer events
    fn grab_button_checked(
        &mut self,
        button: Button,
        modifiers: u16,
    ) -> Result<(), GrabError<Button>>;

    /// apply changes
    fn flush(&mut self);

    /// start event loop
    fn subscribe(&mut self, callback: Box<dyn FnMut(DisplayServerEvent) -> EventHandling>);

    /// subscribe to key events by keysym like x11::keysym::XK_4 for the key 4
    fn grab_keysym(&mut self, keysym: Keysym, modifiers: u16) {
        match self.grab_keysym_checked(keysym, modifiers) {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
    }

    /// subscribe to mouse button events like xcb::BUTTON_INDEX_2 for middle mouse button
    fn grab_button(&mut self, button: Button, modifiers: u16) {
        match self.grab_button_checked(button, modifiers) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
}

#[derive(Debug)]
pub enum DisplayServerEvent {
    KeyRelease(Keycode),
    KeyPress(Keycode),
    ButtonRelease(Button),
    ButtonPress(Button),
}

pub enum EventHandling {
    Replay,
    Hide
}

impl From<EventHandling> for bool {
    fn from(handling: EventHandling) -> Self {
        matches!(handling, EventHandling::Replay)
    }
}

/// An error that raises when the display server couldn't give us keys.
#[derive(Debug)]
pub struct GrabError<T> {
    pub details: String,
    /// key | pointer button
    pub event_type: &'static str,
    pub value: T,
    pub modifiers: u16,
}

impl<T: std::fmt::Debug> std::error::Error for GrabError<T> {}
impl<T: std::fmt::Debug> std::fmt::Display for GrabError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WARN: Could not grab {:?} {:?} with modifiers {:?}: {}",
            self.event_type, self.value, self.modifiers, self.details,
        )
    }
}
