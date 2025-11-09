// tests/new_features_tests.rs
//! Tests for new language features: nested structures, multiline strings, and lambda expressions

use aether::{Aether, Value};

#[test]
fn test_nested_arrays() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set MATRIX [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        MATRIX
    "#,
        )
        .unwrap();

    match result {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 3);
            match &arr[0] {
                Value::Array(inner) => {
                    assert_eq!(inner.len(), 3);
                    assert_eq!(inner[0], Value::Number(1.0));
                    assert_eq!(inner[1], Value::Number(2.0));
                    assert_eq!(inner[2], Value::Number(3.0));
                }
                _ => panic!("Expected inner array"),
            }
        }
        _ => panic!("Expected array"),
    }
}

#[test]
fn test_nested_array_access() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set MATRIX [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        MATRIX[1][2]
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_nested_dicts() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set CONFIG {
            "database": {
                "host": "localhost",
                "port": 3306
            },
            "cache": {
                "enabled": True,
                "ttl": 300
            }
        }
        CONFIG
    "#,
        )
        .unwrap();

    match result {
        Value::Dict(dict) => {
            assert!(dict.contains_key("database"));
            match dict.get("database") {
                Some(Value::Dict(inner)) => {
                    assert_eq!(
                        inner.get("host"),
                        Some(&Value::String("localhost".to_string()))
                    );
                    assert_eq!(inner.get("port"), Some(&Value::Number(3306.0)));
                }
                _ => panic!("Expected nested dict"),
            }
        }
        _ => panic!("Expected dict"),
    }
}

#[test]
fn test_nested_dict_access() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set CONFIG {
            "database": {
                "host": "localhost",
                "port": 3306,
                "credentials": {
                    "username": "admin",
                    "password": "secret"
                }
            }
        }
        CONFIG["database"]["credentials"]["username"]
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::String("admin".to_string()));
}

#[test]
fn test_deeply_nested_structures() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set DATA {
            "level1": {
                "level2": {
                    "level3": [
                        {"name": "item1", "value": 100},
                        {"name": "item2", "value": 200}
                    ]
                }
            }
        }
        DATA["level1"]["level2"]["level3"][1]["value"]
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(200.0));
}

#[test]
fn test_multiline_string_basic() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set TEXT """Hello
World
Aether"""
        TEXT
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::String("Hello\nWorld\nAether".to_string()));
}

#[test]
fn test_multiline_string_with_indentation() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set SQL """
SELECT 
    id, 
    name, 
    email
FROM users
WHERE status = 'active'
ORDER BY created_at DESC
"""
        SQL
    "#,
        )
        .unwrap();

    match result {
        Value::String(s) => {
            assert!(s.contains("SELECT"));
            assert!(s.contains("FROM users"));
            assert!(s.contains("WHERE status"));
        }
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_multiline_string_html() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set HTML """<!DOCTYPE html>
<html>
<head>
    <title>Test</title>
</head>
<body>
    <h1>Hello World</h1>
</body>
</html>"""
        HTML
    "#,
        )
        .unwrap();

    match result {
        Value::String(s) => {
            assert!(s.contains("<!DOCTYPE html>"));
            assert!(s.contains("<html>"));
            assert!(s.contains("<h1>Hello World</h1>"));
        }
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_multiline_string_escape_sequences() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set TEXT """Line 1\nLine 2\tTabbed"""
        TEXT
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::String("Line 1\nLine 2\tTabbed".to_string()));
}

#[test]
fn test_lambda_basic() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set DOUBLE Func(X) { Return (X * 2) }
        DOUBLE(5)
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_lambda_multiple_params() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set ADD Func(A, B) { Return (A + B) }
        ADD(3, 7)
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_lambda_no_params() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set GET_VALUE Func() { Return 42 }
        GET_VALUE()
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_lambda_with_map() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set NUMBERS [1, 2, 3, 4, 5]
        Set SQUARED MAP(NUMBERS, Func(X) { Return (X * X) })
        SQUARED
    "#,
        )
        .unwrap();

    match result {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 5);
            assert_eq!(arr[0], Value::Number(1.0));
            assert_eq!(arr[1], Value::Number(4.0));
            assert_eq!(arr[2], Value::Number(9.0));
            assert_eq!(arr[3], Value::Number(16.0));
            assert_eq!(arr[4], Value::Number(25.0));
        }
        _ => panic!("Expected array"),
    }
}

#[test]
fn test_lambda_with_filter() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set NUMBERS [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        Set EVENS FILTER(NUMBERS, Func(X) { Return (X % 2 == 0) })
        EVENS
    "#,
        )
        .unwrap();

    match result {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 5);
            assert_eq!(arr[0], Value::Number(2.0));
            assert_eq!(arr[1], Value::Number(4.0));
            assert_eq!(arr[2], Value::Number(6.0));
            assert_eq!(arr[3], Value::Number(8.0));
            assert_eq!(arr[4], Value::Number(10.0));
        }
        _ => panic!("Expected array"),
    }
}

#[test]
fn test_lambda_closure() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set MULTIPLIER 10
        Set MULTIPLY_BY_TEN Func(X) { Return (X * MULTIPLIER) }
        MULTIPLY_BY_TEN(5)
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(50.0));
}

#[test]
#[ignore] // REDUCE function not yet implemented in stdlib
fn test_lambda_with_reduce() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set NUMBERS [1, 2, 3, 4, 5]
        Set SUM REDUCE(NUMBERS, Func(ACC, X) { Return (ACC + X) }, 0)
        SUM
    "#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(15.0));
}

#[test]
fn test_combined_nested_and_lambda() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set DATA [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "age": 35}
        ]
        Set NAMES MAP(DATA, Func(PERSON) { Return PERSON["name"] })
        NAMES
    "#,
        )
        .unwrap();

    match result {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Value::String("Alice".to_string()));
            assert_eq!(arr[1], Value::String("Bob".to_string()));
            assert_eq!(arr[2], Value::String("Charlie".to_string()));
        }
        _ => panic!("Expected array"),
    }
}

#[test]
fn test_multiline_string_in_dict() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
        Set CONFIG {
            "query": """
                SELECT * 
                FROM users 
                WHERE active = 1
            """,
            "description": """This is a 
multiline 
description"""
        }
        CONFIG["query"]
    "#,
        )
        .unwrap();

    match result {
        Value::String(s) => {
            assert!(s.contains("SELECT"));
            assert!(s.contains("FROM users"));
        }
        _ => panic!("Expected string"),
    }
}
