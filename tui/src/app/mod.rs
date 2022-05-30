use std::time::Duration;

mod terminal;
mod ui;

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
        }
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn on_tick(&mut self) {
        // Update state
    }
}

pub fn exec(tick_rate: Duration) -> anyhow::Result<()> {
    let mut app = App::new(" cc-demo ");
    terminal::exec(app, tick_rate)?;

    Ok(())
}
