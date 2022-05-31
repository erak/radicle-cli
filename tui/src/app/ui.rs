pub mod widgets;

use crate::app::App;
use widgets::{ApplicationWindow, MenuWidget, Window, StatefulList};

use tui::backend::Backend;
use tui::Frame;

pub struct State {
    pub menu: StatefulList<String>,
    pub should_quit: bool,
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let title = match &app.context.project {
        Some(project) => format!(" rad-tui({}) ", project.name),
        None => " rad-tui ".to_owned(),
    };

    let menu = MenuWidget {
        title: &title,
        tabs: &mut app.state.menu
    };

    let window = ApplicationWindow {
        menu: menu
    };

    window.draw(frame);
}
