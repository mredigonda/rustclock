pub struct State {
    pub current_activity: Option<i32>,
}

impl State {
    pub fn new() -> Self {
        State {
            current_activity: Some(0),
        }
    }
}
