use anyhow::Result;

use radicle_terminal as term;

use term::tui::Application;
use term::tui::store::Value;
use term::tui::store::{STATE_TITLE};


pub fn run() -> Result<()> {
    let mut application = Application::new();

    application.add_state(STATE_TITLE, Value::String("rad-issue".to_owned()));
    application.execute()?;
    
    Ok(())
}
