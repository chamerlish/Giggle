use crate::task_manager::TaskManager;
use crate::task::Task;

mod task;
mod task_manager;

fn main() {
    let mut manager: TaskManager = TaskManager::new();

    manager.add_task(Task{ id: 0, name: "Buy groceries".to_string(), completed: false });

    manager.add_task(Task{ id: 1, name: "Clean the house".to_string(), completed: false });

    println!("Tasks:");
    for task in manager.list_tasks() {
        println!("{}: {} [{}]", task.id, task.name, if task.completed { "Completed" } else { "Pending" });
    }
}

