use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::rc::Rc;

use anyhow::{Error, Result};
use lazy_static::lazy_static;

use tui::layout::Rect;

use radicle_common::cobs::issue::{Issue, IssueId};
use radicle_common::project::Metadata;
use radicle_terminal as term;

use term::tui::editor::Editor;
use term::tui::events::{InputEvent, Key};
use term::tui::store::State;
use term::tui::theme::Theme;
use term::tui::window::Mode;
use term::tui::window::{EditorWidget, EmptyWidget, PageWidget, TitleWidget};
use term::tui::Application;

mod spans;
mod state;
mod widgets;

use state::{Page, Tab};
use widgets::{BrowserWidget, ContextWidget, DetailWidget};

type IssueList = Vec<(IssueId, Issue)>;

#[derive(Clone, Eq, PartialEq)]
pub enum InternalCall {
    New,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Action {
    Quit,
    Up,
    Down,
    New,
    Comment,
}

lazy_static! {
    static ref BINDINGS: HashMap<Key, Action> = [
        (Key::Char('q'), Action::Quit),
        (Key::Up, Action::Up),
        (Key::Down, Action::Down),
        (Key::Char('n'), Action::New),
        (Key::Char('c'), Action::Comment),
    ]
    .iter()
    .cloned()
    .collect();
}

pub fn run(project: &Metadata, issues: IssueList) -> Result<Option<InternalCall>, Error> {
    let internal_call: Option<InternalCall> = None;
    let mut app = Application::new(&update).state(vec![
        ("app.title", Box::new("Issues".to_owned())),
        ("app.mode", Box::new(Mode::Normal)),
        ("app.call.internal", Box::new(internal_call)),
        ("app.page.active", Box::new(Page::Overview as usize)),
        ("app.tab.active", Box::new(Tab::Open as usize)),
        ("app.editor", Box::new(Editor::new())),
        ("app.editor.text", Box::new(String::new())),
        ("project.name", Box::new(project.name.clone())),
        ("project.issue.list", Box::new(issues)),
        ("project.issue.active", Box::new(0_usize)),
        ("project.issue.comment.active", Box::new(0_usize)),
        (
            "app.shortcuts",
            Box::new(vec![
                String::from("c comment"),
                String::from("q quit"),
                String::from("? help"),
            ]),
        ),
    ]);

    let pages = vec![
        PageWidget {
            title: Rc::new(TitleWidget),
            widgets: vec![Rc::new(BrowserWidget)],
            context: Rc::new(EmptyWidget),
            editor: Rc::new(EditorWidget),
        },
        PageWidget {
            title: Rc::new(EmptyWidget),
            widgets: vec![Rc::new(DetailWidget)],
            context: Rc::new(ContextWidget),
            editor: Rc::new(EditorWidget),
        },
    ];

    let theme = Theme::default_dark();
    app.execute(pages, &theme)?;

    match app.state.get::<Option<InternalCall>>("app.call.internal") {
        Ok(Some(call)) => return Ok(Some(call.clone())),
        Ok(None) | Err(_) => return Ok(None),
    }
}

pub fn update(state: &mut State, event: &InputEvent) -> Result<(), Error> {
    let mode = state.get::<Mode>("app.mode")?;
    let page = state.get::<usize>("app.page.active")?;
    let page = Page::try_from(*page)?;
    match event {
        InputEvent::Input(key) => match mode {
            Mode::Normal => match key {
                Key::Enter => {
                    switch_to_page(state, Page::Detail)?;
                    if page == Page::Overview {
                        select_default_comment(state)?;
                    }
                }
                Key::Esc => {
                    switch_to_page(state, Page::Overview)?;
                }
                _ => {
                    handle_action(state, *key)?;
                }
            },
            Mode::Editing => match key {
                Key::Esc => {
                    leave_edit_mode(state)?;
                }
                _ => {
                    handle_editor(state, key)?;
                }
            },
            _ => {}
        },
        InputEvent::Tick => {}
    }
    Ok(())
}

pub fn handle_action(state: &mut State, key: Key) -> Result<(), Error> {
    if let Some(action) = BINDINGS.get(&key) {
        let page = state.get::<usize>("app.page.active")?;
        let page = Page::try_from(*page)?;

        match action {
            Action::Quit => {
                quit_application(state)?;
            }
            Action::Up => match page {
                Page::Overview => {
                    select_previous_issue(state)?;
                }
                Page::Detail => {
                    select_previous_comment(state)?;
                }
            },
            Action::Down => match page {
                Page::Overview => {
                    select_next_issue(state)?;
                }
                Page::Detail => {
                    select_next_comment(state)?;
                }
            },
            Action::Comment => match page {
                Page::Detail => {
                    edit_comment(state)?;
                }
                _ => {}
            },
            Action::New => add_new_issue(state),
            _ => {}
        }
    }
    Ok(())
}

pub fn switch_to_page(state: &mut State, page: Page) -> Result<(), Error> {
    let next: usize = page.try_into()?;
    state.set("app.page.active", Box::new(next));
    Ok(())
}

pub fn leave_edit_mode(state: &mut State) -> Result<(), Error> {
    state.set("app.mode", Box::new(Mode::Normal));
    Ok(())
}

// pub fn exexute_external_command(state: &mut State, command: ExternalCommand) {
//     // let mode = Mode::Command(command);
//     // state.set("app.external.command", Box::new(mode));
//     state.set("app.mode", Box::new(Mode::Halt));
// }

pub fn select_default_comment(state: &mut State) -> Result<(), Error> {
    state.set("project.issue.comment.active", Box::new(0_usize));
    Ok(())
}

pub fn select_next_issue(state: &mut State) -> Result<(), Error> {
    let issues = state.get::<IssueList>("project.issue.list")?;
    let active = state.get::<usize>("project.issue.active")?;
    let active = match *active >= issues.len() - 1 {
        true => issues.len() - 1,
        false => active + 1,
    };
    state.set("project.issue.active", Box::new(active));

    Ok(())
}

pub fn select_previous_issue(state: &mut State) -> Result<(), Error> {
    let active = state.get::<usize>("project.issue.active")?;
    let active = match *active == 0 {
        true => 0,
        false => active - 1,
    };
    state.set("project.issue.active", Box::new(active));

    Ok(())
}

pub fn select_next_comment(state: &mut State) -> Result<(), Error> {
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

    Ok(())
}

pub fn select_previous_comment(state: &mut State) -> Result<(), Error> {
    let active = state.get::<usize>("project.issue.comment.active")?;
    let active = match *active == 0 {
        true => 0,
        false => active - 1,
    };
    state.set("project.issue.comment.active", Box::new(active));

    Ok(())
}

pub fn edit_comment(state: &mut State) -> Result<(), Error> {
    state.set("app.mode", Box::new(Mode::Editing));
    clear_editor(state)?;
    Ok(())
}

pub fn quit_application(state: &mut State) -> Result<(), Error> {
    state.set("app.mode", Box::new(Mode::Exiting));
    Ok(())
}

pub fn clear_editor(state: &mut State) -> Result<(), Error> {
    state.set("app.editor", Box::new(Editor::new()));
    state.set("app.editor.text", Box::new(String::new()));
    Ok(())
}

pub fn handle_editor(state: &mut State, key: &Key) -> Result<(), Error> {
    // let text = state.get::<String>("app.editor.text")?;
    // let text = format!("{}{}", text, character);
    // state.set("app.editor.text", Box::new(text.clone()));
    let area = Rect::new(0, 0, 80, 19);
    let editor = state.get::<Editor>("app.editor")?;

    // let mut editor = Editor::new();
    // editor.set_content(text);

    let mut editor = editor.clone();
    editor.process_keypress(*key, area)?;
    state.set("app.editor", Box::new(editor));
    Ok(())
}

pub fn add_new_issue(state: &mut State) {
    state.set("app.call.internal", Box::new(Some(InternalCall::New)));
    state.set("app.mode", Box::new(Mode::Exiting));
}

// pub fn navigate_editor(state: &mut State, key: Key) -> Result<(), Error> {
//     let text = state.get::<String>("app.editor.text")?;
//     let cursor_x = state.get::<usize>("app.editor.cursor.x")?;

//     Ok(())
// }
