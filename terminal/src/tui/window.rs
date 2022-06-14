use std::rc::Rc;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use super::store::State;
use super::template;
use super::template::Padding;
use super::theme::Theme;

pub trait Widget<B: Backend> {
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State);
}

#[derive(Copy, Clone)]
pub struct TitleWidget;

impl<B> Widget<B> for TitleWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let default = String::from("-");
        let _ = state.get::<String>("app.title").unwrap_or(&default);
        let project = state.get::<String>("project.name").unwrap_or(&default);

        let (block, inner) = template::block(theme, area, Padding { top: 1, left: 4 }, true);
        frame.render_widget(block, area);

        let project = template::paragraph(project, theme.highlight_invert);
        frame.render_widget(project, inner);
    }
}

#[derive(Copy, Clone)]
pub struct ShortcutWidget;

impl<B> Widget<B> for ShortcutWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let default = vec![];
        let shortcuts = state
            .get::<Vec<String>>("app.shortcuts")
            .unwrap_or(&default);

        let constraints = shortcuts
            .iter()
            .map(|s| Constraint::Length(s.len() as u16 + 2))
            .collect::<Vec<_>>();
        let shortcuts = shortcuts
            .iter()
            .map(|s| Span::styled(s, theme.ternary))
            .collect::<Vec<_>>();

        let (_, inner) = template::block(theme, area, Padding { top: 1, left: 2 }, false);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner);
        let mut iter = chunks.iter();

        for shortcut in shortcuts {
            if let Some(chunk) = iter.next() {
                let paragraph = Paragraph::new(Spans::from(shortcut));
                frame.render_widget(paragraph, *chunk);
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct EmptyWidget;

impl<B> Widget<B> for EmptyWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, _theme: &Theme, area: Rect, _state: &State) {
        let block = Block::default().borders(Borders::NONE);
        frame.render_widget(block, area);
    }
}

#[derive(Clone)]
pub struct PageWidget<B: Backend> {
    pub widgets: Vec<Rc<dyn Widget<B>>>,
    pub context: Rc<dyn Widget<B>>,
}

impl<B> Widget<B> for PageWidget<B>
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let mut constraints = self
            .widgets
            .iter()
            .map(|_| Constraint::Length(area.height / 2))
            .collect::<Vec<_>>();
        constraints.push(Constraint::Max(1));

        let chunks = Layout::default().constraints(constraints).split(area);
        let mut iter = chunks.iter();

        for widget in &self.widgets {
            if let Some(chunk) = iter.next() {
                widget.draw(frame, theme, *chunk, state)
            }
        }
        if let Some(chunk) = iter.next() {
            self.context.draw(frame, theme, *chunk, state)
        }
    }
}

pub struct ApplicationWindow<B: Backend> {
    pub title: Rc<dyn Widget<B>>,
    pub pages: Vec<PageWidget<B>>,
    pub shortcuts: Rc<dyn Widget<B>>,
}

impl<B> ApplicationWindow<B>
where
    B: Backend,
{
    pub fn draw(&self, frame: &mut Frame<B>, theme: &Theme, state: &State) {
        let title_height = 3;
        let shortcut_height = 3;
        let page_height = frame.size().height - title_height - shortcut_height;
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(title_height),
                    Constraint::Length(page_height),
                    Constraint::Min(shortcut_height),
                ]
                .as_ref(),
            )
            .split(frame.size());

        self.title.draw(frame, theme, chunks[0], state);
        self.draw_active_page(frame, theme, chunks[1], state);
        self.shortcuts.draw(frame, theme, chunks[2], state);
    }

    pub fn draw_active_page(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let default = 0;
        let index = state.get::<usize>("app.page.index").unwrap_or(&default);
        if let Some(page) = self.pages.get(*index) {
            page.draw(frame, theme, area, state);
        }
    }
}
