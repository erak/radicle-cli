use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, ListState, Tabs, Paragraph, Wrap};
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

pub trait Draw {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect);
}

pub trait Focus {
    fn focus(&self);
}

pub struct MenuWidget<'a> {
    pub title: &'a String,
    pub tabs: &'a StatefulList<String>,
}

impl Draw for MenuWidget<'_> {
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

pub struct ActionWidget<'a> {
    pub items: Vec<&'a String>,
}

impl Draw for ActionWidget<'_> {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from("(Q)uit"),
        ];
        let block = Block::default().borders(Borders::NONE);
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

pub struct ProjectWidget {
    pub project: String,
}

impl Draw for ProjectWidget {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        // let text = vec![
        //     Spans::from("(Q)uit"),
        // ];
        let block = Block::default().borders(Borders::NONE).title(" Project ");
        let paragraph = Paragraph::new("").block(block).wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

impl Focus for ProjectWidget {
    fn focus(&self) {

    }
}

pub struct ApplicationWindow<M: Draw, W: Draw + Focus, A: Draw> {
    pub menu: M,
    pub widgets: Vec<W>,
    pub actions: A,
}

impl<M, W, A> ApplicationWindow<M, W, A>
where
    M: Draw,
    W: Draw + Focus,
    A: Draw
{
    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>) {
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

        self.actions.draw(frame, chunks[2]);
    }
}
