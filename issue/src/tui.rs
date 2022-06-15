use std::rc::Rc;

use anyhow::Result;

use radicle_common::cobs::issue::{Issue, IssueId};
use radicle_common::project::Metadata;
use radicle_terminal as term;

use term::tui::events::Key;
use term::tui::theme::Theme;
use term::tui::window::{EmptyWidget, PageWidget, TitleWidget};
use term::tui::Application;

mod actions;
mod state;
mod widgets;

use actions::{EnterAction, EscAction, DownAction, UpAction};
use state::{Page, Tab};
use widgets::{BrowserWidget, ContextWidget, DetailWidget};

type IssueList = Vec<(IssueId, Issue)>;

pub const ACTION_ENTER: &str = "action.enter";
pub const ACTION_ESC: &str = "action.esc";
pub const ACTION_UP: &str = "action.up";
pub const ACTION_DOWN: &str = "action.down";

pub fn run(project: &Metadata, issues: IssueList) -> Result<()> {
    let mut app = Application::new()
        .state(vec![
            ("app.title", Box::new("Issues".to_owned())),
            ("app.page.selected", Box::new(Page::Overview as usize)),
            ("app.tab.selected", Box::new(Tab::Open as usize)),
            ("project.name", Box::new(project.name.clone())),
            ("project.issues.list", Box::new(issues)),
            ("project.issues.index", Box::new(0_usize)),
        ])
        .bindings(vec![
            (Key::Enter, ACTION_ENTER),
            (Key::Esc, ACTION_ESC),
            (Key::Up, ACTION_UP),
            (Key::Down, ACTION_DOWN),
        ])
        .actions(vec![
            (ACTION_ENTER, Box::new(EnterAction)),
            (ACTION_ESC, Box::new(EscAction)),
            (ACTION_UP, Box::new(UpAction)),
            (ACTION_DOWN, Box::new(DownAction)),
        ]);

    let pages = vec![
        PageWidget {
            title: Rc::new(TitleWidget),
            widgets: vec![Rc::new(BrowserWidget)],
            context: Rc::new(EmptyWidget),
        },
        PageWidget {
            title: Rc::new(EmptyWidget),
            widgets: vec![Rc::new(DetailWidget)],
            context: Rc::new(ContextWidget),
        },
    ];

    let theme = Theme::default_dark();
    app.execute(pages, &theme)?;

    Ok(())
}
