use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

mod action;
mod state;
mod terminal;
mod ui;

use terminal::keys::Key;
use action::{Action, Bindings};

pub struct App {
    pub state: state::Context,
    pub should_quit: bool,
    pub bindings: Bindings,
}

impl App {
    pub fn new() -> App {
        App {
            state: state::Context::default(),
            should_quit: false,
            bindings: Bindings::new()
        }
    }

    pub fn on_key(&mut self, key: Key) {
        match &self.bindings.get(key) {
            Action::Quit => self.should_quit = true,
            _ => {},
        };        
    }

    pub fn on_tick(&mut self) {
        // Update state
    }
}



pub fn exec(tick_rate: Duration) -> anyhow::Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    terminal::exec(app, tick_rate)?;

    Ok(())
}
