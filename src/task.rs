use crate::states::State;
pub struct Task {
    pub id: u32,
    pub name: String,
    pub completed: bool,
    
    pub state: State,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Task {
            id: 0,
            name: name.to_string(),
            completed: false,
            state: State::Idle,
        }
    }

    pub fn complete(&mut self) -> () {
        self.completed = true;
        self.state = State::Completed;
    }
}