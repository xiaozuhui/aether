// src/environment.rs
//! Environment for variable storage and scoping

use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Environment for storing variables
#[derive(Debug, Clone)]
pub struct Environment {
    /// Variables in this scope
    store: HashMap<String, Value>,

    /// Parent environment (for nested scopes)
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Create a new global environment
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            parent: None,
        }
    }

    /// Create a new environment with a parent
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Set a variable in the current scope
    pub fn set(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }

    /// Get a variable from this scope or parent scopes
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.store.get(name) {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        None
    }

    /// Check if a variable exists in this scope or parent scopes
    pub fn has(&self, name: &str) -> bool {
        if self.store.contains_key(name) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().has(name);
        }

        false
    }

    /// Update a variable in the scope where it was defined
    /// Returns true if the variable was found and updated
    pub fn update(&mut self, name: &str, value: Value) -> bool {
        if self.store.contains_key(name) {
            self.store.insert(name.to_string(), value);
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow_mut().update(name, value);
        }

        false
    }

    /// Get all variable names in this scope
    pub fn keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    /// Clear all variables in this scope (not parent scopes)
    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_set_get() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(42.0));

        assert_eq!(env.get("x"), Some(Value::Number(42.0)));
        assert_eq!(env.get("y"), None);
    }

    #[test]
    fn test_environment_has() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(42.0));

        assert!(env.has("x"));
        assert!(!env.has("y"));
    }

    #[test]
    fn test_environment_update() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(42.0));

        assert!(env.update("x", Value::Number(100.0)));
        assert_eq!(env.get("x"), Some(Value::Number(100.0)));

        assert!(!env.update("y", Value::Number(200.0)));
    }

    #[test]
    fn test_environment_parent_scope() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        parent
            .borrow_mut()
            .set("x".to_string(), Value::Number(42.0));

        let mut child = Environment::with_parent(parent.clone());
        child.set("y".to_string(), Value::Number(100.0));

        // Child can access parent's variables
        assert_eq!(child.get("x"), Some(Value::Number(42.0)));
        assert_eq!(child.get("y"), Some(Value::Number(100.0)));

        // Parent cannot access child's variables
        assert_eq!(parent.borrow().get("x"), Some(Value::Number(42.0)));
        assert_eq!(parent.borrow().get("y"), None);
    }

    #[test]
    fn test_environment_shadowing() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        parent
            .borrow_mut()
            .set("x".to_string(), Value::Number(42.0));

        let mut child = Environment::with_parent(parent.clone());
        child.set("x".to_string(), Value::Number(100.0));

        // Child's value shadows parent's
        assert_eq!(child.get("x"), Some(Value::Number(100.0)));
        assert_eq!(parent.borrow().get("x"), Some(Value::Number(42.0)));
    }

    #[test]
    fn test_environment_update_in_parent() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        parent
            .borrow_mut()
            .set("x".to_string(), Value::Number(42.0));

        let mut child = Environment::with_parent(parent.clone());

        // Update parent's variable from child
        assert!(child.update("x", Value::Number(100.0)));
        assert_eq!(parent.borrow().get("x"), Some(Value::Number(100.0)));
        assert_eq!(child.get("x"), Some(Value::Number(100.0)));
    }

    #[test]
    fn test_environment_keys() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(42.0));
        env.set("y".to_string(), Value::String("hello".to_string()));

        let keys = env.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"x".to_string()));
        assert!(keys.contains(&"y".to_string()));
    }

    #[test]
    fn test_environment_clear() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Number(42.0));
        env.set("y".to_string(), Value::String("hello".to_string()));

        env.clear();

        assert_eq!(env.get("x"), None);
        assert_eq!(env.get("y"), None);
        assert_eq!(env.keys().len(), 0);
    }
}
