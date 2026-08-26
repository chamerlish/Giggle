use ratatui::{layout::Rect, widgets::{Block, Widget}};

use crate::{task_manager::TaskManager, widgets::task_view::TaskView};
pub struct TaskManagerViewer<'a> {
    task_views: Vec<TaskView<'a>>,
    block: Option<Block<'a>>
}

impl <'a> TaskManagerViewer <'a> {
    pub fn new() -> Self {
        TaskManagerViewer { task_views: Vec::new(), block: None }
    }
    pub fn sync_task(mut self ,task_manager: &'a TaskManager) -> Self {
        self.task_views.clear();
        for task in task_manager.get_task_list() {
            self.task_views.push(TaskView::new(task));
        }
        self
    }
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Widget for TaskManagerViewer<'_> {
    #[expect(clippy::cast_possible_truncation)]
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized
    {
        let inner: Rect = if let Some(block) = self.block {
            let inner: Rect = block.inner(area);
            block.render(area, buf);
            inner
        } else { area };

        if inner.is_empty() {
            return;
        }

        for task in self.task_views {
            task.render(inner, buf);
        }
    }
}