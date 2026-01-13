use aether::Value;

#[test]
fn test_value_truthy() {
    assert!(Value::Boolean(true).is_truthy());
    assert!(!Value::Boolean(false).is_truthy());
    assert!(!Value::Null.is_truthy());
    assert!(Value::Number(1.0).is_truthy());
    assert!(!Value::Number(0.0).is_truthy());
    assert!(Value::String("hello".to_string()).is_truthy());
    assert!(!Value::String("".to_string()).is_truthy());
}

#[test]
fn test_value_type_name() {
    assert_eq!(Value::Number(42.0).type_name(), "Number");
    assert_eq!(Value::String("test".to_string()).type_name(), "String");
    assert_eq!(Value::Boolean(true).type_name(), "Boolean");
    assert_eq!(Value::Null.type_name(), "Null");
    assert_eq!(Value::Array(vec![]).type_name(), "Array");
}

#[test]
fn test_value_to_number() {
    assert_eq!(Value::Number(42.0).to_number(), Some(42.0));
    assert_eq!(Value::Boolean(true).to_number(), Some(1.0));
    assert_eq!(Value::Boolean(false).to_number(), Some(0.0));
    assert_eq!(Value::String("123".to_string()).to_number(), Some(123.0));
    assert_eq!(Value::String("abc".to_string()).to_number(), None);
    assert_eq!(Value::Null.to_number(), None);
}

#[test]
fn test_value_to_string() {
    assert_eq!(Value::Number(42.0).to_string(), "42");
    #[allow(clippy::approx_constant)]
    {
        assert_eq!(Value::Number(3.14).to_string(), "3.14");
    }
    assert_eq!(Value::String("hello".to_string()).to_string(), "hello");
    assert_eq!(Value::Boolean(true).to_string(), "true");
    assert_eq!(Value::Null.to_string(), "Null");
    assert_eq!(
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]).to_string(),
        "[1, 2]"
    );
}

#[test]
fn test_value_equals() {
    assert!(Value::Number(42.0).equals(&Value::Number(42.0)));
    assert!(!Value::Number(42.0).equals(&Value::Number(43.0)));
    assert!(Value::String("test".to_string()).equals(&Value::String("test".to_string())));
    assert!(Value::Boolean(true).equals(&Value::Boolean(true)));
    assert!(Value::Null.equals(&Value::Null));
}

#[test]
fn test_value_compare() {
    use std::cmp::Ordering;

    assert_eq!(
        Value::Number(42.0).compare(&Value::Number(43.0)),
        Some(Ordering::Less)
    );
    assert_eq!(
        Value::String("a".to_string()).compare(&Value::String("b".to_string())),
        Some(Ordering::Less)
    );
    assert_eq!(
        Value::Boolean(false).compare(&Value::Boolean(true)),
        Some(Ordering::Less)
    );
}

#[test]
fn test_array_equality() {
    let arr1 = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let arr2 = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let arr3 = Value::Array(vec![Value::Number(1.0), Value::Number(3.0)]);

    assert!(arr1.equals(&arr2));
    assert!(!arr1.equals(&arr3));
}
