use std::any::Any;
use std::collections::HashMap;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Invalid type requested: {0}")]
    InvalidType(String),
    #[error("Property not found: {0}")]
    NotFound(String),
}

pub type Value = Box<dyn Any>;

pub struct State {
    properties: HashMap<String, Value>,
}

impl State {
    pub fn set(&mut self, key: &str, value: Value) {
        self.properties.insert(String::from(key), value);
    }

    pub fn get<T: Any>(&self, key: &str) -> Result<&T, StoreError> {
        match self.properties.get(key) {
            Some(prop) => match prop.downcast_ref::<T>() {
                Some(value) => Ok(value),
                None => Err(StoreError::InvalidType(key.to_owned())),
            },
            None => Err(StoreError::NotFound(key.to_owned())),
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
