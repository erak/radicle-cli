use std::collections::HashMap;

pub const STATE_RUNNING: &str = "state.running";
pub const STATE_SHORTCUTS: &str = "state.shortcuts";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    Strings(Vec<String>),
}

pub struct State {
    pub values: HashMap<String, Value>,
}

impl Default for State {
    fn default() -> Self {
        let mut state = State {
            values: HashMap::new(),
        };
        state
            .values
            .insert(STATE_RUNNING.to_owned(), Value::Bool(true));
        state.values.insert(
            STATE_SHORTCUTS.to_owned(),
            Value::Strings(vec!["(Q)uit".to_owned()]),
        );
        state
    }
}
