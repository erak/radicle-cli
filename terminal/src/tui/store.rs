use std::any::Any;
use std::collections::HashMap;

pub type Value = Box<dyn Any>;

pub struct State {
    values: HashMap<String, Value>,
}

impl State {
    pub fn set(&mut self, key: &str, value: Value) {
        self.values.insert(String::from(key), value);
    }

    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        match self.values.get(key) {
            Some(prop) => prop.downcast_ref::<T>(),
            None => None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let mut state = State {
            values: HashMap::new(),
        };
        let shortcuts = vec![String::from("(Q)uit")];
        state.set("state.running", Box::new(true));
        state.set("state.shortcuts", Box::new(shortcuts));
        state
    }
}
