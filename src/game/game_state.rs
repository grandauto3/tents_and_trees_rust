pub struct GameState {
    pub clicked: bool,
}


impl GameState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            clicked: false,
        }
    }
}