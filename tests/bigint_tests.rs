// tests/bigint_tests.rs
//! 大整数运算测试

use aether::{Aether, Value};

#[test]
fn test_big_integer_multiplication() {
    let mut engine = Aether::new();

    // 测试你提供的例子
    let result = engine
        .eval(
            "Set A 3284628396498263948629734587234583548273548253487325
         Set B 4728364875283754872534781253784527635487235478923587423
         Set RESULT (A * B)
         RESULT",
        )
        .unwrap();

    // 验证结果是一个分数（大整数表示为分母为1的分数）
    match result {
        Value::Fraction(frac) => {
            let expected = "15530921538361993565152129229913877304236184424817572492058487603003384389356972658598499493820859259913475";
            assert_eq!(frac.numer().to_string(), expected);
            assert_eq!(frac.denom().to_string(), "1");
        }
        _ => panic!("Expected Fraction, got {:?}", result),
    }
}

#[test]
fn test_big_integer_addition() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            "Set A 999999999999999999999999999999
         Set B 1
         (A + B)",
        )
        .unwrap();

    match result {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "1000000000000000000000000000000");
            assert_eq!(frac.denom().to_string(), "1");
        }
        _ => panic!("Expected Fraction, got {:?}", result),
    }
}

#[test]
fn test_big_integer_subtraction() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            "Set A 1000000000000000000000000000000
         Set B 1
         (A - B)",
        )
        .unwrap();

    match result {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "999999999999999999999999999999");
            assert_eq!(frac.denom().to_string(), "1");
        }
        _ => panic!("Expected Fraction, got {:?}", result),
    }
}

#[test]
fn test_big_integer_division() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            "Set A 1000000000000000000000000000000
         Set B 2
         (A / B)",
        )
        .unwrap();

    match result {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "500000000000000000000000000000");
            assert_eq!(frac.denom().to_string(), "1");
        }
        _ => panic!("Expected Fraction, got {:?}", result),
    }
}

#[test]
fn test_small_numbers_still_use_float() {
    let mut engine = Aether::new();

    // 小数字应该仍然使用 f64
    let result = engine.eval("(123456 * 789012)").unwrap();

    match result {
        Value::Number(n) => {
            assert_eq!(n, 97408265472.0);
        }
        _ => panic!("Expected Number for small integers, got {:?}", result),
    }
}

#[test]
fn test_bigint_threshold() {
    let mut engine = Aether::new();

    // 15位数字应该还是用浮点数
    let result1 = engine.eval("(123456789012345 * 2)").unwrap();
    assert!(matches!(result1, Value::Number(_)));

    // 16位以上应该用大整数
    let result2 = engine.eval("(1234567890123456 * 2)").unwrap();
    assert!(matches!(result2, Value::Fraction(_)));
}

#[test]
fn test_mixed_bigint_operations() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            "Set A 12345678901234567890
         Set B 98765432109876543210
         Set C (A + B)
         (C * 2)",
        )
        .unwrap();

    match result {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "222222222022222222200");
        }
        _ => panic!("Expected Fraction, got {:?}", result),
    }
}
