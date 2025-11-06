// tests/builtins_tests.rs
//! 内置函数集成测试
//!
//! 本文件包含所有内置函数的单元测试，包括：
//! - I/O 函数测试
//! - 类型转换函数测试
//! - 数组操作函数测试
//! - 字符串操作函数测试
//! - 基础数学函数测试
//! - 字典操作函数测试

use aether::builtins::{array, dict, io, math, string, types};
use aether::value::Value;

// ============================================================================
// I/O 函数测试
// ============================================================================

#[test]
fn test_print() {
    // 注意: print 写入 stdout，只检查不报错
    let result = io::print(&[Value::String("test".to_string())]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Null);
}

#[test]
fn test_println() {
    let result = io::println(&[Value::String("test".to_string())]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Null);
}

// ============================================================================
// 类型函数测试
// ============================================================================

#[test]
fn test_type_of() {
    assert_eq!(
        types::type_of(&[Value::Number(42.0)]).unwrap(),
        Value::String("Number".to_string())
    );
    assert_eq!(
        types::type_of(&[Value::String("hello".to_string())]).unwrap(),
        Value::String("String".to_string())
    );
    assert_eq!(
        types::type_of(&[Value::Boolean(true)]).unwrap(),
        Value::String("Boolean".to_string())
    );
    assert_eq!(
        types::type_of(&[Value::Null]).unwrap(),
        Value::String("Null".to_string())
    );
}

#[test]
fn test_to_string() {
    assert_eq!(
        types::to_string(&[Value::Number(42.0)]).unwrap(),
        Value::String("42".to_string())
    );
    assert_eq!(
        types::to_string(&[Value::Boolean(true)]).unwrap(),
        Value::String("true".to_string())
    );
}

#[test]
fn test_to_number() {
    assert_eq!(
        types::to_number(&[Value::String("123".to_string())]).unwrap(),
        Value::Number(123.0)
    );
    assert_eq!(
        types::to_number(&[Value::Boolean(true)]).unwrap(),
        Value::Number(1.0)
    );
    assert_eq!(
        types::to_number(&[Value::Boolean(false)]).unwrap(),
        Value::Number(0.0)
    );
}

#[test]
fn test_len() {
    assert_eq!(
        types::len(&[Value::String("hello".to_string())]).unwrap(),
        Value::Number(5.0)
    );
    assert_eq!(
        types::len(&[Value::Array(vec![Value::Number(1.0), Value::Number(2.0)])]).unwrap(),
        Value::Number(2.0)
    );
}

// ============================================================================
// 数组函数测试
// ============================================================================

#[test]
fn test_range() {
    // Range(5) -> [0, 1, 2, 3, 4]
    let result = array::range(&[Value::Number(5.0)]).unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
        ])
    );

    // Range(2, 5) -> [2, 3, 4]
    let result = array::range(&[Value::Number(2.0), Value::Number(5.0)]).unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
        ])
    );
}

#[test]
fn test_push() {
    let arr = Value::Array(vec![Value::Number(1.0)]);
    let result = array::push(&[arr, Value::Number(2.0)]).unwrap();
    assert_eq!(
        result,
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)])
    );
}

#[test]
fn test_reverse() {
    let arr = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let result = array::reverse(&[arr]).unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(3.0),
            Value::Number(2.0),
            Value::Number(1.0),
        ])
    );
}

#[test]
fn test_sort() {
    let arr = Value::Array(vec![
        Value::Number(5.0),
        Value::Number(2.0),
        Value::Number(8.0),
        Value::Number(1.0),
    ]);
    let result = array::sort(&[arr]).unwrap();
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(5.0),
            Value::Number(8.0),
        ])
    );
}

#[test]
fn test_sum() {
    let arr = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    assert_eq!(array::sum(&[arr]).unwrap(), Value::Number(6.0));
}

#[test]
fn test_max() {
    let arr = Value::Array(vec![
        Value::Number(5.0),
        Value::Number(2.0),
        Value::Number(8.0),
        Value::Number(1.0),
    ]);
    assert_eq!(array::max(&[arr]).unwrap(), Value::Number(8.0));
}

#[test]
fn test_min() {
    let arr = Value::Array(vec![
        Value::Number(5.0),
        Value::Number(2.0),
        Value::Number(8.0),
        Value::Number(1.0),
    ]);
    assert_eq!(array::min(&[arr]).unwrap(), Value::Number(1.0));
}

