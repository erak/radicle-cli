use textwrap::Options;

use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use super::layout;
use super::layout::Padding;
use super::theme::Theme;

pub fn block(theme: &Theme, area: Rect, padding: Padding, borders: bool) -> (Block, Rect) {
    let borders = match borders {
        true => theme.border.borders,
        false => Borders::NONE,
    };
    let block = Block::default()
        .borders(borders)
        .border_style(theme.border.style)
        .border_type(theme.border.border_type);
    let padding = match theme.border.borders {
        Borders::NONE => padding,
        _ => Padding {
            top: padding.top,
            left: padding.left,
        },
    };

    let inner = layout::inner_area(area, padding);
    (block, inner)
}

pub fn paragraph<'a>(text: &'a String, style: Style) -> Paragraph<'a> {
    let text = format!("{:1}{}{:1}", "", text, "");
    let text = Span::styled(text, style);

    Paragraph::new(vec![Spans::from(text)])
}

pub fn paragraph_styled<'a>(text: &'a String, style: Style) -> Paragraph<'a> {
    let text = format!("{:1}{}{:1}", "", text, "");
    let text = Span::styled(text, style);

    Paragraph::new(vec![Spans::from(text)]).style(style)
}

pub fn list<'a>(
    items: Vec<ListItem<'a>>,
    selected: usize,
    theme: &'a Theme,
) -> (List<'a>, ListState) {
    let mut state = ListState::default();
    state.select(Some(selected));

    let items = List::new(items)
        .highlight_style(theme.highlight)
        .highlight_symbol(&theme.list.symbol)
        .repeat_highlight_symbol(true);

    (items, state)
}

pub fn lines<'a>(content: &'a String, width: u16, indent: u16) -> Vec<Spans<'a>> {
    let wrap = width.checked_sub(indent).unwrap_or(80);
    let whitespaces = whitespaces(indent);

    let options = Options::new(wrap as usize)
        .initial_indent(&whitespaces)
        .subsequent_indent(&whitespaces);

    let lines = textwrap::wrap(content, options);
    lines
        .iter()
        .map(|line| Spans::from(Span::styled(String::from(line.clone()), Style::default().fg(Color::Rgb(200, 200, 200)))))
        .collect::<Vec<_>>()
}

pub fn whitespaces(indent: u16) -> String {
    match String::from_utf8(vec![b' '; indent as usize]) {
        Ok(spaces) => spaces,
        Err(_) => String::new(),
    }
}
