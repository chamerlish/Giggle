use std::vec;

use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: (vec::Vec::new()) }
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task: &Task| task.id != id);
    }

    pub fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task: &&mut Task| task.id == id) {
            task.complete();
        }
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
