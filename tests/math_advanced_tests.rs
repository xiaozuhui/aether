// tests/math_advanced_tests.rs
//! 高级数学函数测试
//!
//! 本文件包含所有高级数学函数的单元测试，包括：
//! - 高级三角函数测试
//! - 特殊数学函数测试
//! - 统计函数测试
//! - 向量运算测试
//! - 矩阵运算测试
//! - 数学常数测试

use aether::builtins::math;
use aether::value::Value;

// ============================================================================
// 三角函数测试
// ============================================================================

#[test]
fn test_asin() {
    let result = math::asin(&[Value::Number(0.5)]).unwrap();
    if let Value::Number(n) = result {
        #[allow(clippy::approx_constant)]
        {
            assert!((n - 0.5236).abs() < 0.001);
        }
    } else {
        panic!("Expected number");
    }
}

#[test]
fn test_atan2() {
    let result = math::atan2(&[Value::Number(1.0), Value::Number(1.0)]).unwrap();
    if let Value::Number(n) = result {
        assert!((n - std::f64::consts::PI / 4.0).abs() < 0.0001);
    } else {
        panic!("Expected number");
    }
}

#[test]
fn test_sinh() {
    let result = math::sinh(&[Value::Number(0.0)]).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

// ============================================================================
// 特殊函数测试
// ============================================================================

#[test]
fn test_factorial() {
    assert_eq!(
        math::factorial(&[Value::Number(0.0)]).unwrap(),
        Value::Number(1.0)
    );
    assert_eq!(
        math::factorial(&[Value::Number(5.0)]).unwrap(),
        Value::Number(120.0)
    );
}

#[test]
fn test_gamma() {
    // Gamma(5) = 4! = 24
    let result = math::gamma(&[Value::Number(5.0)]).unwrap();
    assert_eq!(result, Value::Number(24.0));
}

#[test]
fn test_hypot() {
    let result = math::hypot(&[Value::Number(3.0), Value::Number(4.0)]).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_sign() {
    assert_eq!(
        math::sign(&[Value::Number(5.0)]).unwrap(),
        Value::Number(1.0)
    );
    assert_eq!(
        math::sign(&[Value::Number(-5.0)]).unwrap(),
        Value::Number(-1.0)
    );
    assert_eq!(
        math::sign(&[Value::Number(0.0)]).unwrap(),
        Value::Number(0.0)
    );
}

#[test]
fn test_clamp() {
    assert_eq!(
        math::clamp(&[Value::Number(5.0), Value::Number(0.0), Value::Number(10.0)]).unwrap(),
        Value::Number(5.0)
    );
    assert_eq!(
        math::clamp(&[Value::Number(15.0), Value::Number(0.0), Value::Number(10.0)]).unwrap(),
        Value::Number(10.0)
    );
    assert_eq!(
        math::clamp(&[Value::Number(-5.0), Value::Number(0.0), Value::Number(10.0)]).unwrap(),
        Value::Number(0.0)
    );
}

// ============================================================================
// 统计函数测试
// ============================================================================

#[test]
fn test_mean() {
    let arr = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);
    assert_eq!(math::mean(&[arr]).unwrap(), Value::Number(3.0));
}

#[test]
fn test_median() {
    let arr = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);
    assert_eq!(math::median(&[arr]).unwrap(), Value::Number(3.0));

    let arr_even = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
    ]);
    assert_eq!(math::median(&[arr_even]).unwrap(), Value::Number(2.5));
}

#[test]
fn test_variance_and_std() {
    let arr = Value::Array(vec![
        Value::Number(2.0),
        Value::Number(4.0),
        Value::Number(4.0),
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(5.0),
        Value::Number(7.0),
        Value::Number(9.0),
    ]);

    let var = math::variance(&[arr.clone()]).unwrap(); // [arr.clone()] is intentional - variance takes array of arrays
    if let Value::Number(v) = var {
        assert!((v - 4.571).abs() < 0.01);
    } else {
        panic!("Expected number");
    }

    let std_val = math::std(&[arr]).unwrap();
    if let Value::Number(s) = std_val {
        assert!((s - 2.138).abs() < 0.01);
    } else {
        panic!("Expected number");
    }
}

#[test]
fn test_quantile() {
    let arr = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);

    assert_eq!(
        math::quantile(&[arr.clone(), Value::Number(0.0)]).unwrap(),
        Value::Number(1.0)
    );
    assert_eq!(
        math::quantile(&[arr.clone(), Value::Number(0.5)]).unwrap(),
        Value::Number(3.0)
    );
    assert_eq!(
        math::quantile(&[arr, Value::Number(1.0)]).unwrap(),
        Value::Number(5.0)
    );
}

