use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

mod action;
mod terminal;
mod ui;

use terminal::keys::Key;
use action::{Action, Bindings};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub bindings: Bindings,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
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
    let app = Rc::new(RefCell::new(App::new(" cc-demo ")));
    terminal::exec(app, tick_rate)?;

    Ok(())
}
