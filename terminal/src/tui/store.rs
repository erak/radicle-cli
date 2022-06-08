use std::collections::HashMap;

pub const STATE_RUNNING: &str = "state.running";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
}

pub struct State {
    pub values: HashMap<String, Value>,
}

impl Default for State {
    fn default() -> Self {
        let mut state = State {
            values: HashMap::new(),
        };
        state.values.insert(STATE_RUNNING.to_owned(), Value::Bool(true));
        state
    }
}