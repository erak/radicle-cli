use std::rc::Rc;

use anyhow::Result;

use radicle_common::cobs::issue::{Issue, IssueId};
use radicle_common::project::Metadata;
use radicle_terminal as term;

use term::tui::events::Key;
use term::tui::theme::Theme;
use term::tui::window::PageWidget;
use term::tui::Application;

mod actions;
mod widgets;

use actions::{BrowseDownAction, BrowseUpAction};
use widgets::{BrowserWidget, DetailWidget, ContextWidget};

type IssueList = Vec<(IssueId, Issue)>;

pub const ACTION_BROWSE_UP: &str = "action.browse.up";
pub const ACTION_BROWSE_DOWN: &str = "action.browse.down";

pub fn run(project: &Metadata, issues: IssueList) -> Result<()> {
    let mut app = Application::new()
        .state(vec![
            ("app.title", Box::new("Issues".to_owned())),
            ("project.name", Box::new(project.name.clone())),
            ("project.issues.list", Box::new(issues)),
            ("project.issues.index", Box::new(0_usize)),
        ])
        .bindings(vec![
            (Key::Up, ACTION_BROWSE_UP),
            (Key::Down, ACTION_BROWSE_DOWN),
        ])
        .actions(vec![
            (ACTION_BROWSE_UP, Box::new(BrowseUpAction)),
            (ACTION_BROWSE_DOWN, Box::new(BrowseDownAction)),
        ]);

    let pages = vec![PageWidget {
        widgets: vec![Rc::new(BrowserWidget), Rc::new(DetailWidget)],
        context: Rc::new(ContextWidget),
    }];

    let theme = Theme::default_dark();
    app.execute(pages, &theme)?;

    Ok(())
}
