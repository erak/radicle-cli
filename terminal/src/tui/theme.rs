use tui::style::{Color, Modifier, Style};
use tui::widgets::BorderType;

pub struct Theme {
    pub border_style: Style,
    pub border_type: BorderType,
    pub highlight_style: Style,
    pub highlight_symbol: String,
}

impl Theme {
    pub fn classic() -> Self {
        Theme {
            border_style: Style::default(),
            border_type: BorderType::Plain,
            highlight_style: Style::default().bg(Color::Yellow).fg(Color::White),
            highlight_symbol: String::new()
        }
    }

    pub fn modern() -> Self {
        Theme {
            border_style: Style::default().fg(Color::Rgb(238, 111, 248)),
            border_type: BorderType::Rounded,
            highlight_style: Style::default()
                .fg(Color::Rgb(148, 176, 77))
                .add_modifier(Modifier::BOLD),
            highlight_symbol: String::from("> "),
        }
    }
}