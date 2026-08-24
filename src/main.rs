use crate::widgets::task_view::TaskView;
use crate::{task_manager::TaskManager, widgets::task_view};
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
pub mod states;
mod widgets;

fn main() -> color_eyre::Result<()> {



    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut manager: TaskManager = TaskManager::new();

    manager.add_task(Task::new("Buy groceries"));

    manager.add_task(Task::new("Clean the house"));

    println!("Tasks:");
    for task in manager.list_tasks() {
        let task: &Task = task;
        println!("{}: {} [{}]", task.id, task.name, if task.completed { "Completed" } else { "Pending" });
    }
    loop {
        terminal.draw(|frame: &mut Frame| {
            render(frame, &manager);
        })?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, manager: &TaskManager) {
    let block: Block = Block::default()
        .title("Giggle Task Manager")
        .border_style(Style::default().fg(Color::LightCyan))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let task_viewer: TaskView = TaskView::new(manager.get_task(1).unwrap());

    frame.render_widget(block, frame.area());
    frame.render_widget(task_viewer, frame.area());
}