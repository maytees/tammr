use std::collections::HashMap;

use crate::object::Object;

pub struct Env {
    store: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        self.store.get(key).cloned()
    }

    pub fn set(&mut self, k: &str, v: Object) {
        self.store.insert(k.to_string(), v);
    }
}
