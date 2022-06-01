pub mod widgets;

use crate::app::App;
use widgets::{ActionWidget, ApplicationWindow, MenuWidget, ProjectWidget, StatefulList};

use tui::backend::Backend;
use tui::Frame;

pub enum View {
    Status,
    Issues,
    Patches,
}

pub struct State {
    menu: StatefulList<String>,
    should_exit: bool,
}

impl State {
    pub fn new(menu: StatefulList<String>) -> Self {
        State {
            menu: menu,
            should_exit: false,
        }
    }

    pub fn select_view(&mut self, view: View) {
        match view {
            View::Status => self.menu.state.select(Some(0)),
            View::Issues => self.menu.state.select(Some(1)),
            View::Patches => self.menu.state.select(Some(2)),
        }
    }

    pub fn view(&self) -> View {
        match self.menu.state.selected() {
            Some(0) => View::Status,
            Some(1) => View::Issues,
            Some(2) => View::Patches,
            _ => View::Status,
        }
    }

    pub fn request_exit(&mut self) {
        self.should_exit = true;
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let title = match &app.context.project {
        Some(project) => format!(" 🌱 {} ", project.name),
        None => " 🌱 ".to_owned(),
    };

    let window = ApplicationWindow {
        menu: MenuWidget {
            title: &title,
            tabs: &mut app.state.menu,
        },
        widgets: vec![ProjectWidget {
            project: "demo".to_owned(),
        }],
        actions: ActionWidget { items: vec![] },
    };

    window.draw(frame);
}
