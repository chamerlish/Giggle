use std::vec;

use crate::task::{self, Task};

pub struct TaskManager {
    tasks: Vec<Task>,
    max_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: (vec::Vec::new()), max_id: 0 }
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(id as usize)
    }

    pub fn add_task(&mut self, task: Task) -> () {
        self.max_id += 1;
        let mut task = task;
        task.id = self.max_id;

        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: u32) -> () {
        self.tasks.retain(|task: &Task| task.id != id);

        for task in self.tasks.iter_mut() {
            if task.id > id {
                task.id -= 1;
            }
        }

        self.max_id -= 1;
    }

    pub fn complete_task(&mut self, id: u32) -> () {
        if let Some(task) = self.tasks.iter_mut().find(|task: &&mut Task| task.id == id) {
            task.complete();
        }
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
