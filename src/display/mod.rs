/// All possible keysyms
pub use x11::keysym::*;

/// Modifier masks
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Modifier {
    Mod1 = xcb::MOD_MASK_1 as u16,
    Mod2 = xcb::MOD_MASK_2 as u16,
    Mod3 = xcb::MOD_MASK_3 as u16,
    Mod4 = xcb::MOD_MASK_4 as u16,
    Mod5 = xcb::MOD_MASK_5 as u16,
    Any = xcb::MOD_MASK_ANY as u16,
    Control = xcb::MOD_MASK_CONTROL as u16,
    Lock = xcb::MOD_MASK_LOCK as u16,
    Shift = xcb::MOD_MASK_SHIFT as u16,
}
pub struct Modifiers {
    mod1: bool,
    mod2: bool,
    mod3: bool,
    mod4: bool,
    mod5: bool,
    control: bool,
    lock: bool,
    shift: bool,
}

pub type Keysym = xcb::Keysym;
pub type Keycode = xcb::Keycode;
pub type Button = xcb::Button;

mod client;
mod x;

pub use client::*;
pub use x::X11Client;