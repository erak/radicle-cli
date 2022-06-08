use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

use super::store::{State, Value};

pub trait Widget<B: Backend> {
    fn draw(&self, frame: &mut Frame<B>, area: Rect, state: &State);
}

#[derive(Copy, Clone)]
pub struct MenuWidget;

impl<B> Widget<B> for MenuWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, area: Rect, state: &State) {
        let title = match state.get("state.title") {
            Some(Value::String(title)) => title.clone(),
            Some(_) | None => String::new(),
        };
        let project = match state.get("state.project.name") {
            Some(Value::String(name)) => name.clone(),
            Some(_) | None => String::new(),
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(format!(" {title} "), Style::default()));
        let info = vec![Spans::from(Span::styled(
            format!(" {project}"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))];
        let menu = Paragraph::new(info).block(block).wrap(Wrap { trim: false });

        frame.render_widget(menu, area);
    }
}

#[derive(Copy, Clone)]
pub struct ShortcutWidget;

impl<B> Widget<B> for ShortcutWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, area: Rect, state: &State) {
        let text = match state.get("state.shortcuts") {
            Some(Value::Strings(shortcuts)) => shortcuts
                .iter()
                .map(|s| Spans::from(Span::styled(s, Style::default())))
                .collect(),
            Some(_) | None => vec![],
        };
        let block = Block::default().borders(Borders::NONE);
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

#[derive(Copy, Clone)]
pub struct EmptyWidget;

impl<B> Widget<B> for EmptyWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, area: Rect, _state: &State) {
        let block = Block::default().borders(Borders::NONE);
        frame.render_widget(block, area);
    }
}

pub struct ApplicationWindow<B: Backend> {
    pub menu: Box<dyn Widget<B>>,
    pub content: Box<dyn Widget<B>>,
    pub shortcuts: Box<dyn Widget<B>>,
}

impl<B> ApplicationWindow<B>
where
    B: Backend,
{
    pub fn draw(&self, frame: &mut Frame<B>, state: &State) {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(frame.size());
        self.menu.draw(frame, chunks[0], state);
        self.content.draw(frame, chunks[1], state);
        self.shortcuts.draw(frame, chunks[2], state);
    }
}
