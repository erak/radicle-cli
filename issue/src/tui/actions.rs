use radicle_common::cobs::issue::{Issue, IssueId};
use radicle_terminal as term;

use term::tui::store::State;
use term::tui::Action;

type IssueList = Vec<(IssueId, Issue)>;

pub struct BrowseUpAction;
impl Action for BrowseUpAction {
    fn execute(&mut self, state: &mut State) {
        if let Some(issues) = state.get::<IssueList>("project.issues.list") {
            if let Some(index) = state.get::<usize>("project.issues.index") {
                let select = match *index == 0 {
                    true => issues.len() - 1,
                    false => index - 1,
                };
                state.set("project.issues.index", Box::new(select));
            }
        }
    }
}

pub struct BrowseDownAction;
impl Action for BrowseDownAction {
    fn execute(&mut self, state: &mut State) {
        if let Some(issues) = state.get::<IssueList>("project.issues.list") {
            if let Some(index) = state.get::<usize>("project.issues.index") {
                let select = match *index >= issues.len() - 1 {
                    true => 0,
                    false => index + 1,
                };
                state.set("project.issues.index", Box::new(select));
            }
        }
    }
}
