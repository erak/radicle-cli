use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap};
use tui::Frame;

use radicle_common::cobs::issue::{CloseReason, Issue, IssueId, State as IssueState};
use radicle_common::cobs::shared::Author;
use radicle_terminal as term;

use term::tui::store::State;
use term::tui::theme::Theme;
use term::tui::window::Widget;

type IssueList = Vec<(IssueId, Issue)>;

#[derive(Clone)]
pub struct BrowserWidget;

impl<B> Widget<B> for BrowserWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let default = 0;
        let issues = state.get::<IssueList>("project.issues.list");
        let selected = state
            .get::<usize>("project.issues.index")
            .unwrap_or(&default);

        let mut list_state = TableState::default();
        list_state.select(Some(*selected));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style)
            .border_type(theme.border_type);

        if issues.is_some() && !issues.unwrap().is_empty() {
            let items: Vec<Row> = issues
                .unwrap()
                .iter()
                .map(|issue| self.row(&issue.0, &issue.1))
                .collect();
            let table = Table::new(items)
                .block(block)
                .widths(&[
                    Constraint::Ratio(2, 32),
                    Constraint::Ratio(16, 32),
                    Constraint::Ratio(6, 32),
                ])
                .highlight_style(theme.highlight_style)
                .highlight_symbol(&theme.highlight_symbol);

            frame.render_stateful_widget(table, area, &mut list_state);
        } else {
            let text = vec![Spans::from(Span::styled(
                "No issues found",
                Style::default(),
            ))];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
    }
}

impl BrowserWidget {
    fn row(&self, _id: &IssueId, issue: &Issue) -> Row {
        let state = match issue.state {
            IssueState::Open => String::from("[Open]"),
            IssueState::Closed {
                reason: CloseReason::Solved,
            } => String::from("[Solved]"),
            IssueState::Closed {
                reason: CloseReason::Other,
            } => String::from("[Closed]"),
        };
        let author = match &issue.author {
            Author::Urn { urn } => format!("{}", urn),
            Author::Resolved(identity) => identity.name.clone(),
        };

        let cells = vec![
            Cell::from(Span::styled(state, Style::default())),
            Cell::from(Span::styled(issue.title.clone(), Style::default())),
            Cell::from(Span::styled(
                author,
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )),
        ];
        Row::new(cells)
    }
}

#[derive(Clone)]
pub struct DetailWidget;

impl<B> Widget<B> for DetailWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let default = 0;
        let issues = state.get::<IssueList>("project.issues.list");
        let selected = state
            .get::<usize>("project.issues.index")
            .unwrap_or(&default);
        let issue = issues.unwrap().get(*selected);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style)
            .border_type(theme.border_type);

        if issues.is_some() && issue.is_some() {
            let issue = issue.unwrap();

            let id = vec![
                Span::styled("ID: ", Style::default()),
                Span::styled(format!("{}", issue.0), Style::default().fg(Color::Cyan)),
            ];

            let title = vec![
                Span::styled("Title: ", Style::default()),
                Span::styled(
                    issue.1.title.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ];

            let author = match &issue.1.author {
                Author::Urn { urn } => format!("{}", urn),
                Author::Resolved(identity) => identity.name.clone(),
            };
            let author = vec![
                Span::styled("Author: ", Style::default()),
                Span::styled(
                    author,
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                ),
            ];

            let mut comment = issue
                .1
                .comment
                .body
                .lines()
                .map(|line| Spans::from(line))
                .collect::<Vec<_>>();

            let mut content = vec![
                Spans::from(title),
                Spans::from(author),
                Spans::from(id),
                Spans::from(String::new()),
            ];
            content.append(&mut comment);

            let details = Paragraph::new(content)
                .block(block)
                .wrap(Wrap { trim: true });

            frame.render_widget(details, area);
        }
    }
}
