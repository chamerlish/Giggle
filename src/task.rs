
pub struct Task {
    pub id: u32,
    pub name: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task {
            id,
            name,
            completed: false,
        }
    }

    pub fn complete(&mut self) -> () {
        self.completed = true;
    }
}