#[test]
fn test_join() {
    let arr = Value::Array(vec![
        Value::String("Hello".to_string()),
        Value::String("World".to_string()),
    ]);
    let sep = Value::String(" ".to_string());
    assert_eq!(
        array::join(&[arr, sep]).unwrap(),
        Value::String("Hello World".to_string())
    );
}

// ============================================================================
// 字符串函数测试
// ============================================================================

#[test]
fn test_upper() {
    assert_eq!(
        string::upper(&[Value::String("hello".to_string())]).unwrap(),
        Value::String("HELLO".to_string())
    );
}

#[test]
fn test_lower() {
    assert_eq!(
        string::lower(&[Value::String("HELLO".to_string())]).unwrap(),
        Value::String("hello".to_string())
    );
}

#[test]
fn test_trim() {
    assert_eq!(
        string::trim(&[Value::String("  hello  ".to_string())]).unwrap(),
        Value::String("hello".to_string())
    );
}

#[test]
fn test_contains() {
    let s = Value::String("Hello World".to_string());
    let sub = Value::String("World".to_string());
    assert_eq!(string::contains(&[s, sub]).unwrap(), Value::Boolean(true));
}

#[test]
fn test_starts_with() {
    let s = Value::String("Hello World".to_string());
    let prefix = Value::String("Hello".to_string());
    assert_eq!(
        string::starts_with(&[s, prefix]).unwrap(),
        Value::Boolean(true)
    );
}

#[test]
fn test_ends_with() {
    let s = Value::String("Hello World".to_string());
    let suffix = Value::String("World".to_string());
    assert_eq!(
        string::ends_with(&[s, suffix]).unwrap(),
        Value::Boolean(true)
    );
}

#[test]
fn test_replace() {
    let s = Value::String("Hello World".to_string());
    let from = Value::String("World".to_string());
    let to = Value::String("Aether".to_string());
    assert_eq!(
        string::replace(&[s, from, to]).unwrap(),
        Value::String("Hello Aether".to_string())
    );
}

#[test]
fn test_repeat() {
    let s = Value::String("Hi ".to_string());
    let n = Value::Number(3.0);
    assert_eq!(
        string::repeat(&[s, n]).unwrap(),
        Value::String("Hi Hi Hi ".to_string())
    );
}

#[test]
fn test_split() {
    let s = Value::String("a,b,c".to_string());
    let sep = Value::String(",".to_string());
    assert_eq!(
        string::split(&[s, sep]).unwrap(),
        Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
        ])
    );
}

// ============================================================================
// 基础数学函数测试
// ============================================================================

#[test]
fn test_abs() {
    assert_eq!(
        math::abs(&[Value::Number(-42.0)]).unwrap(),
        Value::Number(42.0)
    );
    assert_eq!(
        math::abs(&[Value::Number(42.0)]).unwrap(),
        Value::Number(42.0)
    );
}

#[test]
fn test_floor() {
    assert_eq!(
        math::floor(&[Value::Number(3.7)]).unwrap(),
        Value::Number(3.0)
    );
}

#[test]
fn test_ceil() {
    assert_eq!(
        math::ceil(&[Value::Number(3.2)]).unwrap(),
        Value::Number(4.0)
    );
}

#[test]
fn test_round() {
    assert_eq!(
        math::round(&[Value::Number(3.5)]).unwrap(),
        Value::Number(4.0)
    );
    assert_eq!(
        math::round(&[Value::Number(3.4)]).unwrap(),
        Value::Number(3.0)
    );
}

#[test]
fn test_sqrt() {
    assert_eq!(
        math::sqrt(&[Value::Number(16.0)]).unwrap(),
        Value::Number(4.0)
    );
}

#[test]
fn test_pow() {
    assert_eq!(
        math::pow(&[Value::Number(2.0), Value::Number(10.0)]).unwrap(),
        Value::Number(1024.0)
    );
}

// ============================================================================
// 字典函数测试
// ============================================================================

#[test]
fn test_keys() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Number(1.0));
    map.insert("b".to_string(), Value::Number(2.0));
    let dict = Value::Dict(map);

    let result = dict::keys(&[dict]).unwrap();
    if let Value::Array(keys) = result {
        assert_eq!(keys.len(), 2);
        // 顺序可能不同
    } else {
        panic!("Expected array");
    }
}

#[test]
fn test_has() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("name".to_string(), Value::String("Alice".to_string()));
    let dict = Value::Dict(map);

    assert_eq!(
        dict::has(&[dict.clone(), Value::String("name".to_string())]).unwrap(),
        Value::Boolean(true)
    );
    assert_eq!(
        dict::has(&[dict, Value::String("age".to_string())]).unwrap(),
        Value::Boolean(false)
    );
}
