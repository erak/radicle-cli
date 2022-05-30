use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, _app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" rad-tui ", Style::default()));
    let paragraph = Paragraph::new("Running...")
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, chunks[0]);
}
