use std::rc::Rc;

use tui::backend::Backend;
use tui::layout::{Direction, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use super::layout;
use super::layout::Padding;
use super::store::State;
use super::template;
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
        let lengths = shortcuts
            .iter()
            .map(|s| s.len() as u16 + 2)
            .collect::<Vec<_>>();

        let (_, inner) = template::block(theme, area, Padding { top: 1, left: 2 }, false);
        let areas = layout::split_area(inner, lengths, Direction::Horizontal);
        let mut areas = areas.iter();

        let shortcuts = shortcuts
            .iter()
            .map(|s| Span::styled(s, theme.ternary))
            .collect::<Vec<_>>();
        for shortcut in shortcuts {
            if let Some(area) = areas.next() {
                let paragraph = Paragraph::new(Spans::from(shortcut));
                frame.render_widget(paragraph, *area);
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
    pub title: Rc<dyn Widget<B>>,
    pub widgets: Vec<Rc<dyn Widget<B>>>,
    pub context: Rc<dyn Widget<B>>,
}

impl<B> Widget<B> for PageWidget<B>
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let title_h = 3;
        let widget_h = (area.height / self.widgets.len() as u16) - 2;
        let context_h = 1;

        let lengths = [
            vec![title_h],
            vec![widget_h; self.widgets.len()],
            vec![context_h],
        ]
        .concat();

        let areas = layout::split_area(area, lengths, Direction::Vertical);
        let mut areas = areas.iter();

        if let Some(area) = areas.next() {
            self.title.draw(frame, theme, *area, state)
        }
        for widget in &self.widgets {
            if let Some(area) = areas.next() {
                widget.draw(frame, theme, *area, state)
            }
        }
        if let Some(area) = areas.next() {
            self.context.draw(frame, theme, *area, state)
        }
    }
}

pub struct ApplicationWindow<B: Backend> {
    pub pages: Vec<PageWidget<B>>,
    pub shortcuts: Rc<dyn Widget<B>>,
}

impl<B> ApplicationWindow<B>
where
    B: Backend,
{
    pub fn draw(&self, frame: &mut Frame<B>, theme: &Theme, state: &State) {
        let shortcut_h = 3;
        let page_h = frame.size().height - shortcut_h;
        let areas = layout::split_area(frame.size(), vec![page_h, shortcut_h], Direction::Vertical);

        self.draw_active_page(frame, theme, areas[0], state);
        self.shortcuts.draw(frame, theme, areas[1], state);
    }

    pub fn draw_active_page(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let default = 0;
        let index = state.get::<usize>("app.page.index").unwrap_or(&default);
        if let Some(page) = self.pages.get(*index) {
            page.draw(frame, theme, area, state);
        }
    }
}
