pub enum NodeOut<KeyState, UserState> {
    Next(fn(key_state: &KeyState, user_state: &mut UserState) -> Self),
    // idea: NextMouse(fn(pos: Pos) -> Self),
    None,
}
