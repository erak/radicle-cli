use std::any::Any;
use std::collections::HashMap;

pub type Value = Box<dyn Any>;

pub struct State {
    properties: HashMap<String, Value>,
}

impl State {
    pub fn set(&mut self, key: &str, value: Value) {
        self.properties.insert(String::from(key), value);
    }

    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        match self.properties.get(key) {
            Some(prop) => prop.downcast_ref::<T>(),
            None => None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            properties: HashMap::new(),
        }
    }
}
