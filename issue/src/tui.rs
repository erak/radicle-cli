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
        .set("state.title", Box::new("🌱".to_owned()));
    application
        .state()
        .set("state.project.name", Box::new(project.name.clone()));
    application
        .state()
        .set("state.issues.list", Box::new(project.name.clone()));

    let pages = vec![PageWidget {
        widgets: vec![Rc::new(EmptyWidget)],
    }];
    application.execute(pages)?;

    Ok(())
}
