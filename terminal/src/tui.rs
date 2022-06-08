use std::io::stdout;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

pub mod window;

pub struct Application {
    pub title: String,
}
impl Application {
    pub fn new(title: String) -> Self {
        Application { title: title }
    }
    pub fn execute(self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = &self.run(&mut terminal);

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
        let mut window = window::ApplicationWindow {
            title: self.title.clone(),
        };
        let events = Events::new(Duration::from_millis(TICK_RATE));

        loop {
            terminal.draw(|f| window.draw(f))?;

            match events.next()? {
            };

        }
    }
}
