use std::collections::HashMap;

use crate::object::Object;

#[derive(Clone, PartialEq, Eq)]
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

    pub fn extend(env: Env) -> Self {
        Self { store: env.store }
    }
}

impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut store = String::new();
        for (k, v) in &self.store {
            store.push_str(&format!("{}: {}, ", k, v));
        }
        write!(f, "{}", store)
    }
}
