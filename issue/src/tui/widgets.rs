use std::time::{Duration, SystemTime, UNIX_EPOCH};

use timeago;

use tui::backend::Backend;
use tui::layout::{Alignment, Direction, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{ListItem, Paragraph};
use tui::Frame;

use radicle_common::cobs::issue::{CloseReason, Issue, IssueId, State as IssueState};
use radicle_common::cobs::shared::Author;
use radicle_terminal as term;

use term::tui::layout;
use term::tui::layout::Padding;
use term::tui::store::State;
use term::tui::template;
use term::tui::theme::Theme;
use term::tui::window::Widget;

type IssueList = Vec<(IssueId, Issue)>;

#[derive(Clone)]
pub struct BrowserWidget;

impl BrowserWidget {
    fn items(&self, _id: &IssueId, issue: &Issue, theme: &Theme) -> ListItem {
        let fmt = timeago::Formatter::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timeago = Duration::from_secs(now - issue.comment.timestamp.as_secs());

        let lines = vec![
            Spans::from(Span::styled(issue.title.clone(), theme.primary)),
            Spans::from(vec![
                Span::styled(
                    issue.author.name(),
                    theme.primary_dim.add_modifier(Modifier::ITALIC),
                ),
                Span::raw(template::whitespaces(1)),
                Span::styled(
                    fmt.convert(timeago),
                    theme.ternary_dim.add_modifier(Modifier::ITALIC),
                ),
            ]),
        ];
        ListItem::new(lines)
    }
}

impl<B> Widget<B> for BrowserWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let issues = state.get::<IssueList>("project.issue.list");
        let active = state.get::<usize>("project.issue.active");

        let (block, inner) = template::block(theme, area, Padding { top: 1, left: 2 }, true);
        frame.render_widget(block, area);

        if issues.is_some() && active.is_some() && !issues.unwrap().is_empty() {
            let items: Vec<ListItem> = issues
                .unwrap()
                .iter()
                .map(|(id, issue)| self.items(&id, &issue, &theme))
                .collect();

            let (list, mut state) = template::list(items, *active.unwrap(), &theme);
            frame.render_stateful_widget(list, inner, &mut state);
        } else {
            let message = String::from("No issues found");
            let message =
                template::paragraph(&message, Style::default()).alignment(Alignment::Center);
            frame.render_widget(message, inner);
        }
    }
}

#[derive(Clone)]
pub struct DetailWidget;

impl DetailWidget {
    fn items<'a>(
        &self,
        _id: &IssueId,
        issue: &'a Issue,
        theme: &Theme,
        width: u16,
    ) -> Vec<ListItem<'a>> {
        let fmt = timeago::Formatter::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timeago = Duration::from_secs(now - issue.comment.timestamp.as_secs());

        let reactions = issue.comment.reactions.iter().collect::<Vec<_>>();
        let reactions = reactions
            .iter()
            .map(|(r, _)| format!("{} ", r.emoji))
            .collect::<String>();

        let meta = vec![
            Span::raw(template::whitespaces(0)),
            Span::styled(
                issue.author.name(),
                theme.primary_dim.add_modifier(Modifier::ITALIC),
            ),
            Span::raw(template::whitespaces(1)),
            Span::styled(
                fmt.convert(timeago),
                theme.ternary_dim.add_modifier(Modifier::ITALIC),
            ),
            Span::raw(template::whitespaces(1)),
            Span::raw(reactions),
        ];
        let root = [
            template::lines(&issue.comment.body, width, 0),
            vec![Spans::from(String::new()), Spans::from(meta)],
            vec![Spans::from(String::new())],
        ]
        .concat();
        let root = ListItem::new(root);

        let comments = issue
            .comments()
            .iter()
            .map(|comment| {
                let fmt = timeago::Formatter::new();
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let timeago = Duration::from_secs(now - comment.timestamp.as_secs());

                let reactions = comment.reactions.iter().collect::<Vec<_>>();
                let reactions = reactions
                    .iter()
                    .map(|(r, _)| format!("{} ", r.emoji))
                    .collect::<String>();
                let meta = vec![
                    Span::raw(template::whitespaces(4)),
                    Span::styled(
                        comment.author.name(),
                        theme.primary_dim.add_modifier(Modifier::ITALIC),
                    ),
                    Span::raw(template::whitespaces(1)),
                    Span::styled(
                        fmt.convert(timeago),
                        theme.ternary_dim.add_modifier(Modifier::ITALIC),
                    ),
                    Span::raw(template::whitespaces(1)),
                    Span::raw(reactions),
                ];
                let comment = [
                    template::lines(&comment.body, width, 4),
                    vec![Spans::from(String::new()), Spans::from(meta)],
                    vec![Spans::from(String::new())],
                ]
                .concat();
                ListItem::new(comment)
            })
            .collect::<Vec<_>>();

        [vec![root], comments].concat()
    }
}

