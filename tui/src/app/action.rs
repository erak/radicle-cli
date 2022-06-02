use std::collections::HashMap;

use crate::app::terminal::keys::Key;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Exit,
    MenuStatus,
    MenuIssues,
    MenuPatches,
    BrowseUp,
    BrowseDown,
    Unknown,
}

pub struct Bindings {
    map: HashMap<Key, Action>,
}

impl Bindings {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        
        bindings.insert(Key::Char('q'), Action::Exit);
        bindings.insert(Key::Char('1'), Action::MenuStatus);
        bindings.insert(Key::Char('2'), Action::MenuIssues);
        bindings.insert(Key::Char('3'), Action::MenuPatches);

        bindings.insert(Key::Up, Action::BrowseUp);
        bindings.insert(Key::Down, Action::BrowseDown);
    
        Bindings {
            map: bindings
        }
    }

    pub fn get(&self, key: Key) -> Action {
        match self.map.get(&key) {
            Some(action) => action.clone(),
            None => Action::Unknown,
        }
    }
}