use crate::widgets::task_manager_viewer::TaskManagerViewer;
use crate::widgets::task_view::TaskView;
use crate::{task_manager::TaskManager};
use crate::task::Task;

use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::macros::span;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};


use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, BorderType},
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

    let mut list_state: ListState = ListState::default().with_selected(Some(0));


    loop {
        terminal.draw(|frame: &mut Frame| {
            render(frame, list_state, &manager);
        })?;
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
                KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, mut list_state: ListState, manager: &TaskManager) {

    let constraints: [Constraint; 3] = [
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ];

    let layout: Layout = Layout::vertical(constraints).spacing(1);
    let [top, first, second] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("Shits and giggles").bold(),
        Span::from("meow")
    ]);


    frame.render_widget(title.centered(), top);

    render_list(frame, frame.area(), &mut list_state, &manager);


}

fn render_list(frame: &mut Frame, area: Rect, list_state: &mut ListState, manager: &TaskManager) {
    let items = manager
        .get_task_list()
        .iter()
        .cloned()
        .map(|task| {
            ListItem::from(task)
        });
    let list = List::new(items);

    frame.render_stateful_widget(list, area, list_state);
}