impl<B> Widget<B> for DetailWidget
where
    B: Backend,
{
    fn draw(&self, frame: &mut Frame<B>, theme: &Theme, area: Rect, state: &State) {
        let issues = state.get::<IssueList>("project.issue.list");
        let active = state.get::<usize>("project.issue.active");

        let (block, inner) = template::block(theme, area, Padding { top: 1, left: 2 }, true);
        frame.render_widget(block, area);

        if issues.is_some() && active.is_some() {
            if let Some((id, issue)) = issues.unwrap().get(*active.unwrap()) {
                let active_comment = 0;
                let active = state
                    .get::<usize>("project.issue.comment.active")
                    .unwrap_or(&active_comment);

                let items = self.items(&id, &issue, &theme, inner.width);

                let (list, mut state) = template::list(items, *active, &theme);
                frame.render_stateful_widget(list, inner, &mut state);
            }
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
        let project = state.get::<String>("project.name");
        let issues = state.get::<IssueList>("project.issue.list");
        let active = state.get::<usize>("project.issue.active");

        let (block, _) = template::block(theme, area, Padding { top: 0, left: 0 }, false);
        frame.render_widget(block, area);

        if issues.is_some() && active.is_some() && project.is_some() {
            if let Some((_, issue)) = issues.unwrap().get(*active.unwrap()) {
                let project = project.unwrap();
                let author = match &issue.author {
                    Author::Urn { urn } => format!("{}", urn),
                    Author::Resolved(identity) => identity.name.clone(),
                };
                let state = match issue.state {
                    IssueState::Open => Span::styled(String::from(" ● "), theme.open),
                    IssueState::Closed {
                        reason: CloseReason::Solved,
                    } => Span::styled(String::from(" ✔ "), theme.solved),
                    IssueState::Closed {
                        reason: CloseReason::Other,
                    } => Span::styled(String::from(" ✖ "), theme.closed),
                };

                let project_w = project.len() as u16 + 2;
                let state_w = 3;
                let author_w = author.len() as u16 + 2;
                let comments_w = issue.comments().len().to_string().len() as u16 + 2;
                let title_w = area
                    .width
                    .checked_sub(project_w + state_w + comments_w + author_w)
                    .unwrap_or(0);

                let widths = vec![project_w, state_w, title_w, author_w, comments_w];
                let areas = layout::split_area(area, widths, Direction::Horizontal);

                let project = template::paragraph_styled(project, theme.highlight_invert);
                frame.render_widget(project, areas[0]);

                let state = Paragraph::new(vec![Spans::from(state)]).style(theme.bg_bright_ternary);
                frame.render_widget(state, areas[1]);

                let title = template::paragraph_styled(&issue.title, theme.bg_bright_ternary);
                frame.render_widget(title, areas[2]);

                let author = template::paragraph_styled(&author, theme.bg_bright_primary);
                frame.render_widget(author, areas[3]);

                let count = &issue.comments().len().to_string();
                let comments = template::paragraph(count, theme.bg_dark_secondary);
                frame.render_widget(comments, areas[4]);
            }
        }
    }
}
