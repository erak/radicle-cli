use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, ListState, Tabs};
use tui::Frame;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(0));

        StatefulList {
            state: state,
            items,
        }
    }
}

pub trait Widget {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect);
}

pub trait Window {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>);
}

pub struct MenuWidget<'a> {
    pub title: &'a String,
    pub tabs: &'a mut StatefulList<String>,
}

impl<'a> Widget for MenuWidget<'a> {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let titles = self
            .tabs
            .items
            .iter()
            .map(|tab| Spans::from(Span::styled(tab, Style::default().fg(Color::Green))))
            .collect();

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(self.title, Style::default())),
            )
            .highlight_style(Style::default().fg(Color::Magenta))
            .select(self.tabs.state.selected().unwrap_or(0));
        frame.render_widget(tabs, area);
    }
}

pub struct ApplicationWindow<'a> {
    pub menu: MenuWidget<'a>,
}

impl<'a> Window for ApplicationWindow<'a> {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Max(2),
                ]
                .as_ref(),
            )
            .split(frame.size());

        self.menu.draw(frame, chunks[0]);
    }
}
