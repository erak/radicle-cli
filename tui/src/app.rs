use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

mod action;
mod state;
mod terminal;
mod ui;

use action::{Action, Bindings};
use terminal::keys::Key;
use ui::State;

pub struct App {
    pub context: state::Context,
    pub state: State,
    pub bindings: Bindings,
}

impl App {
    pub fn new() -> App {
        App {
            context: state::Context::default(),
            state: State::new(),
            bindings: Bindings::new(),
        }
    }

    pub fn on_key(&mut self, key: Key) {
        match &self.bindings.get(key) {
            Action::Exit => self.state.request_exit(),
            _ => {}
        };
    }

    pub fn on_tick(&mut self) {
        // Update state
    }
}

pub fn exec(tick_rate: Duration) -> anyhow::Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    ui::exec(app, tick_rate)?;

    Ok(())
}
