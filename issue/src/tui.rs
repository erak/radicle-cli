use std::rc::Rc;

use anyhow::Result;

use radicle_common::cobs::issue::{Issue, IssueId};
use radicle_common::project::Metadata;
use radicle_terminal as term;

use term::tui::window::PageWidget;
use term::tui::Application;

mod widgets;

use widgets::BrowserWidget;

type IssueList = Vec<(IssueId, Issue)>;

pub fn run(project: &Metadata, issues: IssueList) -> Result<()> {
    let mut app = Application::new().state(vec![
        ("app.title", Box::new("🌱".to_owned())),
        ("project.name", Box::new(project.name.clone())),
        ("project.issues.list", Box::new(issues)),
    ]);

    let pages = vec![PageWidget {
        widgets: vec![Rc::new(BrowserWidget)],
    }];
    app.execute(pages)?;

    Ok(())
}
