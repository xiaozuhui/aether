// src/builtins/precise.rs
use crate::evaluator::RuntimeError;
use crate::value::Value;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{ToPrimitive, Zero};

/// 将数字转换为分数
///
/// 参数：
/// - args[0]: 数字或分数值
///
/// 返回：
/// - 转换后的分数值
pub fn to_fraction(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        // 连分数重建：还原「本意」分数并保证浮点往返恒等
        //（TO_FRACTION(1/3) → 1/3、TO_FRACTION(0.1) → 1/10、TO_FRACTION(1e-7) → 1/10^7）
        Value::Number(n) => crate::numeric::f64_to_fraction(*n)
            .map(Value::Fraction)
            .ok_or_else(|| {
                RuntimeError::TypeError(format!("无法将非有限值 {} 转换为 Fraction", n))
            }),
        Value::Fraction(f) => Ok(Value::Fraction(f.clone())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number or Fraction".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 将分数转换为浮点数
///
/// 参数：
/// - args[0]: 分数或数字值
///
/// 返回：
/// - 转换后的浮点数值
pub fn to_float(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => {
            let num = f.numer().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert numerator".to_string())
            })?;
            let den = f.denom().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert denominator".to_string())
            })?;
            Ok(Value::Number(num / den))
        }
        Value::Number(n) => Ok(Value::Number(*n)),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction or Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 将 Value 类型转换为 Ratio<BigInt> 分数类型
///
/// 参数：
/// - value: 待转换的值（数字或分数）
///
/// 返回：
/// - Ratio<BigInt> 类型的分数
fn value_to_fraction(value: &Value) -> Result<Ratio<BigInt>, RuntimeError> {
    match value {
        Value::Fraction(f) => Ok(f.clone()),
        // Number 经数值核心统一转换（连分数重建，与 TO_FRACTION 同源）
        Value::Number(n) => crate::numeric::f64_to_fraction(*n).ok_or_else(|| {
            RuntimeError::TypeError(format!("无法将非有限值 {} 提升为 Fraction", n))
        }),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction or Number".to_string(),
            got: format!("{:?}", value),
        }),
    }
}

/// 分数加法运算
///
/// 参数：
/// - args[0]: 第一个加数（数字或分数）
/// - args[1]: 第二个加数（数字或分数）
///
/// 返回：
/// - 两个分数相加的结果
pub fn frac_add(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    Ok(Value::Fraction(frac1 + frac2))
}

/// 分数减法运算
///
/// 参数：
/// - args[0]: 被减数（数字或分数）
/// - args[1]: 减数（数字或分数）
///
/// 返回：
/// - 两个分数相减的结果
pub fn frac_sub(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    Ok(Value::Fraction(frac1 - frac2))
}

/// 分数乘法运算
///
/// 参数：
/// - args[0]: 第一个乘数（数字或分数）
/// - args[1]: 第二个乘数（数字或分数）
///
/// 返回：
/// - 两个分数相乘的结果
pub fn frac_mul(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    Ok(Value::Fraction(frac1 * frac2))
}

/// 分数除法运算
///
/// 参数：
/// - args[0]: 被除数（数字或分数）
/// - args[1]: 除数（数字或分数）
///
/// 返回：
/// - 两个分数相除的结果
///
/// 注意：除数不能为零
pub fn frac_div(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    if frac2.is_zero() {
        return Err(RuntimeError::InvalidOperation(
            "Division by zero".to_string(),
        ));
    }
    Ok(Value::Fraction(frac1 / frac2))
}

/// 获取分数的分子
///
/// 参数：
/// - args[0]: 分数值
///
/// 返回：
/// - 分数的分子（转换为浮点数）
pub fn numerator(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => {
            let num = f.numer().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert numerator".to_string())
            })?;
            Ok(Value::Number(num))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 获取分数的分母
///
/// 参数：
/// - args[0]: 分数值
///
/// 返回：
/// - 分数的分母（转换为浮点数）
pub fn denominator(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => {
            let den = f.denom().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert denominator".to_string())
            })?;
            Ok(Value::Number(den))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 计算两个整数的最大公约数（Greatest Common Divisor）
///
/// 参数：
/// - args[0]: 第一个整数
/// - args[1]: 第二个整数
///
/// 返回：
/// - 两个数的最大公约数
pub fn gcd(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let a = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            });
        }
    };
    let b = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[1]),
            });
        }
    };
    // 欧几里得算法实现最大公约数计算
    fn gcd_impl(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }
    Ok(Value::Number(gcd_impl(a, b) as f64))
}

/// 计算两个整数的最小公倍数（Least Common Multiple）
///
/// 参数：
/// - args[0]: 第一个整数
/// - args[1]: 第二个整数
///
/// 返回：
/// - 两个数的最小公倍数
pub fn lcm(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let a = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            });
        }
    };
    let b = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[1]),
            });
        }
    };
    // 使用公式：lcm(a,b) = |a*b| / gcd(a,b)
    fn gcd_impl(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }
    let result = (a.abs() * b.abs()) / gcd_impl(a, b);
    Ok(Value::Number(result as f64))
}
