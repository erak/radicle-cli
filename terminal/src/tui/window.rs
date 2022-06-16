use std::rc::Rc;

use anyhow::{Error, Result};

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
    fn draw(
        &self,
        frame: &mut Frame<B>,
        theme: &Theme,
        area: Rect,
        state: &State,
    ) -> Result<(), Error>;
    fn height(&self, area: Rect) -> u16;
}

#[derive(Copy, Clone)]
pub struct TitleWidget;

impl<B> Widget<B> for TitleWidget
where
    B: Backend,
{
    fn draw(
        &self,
        frame: &mut Frame<B>,
        theme: &Theme,
        area: Rect,
        state: &State,
    ) -> Result<(), Error> {
        let project = state.get::<String>("project.name")?;

        let (block, inner) = template::block(theme, area, Padding { top: 1, left: 4 }, true);
        frame.render_widget(block, area);

        let project = template::paragraph(project, theme.highlight_invert);
        frame.render_widget(project, inner);

        Ok(())
    }

    fn height(&self, _area: Rect) -> u16 {
        3_u16
    }
}

#[derive(Copy, Clone)]
pub struct ShortcutWidget;

impl<B> Widget<B> for ShortcutWidget
where
    B: Backend,
{
    fn draw(
        &self,
        frame: &mut Frame<B>,
        theme: &Theme,
        area: Rect,
        state: &State,
    ) -> Result<(), Error> {
        let shortcuts = state.get::<Vec<String>>("app.shortcuts")?;
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

        Ok(())
    }

    fn height(&self, _area: Rect) -> u16 {
        3_u16
    }
}

#[derive(Copy, Clone)]
pub struct EmptyWidget;

impl<B> Widget<B> for EmptyWidget
where
    B: Backend,
{
    fn draw(
        &self,
        frame: &mut Frame<B>,
        _theme: &Theme,
        area: Rect,
        _state: &State,
    ) -> Result<(), Error> {
        let block = Block::default().borders(Borders::NONE);
        frame.render_widget(block, area);

        Ok(())
    }

    fn height(&self, _area: Rect) -> u16 {
        0_u16
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
    fn draw(
        &self,
        frame: &mut Frame<B>,
        theme: &Theme,
        area: Rect,
        state: &State,
    ) -> Result<(), Error> {
        let title_h = self.title.height(area);
        let context_h = self.context.height(area);
        let area_h = area.height.checked_sub(title_h + context_h).unwrap_or(0);
        let widget_h = area_h.checked_div(self.widgets.len() as u16).unwrap_or(0);

        let lengths = [
            vec![title_h],
            vec![widget_h; self.widgets.len()],
            vec![context_h],
        ]
        .concat();

        let areas = layout::split_area(area, lengths, Direction::Vertical);
        let mut areas = areas.iter();

        if let Some(area) = areas.next() {
            self.title.draw(frame, theme, *area, state)?;
        }
        for widget in &self.widgets {
            if let Some(area) = areas.next() {
                widget.draw(frame, theme, *area, state)?;
            }
        }
        if let Some(area) = areas.next() {
            self.context.draw(frame, theme, *area, state)?;
        }

        Ok(())
    }

    fn height(&self, area: Rect) -> u16 {
        area.height
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
    pub fn draw(&self, frame: &mut Frame<B>, theme: &Theme, state: &State) -> Result<(), Error> {
        let shortcut_h = self.shortcuts.height(frame.size());
        let page_h = frame.size().height - shortcut_h;
        let areas = layout::split_area(frame.size(), vec![page_h, shortcut_h], Direction::Vertical);

        self.draw_active_page(frame, theme, areas[0], state)?;
        self.shortcuts.draw(frame, theme, areas[1], state)?;

        Ok(())
    }

    pub fn draw_active_page(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) -> Result<(), Error> {
        let active = state.get::<usize>("app.page.active")?;
        if let Some(page) = self.pages.get(*active) {
            page.draw(frame, theme, area, state)?;
        }
        Ok(())
    }
}
