use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::Span;
use tui::widgets::{Block, Borders};

use crate::app::App;

pub trait Widget<'a> {
    fn new(app: &'a mut App) -> Self;
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect);
}

pub struct MenuWidget<'a> {
    pub app: &'a mut App,
}

impl<'a> Widget<'a> for MenuWidget<'a> {
    fn new(app: &'a mut App) -> Self {
        MenuWidget {
            app: app,
        }
    }

    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let title = match &self.app.state.project {
            Some(project) => format!(" rad-tui({}) ", project.name),
            None => " rad-tui ".to_owned(),
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(title, Style::default()));

        frame.render_widget(block, area);
    }
}
