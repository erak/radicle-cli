use std::collections::HashMap;
use std::io::stdout;
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
pub mod window;

use events::{Events, InputEvent, Key};
use store::{State, Value, STATE_RUNNING};
use window::{EmptyWidget, MenuWidget, ShortcutWidget};

pub const TICK_RATE: u64 = 200;
pub const ACTION_QUIT: &str = "action.quit";

pub type BoxedAction = Box<dyn Action>;
pub trait Action {
    fn execute(&mut self, state: &mut State);
}

pub struct QuitAction;
impl Action for QuitAction {
    fn execute(&mut self, state: &mut State) {
        if let Some(running) = state.values.get_mut(STATE_RUNNING) {
            *running = Value::Bool(false);
        }
    }
}

pub struct Application {
    bindings: HashMap<Key, String>,
    actions: HashMap<String, BoxedAction>,
    state: State,
}

impl Application {
    pub fn new() -> Self {
        Application {
            ..Default::default()
        }
    }

    pub fn execute(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run(&mut terminal);

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

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
        let window = window::ApplicationWindow {
            menu: Box::new(MenuWidget),
            content: Box::new(EmptyWidget),
            shortcuts: Box::new(ShortcutWidget),
        };
        let events = Events::new(Duration::from_millis(TICK_RATE));

        loop {
            terminal.draw(|f| window.draw(f, &self.state))?;

            match events.next()? {
                InputEvent::Input(key) => self.on_key(&key),
                InputEvent::Tick => self.on_tick(),
            };

            if let Some(running) = self.state.values.get(STATE_RUNNING) {
                if running == &Value::Bool(false) {
                    return Ok(());
                }
            }
        }
    }

    pub fn add_state(&mut self, id: &str, value: Value) {
        self.state.values.insert(id.to_owned(), value);
    }

    pub fn add_binding(&mut self, key: Key, id: &str) {
        self.bindings.insert(key, id.to_owned());
    }

    pub fn add_action(&mut self, id: &str, action: BoxedAction) {
        self.actions.insert(id.to_owned(), action);
    }

    fn on_key(&mut self, key: &Key) {
        if let Some(id) = self.bindings.get(key) {
            if let Some(action) = self.actions.get_mut(id) {
                action.execute(&mut self.state);
            }
        }
    }

    fn on_tick(&mut self) {}
}

impl Default for Application {
    fn default() -> Self {
        let mut application = Application {
            bindings: HashMap::new(),
            actions: HashMap::new(),
            state: Default::default(),
        };
        application.add_action(ACTION_QUIT, Box::new(QuitAction));
        application.add_binding(Key::Char('q'), ACTION_QUIT);
        application
    }
}
