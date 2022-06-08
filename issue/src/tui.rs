use anyhow::Result;

use radicle_terminal as term;

use term::tui::store::Value;
use term::tui::Application;

pub fn run() -> Result<()> {
    let mut application = Application::new();
    application.add_state("state.title", Value::String("rad-issue".to_owned()));
    application.execute()?;

    Ok(())
}
