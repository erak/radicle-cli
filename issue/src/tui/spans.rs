use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tui::style::Modifier;
use tui::text::Span;

use radicle_common::cobs::shared::{Comment};
use radicle_terminal as term;

use term::tui::strings;
use term::tui::theme::Theme;

pub fn comment_meta<'a, R>(comment: &Comment<R>, theme: &Theme, indent: u16) -> Vec<Span<'a>> {
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

    vec![
        Span::raw(strings::whitespaces(indent)),
        Span::styled(
            comment.author.name(),
            theme.primary_dim.add_modifier(Modifier::ITALIC),
        ),
        Span::raw(strings::whitespaces(1)),
        Span::styled(
            fmt.convert(timeago),
            theme.ternary_dim.add_modifier(Modifier::ITALIC),
        ),
        Span::raw(strings::whitespaces(1)),
        Span::raw(reactions),
    ]
}
