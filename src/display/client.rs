use super::{Keycode, Button, Keysym};

/// Generic display server.
///
/// Note that this trait is not 100% independent from X11,
/// but xcb::Keycode and Button could be part of the interface.
///
/// In case of other display servers we will see how will this fit.
pub trait DisplayServerClient {
    /// get next event
    fn wait_for_event(&mut self) -> Option<DisplayServerEvent>;

    /// let other clients get the event
    /// this must be called after every event
    fn release_event(&mut self, event: DisplayServerEvent, handling: EventHandling);

    /// subscribe to key events
    fn grab_keysym_checked(
        &mut self,
        keysym: Keysym,
        modifiers: u16,
    ) -> (Vec<Keycode>, Vec<GrabError<Keycode>>);

    /// subscribe to key events
    fn grab_keycode_checked(
        &mut self,
        keysym: Keycode,
        modifiers: u16,
    ) -> Result<Keycode, GrabError<Keycode>>;

    /// subscribe to pointer events
    fn grab_button_checked(
        &mut self,
        button: Button,
        modifiers: u16,
    ) -> Result<(), GrabError<Button>>;

    /// apply changes
    fn flush(&mut self);
}

#[derive(Debug, Clone, Copy)]
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
