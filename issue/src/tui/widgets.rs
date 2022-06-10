use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};
use tui::Frame;

use radicle_common::cobs::issue::{CloseReason, Issue, IssueId, State as IssueState};
use radicle_common::cobs::shared::Author;
use radicle_terminal as term;

use term::tui::store::State;
use term::tui::window::Widget;

type IssueList = Vec<(IssueId, Issue)>;

#[derive(Clone)]
pub struct BrowserWidget;

impl<B> Widget<B> for BrowserWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, area: Rect, state: &State) {
        let block = Block::default().borders(Borders::ALL).title(" Issues ");

        let mut list_state = TableState::default();
        list_state.select(Some(0));

        let issues = state.get::<IssueList>("project.issues.list");
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
                .highlight_style(
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::White),
                );

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
            Cell::from(Span::styled(
                issue.title.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            )),
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