pub trait ManageKeyState {
    fn new(client: &mut dyn display::DisplayServerClient) -> Self;
    fn update(&mut self, e: display::DisplayServerEvent);
}

pub const SIZE_OF_THE_ACTIVE_KEYS: usize = 100;
macro_rules! define_keys {
    ($($name:ident => $key:expr),* $(,)?) => {
        pub struct KeyState {
            // keycode: a number marking the physical place of keys
            keycodes: std::collections::HashSet<crate::display::Keycode>,
            // keysym: a number assigned to the name of the key (understandable by both programs and humans). Like enter.
            // 1 keysym could need multiple keycodes
            // keys:
            $( $name: Vec<crate::display::Keycode>, )*
        }

        impl KeyState {

            // keys:
            $(
                fn $name(&self)-> bool {
                    println!("{:?}", self.$name);
                    println!("{:?}", self.keycodes);
                    self.$name.iter().map(|code| self.keycodes.contains(code)).all(|r| r)
                }
            )*
        }

        impl crate::state_machine::ManageKeyState for KeyState {

            fn new(client: &mut dyn crate::display::DisplayServerClient) -> Self {
                Self {
                    keycodes: std::collections::HashSet::with_capacity(crate::state_machine::SIZE_OF_THE_ACTIVE_KEYS),
                    $( $name: crate::state_machine::grab_keysym(client, $key),)*
                }
            }
            fn update(&mut self, e: crate::display::DisplayServerEvent) {
                match e {
                    crate::display::DisplayServerEvent::KeyRelease(k) => {
                        println!("relesae: {:?}", k);
                        self.keycodes.remove(&k);},
                    crate::display::DisplayServerEvent::KeyPress(k) => {self.keycodes.insert(k);},
                    _ => {
                        // do nothing
                    }
                }
            }
        }
    };
}
pub(crate) use define_keys;

use crate::display;

pub fn grab_keysym(
    client: &mut dyn display::DisplayServerClient,
    keysym: display::Keysym,
) -> Vec<display::Keycode> {
    let results = client.grab_keysym_checked(keysym, display::Modifier::Any as u16);
    results.1.iter().for_each(|err| {
        println!("{:?}", err);
    });
    results.0
}
