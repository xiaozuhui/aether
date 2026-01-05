// src/environment.rs
//! Environment for variable storage and scoping
//! 优化版本: 减少Rc/RefCell开销, 使用索引代替指针

use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// 环境池,用于复用环境对象
pub struct EnvironmentPool {
    /// 可复用的环境对象池
    pool: Vec<Environment>,
    /// 最大池大小
    max_size: usize,
}

impl EnvironmentPool {
    /// 创建新的环境池
    pub fn new() -> Self {
        Self::with_capacity(50)
    }

    /// 创建指定容量的环境池
    pub fn with_capacity(max_size: usize) -> Self {
        EnvironmentPool {
            pool: Vec::with_capacity(max_size.min(50)),
            max_size,
        }
    }

    /// 从池中获取或创建新环境
    pub fn acquire(&mut self) -> Environment {
        self.pool.pop().unwrap_or_default()
    }

    /// 将环境归还到池中
    pub fn release(&mut self, mut env: Environment) {
        if self.pool.len() < self.max_size {
            env.clear();
            env.parent = None;
            self.pool.push(env);
        }
    }

    /// 清空池
    pub fn clear(&mut self) {
        self.pool.clear();
    }
}

impl Default for EnvironmentPool {
    fn default() -> Self {
        Self::new()
    }
}

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
