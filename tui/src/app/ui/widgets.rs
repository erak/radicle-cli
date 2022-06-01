use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, ListState, Paragraph, Row, Table, Tabs, Wrap};
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

pub trait Index {
    fn index(&self) -> Option<usize>;
}

pub trait Group {
    fn group<B: Backend>(&self) -> Vec<Self>
    where
        Self: std::marker::Sized;
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

impl Index for MenuWidget<'_> {
    fn index(&self) -> Option<usize> {
        self.tabs.state.selected()
    }
}

pub struct PageWidget<W: Draw + Focus> {
    pub widgets: Vec<W>,
}

impl<W: Draw + Focus> Draw for PageWidget<W> {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let constraints = self
            .widgets
            .iter()
            .map(|_| Constraint::Percentage(100 / self.widgets.len() as u16))
            .collect::<Vec<_>>();

        let chunks = Layout::default().constraints(constraints).split(area);

        for widget in &self.widgets {
            if let Some(rect) = chunks.iter().next() {
                widget.draw(frame, *rect)
            }
        }
    }
}

pub struct ActionWidget<'a> {
    pub items: Vec<&'a String>,
}

impl Draw for ActionWidget<'_> {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let text = vec![Spans::from("(Q)uit")];
        let block = Block::default().borders(Borders::NONE);
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

pub struct ProjectWidget {
    pub name: String,
    pub urn: String,
    pub issues: (usize, usize),
    pub patches: (usize, usize),
}

impl Draw for ProjectWidget {
    fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let rows = vec![
            Row::new(vec![Cell::from(Span::from("")), Cell::from(Span::from(""))]),
            Row::new(vec![
                Cell::from(Span::from(" Project")),
                Cell::from(Span::styled(
                    self.urn.clone(),
                    Style::default().fg(Color::Blue),
                )),
            ]),
            Row::new(vec![
                Cell::from(Span::from(" └── Name")),
                Cell::from(Span::styled(
                    self.name.clone(),
                    Style::default().fg(Color::Blue),
                )),
            ]),
            Row::new(vec![Cell::from(Span::from("")), Cell::from(Span::from(""))]),
            Row::new(vec![
                Cell::from(Span::from(" Issues")),
                Cell::from(Span::styled(
                    format!("{} Open, {} Closed", self.issues.0, self.issues.1),
                    Style::default().fg(Color::Blue),
                )),
            ]),
            Row::new(vec![
                Cell::from(Span::from(" Patches")),
                Cell::from(Span::styled(
                    format!("{} Open, {} Merged", self.patches.0, self.patches.1),
                    Style::default().fg(Color::Blue),
                )),
            ]),
        ];
        let table = Table::new(rows)
            .block(Block::default().title(" Status ").borders(Borders::ALL))
            .widths(&[Constraint::Ratio(1, 8), Constraint::Ratio(7, 8)]);
        frame.render_widget(table, area);
    }
}

impl Focus for ProjectWidget {
    fn focus(&self) {}
}

pub struct ApplicationWindow<M: Draw + Index, P: Draw, A: Draw> {
    pub menu: M,
    pub pages: Vec<P>,
    pub actions: A,
}

impl<M, P, A> ApplicationWindow<M, P, A>
where
    M: Draw + Index,
    P: Draw,
    A: Draw,
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
        if let Some(page) = self.pages.get(self.menu.index().unwrap_or(0)) {
            page.draw(frame, chunks[1]);
        }
        self.actions.draw(frame, chunks[2]);
    }
}
