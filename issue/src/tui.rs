use std::rc::Rc;

use anyhow::Result;

use radicle_terminal as term;

use term::tui::store::Value;
use term::tui::window::{EmptyWidget, PageWidget};
use term::tui::Application;

use radicle_common::project;

pub fn run(project: &project::Metadata) -> Result<()> {
    let mut application = Application::new();
    application
        .state()
        .set("state.title", Value::String("🌱".to_owned()));
    application
        .state()
        .set("state.project.name", Value::String(project.name.clone()));

    let pages = vec![PageWidget {
        widgets: vec![Rc::new(EmptyWidget)],
    }];
    application.execute(pages)?;

    Ok(())
}
