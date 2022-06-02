use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs, Wrap};
use tui::Frame;

use crate::app::action::Action;
use crate::app::state::Issue;

pub enum View {
    Status = 0,
    Issues = 1,
    Patches = 2,
}

impl ToString for View {
    fn to_string(&self) -> String {
        match self {
            View::Status => "Status".to_owned(),
            View::Issues => "Issues".to_owned(),
            View::Patches => "Patches".to_owned(),
        }
    }
}

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

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }
}

pub trait Widget<B: Backend> {
    fn draw(&mut self, frame: &mut Frame<B>, area: Rect);
    fn on_action(&mut self, action: Action);
}

pub trait ListWidget<B, T>: Widget<B>
where
    B: Backend,
{
    fn items(&self) -> &Box<StatefulList<T>>;
}

pub struct MenuWidget {
    pub title: String,
    pub views: Box<StatefulList<View>>,
}

impl<B> Widget<B> for MenuWidget
where
    B: Backend,
{
    fn draw(&mut self, frame: &mut Frame<B>, area: Rect) {
        let titles = self
            .views
            .items
            .iter()
            .map(|view| {
                Spans::from(Span::styled(
                    view.to_string(),
                    Style::default().fg(Color::Green),
                ))
            })
            .collect();

        let views = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(self.title.clone(), Style::default())),
            )
            .highlight_style(Style::default().fg(Color::Magenta))
            .select(self.views.state.selected().unwrap_or(0));

        frame.render_widget(views, area);
    }

    fn on_action(&mut self, action: Action) {
        match action {
            Action::MenuStatus => self.views.select(0),
            Action::MenuIssues => self.views.select(1),
            // Action::MenuPatches => self.views.select(2),
            _ => {}
        }
    }
}

impl<B> ListWidget<B, View> for MenuWidget
where
    B: Backend,
{
    fn items(&self) -> &Box<StatefulList<View>> {
        &self.views
    }
}

pub struct PageWidget<B: Backend> {
    pub widgets: Vec<Box<dyn Widget<B>>>,
}

impl<B> Widget<B> for PageWidget<B>
where
    B: Backend,
{
    fn draw(&mut self, frame: &mut Frame<B>, area: Rect) {
        let constraints = self
            .widgets
            .iter()
            .map(|_| Constraint::Percentage(100 / self.widgets.len() as u16))
            .collect::<Vec<_>>();

        let chunks = Layout::default().constraints(constraints).split(area);

        for widget in &mut self.widgets {
            if let Some(rect) = chunks.iter().next() {
                widget.draw(frame, *rect);
            }
        }
    }

    fn on_action(&mut self, action: Action) {
        for widget in &mut self.widgets {
            widget.on_action(action);
        }
    }
}

pub struct ActionWidget<'a> {
    pub items: Vec<&'a String>,
}

impl<B> Widget<B> for ActionWidget<'_>
where
    B: Backend,
{
    fn draw(&mut self, frame: &mut Frame<B>, area: Rect) {
        let text = vec![Spans::from("(Q)uit")];
        let block = Block::default().borders(Borders::NONE);
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn on_action(&mut self, _action: Action) {
        // handle action that are of interest for this widget
    }
}

pub struct ProjectWidget {
    pub name: String,
    pub urn: String,
    pub issues: (usize, usize),
    pub patches: (usize, usize),
}

impl<B> Widget<B> for ProjectWidget
where
    B: Backend,
{
    fn draw(&mut self, frame: &mut Frame<B>, area: Rect) {
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

    fn on_action(&mut self, _action: Action) {
        // handle action that are of interest for this widget
    }
}

pub struct BrowserWidget<T> {
    pub issues: Box<StatefulList<T>>,
}

impl<B> Widget<B> for BrowserWidget<Issue>
where
    B: Backend,
{
    fn draw(&mut self, frame: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self
            .issues
            .items
            .iter()
            .map(|issue| {
                let mut lines = vec![Spans::from(Span::styled(
                    issue.title.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ))];
                lines.push(Spans::from(Span::styled(
                    format!("└── {}", issue.author.clone()),
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::DarkGray),
                )));
                ListItem::new(lines)
            })
            .collect();
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Browser "))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Magenta),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(items, area, &mut self.issues.state);
        // frame.render_widget(items, area);
    }

    fn on_action(&mut self, action: Action) {
        match action {
            Action::BrowseUp => self.issues.previous(),
            Action::BrowseDown => self.issues.next(),
            _ => {}
        }
    }
}

impl<B> ListWidget<B, Issue> for BrowserWidget<Issue>
where
    B: Backend,
{
    fn items(&self) -> &Box<StatefulList<Issue>> {
        &self.issues
    }
}

pub struct ApplicationWindow<'a, B: Backend> {
    pub menu: Box<dyn ListWidget<B, View>>,
    pub pages: Vec<PageWidget<B>>,
    pub actions: ActionWidget<'a>,
}

impl<'a, B> ApplicationWindow<'a, B>
where
    B: Backend,
{
    pub fn draw(&mut self, frame: &mut Frame<B>) {
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
        if let Some(page) = self.pages.get_mut(self.menu.items().selected().unwrap_or(0)) {
            page.draw(frame, chunks[1]);
        }
        self.actions.draw(frame, chunks[2]);
    }

    pub fn on_action(&mut self, action: Action) {
        self.menu.on_action(action);

        if let Some(page) = self.pages.get_mut(self.menu.items().selected().unwrap_or(0)) {
            page.on_action(action);
        }
    }
}
