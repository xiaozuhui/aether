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

// ============================================================
// 位运算测试
// ============================================================

#[test]
fn test_bitwise_and_or_xor() {
    let mut engine = Aether::new();

    assert_eq!(engine.eval("(12 & 10)").unwrap(), Value::Number(8.0));
    assert_eq!(engine.eval("(12 | 3)").unwrap(), Value::Number(15.0));
    assert_eq!(engine.eval("(12 ^ 10)").unwrap(), Value::Number(6.0));
}

#[test]
fn test_bitwise_shift() {
    let mut engine = Aether::new();

    assert_eq!(engine.eval("(1 << 10)").unwrap(), Value::Number(1024.0));
    assert_eq!(engine.eval("(1024 >> 3)").unwrap(), Value::Number(128.0));
}

#[test]
fn test_bitwise_on_bigint() {
    let mut engine = Aether::new();

    // 大整数右移
    let result = engine
        .eval("(123456789012345678901234567890 >> 60)")
        .unwrap();
    match result {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "107081695084");
            assert_eq!(frac.denom().to_string(), "1");
        }
        _ => panic!("Expected Fraction, got {:?}", result),
    }

    // 大整数按位与
    let result2 = engine
        .eval("(123456789012345678901234567890 & 255)")
        .unwrap();
    match result2 {
        Value::Fraction(frac) => assert_eq!(frac.numer().to_string(), "210"),
        _ => panic!("Expected Fraction, got {:?}", result2),
    }
}

#[test]
fn test_bitwise_requires_integer_operands() {
    let mut engine = Aether::new();

    // 小数参与位运算应报错
    assert!(engine.eval("(1.5 & 1)").is_err());
    // 分母非 1 的分数参与位运算应报错
    assert!(engine.eval("(1 & 0.5)").is_err());
}

#[test]
fn test_bitwise_precedence() {
    let mut engine = Aether::new();

    // 移位优先级高于加减：2 + (1 << 3) = 10
    assert_eq!(engine.eval("(2 + (1 << 3))").unwrap(), Value::Number(10.0));
    // 位运算优先级低于比较
    let result = engine.eval("((12 & 10) == 8)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

// ============================================================
// 科学计数法测试
// ============================================================

#[test]
fn test_scientific_notation_positive_exponent() {
    let mut engine = Aether::new();

    // 0.6.0 语义冻结：科学计数法一律 f64（与「小数保持 f64」一致）。
    // 需要精确大整数请书写完整数字（超阈值自动 BigInteger）
    // 或显式 TO_FRACTION。
    let result = engine.eval("1e30").unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 1e30),
        _ => panic!("Expected Number, got {:?}", result),
    }
    // 全数字书写的大整数仍是精确 Fraction
    let exact = engine.eval("1000000000000000000000000000000").unwrap();
    match exact {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "1000000000000000000000000000000");
            assert_eq!(frac.denom().to_string(), "1");
        }
        _ => panic!("Expected Fraction, got {:?}", exact),
    }
    // 科学计数法经 TO_FRACTION 还原该 double 的精确有理值
    let recovered = engine.eval("TO_FRACTION(1e30)").unwrap();
    match recovered {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "1000000000000000019884624838656");
            assert_eq!(frac.denom().to_string(), "1");
        }
        _ => panic!("Expected Fraction, got {:?}", recovered),
    }
}

#[test]
fn test_scientific_notation_small_values() {
    let mut engine = Aether::new();

    assert_eq!(engine.eval("1.5e3").unwrap(), Value::Number(1500.0));
    assert_eq!(engine.eval("2e2").unwrap(), Value::Number(200.0));
}

#[test]
fn test_scientific_notation_negative_exponent() {
    let mut engine = Aether::new();

    // 负指数降级为 f64
    let result = engine.eval("2e-2").unwrap();
    match result {
        Value::Number(n) => assert!((n - 0.02).abs() < 1e-12),
        _ => panic!("Expected Number, got {:?}", result),
    }
}

#[test]
fn test_scientific_notation_with_plus_sign() {
    let mut engine = Aether::new();

    assert_eq!(engine.eval("3e+2").unwrap(), Value::Number(300.0));
}

// ============================================================
// 可配置阈值测试
// ============================================================

#[test]
fn test_configurable_bigint_threshold() {
    let mut engine = Aether::new();
    assert_eq!(engine.bigint_threshold(), 15);

    // 默认阈值下 1234 是 Number
    let default_result = engine.eval("1234").unwrap();
    assert!(matches!(default_result, Value::Number(_)));

    // 调低阈值后 1234 变成 BigInteger（Fraction 表示）
    engine.set_bigint_threshold(3);
    assert_eq!(engine.bigint_threshold(), 3);
    let lowered_result = engine.eval("5678").unwrap();
    assert!(matches!(lowered_result, Value::Fraction(_)));
}

// ============================================================
// 性能优化相关测试
// ============================================================

#[test]
fn test_bigint_constant_folding() {
    let mut engine = Aether::new();

    // 两个大整数字面量的乘法应在优化期折叠，结果精确
    let result = engine
        .eval("(1000000000000000000000 * 1000000000000000000000)")
        .unwrap();
    match result {
        Value::Fraction(frac) => {
            assert_eq!(
                frac.numer().to_string(),
                "1000000000000000000000000000000000000000000"
            );
        }
        _ => panic!("Expected Fraction, got {:?}", result),
    }
}

#[test]
fn test_large_f64_fraction_mixing_precision() {
    let mut engine = Aether::new();

    // f64 整数值超出 i64 范围时与 Fraction 相加，不得静默截断
    // 1e19 > i64::MAX (约 9.22e18)
    let result = engine
        .eval(
            "Set A 10000000000000000000
         Set B 1
         (A + B)",
        )
        .unwrap();
    match result {
        Value::Fraction(frac) => {
            assert_eq!(frac.numer().to_string(), "10000000000000000001");
        }
        Value::Number(n) => panic!("Expected exact Fraction, got f64 {}", n),
        _ => panic!("Unexpected value: {:?}", result),
    }
}
