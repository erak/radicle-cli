use std::io::{stdout, Stdout};
use std::rc::Rc;
use std::time::Duration;

use anyhow::{Error, Result};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

pub mod editor;
pub mod events;
pub mod layout;
pub mod spans;
pub mod store;
pub mod strings;
pub mod template;
pub mod theme;
pub mod window;

use events::{Events, InputEvent};
use store::{State, Value};
use theme::Theme;
use window::{Mode, PageWidget, ShortcutWidget};

pub const TICK_RATE: u64 = 200;

pub type Update = dyn Fn(&mut State, &InputEvent) -> Result<(), Error>;

pub struct Application<'a> {
    update: &'a Update,
    state: State,
}

impl<'a> Application<'a> {
    pub fn new(update: &'a Update) -> Self {
        let application = Application {
            update: update,
            state: Default::default(),
        };
        application
            .state(vec![
                ("app.running", Box::new(true)),
                ("app.page.index", Box::new(0_usize)),
                ("app.mode", Box::new(Mode::Normal)),
            ])
    }

    pub fn execute(
        &mut self,
        pages: Vec<PageWidget<CrosstermBackend<Stdout>>>,
        theme: &Theme,
    ) -> anyhow::Result<(), Error> {
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
    ) -> anyhow::Result<(), Error> {
        let window = window::ApplicationWindow {
            pages: pages,
            shortcuts: Rc::new(ShortcutWidget),
        };
        let events = Events::new(Duration::from_millis(TICK_RATE));

        loop {
            let mut error: Option<Error> = None;
            terminal.draw(|f| {
                error = window.draw(f, theme, &self.state).err();
            })?;
            if let Some(err) = error {
                return Err(err.into());
            }

            let event = events.next()?;
            self.on_input_event(&event)?;

            let running = self.state.get::<bool>("app.running")?;
            if !running {
                return Ok(());
            }
        }
    }

    pub fn state(mut self, props: Vec<(&str, Value)>) -> Self {
        for prop in props {
            self.state.set(&prop.0, prop.1);
        }
        self
    }

    pub fn on_input_event(&mut self, event: &InputEvent) -> Result<(), Error> {
        (self.update)(&mut self.state, event)?;
        Ok(())
    }
}
