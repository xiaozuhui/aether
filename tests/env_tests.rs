use std::{cell::RefCell, rc::Rc};

use aether::{Environment, Value};

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
