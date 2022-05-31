use std::collections::HashMap;

use crate::app::terminal::keys::Key;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    Unknown,
}

pub struct Bindings {
    map: HashMap<Key, Action>,
}

impl Bindings {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(Key::Char('q'), Action::Quit);
    
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