// ============================================================================
// 向量运算测试
// ============================================================================

#[test]
fn test_dot() {
    let v1 = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let v2 = Value::Array(vec![
        Value::Number(4.0),
        Value::Number(5.0),
        Value::Number(6.0),
    ]);

    // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    assert_eq!(math::dot(&[v1, v2]).unwrap(), Value::Number(32.0));
}

#[test]
fn test_norm() {
    let v = Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]);

    assert_eq!(math::norm(&[v]).unwrap(), Value::Number(5.0));
}

#[test]
fn test_cross() {
    let v1 = Value::Array(vec![
        Value::Number(1.0),
        Value::Number(0.0),
        Value::Number(0.0),
    ]);
    let v2 = Value::Array(vec![
        Value::Number(0.0),
        Value::Number(1.0),
        Value::Number(0.0),
    ]);

    let result = math::cross(&[v1, v2]).unwrap();
    let expected = Value::Array(vec![
        Value::Number(0.0),
        Value::Number(0.0),
        Value::Number(1.0),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_distance() {
    let v1 = Value::Array(vec![Value::Number(0.0), Value::Number(0.0)]);
    let v2 = Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]);

    assert_eq!(math::distance(&[v1, v2]).unwrap(), Value::Number(5.0));
}

#[test]
fn test_normalize() {
    let v = Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]);

    let result = math::normalize(&[v]).unwrap();
    if let Value::Array(arr) = result {
        assert_eq!(arr.len(), 2);
        if let (Value::Number(x), Value::Number(y)) = (&arr[0], &arr[1]) {
            assert!((x - 0.6).abs() < 0.0001);
            assert!((y - 0.8).abs() < 0.0001);
        } else {
            panic!("Expected numbers in array");
        }
    } else {
        panic!("Expected array");
    }
}

// ============================================================================
// 矩阵运算测试
// ============================================================================

#[test]
fn test_transpose() {
    let m = Value::Array(vec![
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]),
    ]);

    let result = math::transpose(&[m]).unwrap();
    let expected = Value::Array(vec![
        Value::Array(vec![Value::Number(1.0), Value::Number(3.0)]),
        Value::Array(vec![Value::Number(2.0), Value::Number(4.0)]),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_determinant_2x2() {
    let m = Value::Array(vec![
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]),
    ]);

    // det = 1*4 - 2*3 = -2
    assert_eq!(math::determinant(&[m]).unwrap(), Value::Number(-2.0));
}

#[test]
fn test_determinant_3x3() {
    let m = Value::Array(vec![
        Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]),
        Value::Array(vec![
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(4.0),
        ]),
        Value::Array(vec![
            Value::Number(5.0),
            Value::Number(6.0),
            Value::Number(0.0),
        ]),
    ]);

    let result = math::determinant(&[m]).unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_matmul() {
    let m1 = Value::Array(vec![
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]),
    ]);

    let m2 = Value::Array(vec![
        Value::Array(vec![Value::Number(5.0), Value::Number(6.0)]),
        Value::Array(vec![Value::Number(7.0), Value::Number(8.0)]),
    ]);

    let result = math::matmul(&[m1, m2]).unwrap();
    let expected = Value::Array(vec![
        Value::Array(vec![Value::Number(19.0), Value::Number(22.0)]),
        Value::Array(vec![Value::Number(43.0), Value::Number(50.0)]),
    ]);

    assert_eq!(result, expected);
}

// ============================================================================
// 常数测试
// ============================================================================

#[test]
fn test_constants() {
    let pi = math::pi(&[]).unwrap();
    if let Value::Number(n) = pi {
        #[allow(clippy::approx_constant)]
        {
            assert!((n - 3.14159).abs() < 0.001);
        }
    } else {
        panic!("Expected number");
    }

    let e = math::e(&[]).unwrap();
    if let Value::Number(n) = e {
        #[allow(clippy::approx_constant)]
        {
            assert!((n - 2.71828).abs() < 0.001);
        }
    } else {
        panic!("Expected number");
    }

    let phi = math::phi(&[]).unwrap();
    if let Value::Number(n) = phi {
        assert!((n - 1.61803).abs() < 0.001);
    } else {
        panic!("Expected number");
    }
}
