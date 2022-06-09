use anyhow::Result;

use radicle_terminal as term;

use term::tui::store::Value;
use term::tui::Application;

use radicle_common::project;

pub fn run(project: &project::Metadata) -> Result<()> {
    let mut application = Application::new();
    application.add_state("state.title", Value::String("🌱".to_owned()));
    application.add_state("state.project.name", Value::String(project.name.clone()));
    application.execute()?;

    Ok(())
}
