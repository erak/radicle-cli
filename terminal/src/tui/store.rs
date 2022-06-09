use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Index(usize),
    Bool(bool),
    String(String),
    Strings(Vec<String>),
}

pub struct State {
    values: HashMap<String, Value>,
}

impl State {
    pub fn set(&mut self, key: &str, value: Value) {
        self.values.insert(String::from(key), value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(&String::from(key))
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.values.get_mut(&String::from(key))
    }
}

impl Default for State {
    fn default() -> Self {
        let mut state = State {
            values: HashMap::new(),
        };
        let shortcuts = vec![String::from("(Q)uit")];
        state.set("state.running", Value::Bool(true));
        state.set("state.shortcuts", Value::Strings(shortcuts));
        state
    }
}
