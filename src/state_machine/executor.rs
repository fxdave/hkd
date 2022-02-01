use crate::display;

use super::{transition::NodeOut, key_state::ManageKeyState};

pub struct Executor<KeyState, UserState> {
    key_state: KeyState,
    user_state: UserState,
    actual_node: NodeOut<KeyState, UserState>,
}
impl<KeyState: ManageKeyState, UserState> Executor<KeyState, UserState> {
    pub fn new(
        client: &mut dyn display::DisplayServerClient,
        start: fn(&KeyState, &mut UserState) -> NodeOut<KeyState, UserState>,
        user_state: UserState,
    ) -> Self {
        Self {
            key_state: KeyState::new(client),
            actual_node: NodeOut::Next(start),
            user_state
        }
    }
    pub fn next(&mut self, e: display::DisplayServerEvent, client: &mut dyn display::DisplayServerClient) {
        self.key_state.update(e);
        match self.actual_node {
            NodeOut::Next(fnptr) => {
                if let NodeOut::Next(new_fnptr) = (fnptr)(&self.key_state, &mut self.user_state) {
                    client.release_event(e, display::EventHandling::Hide);
                    self.actual_node = NodeOut::Next(new_fnptr)
                } else {
                    client.release_event(e, display::EventHandling::Replay);
                }
            }
            NodeOut::None => unreachable!(),
        }
    }
}
