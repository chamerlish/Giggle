use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use crate::{states::{self, ColorScheme}, task::Task};

pub struct TaskView<'a> {
    task: &'a Task,
    color: Color
}

impl <'a> TaskView<'a> {
    pub fn new(task: &'a Task) -> Self {
        TaskView { task: task, color: states::ColorScheme::IDLE }
    }

}

impl Widget for TaskView<'_> {
    #[expect(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let task: &Task = &self.task;
        if (area.height > 1) {
            buf.set_string(
                area.x,
                area.y,
                "_".repeat(area.width as usize), 
                ColorScheme::state_to_color(&task.state));
        }
        buf.set_string(
            //area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.left(),
            // area.y + (area.height.saturating_sub(1)) / 2,
            area.top(),
            &self.task.name,
            ColorScheme::state_to_color(&task.state));
    }
}