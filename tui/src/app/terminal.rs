pub mod events;
pub mod keys;

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

use crate::app::{App, ui};
use events::{Events, InputEvent};
use keys::Key;

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
    
    loop {
        let mut app = app.borrow_mut();

        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            InputEvent::Input(Key::Char('q')) => app.on_quit(),
            InputEvent::Tick => app.on_tick(),
            _ => {},
        };
        
        if app.should_quit {
            return Ok(());
        }
    }
}
