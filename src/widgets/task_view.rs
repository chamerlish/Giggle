use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::{Block, Widget}};

use crate::{states::{self, ColorScheme}, task::Task};

pub struct TaskView<'a> {
    task: &'a Task,
    color: Color,
    block: Option<Block<'a>>
}

impl <'a> TaskView<'a> {
    pub fn new(task: &'a Task) -> Self {
        TaskView { task: task, color: states::ColorScheme::IDLE, block: None}
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

}

impl Widget for TaskView<'_> {
    #[expect(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {

        let inner = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        } else { area };

        if inner.is_empty() {
            return;
        }

        let task: &Task = &self.task;
        if (area.height > 1) {
            buf.set_string(
                inner.x,
                inner.y,
                "_".repeat(area.width as usize), 
                ColorScheme::state_to_color(&task.state));
        }
        buf.set_string(
            //area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            inner.left(),
            // area.y + (area.height.saturating_sub(1)) / 2,
            inner.top(),
            &task.name,
            ColorScheme::state_to_color(&task.state));
    }
}