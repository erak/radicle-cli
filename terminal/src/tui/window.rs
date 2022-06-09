use std::rc::Rc;

use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

use super::store::State;

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
        let default = String::from("-");
        let title = state.get::<String>("state.title").unwrap_or(&default);
        let project = state
            .get::<String>("state.project.name")
            .unwrap_or(&default);

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
        let default = vec![];
        let shortcuts = state.get::<Vec<String>>("state.shortcuts").unwrap_or(&default);
        let text = shortcuts
            .iter()
            .map(|s| Spans::from(Span::styled(s, Style::default())))
            .collect::<Vec<_>>();
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

#[derive(Clone)]
pub struct PageWidget<B: Backend> {
    pub widgets: Vec<Rc<dyn Widget<B>>>,
}

impl<B> Widget<B> for PageWidget<B>
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, area: Rect, state: &State) {
        let constraints = self
            .widgets
            .iter()
            .map(|_| Constraint::Percentage(100 / self.widgets.len() as u16))
            .collect::<Vec<_>>();
        let chunks = Layout::default().constraints(constraints).split(area);

        for widget in &self.widgets {
            if let Some(chunk) = chunks.iter().next() {
                widget.draw(frame, *chunk, state)
            }
        }
    }
}

pub struct ApplicationWindow<B: Backend> {
    pub menu: Rc<dyn Widget<B>>,
    pub pages: Vec<PageWidget<B>>,
    pub shortcuts: Rc<dyn Widget<B>>,
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
        self.draw_active_page(frame, chunks[1], state);
        self.shortcuts.draw(frame, chunks[2], state);
    }

    pub fn draw_active_page(&self, frame: &mut Frame<B>, area: Rect, state: &State) {
        let default = 0;
        let index = state.get::<usize>("state.view.page.index").unwrap_or(&default);
        if let Some(page) = self.pages.get(*index) {
            page.draw(frame, area, state);
        }
    }
}
