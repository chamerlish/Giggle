use crate::task_manager::TaskManager;
use crate::task::Task;

use std::io;

use crossterm::event;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, BorderType, Paragraph, Widget},
    DefaultTerminal, Frame,
};

mod task;
mod task_manager;

fn main() -> color_eyre::Result<()> {

    let mut manager: TaskManager = TaskManager::new();

    manager.add_task(Task{ id: 0, name: "Buy groceries".to_string(), completed: false });

    manager.add_task(Task{ id: 1, name: "Clean the house".to_string(), completed: false });

    println!("Tasks:");
    for task in manager.list_tasks() {
        let task: &Task = task;
        println!("{}: {} [{}]", task.id, task.name, if task.completed { "Completed" } else { "Pending" });
    }

    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let block: Block = Block::default()
        .title("Giggle Task Manager")
        .border_style(Style::default().fg(Color::LightCyan))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
    frame.render_widget(block, frame.area());
}