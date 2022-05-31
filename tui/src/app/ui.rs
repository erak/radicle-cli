use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3)].as_ref())
        .split(frame.size());

    let title = match &app.state.project {
        Some(project) => format!(" rad-tui({}) ", project.name),
        None => " rad-tui ".to_owned(),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(title, Style::default()));

    frame.render_widget(block, chunks[0]);
}
