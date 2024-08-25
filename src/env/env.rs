use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::object::Object;

#[derive(Clone, PartialEq, Eq)]
pub struct Env {
    store: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        // TODO: If builtins dont work, look at this
        Self {
            store: HashMap::new(),
            parent: None,
        }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.store.get(key) {
            Some(obj) => Some(obj.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, k: &str, v: Object) {
        self.store.insert(k.to_string(), v);
    }

    pub fn extend(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            store: HashMap::new(),
            parent: Some(parent),
        }
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