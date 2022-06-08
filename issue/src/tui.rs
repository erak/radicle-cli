use anyhow::Result;
use radicle_terminal as term;
use term::tui::Application;

pub fn run() -> Result<()> {
    let mut application = Application::new("rad issue".to_owned());
    application.execute()?;
    
    Ok(())
}
