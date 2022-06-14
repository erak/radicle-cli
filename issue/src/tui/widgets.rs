use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Borders, ListItem, Paragraph, Wrap};
use tui::Frame;

use radicle_common::cobs::issue::{CloseReason, Issue, IssueId, State as IssueState};
use radicle_common::cobs::shared::Author;
use radicle_terminal as term;

use term::tui::store::State;
use term::tui::template;
use term::tui::template::Padding;
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

        let (block, inner) = template::block(theme, area, Padding { top: 1, left: 2 }, true);
        frame.render_widget(block, area);

        if issues.is_some() && !issues.unwrap().is_empty() {
            let items: Vec<ListItem> = issues
                .unwrap()
                .iter()
                .map(|issue| self.items(&issue.0, &issue.1, &theme))
                .collect();
            let (list, mut state) = template::list(items, *selected, &theme);

            frame.render_stateful_widget(list, inner, &mut state);
        } else {
            let message = String::from("No issues found");
            let message =
                template::paragraph(&message, Style::default()).alignment(Alignment::Center);
            frame.render_widget(message, inner);
        }
    }
}

impl BrowserWidget {
    fn items(&self, _id: &IssueId, issue: &Issue, theme: &Theme) -> ListItem {
        let author = match &issue.author {
            Author::Urn { urn } => format!("{}", urn),
            Author::Resolved(identity) => identity.name.clone(),
        };

        let lines = vec![
            Spans::from(Span::styled(issue.title.clone(), theme.primary)),
            Spans::from(Span::styled(
                format!("{}", author),
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::DarkGray),
            )),
        ];
        ListItem::new(lines)
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

        let (block, inner) = template::block(theme, area, Padding { top: 1, left: 2 }, true);
        frame.render_widget(block, area);

        if issues.is_some() && issue.is_some() {
            let issue = issue.unwrap();
            let comment = issue
                .1
                .comment
                .body
                .lines()
                .map(|line| Spans::from(line))
                .collect::<Vec<_>>();

            let details = Paragraph::new(comment).wrap(Wrap { trim: true });
            frame.render_widget(details, inner);
        }
    }
}

#[derive(Clone)]
pub struct ContextWidget;

impl<B> Widget<B> for ContextWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let default = 0;
        let default_project = String::from("-");
        let project = state
            .get::<String>("project.name")
            .unwrap_or(&default_project);
        let issues = state.get::<IssueList>("project.issues.list");
        let selected = state
            .get::<usize>("project.issues.index")
            .unwrap_or(&default);
        let issue = issues.unwrap().get(*selected);

        let (block, _) = template::block(theme, area, Padding { top: 0, left: 0 }, false);
        frame.render_widget(block, area);

        if issues.is_some() && issue.is_some() {
            let issue = issue.unwrap();
            let author = match &issue.1.author {
                Author::Urn { urn } => format!("{}", urn),
                Author::Resolved(identity) => identity.name.clone(),
            };
            let state = match issue.1.state {
                IssueState::Open => Span::styled(String::from(" ● "), theme.open),
                IssueState::Closed {
                    reason: CloseReason::Solved,
                } => Span::styled(String::from(" ✔ "), theme.solved),
                IssueState::Closed {
                    reason: CloseReason::Other,
                } => Span::styled(String::from(" ✖ "), theme.closed),
            };

            let length_project = project.len() as u16 + 2;
            let length_state = 3;
            let length_comments = issue.1.comments().len().to_string().len() as u16 + 2;
            let length_author = author.len() as u16 + 2;

            let length_others = length_project + length_state + length_comments + length_author;
            let length_title = area.width.checked_sub(length_others).unwrap_or(0);

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(length_project),
                        Constraint::Length(length_state),
                        Constraint::Length(length_title),
                        Constraint::Length(length_author),
                        Constraint::Length(length_comments),
                    ]
                    .as_ref(),
                )
                .split(area);

            let project = template::paragraph_styled(project, theme.highlight_invert);
            frame.render_widget(project, chunks[0]);
            let state = Paragraph::new(vec![Spans::from(state)]).style(theme.bg_bright_ternary);
            frame.render_widget(state, chunks[1]);

            let title = template::paragraph_styled(&issue.1.title, theme.bg_bright_ternary);
            frame.render_widget(title, chunks[2]);

            let author = template::paragraph_styled(&author, theme.bg_bright_primary);
            frame.render_widget(author, chunks[3]);

            let count = &issue.1.comments().len().to_string();
            let comments = template::paragraph(count, theme.bg_dark_secondary);
            frame.render_widget(comments, chunks[4]);
        }
    }
}
