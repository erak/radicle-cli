pub mod widgets;

use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use std::time::Duration;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use crate::app::terminal::events::{Events, InputEvent};
use crate::app::App;

use widgets::{ApplicationWindow, MenuWidget, StatefulList, View};

pub struct State {
    pub should_exit: bool,
}

impl State {
    pub fn new() -> Self {
        State { should_exit: false }
    }

    pub fn request_exit(&mut self) {
        self.should_exit = true;
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}

pub fn exec(app: Rc<RefCell<App>>, tick_rate: Duration) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Rc<RefCell<App>>,
    tick_rate: Duration,
) -> anyhow::Result<()> {
    let events = Events::new(tick_rate);

    let mut app = app.borrow_mut();

    let title = match &app.context.project {
        Some(project) => format!(" 🌱 {} ", project.name),
        None => " 🌱 ".to_owned(),
    };
    let pages = match &app.context.project {
        Some(project) => vec![
            widgets::PageWidget {
                widgets: vec![Box::new(widgets::ProjectWidget {
                    name: project.name.clone(),
                    urn: project.urn.clone(),
                    issues: project.issues.clone(),
                    patches: project.patches.clone(),
                })],
            },
            widgets::PageWidget {
                widgets: vec![Box::new(widgets::BrowserWidget {
                    issues: Box::new(StatefulList::with_items(project.issue_list.clone())),
                })],
            },
        ],
        None => vec![],
    };

    let mut window = ApplicationWindow {
        menu: Box::new(MenuWidget {
            title: title,
            views: Box::new(StatefulList::with_items(vec![
                View::Status,
                View::Issues,
            ])),
        }),
        pages: pages,
        actions: widgets::ActionWidget { items: vec![] },
    };
    loop {
        terminal.draw(|f| window.draw(f))?;

        match events.next()? {
            InputEvent::Input(key) => {
                let action = &app.bindings.get(key);

                app.on_key(key);
                window.on_action(*action);
            }
            InputEvent::Tick => app.on_tick(),
        };
        if app.state.should_exit() {
            return Ok(());
        }
    }
}
