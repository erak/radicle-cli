use std::collections::HashMap;
use std::io::{stdout, Stdout};
use std::rc::Rc;
use std::time::Duration;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

pub mod events;
pub mod store;
pub mod theme;
pub mod template;
pub mod window;

use events::{Events, InputEvent, Key};
use store::{State, Value};
use theme::Theme;
use window::{TitleWidget, PageWidget, ShortcutWidget};

pub const TICK_RATE: u64 = 200;
pub const ACTION_QUIT: &str = "action.quit";

pub type BoxedAction = Box<dyn Action>;
pub trait Action {
    fn execute(&mut self, state: &mut State);
}

pub struct QuitAction;
impl Action for QuitAction {
    fn execute(&mut self, state: &mut State) {
        state.set("app.running", Box::new(false));
    }
}

pub struct Bindings {
    entries: HashMap<Key, String>,
}

impl Bindings {
    pub fn add(&mut self, key: Key, id: &str) {
        self.entries.insert(key, id.to_owned());
    }

    pub fn get(&self, key: Key) -> Option<&String> {
        self.entries.get(&key)
    }
}

pub struct Actions {
    entries: HashMap<String, BoxedAction>,
}

impl Actions {
    pub fn add(&mut self, id: &str, action: BoxedAction) {
        self.entries.insert(id.to_owned(), action);
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut BoxedAction> {
        self.entries.get_mut(&id.to_owned())
    }
}

pub struct Application {
    bindings: Bindings,
    actions: Actions,
    state: State,
}

impl<'a> Application {
    pub fn new() -> Self {
        Application {
            ..Default::default()
        }
    }

    pub fn execute(
        &mut self,
        pages: Vec<PageWidget<CrosstermBackend<Stdout>>>,
        theme: &Theme,
    ) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run(&mut terminal, pages, theme);

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
        &mut self,
        terminal: &mut Terminal<B>,
        pages: Vec<PageWidget<B>>,
        theme: &Theme,
    ) -> anyhow::Result<()> {
        let window = window::ApplicationWindow {
            title: Rc::new(TitleWidget),
            pages: pages,
            shortcuts: Rc::new(ShortcutWidget),
        };
        let events = Events::new(Duration::from_millis(TICK_RATE));

        loop {
            terminal.draw(|f| window.draw(f, theme, &self.state))?;

            match events.next()? {
                InputEvent::Input(key) => self.on_key(&key),
                InputEvent::Tick => self.on_tick(),
            };

            if let Some(running) = self.state.get::<bool>("app.running") {
                if !running {
                    return Ok(());
                }
            }
        }
    }

    pub fn state(mut self, props: Vec<(&str, Value)>) -> Self {
        for prop in props {
            self.state.set(&prop.0, prop.1);
        }
        self
    }

    pub fn bindings(mut self, bindings: Vec<(Key, &str)>) -> Self {
        for binding in bindings {
            self.bindings.add(binding.0, binding.1);
        }
        self
    }

    pub fn actions(mut self, actions: Vec<(&str, BoxedAction)>) -> Self {
        for action in actions {
            self.actions.add(action.0, action.1);
        }
        self
    }

    fn on_key(&mut self, key: &Key) {
        if let Some(id) = self.bindings.get(*key) {
            if let Some(action) = self.actions.get_mut(id) {
                action.execute(&mut self.state);
            }
        }
    }

    fn on_tick(&mut self) {}
}

impl Default for Application {
    fn default() -> Self {
        let application = Application {
            bindings: Bindings {
                entries: HashMap::new(),
            },
            actions: Actions {
                entries: HashMap::new(),
            },
            state: Default::default(),
        };
        application
            .state(vec![
                ("app.running", Box::new(true)),
                ("app.page.index", Box::new(0_usize)),
                ("app.shortcuts", Box::new(vec![String::from("q quit"), String::from("? help")])),
            ])
            .bindings(vec![(Key::Char('q'), ACTION_QUIT)])
            .actions(vec![(ACTION_QUIT, Box::new(QuitAction))])
    }
}
