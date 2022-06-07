use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::Style;
use tui::text::Span;
use tui::widgets::{Block, Borders};
use tui::Frame;

pub struct ApplicationWindow {
    pub title: String,
}

impl ApplicationWindow {
    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3)].as_ref())
            .split(frame.size());
        let widget = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(self.title.clone(), Style::default()));

        frame.render_widget(widget, chunks[0]);
    }
}
