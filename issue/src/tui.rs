use anyhow::Result;

use radicle_terminal as term;

pub fn run() -> Result<()> {
    let application = term::tui::Application::new("issues".to_owned());
    application.execute()?;
    
    Ok(())
}
