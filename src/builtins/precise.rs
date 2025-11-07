// src/builtins/precise.rs
use crate::evaluator::RuntimeError;
use crate::value::Value;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, ToPrimitive, Zero};

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
        Value::Number(n) => {
            let s = format!("{}", n);
            if let Some(dot_pos) = s.find('.') {
                let decimal_places = s.len() - dot_pos - 1;
                let denominator = 10_i64.pow(decimal_places as u32);
                let numerator = (n * denominator as f64).round() as i64;
                let frac = Ratio::new(BigInt::from(numerator), BigInt::from(denominator));
                Ok(Value::Fraction(frac))
            } else {
                let frac = Ratio::new(BigInt::from(*n as i64), BigInt::one());
                Ok(Value::Fraction(frac))
            }
        }
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

/// 化简分数（约分）
///
/// 参数：
/// - args[0]: 要化简的分数
///
/// 返回：
/// - 化简后的最简分数
pub fn simplify(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => Ok(Value::Fraction(f.reduced())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction".to_string(),
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
        Value::Number(n) => {
            let s = format!("{}", n);
            if let Some(dot_pos) = s.find('.') {
                let decimal_places = s.len() - dot_pos - 1;
                let denominator = 10_i64.pow(decimal_places as u32);
                let numerator = (n * denominator as f64).round() as i64;
                Ok(Ratio::new(
                    BigInt::from(numerator),
                    BigInt::from(denominator),
                ))
            } else {
                Ok(Ratio::new(BigInt::from(*n as i64), BigInt::one()))
            }
        }
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
            })
        }
    };
    let b = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[1]),
            })
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
            })
        }
    };
    let b = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[1]),
            })
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
