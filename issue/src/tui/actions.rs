use std::convert::{TryFrom, TryInto};

use radicle_common::cobs::issue::{Issue, IssueId};
use radicle_terminal as term;

use term::tui::store::State;
use term::tui::Action;

use super::state::{Page, Tab};

type IssueList = Vec<(IssueId, Issue)>;

pub struct EnterAction;
impl Action for EnterAction {
    fn execute(&mut self, state: &mut State) {
        if let Some(page) = state.get::<usize>("app.page.selected") {
            if let Ok(page) = Page::try_from(*page) {
                let next = match page {
                    Page::Overview => Some(Page::Edit),
                    _ => None,
                };
                if let Some(next) = next {
                    let next: Result<usize, &'static str> = next.try_into();
                    if let Ok(next) = next {
                        state.set("app.page.selected", Box::new(next))
                    }
                }
            }
        }
    }
}

pub struct EscAction;
impl Action for EscAction {
    fn execute(&mut self, state: &mut State) {
        if let Some(page) = state.get::<usize>("app.page.selected") {
            if let Ok(page) = Page::try_from(*page) {
                let next = match page {
                    Page::Edit => Some(Page::Overview),
                    _ => None,
                };
                if let Some(next) = next {
                    let next: Result<usize, &'static str> = next.try_into();
                    if let Ok(next) = next {
                        state.set("app.page.selected", Box::new(next))
                    }
                }
            }
        }
    }
}

pub struct UpAction;
impl Action for UpAction {
    fn execute(&mut self, state: &mut State) {
        if let Some(page) = state.get::<usize>("app.page.selected") {
            if let Ok(page) = Page::try_from(*page) {
                match page {
                    Page::Overview => {
                        if let Some(issues) = state.get::<IssueList>("project.issues.list") {
                            if let Some(index) = state.get::<usize>("project.issues.index") {
                                let select = match *index == 0 {
                                    true => issues.len() - 1,
                                    false => index - 1,
                                };
                                state.set("project.issues.index", Box::new(select));
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }
}

pub struct DownAction;
impl Action for DownAction {
    fn execute(&mut self, state: &mut State) {
        if let Some(page) = state.get::<usize>("app.page.selected") {
            if let Ok(page) = Page::try_from(*page) {
                match page {
                    Page::Overview => {
                        if let Some(issues) = state.get::<IssueList>("project.issues.list") {
                            if let Some(index) = state.get::<usize>("project.issues.index") {
                                let select = match *index >= issues.len() - 1 {
                                    true => 0,
                                    false => index + 1,
                                };
                                state.set("project.issues.index", Box::new(select));
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }
}
