use std::convert::{TryFrom, TryInto};

use anyhow::{Error, Result};

use radicle_common::cobs::issue::{Issue, IssueId};
use radicle_terminal as term;

use term::tui::Action;
use term::tui::store::State;
use term::tui::window::Mode;

use super::state::Page;

type IssueList = Vec<(IssueId, Issue)>;

pub struct EnterAction;
impl Action for EnterAction {
    fn execute(&mut self, state: &mut State) -> Result<(), Error> {
        let page = state.get::<usize>("app.page.active")?;
        if let Ok(page) = Page::try_from(*page) {
            let next = match page {
                Page::Overview => {
                    state.set("project.issue.comment.active", Box::new(0_usize));
                    Some(Page::Detail)
                }
                _ => None,
            };
            if let Some(next) = next {
                let next: Result<usize, &'static str> = next.try_into();
                if let Ok(next) = next {
                    state.set("app.page.active", Box::new(next))
                }
            }
        }
        Ok(())
    }
}

pub struct EscAction;
impl Action for EscAction {
    fn execute(&mut self, state: &mut State) -> Result<(), Error> {
        let page = state.get::<usize>("app.page.active")?;
        if let Ok(page) = Page::try_from(*page) {
            let next = match page {
                Page::Edit => Some(Page::Overview),
                _ => None,
            };
            if let Some(next) = next {
                let next: Result<usize, &'static str> = next.try_into();
                if let Ok(next) = next {
                    state.set("app.page.active", Box::new(next))
                }
            }
        }
        Ok(())
    }
}

pub struct UpAction;
impl Action for UpAction {
    fn execute(&mut self, state: &mut State) -> Result<(), Error> {
        let page = state.get::<usize>("app.page.active")?;
        if let Ok(page) = Page::try_from(*page) {
            match page {
                Page::Overview => {
                    let active = state.get::<usize>("project.issue.active")?;
                    let active = match *active == 0 {
                        true => 0,
                        false => active - 1,
                    };
                    state.set("project.issue.active", Box::new(active));
                }
                Page::Detail => {
                    let active = state.get::<usize>("project.issue.comment.active")?;
                    let active = match *active == 0 {
                        true => 0,
                        false => active - 1,
                    };
                    state.set("project.issue.comment.active", Box::new(active));
                }
            }
        }
        Ok(())
    }
}

pub struct DownAction;
impl Action for DownAction {
    fn execute(&mut self, state: &mut State) -> Result<(), Error> {
        let page = state.get::<usize>("app.page.active")?;
        if let Ok(page) = Page::try_from(*page) {
            match page {
                Page::Overview => {
                    let issues = state.get::<IssueList>("project.issue.list")?;
                    let active = state.get::<usize>("project.issue.active")?;
                    let active = match *active >= issues.len() - 1 {
                        true => issues.len() - 1,
                        false => active + 1,
                    };
                    state.set("project.issue.active", Box::new(active));
                }
                Page::Detail => {
                    let issues = state.get::<IssueList>("project.issue.list")?;
                    let active = state.get::<usize>("project.issue.active")?;
                    if let Some((_, issue)) = issues.get(*active) {
                        let len = issue.comments().len() + 1;
                        let active = state.get::<usize>("project.issue.comment.active")?;
                        let active = match *active >= len - 1 {
                            true => len - 1,
                            false => active + 1,
                        };
                        state.set("project.issue.comment.active", Box::new(active));
                    }
                }
            }
        }
        Ok(())
    }
}
