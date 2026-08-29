// src/environment.rs
//! Environment for variable storage and scoping
//! 优化版本: 减少Rc/RefCell开销, 使用索引代替指针

use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Environment for storing variables
#[derive(Debug, Clone)]
pub struct Environment {
    /// Variables in this scope (使用预分配容量优化)
    store: HashMap<String, Value>,

    /// Parent environment (for nested scopes)
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Create a new global environment (with pre-allocated capacity)
    pub fn new() -> Self {
        Environment {
            store: HashMap::with_capacity(16), // 预分配容量减少rehash
            parent: None,
        }
    }

    /// Create a new environment with a parent
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::with_capacity(8), // 子环境通常变量较少
            parent: Some(parent),
        }
    }

    /// Set a variable in the current scope
    pub fn set(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }

    /// Get a variable from this scope or parent scopes (优化路径)
    pub fn get(&self, name: &str) -> Option<Value> {
        // 快速路径: 直接在当前作用域查找
        if let Some(value) = self.store.get(name) {
            return Some(value.clone());
        }

        // 慢速路径: 递归查找父作用域
        self.get_from_parent(name)
    }

    /// 从父作用域获取变量 (分离热路径和冷路径)
    #[inline(never)]
    fn get_from_parent(&self, name: &str) -> Option<Value> {
        self.parent.as_ref()?.borrow().get(name)
    }

    /// Check if a variable exists in this scope or parent scopes
    pub fn has(&self, name: &str) -> bool {
        self.store.contains_key(name) || self.parent.as_ref().is_some_and(|p| p.borrow().has(name))
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
