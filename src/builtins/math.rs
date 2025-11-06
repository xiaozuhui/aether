// src/builtins/math.rs
//! Mathematical built-in functions
//!
//! This module provides:
//! - Basic math: abs, floor, ceil, round, sqrt, pow
//! - Trigonometry: sin, cos, tan, asin, acos, atan, atan2
//! - Logarithms: log, ln, log2
//! - Exponentials: exp, exp2
//! - Advanced: factorial, gamma, erf, hypot
//! - Statistics: mean, median, std, variance, quantile
//! - Vector operations: dot, norm, cross, distance
//! - Matrix operations: determinant, transpose, matmul
//! - Constants: PI, E, TAU, PHI

use crate::evaluator::RuntimeError;
use crate::value::Value;
use std::f64::consts;

// ============================================================================
// 基础数学函数
// ============================================================================

/// 绝对值
///
/// # 功能
/// 返回数字的绝对值（非负值）。
///
/// # 参数
/// - `x`: Number - 输入数字
///
/// # 返回值
/// Number - 输入数字的绝对值
///
/// # 公式
/// ```
/// |x| = { x  if x ≥ 0
///       {-x  if x < 0
/// ```
///
/// # 示例
/// ```aether
/// Set a Abs(-5)           # 5
/// Set b Abs(3.14)         # 3.14
/// Set c Abs(-42.7)        # 42.7
/// ```
pub fn abs(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 向下取整
///
/// # 功能
/// 返回不大于输入数字的最大整数（向负无穷方向取整）。
///
/// # 参数
/// - `x`: Number - 输入数字
///
/// # 返回值
/// Number - 向下取整后的整数值
///
/// # 示例
/// ```aether
/// Set a Floor(3.7)        # 3.0
/// Set b Floor(-2.3)       # -3.0
/// Set c Floor(5.0)        # 5.0
/// ```
pub fn floor(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 向上取整
///
/// # 功能
/// 返回不小于输入数字的最小整数（向正无穷方向取整）。
///
/// # 参数
/// - `x`: Number - 输入数字
///
/// # 返回值
/// Number - 向上取整后的整数值
///
/// # 示例
/// ```aether
/// Set a Ceil(3.2)         # 4.0
/// Set b Ceil(-2.7)        # -2.0
/// Set c Ceil(5.0)         # 5.0
/// ```
pub fn ceil(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 四舍五入
///
/// # 功能
/// 将数字四舍五入到最接近的整数。
///
/// # 参数
/// - `x`: Number - 输入数字
///
/// # 返回值
/// Number - 四舍五入后的整数值
///
/// # 规则
/// - 0.5 向上取整（远离零）
///
/// # 示例
/// ```aether
/// Set a Round(3.4)        # 3.0
/// Set b Round(3.5)        # 4.0
/// Set c Round(-2.5)       # -3.0
/// ```
pub fn round(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.round())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 平方根
///
/// # 功能
/// 计算数字的平方根。
///
/// # 参数
/// - `x`: Number - 输入数字（必须非负）
///
/// # 返回值
/// Number - 平方根值
///
/// # 公式
/// ```
/// √x = y, where y² = x
/// ```
///
/// # 错误
/// - 负数会抛出错误
///
/// # 示例
/// ```aether
/// Set a Sqrt(16)          # 4.0
/// Set b Sqrt(2)           # 1.4142135623730951
/// Set c Sqrt(0)           # 0.0
/// ```
pub fn sqrt(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Cannot take square root of negative number: {}",
                    n
                )));
            }
            Ok(Value::Number(n.sqrt()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 幂运算
///
/// # 功能
/// 计算底数的指数次幂。
///
/// # 参数
/// - `base`: Number - 底数
/// - `exponent`: Number - 指数
///
/// # 返回值
/// Number - base^exponent 的结果
///
/// # 公式
/// ```
/// pow(base, exp) = base^exp
/// ```
///
/// # 示例
/// ```aether
/// Set a Pow(2, 3)         # 8.0 (2³)
/// Set b Pow(10, 2)        # 100.0
/// Set c Pow(4, 0.5)       # 2.0 (√4)
/// Set d Pow(2, -1)        # 0.5 (1/2)
/// ```
pub fn pow(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

// ============================================================================
// 三角函数
// ============================================================================

/// 正弦函数
///
/// # 功能
/// 计算角度的正弦值（输入为弧度）。
///
/// # 参数
/// - `x`: Number - 角度（弧度）
///
/// # 返回值
/// Number - 正弦值，范围 [-1, 1]
///
/// # 公式
/// ```
/// sin(x): 直角三角形中，对边与斜边的比值
/// ```
///
/// # 示例
/// ```aether
/// Set pi PI()
/// Set a Sin(0)            # 0.0
/// Set b Sin(pi / 2)       # 1.0
/// Set c Sin(pi)           # 0.0 (约等于)
/// ```
pub fn sin(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sin())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 余弦函数
///
/// # 功能
/// 计算角度的余弦值（输入为弧度）。
///
/// # 参数
/// - `x`: Number - 角度（弧度）
///
/// # 返回值
/// Number - 余弦值，范围 [-1, 1]
///
/// # 公式
/// ```
/// cos(x): 直角三角形中，邻边与斜边的比值
/// ```
///
/// # 示例
/// ```aether
/// Set pi PI()
/// Set a Cos(0)            # 1.0
/// Set b Cos(pi / 2)       # 0.0 (约等于)
/// Set c Cos(pi)           # -1.0
/// ```
pub fn cos(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.cos())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 正切函数
///
/// # 功能
/// 计算角度的正切值（输入为弧度）。
///
/// # 参数
/// - `x`: Number - 角度（弧度）
///
/// # 返回值
/// Number - 正切值
///
/// # 公式
/// ```
/// tan(x) = sin(x) / cos(x)
/// ```
///
/// # 示例
/// ```aether
/// Set pi PI()
/// Set a Tan(0)            # 0.0
/// Set b Tan(pi / 4)       # 1.0 (约等于)
/// Set c Tan(pi / 6)       # 0.577... (√3/3)
/// ```
pub fn tan(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.tan())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

// ============================================================================
// 对数和指数函数
// ============================================================================

/// 常用对数（以10为底）
///
/// # 功能
/// 计算以 10 为底的对数。
///
/// # 参数
/// - `x`: Number - 输入数字（必须 > 0）
///
/// # 返回值
/// Number - log₁₀(x)
///
/// # 公式
/// ```
/// log₁₀(x) = y  ⟺  10^y = x
/// ```
///
/// # 错误
/// - 非正数会抛出错误
///
/// # 示例
/// ```aether
/// Set a Log(10)           # 1.0
/// Set b Log(100)          # 2.0
/// Set c Log(1000)         # 3.0
/// ```
pub fn log(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n <= 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Cannot take logarithm of non-positive number: {}",
                    n
                )));
            }
            Ok(Value::Number(n.log10()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 自然对数（以e为底）
///
/// # 功能
/// 计算以自然常数 e 为底的对数。
///
/// # 参数
/// - `x`: Number - 输入数字（必须 > 0）
///
/// # 返回值
/// Number - ln(x) = logₑ(x)
///
/// # 公式
/// ```
/// ln(x) = y  ⟺  e^y = x
/// ```
///
/// # 错误
/// - 非正数会抛出错误
///
/// # 示例
/// ```aether
/// Set e E()
/// Set a Ln(e)             # 1.0
/// Set b Ln(1)             # 0.0
/// Set c Ln(e * e)         # 2.0
/// ```
pub fn ln(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n <= 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Cannot take natural logarithm of non-positive number: {}",
                    n
                )));
            }
            Ok(Value::Number(n.ln()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 自然指数函数
///
/// # 功能
/// 计算 e 的 x 次幂。
///
/// # 参数
/// - `x`: Number - 指数
///
/// # 返回值
/// Number - e^x
///
/// # 公式
/// ```
/// exp(x) = e^x
/// ```
///
/// # 示例
/// ```aether
/// Set a Exp(0)            # 1.0
/// Set b Exp(1)            # 2.718281828... (e)
/// Set c Exp(2)            # 7.389056... (e²)
/// ```
pub fn exp(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.exp())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

// ============================================================================
// Advanced Trigonometric Functions
// ============================================================================

/// 反正弦函数
///
/// # 功能
/// 计算正弦的反函数，返回角度（弧度）。
///
/// # 参数
/// - `x`: Number - 正弦值，必须在 [-1, 1] 范围内
///
/// # 返回值
/// Number - 角度（弧度），范围 [-π/2, π/2]
///
/// # 错误
/// - 输入不在 [-1, 1] 范围时抛出错误
///
/// # 示例
/// ```aether
/// Set a Asin(0)           # 0.0
/// Set b Asin(1)           # π/2 ≈ 1.5708
/// Set c Asin(-1)          # -π/2
/// ```
pub fn asin(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n < -1.0 || *n > 1.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "asin domain error: argument must be in [-1, 1], got {}",
                    n
                )));
            }
            Ok(Value::Number(n.asin()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 反余弦函数
///
/// # 功能
/// 计算余弦的反函数，返回角度（弧度）。
///
/// # 参数
/// - `x`: Number - 余弦值，必须在 [-1, 1] 范围内
///
/// # 返回值
/// Number - 角度（弧度），范围 [0, π]
///
/// # 错误
/// - 输入不在 [-1, 1] 范围时抛出错误
///
/// # 示例
/// ```aether
/// Set a Acos(1)           # 0.0
/// Set b Acos(0)           # π/2 ≈ 1.5708
/// Set c Acos(-1)          # π ≈ 3.1416
/// ```
pub fn acos(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n < -1.0 || *n > 1.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "acos domain error: argument must be in [-1, 1], got {}",
                    n
                )));
            }
            Ok(Value::Number(n.acos()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 反正切函数
///
/// # 功能
/// 计算正切的反函数，返回角度（弧度）。
///
/// # 参数
/// - `x`: Number - 正切值
///
/// # 返回值
/// Number - 角度（弧度），范围 (-π/2, π/2)
///
/// # 示例
/// ```aether
/// Set a Atan(0)           # 0.0
/// Set b Atan(1)           # π/4 ≈ 0.7854
/// Set c Atan(-1)          # -π/4
/// ```
pub fn atan(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.atan())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 双参数反正切函数
///
/// # 功能
/// 计算 y/x 的反正切，考虑象限，返回正确的角度。
///
/// # 参数
/// - `y`: Number - y 坐标
/// - `x`: Number - x 坐标
///
/// # 返回值
/// Number - 角度（弧度），范围 [-π, π]
///
/// # 说明
/// 相比 atan(y/x)，atan2 能正确处理所有象限。
///
/// # 示例
/// ```aether
/// Set a Atan2(1, 1)       # π/4 (第一象限)
/// Set b Atan2(1, -1)      # 3π/4 (第二象限)
/// Set c Atan2(-1, -1)     # -3π/4 (第三象限)
/// Set d Atan2(-1, 1)      # -π/4 (第四象限)
/// ```
pub fn atan2(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(y), Value::Number(x)) => Ok(Value::Number(y.atan2(*x))),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 双曲正弦函数
///
/// # 功能
/// 计算双曲正弦值。
///
/// # 参数
/// - `x`: Number - 输入值
///
/// # 返回值
/// Number - sinh(x)
///
/// # 公式
/// ```
/// sinh(x) = (e^x - e^(-x)) / 2
/// ```
///
/// # 示例
/// ```aether
/// Set a Sinh(0)           # 0.0
/// Set b Sinh(1)           # 1.1752...
/// ```
pub fn sinh(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sinh())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 双曲余弦函数
///
/// # 功能
/// 计算双曲余弦值。
///
/// # 参数
/// - `x`: Number - 输入值
///
/// # 返回值
/// Number - cosh(x)
///
/// # 公式
/// ```
/// cosh(x) = (e^x + e^(-x)) / 2
/// ```
///
/// # 示例
/// ```aether
/// Set a Cosh(0)           # 1.0
/// Set b Cosh(1)           # 1.5431...
/// ```
pub fn cosh(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.cosh())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 双曲正切函数
///
/// # 功能
/// 计算双曲正切值。
///
/// # 参数
/// - `x`: Number - 输入值
///
/// # 返回值
/// Number - tanh(x)，范围 (-1, 1)
///
/// # 公式
/// ```
/// tanh(x) = sinh(x) / cosh(x) = (e^x - e^(-x)) / (e^x + e^(-x))
/// ```
///
/// # 示例
/// ```aether
/// Set a Tanh(0)           # 0.0
/// Set b Tanh(1)           # 0.7616...
/// ```
pub fn tanh(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.tanh())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

// ============================================================================
// Advanced Exponential and Logarithmic Functions
// ============================================================================

/// 以2为底的对数
///
/// # 功能
/// 计算以 2 为底的对数。
///
/// # 参数
/// - `x`: Number - 输入数字（必须 > 0）
///
/// # 返回值
/// Number - log₂(x)
///
/// # 公式
/// ```
/// log₂(x) = y  ⟺  2^y = x
/// ```
///
/// # 错误
/// - 非正数会抛出错误
///
/// # 示例
/// ```aether
/// Set a Log2(2)           # 1.0
/// Set b Log2(8)           # 3.0
/// Set c Log2(1024)        # 10.0
/// ```
pub fn log2(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n <= 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Cannot take log2 of non-positive number: {}",
                    n
                )));
            }
            Ok(Value::Number(n.log2()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 2的幂运算
///
/// # 功能
/// 计算 2 的 x 次幂。
///
/// # 参数
/// - `x`: Number - 指数
///
/// # 返回值
/// Number - 2^x
///
/// # 公式
/// ```
/// exp2(x) = 2^x
/// ```
///
/// # 示例
/// ```aether
/// Set a Exp2(3)           # 8.0
/// Set b Exp2(10)          # 1024.0
/// Set c Exp2(-1)          # 0.5
/// ```
pub fn exp2(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.exp2())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// exp(x) - 1 精确计算
///
/// # 功能
/// 计算 e^x - 1，对于接近 0 的 x 值更精确。
///
/// # 参数
/// - `x`: Number - 指数
///
/// # 返回值
/// Number - e^x - 1
///
/// # 说明
/// 当 x 接近 0 时，直接计算 exp(x) - 1 会有精度损失，此函数使用特殊算法避免这个问题。
///
/// # 示例
/// ```aether
/// Set a Expm1(0)          # 0.0
/// Set b Expm1(0.001)      # 0.0010005...
/// ```
pub fn expm1(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.exp_m1())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// ln(1 + x) 精确计算
///
/// # 功能
/// 计算 ln(1 + x)，对于接近 0 的 x 值更精确。
///
/// # 参数
/// - `x`: Number - 输入值（必须 > -1）
///
/// # 返回值
/// Number - ln(1 + x)
///
/// # 错误
/// - x ≤ -1 时抛出错误
///
/// # 说明
/// 当 x 接近 0 时，直接计算 ln(1 + x) 会有精度损失，此函数使用特殊算法避免这个问题。
///
/// # 示例
/// ```aether
/// Set a Log1p(0)          # 0.0
/// Set b Log1p(0.001)      # 0.0009995...
/// ```
pub fn log1p(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n <= -1.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "log1p domain error: argument must be > -1, got {}",
                    n
                )));
            }
            Ok(Value::Number(n.ln_1p()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

// ============================================================================
// Special Mathematical Functions
// ============================================================================

/// 阶乘
///
/// # 功能
/// 计算非负整数的阶乘。
///
/// # 参数
/// - `n`: Number - 非负整数
///
/// # 返回值
/// Number - n! = n × (n-1) × ... × 2 × 1
///
/// # 公式
/// ```
/// 0! = 1
/// n! = n × (n-1)!  (n > 0)
/// ```
///
/// # 错误
/// - 非整数或负数会抛出错误
/// - n > 170 会溢出
///
/// # 示例
/// ```aether
/// Set a Factorial(0)      # 1
/// Set b Factorial(5)      # 120
/// Set c Factorial(10)     # 3628800
/// ```
pub fn factorial(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Factorial requires non-negative integer, got {}",
                    n
                )));
            }

            let n_int = *n as u32;
            if n_int > 170 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Factorial overflow: {} is too large",
                    n_int
                )));
            }

            let mut result = 1.0;
            for i in 2..=n_int {
                result *= i as f64;
            }
            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// Gamma 函数（广义阶乘）
///
/// # 功能
/// 计算 Gamma 函数，是阶乘的连续扩展。
///
/// # 参数
/// - `x`: Number - 输入值
///
/// # 返回值
/// Number - Γ(x)
///
/// # 公式
/// ```
/// Γ(n) = (n-1)!  (n为正整数)
/// Γ(x) ≈ √(2π/x) × (x/e)^x  (Stirling近似)
/// ```
///
/// # 错误
/// - 非正整数会抛出错误
///
/// # 示例
/// ```aether
/// Set a Gamma(1)          # 1 (0!)
/// Set b Gamma(5)          # 24 (4!)
/// Set c Gamma(0.5)        # √π ≈ 1.77245
/// ```
pub fn gamma(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            // Using Stirling's approximation for gamma function
            // Gamma(x) ≈ sqrt(2*pi/x) * (x/e)^x
            if *n <= 0.0 && n.fract() == 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Gamma function undefined for non-positive integers: {}",
                    n
                )));
            }

            // For integers, use factorial
            if n.fract() == 0.0 && *n > 0.0 {
                return factorial(&[Value::Number(*n - 1.0)]);
            }

            // Stirling's approximation
            let x = *n;
            let result = (2.0 * consts::PI / x).sqrt() * (x / consts::E).powf(x);
            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 误差函数
///
/// # 功能
/// 计算高斯误差函数，常用于概率和统计。
///
/// # 参数
/// - `x`: Number - 输入值
///
/// # 返回值
/// Number - erf(x)，范围 (-1, 1)
///
/// # 公式
/// ```
/// erf(x) = (2/√π) ∫₀ˣ e^(-t²) dt
/// ```
///
/// # 应用
/// - 正态分布累积分布函数
/// - 概率计算
///
/// # 示例
/// ```aether
/// Set a Erf(0)            # 0.0
/// Set b Erf(1)            # 0.8427... (约84.27%概率)
/// Set c Erf(-1)           # -0.8427...
/// ```
pub fn erf(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(x) => {
            // Abramowitz and Stegun approximation
            let a1 = 0.254829592;
            let a2 = -0.284496736;
            let a3 = 1.421413741;
            let a4 = -1.453152027;
            let a5 = 1.061405429;
            let p = 0.3275911;

            let sign = if *x < 0.0 { -1.0 } else { 1.0 };
            let x_abs = x.abs();

            let t = 1.0 / (1.0 + p * x_abs);
            let y =
                1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x_abs * x_abs).exp();

            Ok(Value::Number(sign * y))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 斜边长度（勾股定理）
///
/// # 功能
/// 计算直角三角形的斜边长度，即 √(x² + y²)。
///
/// # 参数
/// - `x`: Number - 直角边 x
/// - `y`: Number - 直角边 y
///
/// # 返回值
/// Number - 斜边长度
///
/// # 公式
/// ```
/// hypot(x, y) = √(x² + y²)
/// ```
///
/// # 说明
/// 使用特殊算法避免中间计算溢出。
///
/// # 示例
/// ```aether
/// Set c Hypot(3, 4)       # 5.0
/// Set c Hypot(5, 12)      # 13.0
/// Set dist Hypot(1, 1)    # √2 ≈ 1.414
/// ```
pub fn hypot(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x.hypot(*y))),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 符号函数
///
/// # 功能
/// 返回数字的符号：正数返回 1，负数返回 -1，零返回 0。
///
/// # 参数
/// - `x`: Number - 输入数字
///
/// # 返回值
/// Number - 1、0 或 -1
///
/// # 公式
/// ```
/// sign(x) = { -1  if x < 0
///           {  0  if x = 0
///           {  1  if x > 0
/// ```
///
/// # 示例
/// ```aether
/// Set a Sign(5)           # 1
/// Set b Sign(-3.14)       # -1
/// Set c Sign(0)           # 0
/// ```
pub fn sign(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            let result = if *n > 0.0 {
                1.0
            } else if *n < 0.0 {
                -1.0
            } else {
                0.0
            };
            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 限制值在指定范围内
///
/// # 功能
/// 将数值限制在指定的最小值和最大值之间。
///
/// # 参数
/// - `x`: Number - 要限制的值
/// - `min`: Number - 最小值
/// - `max`: Number - 最大值
///
/// # 返回值
/// Number - 限制后的值
///
/// # 公式
/// ```
/// clamp(x, min, max) = { min  if x < min
///                      { x    if min ≤ x ≤ max
///                      { max  if x > max
/// ```
///
/// # 错误
/// - min > max 时抛出错误
///
/// # 示例
/// ```aether
/// Set a Clamp(5, 0, 10)       # 5 (在范围内)
/// Set b Clamp(-5, 0, 10)      # 0 (小于min)
/// Set c Clamp(15, 0, 10)      # 10 (大于max)
/// ```
pub fn clamp(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::Number(x), Value::Number(min), Value::Number(max)) => {
            if min > max {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Clamp: min ({}) must be <= max ({})",
                    min, max
                )));
            }
            Ok(Value::Number(x.clamp(*min, *max)))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number, Number".to_string(),
            got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
        }),
    }
}

// ============================================================================
// Statistics Functions
// ============================================================================

/// 计算平均值（均值）
///
/// # 功能
/// 计算数字数组的算术平均值。
///
/// # 参数
/// - `array`: Array - 数字数组
///
/// # 返回值
/// Number - 平均值
///
/// # 公式
/// ```
/// mean = (x₁ + x₂ + ... + xₙ) / n
/// ```
///
/// # 错误
/// - 空数组会抛出错误
/// - 数组包含非数字元素时抛出类型错误
///
/// # 示例
/// ```aether
/// Set scores [85, 90, 78, 92, 88]
/// Set avg Mean(scores)        # 86.6
/// Set temps [20.5, 22.0, 21.5, 19.5]
/// Set avg Mean(temps)         # 20.875
/// ```
pub fn mean(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(RuntimeError::InvalidOperation(
                    "Cannot compute mean of empty array".to_string(),
                ));
            }

            let mut sum = 0.0;
            for val in arr {
                match val {
                    Value::Number(n) => sum += n,
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }
            Ok(Value::Number(sum / arr.len() as f64))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 计算中位数
///
/// # 功能
/// 计算数字数组的中位数（排序后的中间值）。
///
/// # 参数
/// - `array`: Array - 数字数组
///
/// # 返回值
/// Number - 中位数
///
/// # 规则
/// - 奇数个元素：返回中间的值
/// - 偶数个元素：返回中间两个值的平均
///
/// # 错误
/// - 空数组会抛出错误
///
/// # 示例
/// ```aether
/// Set nums [1, 3, 5, 7, 9]
/// Set med Median(nums)        # 5 (中间值)
/// Set nums [1, 2, 3, 4]
/// Set med Median(nums)        # 2.5 ((2+3)/2)
/// ```
pub fn median(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(RuntimeError::InvalidOperation(
                    "Cannot compute median of empty array".to_string(),
                ));
            }

            let mut numbers: Vec<f64> = Vec::new();
            for val in arr {
                match val {
                    Value::Number(n) => numbers.push(*n),
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }

            numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = numbers.len() / 2;

            let result = if numbers.len() % 2 == 0 {
                (numbers[mid - 1] + numbers[mid]) / 2.0
            } else {
                numbers[mid]
            };

            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 计算方差
///
/// # 功能
/// 计算数字数组的样本方差（数据分散程度的度量）。
///
/// # 参数
/// - `array`: Array - 数字数组
///
/// # 返回值
/// Number - 样本方差
///
/// # 公式
/// ```
/// variance = Σ(xᵢ - mean)² / (n - 1)
/// ```
///
/// # 错误
/// - 少于 2 个元素时抛出错误
///
/// # 示例
/// ```aether
/// Set data [2, 4, 6, 8, 10]
/// Set var Variance(data)      # 10.0
/// Set scores [85, 90, 78]
/// Set var Variance(scores)    # 36.0
/// ```
pub fn variance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.len() < 2 {
                return Err(RuntimeError::InvalidOperation(
                    "Variance requires at least 2 values".to_string(),
                ));
            }

            // Calculate mean
            let mean_result = mean(args)?;
            let mean_val = match mean_result {
                Value::Number(n) => n,
                _ => unreachable!(),
            };

            // Calculate variance
            let mut sum_sq_diff = 0.0;
            for val in arr {
                match val {
                    Value::Number(n) => {
                        let diff = n - mean_val;
                        sum_sq_diff += diff * diff;
                    }
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }

            Ok(Value::Number(sum_sq_diff / (arr.len() - 1) as f64))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 计算标准差
///
/// # 功能
/// 计算数字数组的样本标准差（方差的平方根）。
///
/// # 参数
/// - `array`: Array - 数字数组
///
/// # 返回值
/// Number - 样本标准差
///
/// # 公式
/// ```
/// std = √variance = √[Σ(xᵢ - mean)² / (n - 1)]
/// ```
///
/// # 错误
/// - 少于 2 个元素时抛出错误
///
/// # 示例
/// ```aether
/// Set data [2, 4, 6, 8, 10]
/// Set sd Std(data)            # 3.162...
/// Set scores [85, 90, 78]
/// Set sd Std(scores)          # 6.0
/// ```
pub fn std(args: &[Value]) -> Result<Value, RuntimeError> {
    let var = variance(args)?;
    match var {
        Value::Number(v) => Ok(Value::Number(v.sqrt())),
        _ => unreachable!(),
    }
}

/// 计算分位数（百分位数）
///
/// # 功能
/// 计算数组的指定分位数。
///
/// # 参数
/// - `array`: Array - 数字数组
/// - `q`: Number - 分位数，范围 [0, 1]
///
/// # 返回值
/// Number - 第 q 分位数的值
///
/// # 说明
/// - 0.0 返回最小值
/// - 0.5 返回中位数
/// - 1.0 返回最大值
/// - 使用线性插值
///
/// # 错误
/// - 空数组或 q 不在 [0, 1] 范围时抛出错误
///
/// # 示例
/// ```aether
/// Set data [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// Set q25 Quantile(data, 0.25)    # 3.25 (第一四分位数)
/// Set q50 Quantile(data, 0.5)     # 5.5 (中位数)
/// Set q75 Quantile(data, 0.75)    # 7.75 (第三四分位数)
/// ```
pub fn quantile(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::Number(q)) => {
            if arr.is_empty() {
                return Err(RuntimeError::InvalidOperation(
                    "Cannot compute quantile of empty array".to_string(),
                ));
            }

            if *q < 0.0 || *q > 1.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Quantile must be in [0, 1], got {}",
                    q
                )));
            }

            let mut numbers: Vec<f64> = Vec::new();
            for val in arr {
                match val {
                    Value::Number(n) => numbers.push(*n),
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }

            numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let index = q * (numbers.len() - 1) as f64;
            let lower = index.floor() as usize;
            let upper = index.ceil() as usize;

            let result = if lower == upper {
                numbers[lower]
            } else {
                let weight = index - lower as f64;
                numbers[lower] * (1.0 - weight) + numbers[upper] * weight
            };

            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

// ============================================================================
// Vector Operations (NumPy-like)
// ============================================================================

/// 向量点积
///
/// # 功能
/// 计算两个向量的点积（内积）。
///
/// # 参数
/// - `a`: Array - 第一个向量（数字数组）
/// - `b`: Array - 第二个向量（数字数组）
///
/// # 返回值
/// Number - 点积结果
///
/// # 公式
/// ```
/// dot(a, b) = a₁b₁ + a₂b₂ + ... + aₙbₙ
/// ```
///
/// # 错误
/// - 两个向量长度不同时抛出错误
///
/// # 示例
/// ```aether
/// Set a [1, 2, 3]
/// Set b [4, 5, 6]
/// Set d Dot(a, b)             # 32 (1*4 + 2*5 + 3*6)
/// Set v1 [1, 0, 0]
/// Set v2 [0, 1, 0]
/// Set d Dot(v1, v2)           # 0 (正交向量)
/// ```
pub fn dot(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Dot product requires equal length vectors: {} vs {}",
                    a.len(),
                    b.len()
                )));
            }

            let mut result = 0.0;
            for (val_a, val_b) in a.iter().zip(b.iter()) {
                match (val_a, val_b) {
                    (Value::Number(na), Value::Number(nb)) => result += na * nb,
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Arrays containing {:?} and {:?}", val_a, val_b),
                        })
                    }
                }
            }

            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, Array".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 向量范数（模长）
///
/// # 功能
/// 计算向量的欧几里得范数（L2范数、模长、大小）。
///
/// # 参数
/// - `vector`: Array - 向量（数字数组）
///
/// # 返回值
/// Number - 向量的模长
///
/// # 公式
/// ```
/// ||v|| = √(v₁² + v₂² + ... + vₙ²)
/// ```
///
/// # 示例
/// ```aether
/// Set v [3, 4]
/// Set len Norm(v)             # 5.0 (√(3²+4²))
/// Set v [1, 1, 1]
/// Set len Norm(v)             # 1.732... (√3)
/// ```
pub fn norm(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let mut sum_sq = 0.0;
            for val in arr {
                match val {
                    Value::Number(n) => sum_sq += n * n,
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }
            Ok(Value::Number(sum_sq.sqrt()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 向量叉积
///
/// # 功能
/// 计算两个三维向量的叉积（外积）。
///
/// # 参数
/// - `a`: Array - 第一个3D向量 [x₁, y₁, z₁]
/// - `b`: Array - 第二个3D向量 [x₂, y₂, z₂]
///
/// # 返回值
/// Array - 叉积向量，垂直于输入的两个向量
///
/// # 公式
/// ```
/// a × b = [a₂b₃ - a₃b₂, a₃b₁ - a₁b₃, a₁b₂ - a₂b₁]
/// ```
///
/// # 错误
/// - 输入不是3D向量时抛出错误
///
/// # 示例
/// ```aether
/// Set a [1, 0, 0]
/// Set b [0, 1, 0]
/// Set c Cross(a, b)           # [0, 0, 1] (z轴方向)
/// Set a [1, 2, 3]
/// Set b [4, 5, 6]
/// Set c Cross(a, b)           # [-3, 6, -3]
/// ```
pub fn cross(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != 3 || b.len() != 3 {
                return Err(RuntimeError::InvalidOperation(
                    "Cross product requires 3D vectors".to_string(),
                ));
            }

            let (a1, a2, a3) = match (&a[0], &a[1], &a[2]) {
                (Value::Number(x), Value::Number(y), Value::Number(z)) => (*x, *y, *z),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "Array of Numbers".to_string(),
                        got: format!("Array containing non-numbers"),
                    })
                }
            };

            let (b1, b2, b3) = match (&b[0], &b[1], &b[2]) {
                (Value::Number(x), Value::Number(y), Value::Number(z)) => (*x, *y, *z),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "Array of Numbers".to_string(),
                        got: format!("Array containing non-numbers"),
                    })
                }
            };

            Ok(Value::Array(vec![
                Value::Number(a2 * b3 - a3 * b2),
                Value::Number(a3 * b1 - a1 * b3),
                Value::Number(a1 * b2 - a2 * b1),
            ]))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, Array".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 向量间的欧几里得距离
///
/// # 功能
/// 计算两个向量之间的欧几里得距离。
///
/// # 参数
/// - `a`: Array - 第一个向量
/// - `b`: Array - 第二个向量
///
/// # 返回值
/// Number - 欧几里得距离
///
/// # 公式
/// ```
/// distance(a, b) = ||a - b|| = √(Σ(aᵢ - bᵢ)²)
/// ```
///
/// # 错误
/// - 两个向量长度不同时抛出错误
///
/// # 示例
/// ```aether
/// Set p1 [0, 0]
/// Set p2 [3, 4]
/// Set d Distance(p1, p2)      # 5.0
/// Set a [1, 2, 3]
/// Set b [4, 5, 6]
/// Set d Distance(a, b)        # 5.196... (√27)
/// ```
pub fn distance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Distance requires equal length vectors: {} vs {}",
                    a.len(),
                    b.len()
                )));
            }

            let mut sum_sq = 0.0;
            for (val_a, val_b) in a.iter().zip(b.iter()) {
                match (val_a, val_b) {
                    (Value::Number(na), Value::Number(nb)) => {
                        let diff = na - nb;
                        sum_sq += diff * diff;
                    }
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Arrays containing {:?} and {:?}", val_a, val_b),
                        })
                    }
                }
            }

            Ok(Value::Number(sum_sq.sqrt()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, Array".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 向量归一化
///
/// # 功能
/// 将向量归一化为单位向量（模长为1）。
///
/// # 参数
/// - `vector`: Array - 输入向量
///
/// # 返回值
/// Array - 归一化后的单位向量
///
/// # 公式
/// ```
/// normalize(v) = v / ||v||
/// ```
///
/// # 错误
/// - 零向量无法归一化
///
/// # 示例
/// ```aether
/// Set v [3, 4]
/// Set unit Normalize(v)       # [0.6, 0.8]
/// Set v [1, 1, 1]
/// Set unit Normalize(v)       # [0.577..., 0.577..., 0.577...]
/// ```
pub fn normalize(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    let norm_result = norm(args)?;
    let norm_val = match norm_result {
        Value::Number(n) => n,
        _ => unreachable!(),
    };

    if norm_val == 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "Cannot normalize zero vector".to_string(),
        ));
    }

    match &args[0] {
        Value::Array(arr) => {
            let normalized: Vec<Value> = arr
                .iter()
                .map(|v| {
                    match v {
                        Value::Number(n) => Value::Number(n / norm_val),
                        _ => unreachable!(), // Already validated in norm()
                    }
                })
                .collect();

            Ok(Value::Array(normalized))
        }
        _ => unreachable!(), // Already validated in norm()
    }
}

// ============================================================================
// Matrix Operations (NumPy-like)
// ============================================================================

/// 矩阵乘法
///
/// # 功能
/// 计算两个矩阵的乘积。
///
/// # 参数
/// - `A`: Array - 第一个矩阵（二维数组）[m × n]
/// - `B`: Array - 第二个矩阵（二维数组）[n × p]
///
/// # 返回值
/// Array - 矩阵乘积 [m × p]
///
/// # 公式
/// ```
/// C[i][j] = Σ A[i][k] × B[k][j]  (k = 0 to n-1)
/// ```
///
/// # 错误
/// - A的列数必须等于B的行数
///
/// # 示例
/// ```aether
/// Set A [[1, 2], [3, 4]]
/// Set B [[5, 6], [7, 8]]
/// Set C Matmul(A, B)          # [[19, 22], [43, 50]]
/// Set I [[1, 0], [0, 1]]
/// Set R Matmul(A, I)          # [[1, 2], [3, 4]] (单位矩阵)
/// ```
pub fn matmul(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(a), Value::Array(b)) => {
            // Extract dimensions
            let rows_a = a.len();
            if rows_a == 0 {
                return Err(RuntimeError::InvalidOperation(
                    "Matrix A is empty".to_string(),
                ));
            }

            let cols_a = match &a[0] {
                Value::Array(row) => row.len(),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "2D Array (Array of Arrays)".to_string(),
                        got: format!("Array containing {:?}", a[0]),
                    })
                }
            };

            let rows_b = b.len();
            if rows_b == 0 {
                return Err(RuntimeError::InvalidOperation(
                    "Matrix B is empty".to_string(),
                ));
            }

            let cols_b = match &b[0] {
                Value::Array(row) => row.len(),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "2D Array (Array of Arrays)".to_string(),
                        got: format!("Array containing {:?}", b[0]),
                    })
                }
            };

            if cols_a != rows_b {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Matrix dimensions incompatible: ({}, {}) × ({}, {})",
                    rows_a, cols_a, rows_b, cols_b
                )));
            }

            // Perform multiplication
            let mut result = Vec::new();
            for i in 0..rows_a {
                let row_a = match &a[i] {
                    Value::Array(r) => r,
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "2D Array".to_string(),
                            got: format!("Non-uniform array structure"),
                        })
                    }
                };

                let mut result_row = Vec::new();
                for j in 0..cols_b {
                    let mut sum = 0.0;
                    for k in 0..cols_a {
                        let a_val = match &row_a[k] {
                            Value::Number(n) => *n,
                            _ => {
                                return Err(RuntimeError::TypeErrorDetailed {
                                    expected: "Number".to_string(),
                                    got: format!("{:?}", row_a[k]),
                                })
                            }
                        };

                        let row_b = match &b[k] {
                            Value::Array(r) => r,
                            _ => {
                                return Err(RuntimeError::TypeErrorDetailed {
                                    expected: "2D Array".to_string(),
                                    got: format!("Non-uniform array structure"),
                                })
                            }
                        };

                        let b_val = match &row_b[j] {
                            Value::Number(n) => *n,
                            _ => {
                                return Err(RuntimeError::TypeErrorDetailed {
                                    expected: "Number".to_string(),
                                    got: format!("{:?}", row_b[j]),
                                })
                            }
                        };

                        sum += a_val * b_val;
                    }
                    result_row.push(Value::Number(sum));
                }
                result.push(Value::Array(result_row));
            }

            Ok(Value::Array(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, Array".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 矩阵转置
///
/// # 功能
/// 计算矩阵的转置（行列互换）。
///
/// # 参数
/// - `matrix`: Array - 输入矩阵（二维数组）[m × n]
///
/// # 返回值
/// Array - 转置后的矩阵 [n × m]
///
/// # 公式
/// ```
/// B[j][i] = A[i][j]
/// ```
///
/// # 示例
/// ```aether
/// Set A [[1, 2, 3], [4, 5, 6]]
/// Set B Transpose(A)          # [[1, 4], [2, 5], [3, 6]]
/// Set M [[1, 2], [3, 4]]
/// Set MT Transpose(M)         # [[1, 3], [2, 4]]
/// ```
pub fn transpose(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(matrix) => {
            if matrix.is_empty() {
                return Ok(Value::Array(vec![]));
            }

            let rows = matrix.len();
            let cols = match &matrix[0] {
                Value::Array(row) => row.len(),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "2D Array".to_string(),
                        got: format!("Array containing {:?}", matrix[0]),
                    })
                }
            };

            if cols == 0 {
                return Ok(Value::Array(vec![]));
            }

            let mut result = vec![vec![Value::Null; rows]; cols];

            for i in 0..rows {
                let row = match &matrix[i] {
                    Value::Array(r) => r,
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "2D Array".to_string(),
                            got: format!("Non-uniform array structure"),
                        })
                    }
                };

                if row.len() != cols {
                    return Err(RuntimeError::InvalidOperation(
                        "All rows must have same length".to_string(),
                    ));
                }

                for j in 0..cols {
                    result[j][i] = row[j].clone();
                }
            }

            let result_arrays: Vec<Value> = result.into_iter().map(Value::Array).collect();

            Ok(Value::Array(result_arrays))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 计算行列式
///
/// # 功能
/// 计算方阵的行列式（支持 1×1、2×2、3×3 矩阵）。
///
/// # 参数
/// - `matrix`: Array - 方阵（二维数组）
///
/// # 返回值
/// Number - 行列式的值
///
/// # 公式
/// - 2×2: det = ad - bc
/// - 3×3: Sarrus规则
///
/// # 错误
/// - 非方阵或维度 > 3 时抛出错误
///
/// # 示例
/// ```aether
/// Set A [[1, 2], [3, 4]]
/// Set det Determinant(A)      # -2.0 (1*4 - 2*3)
/// Set I [[1, 0], [0, 1]]
/// Set det Determinant(I)      # 1.0 (单位矩阵)
/// Set B [[1, 2, 3], [0, 1, 4], [5, 6, 0]]
/// Set det Determinant(B)      # 1.0
/// ```
pub fn determinant(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(matrix) => {
            let n = matrix.len();

            if n == 0 {
                return Err(RuntimeError::InvalidOperation(
                    "Matrix is empty".to_string(),
                ));
            }

            // Verify square matrix
            for row in matrix {
                match row {
                    Value::Array(r) => {
                        if r.len() != n {
                            return Err(RuntimeError::InvalidOperation(
                                "Determinant requires square matrix".to_string(),
                            ));
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "2D Array".to_string(),
                            got: format!("Array containing {:?}", row),
                        })
                    }
                }
            }

            // Calculate determinant based on size
            match n {
                1 => {
                    // 1x1 matrix
                    match &matrix[0] {
                        Value::Array(row) => match &row[0] {
                            Value::Number(n) => Ok(Value::Number(*n)),
                            _ => Err(RuntimeError::TypeErrorDetailed {
                                expected: "Number".to_string(),
                                got: format!("{:?}", row[0]),
                            }),
                        },
                        _ => unreachable!(),
                    }
                }
                2 => {
                    // 2x2 matrix: ad - bc
                    let a = get_matrix_element(matrix, 0, 0)?;
                    let b = get_matrix_element(matrix, 0, 1)?;
                    let c = get_matrix_element(matrix, 1, 0)?;
                    let d = get_matrix_element(matrix, 1, 1)?;

                    Ok(Value::Number(a * d - b * c))
                }
                3 => {
                    // 3x3 matrix: Sarrus' rule
                    let a = get_matrix_element(matrix, 0, 0)?;
                    let b = get_matrix_element(matrix, 0, 1)?;
                    let c = get_matrix_element(matrix, 0, 2)?;
                    let d = get_matrix_element(matrix, 1, 0)?;
                    let e = get_matrix_element(matrix, 1, 1)?;
                    let f = get_matrix_element(matrix, 1, 2)?;
                    let g = get_matrix_element(matrix, 2, 0)?;
                    let h = get_matrix_element(matrix, 2, 1)?;
                    let i = get_matrix_element(matrix, 2, 2)?;

                    let det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
                    Ok(Value::Number(det))
                }
                _ => {
                    // For larger matrices, use recursive cofactor expansion
                    determinant_recursive(matrix)
                }
            }
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 递归计算任意大小方阵的行列式（余子式展开法）
fn determinant_recursive(matrix: &[Value]) -> Result<Value, RuntimeError> {
    let n = matrix.len();

    if n == 1 {
        return match &matrix[0] {
            Value::Array(row) => match &row[0] {
                Value::Number(val) => Ok(Value::Number(*val)),
                _ => Err(RuntimeError::TypeErrorDetailed {
                    expected: "Number".to_string(),
                    got: format!("Non-numeric value in matrix"),
                }),
            },
            _ => Err(RuntimeError::TypeErrorDetailed {
                expected: "Array".to_string(),
                got: format!("Invalid matrix structure"),
            }),
        };
    }

    let mut det = 0.0;

    // Expand along first row
    for j in 0..n {
        let element = get_matrix_element(matrix, 0, j)?;

        // Create minor matrix (remove row 0 and column j)
        let mut minor = Vec::new();
        for i in 1..n {
            let mut row = Vec::new();
            match &matrix[i] {
                Value::Array(matrix_row) => {
                    for k in 0..n {
                        if k != j {
                            row.push(matrix_row[k].clone());
                        }
                    }
                }
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "Array".to_string(),
                        got: format!("Invalid matrix row"),
                    })
                }
            }
            minor.push(Value::Array(row));
        }

        // Recursive call
        let minor_det = determinant_recursive(&minor)?;
        let minor_val = match minor_det {
            Value::Number(v) => v,
            _ => unreachable!(),
        };

        // Add to determinant with alternating signs
        let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
        det += sign * element * minor_val;
    }

    Ok(Value::Number(det))
}

/// 矩阵求逆
///
/// # 功能
/// 计算方阵的逆矩阵（使用高斯-约旦消元法）。
///
/// # 参数
/// - `matrix`: Array - 可逆方阵（二维数组）
///
/// # 返回值
/// Array - 逆矩阵
///
/// # 公式
/// ```
/// A * A⁻¹ = I (单位矩阵)
/// ```
///
/// # 错误
/// - 非方阵或奇异矩阵（行列式为0）时抛出错误
///
/// # 示例
/// ```aether
/// Set A [[4, 7], [2, 6]]
/// Set invA Inverse(A)         # [[0.6, -0.7], [-0.2, 0.4]]
/// Set I Matmul(A, invA)       # [[1, 0], [0, 1]]
/// Set A [[1, 2, 3], [0, 1, 4], [5, 6, 0]]
/// Set invA Inverse(A)
/// ```
pub fn matrix_inverse(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(matrix) => {
            let n = matrix.len();

            if n == 0 {
                return Err(RuntimeError::InvalidOperation(
                    "Matrix is empty".to_string(),
                ));
            }

            // Verify square matrix and extract values
            let mut mat: Vec<Vec<f64>> = Vec::new();
            for row_val in matrix {
                match row_val {
                    Value::Array(row) => {
                        if row.len() != n {
                            return Err(RuntimeError::InvalidOperation(
                                "Matrix must be square".to_string(),
                            ));
                        }
                        let mut num_row = Vec::new();
                        for val in row {
                            match val {
                                Value::Number(num) => num_row.push(*num),
                                _ => {
                                    return Err(RuntimeError::TypeErrorDetailed {
                                        expected: "Number".to_string(),
                                        got: format!("{:?}", val),
                                    })
                                }
                            }
                        }
                        mat.push(num_row);
                    }
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array".to_string(),
                            got: format!("{:?}", row_val),
                        })
                    }
                }
            }

            // Create augmented matrix [A | I]
            let mut aug = vec![vec![0.0; 2 * n]; n];
            for i in 0..n {
                for j in 0..n {
                    aug[i][j] = mat[i][j];
                    aug[i][n + j] = if i == j { 1.0 } else { 0.0 };
                }
            }

            // Gaussian-Jordan elimination
            for i in 0..n {
                // Find pivot
                let mut max_row = i;
                for k in (i + 1)..n {
                    if aug[k][i].abs() > aug[max_row][i].abs() {
                        max_row = k;
                    }
                }

                // Swap rows
                aug.swap(i, max_row);

                // Check for singular matrix
                if aug[i][i].abs() < 1e-10 {
                    return Err(RuntimeError::InvalidOperation(
                        "Matrix is singular (not invertible)".to_string(),
                    ));
                }

                // Scale pivot row
                let pivot = aug[i][i];
                for j in 0..(2 * n) {
                    aug[i][j] /= pivot;
                }

                // Eliminate column
                for k in 0..n {
                    if k != i {
                        let factor = aug[k][i];
                        for j in 0..(2 * n) {
                            aug[k][j] -= factor * aug[i][j];
                        }
                    }
                }
            }

            // Extract inverse matrix from augmented matrix
            let mut result = Vec::new();
            for i in 0..n {
                let mut row = Vec::new();
                for j in n..(2 * n) {
                    row.push(Value::Number(aug[i][j]));
                }
                result.push(Value::Array(row));
            }

            Ok(Value::Array(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

// Helper function for matrix element access
fn get_matrix_element(matrix: &[Value], i: usize, j: usize) -> Result<f64, RuntimeError> {
    match &matrix[i] {
        Value::Array(row) => match &row[j] {
            Value::Number(n) => Ok(*n),
            _ => Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", row[j]),
            }),
        },
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", matrix[i]),
        }),
    }
}

// ============================================================================
// Mathematical Constants
// ============================================================================

/// 圆周率 π
///
/// # 功能
/// 返回数学常数 π（圆周率）。
///
/// # 返回值
/// Number - π ≈ 3.141592653589793
///
/// # 说明
/// π 是圆的周长与直径的比值。
///
/// # 示例
/// ```aether
/// Set pi PI()                 # 3.141592653589793
/// Set circumference PI() * 2 * radius
/// Set angle PI() / 4          # 45度 (弧度制)
/// ```
pub fn pi(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Number(consts::PI))
}

/// 自然常数 e
///
/// # 功能
/// 返回数学常数 e（自然常数、欧拉数）。
///
/// # 返回值
/// Number - e ≈ 2.718281828459045
///
/// # 说明
/// e 是自然对数的底数。
///
/// # 示例
/// ```aether
/// Set e E()                   # 2.718281828459045
/// Set y Exp(1)                # 也等于 e
/// Set growth E() * rate       # 指数增长
/// ```
pub fn e(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Number(consts::E))
}

/// 圆周率 τ (TAU)
///
/// # 功能
/// 返回数学常数 τ = 2π。
///
/// # 返回值
/// Number - τ ≈ 6.283185307179586
///
/// # 说明
/// τ 是圆的周长与半径的比值，等于 2π。
///
/// # 示例
/// ```aether
/// Set tau TAU()               # 6.283185307179586
/// Set fullCircle TAU()        # 完整圆周 (360度)
/// Set angle TAU() / 8         # 1/8圆周 (45度)
/// ```
pub fn tau(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Number(consts::TAU))
}

/// 黄金比例 φ (PHI)
///
/// # 功能
/// 返回数学常数 φ（黄金比例）。
///
/// # 返回值
/// Number - φ ≈ 1.618033988749895
///
/// # 公式
/// ```
/// φ = (1 + √5) / 2
/// ```
///
/// # 说明
/// 黄金比例在几何、艺术和自然界中广泛出现。
///
/// # 示例
/// ```aether
/// Set phi PHI()               # 1.618033988749895
/// Set ratio PHI()             # 黄金分割比例
/// Set fibonacci PHI() * PHI() # φ² ≈ 2.618
/// ```
pub fn phi(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Number((1.0 + 5.0_f64.sqrt()) / 2.0))
}

// ============================================================================
// Advanced Statistics - Linear Regression
// ============================================================================

/// 简单线性回归
///
/// # 功能
/// 对数据进行简单线性回归分析，返回斜率和截距。
///
/// # 参数
/// - `x`: Array - 自变量数组
/// - `y`: Array - 因变量数组
///
/// # 返回值
/// Array - [slope, intercept, r_squared]
/// - slope: 斜率
/// - intercept: 截距
/// - r_squared: 决定系数 R²
///
/// # 公式
/// ```
/// y = slope * x + intercept
/// slope = Σ[(xi - x̄)(yi - ȳ)] / Σ(xi - x̄)²
/// intercept = ȳ - slope * x̄
/// R² = 1 - SS_res / SS_tot
/// ```
///
/// # 错误
/// - 两个数组长度必须相同
/// - 至少需要 2 个数据点
///
/// # 示例
/// ```aether
/// Set x [1, 2, 3, 4, 5]
/// Set y [2, 4, 5, 4, 5]
/// Set result LinearRegression(x, y)
/// Set slope result[0]         # 0.6
/// Set intercept result[1]     # 2.2
/// Set r2 result[2]            # 0.4286
/// ```
pub fn linear_regression(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(x_arr), Value::Array(y_arr)) => {
            if x_arr.len() != y_arr.len() {
                return Err(RuntimeError::InvalidOperation(format!(
                    "X and Y arrays must have same length: {} vs {}",
                    x_arr.len(),
                    y_arr.len()
                )));
            }

            if x_arr.len() < 2 {
                return Err(RuntimeError::InvalidOperation(
                    "Linear regression requires at least 2 data points".to_string(),
                ));
            }

            // Extract numbers
            let mut x_vals = Vec::new();
            let mut y_vals = Vec::new();

            for val in x_arr {
                match val {
                    Value::Number(n) => x_vals.push(*n),
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }

            for val in y_arr {
                match val {
                    Value::Number(n) => y_vals.push(*n),
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }

            let n = x_vals.len() as f64;

            // Calculate means
            let x_mean = x_vals.iter().sum::<f64>() / n;
            let y_mean = y_vals.iter().sum::<f64>() / n;

            // Calculate slope
            let mut numerator = 0.0;
            let mut denominator = 0.0;

            for i in 0..x_vals.len() {
                let x_diff = x_vals[i] - x_mean;
                let y_diff = y_vals[i] - y_mean;
                numerator += x_diff * y_diff;
                denominator += x_diff * x_diff;
            }

            if denominator == 0.0 {
                return Err(RuntimeError::InvalidOperation(
                    "Cannot compute regression: X values have no variance".to_string(),
                ));
            }

            let slope = numerator / denominator;
            let intercept = y_mean - slope * x_mean;

            // Calculate R²
            let mut ss_res = 0.0; // Residual sum of squares
            let mut ss_tot = 0.0; // Total sum of squares

            for i in 0..x_vals.len() {
                let y_pred = slope * x_vals[i] + intercept;
                let residual = y_vals[i] - y_pred;
                ss_res += residual * residual;

                let total_diff = y_vals[i] - y_mean;
                ss_tot += total_diff * total_diff;
            }

            let r_squared = if ss_tot == 0.0 {
                1.0 // Perfect fit if no variance
            } else {
                1.0 - (ss_res / ss_tot)
            };

            Ok(Value::Array(vec![
                Value::Number(slope),
                Value::Number(intercept),
                Value::Number(r_squared),
            ]))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, Array".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

// ============================================================================
// Probability Distributions
// ============================================================================

/// 正态分布的概率密度函数 (PDF)
///
/// # 功能
/// 计算正态分布在指定点的概率密度。
///
/// # 参数
/// - `x`: Number - 计算点
/// - `mean`: Number - 均值 μ（可选，默认0）
/// - `std`: Number - 标准差 σ（可选，默认1）
///
/// # 返回值
/// Number - 概率密度值
///
/// # 公式
/// ```
/// PDF(x) = (1 / (σ√(2π))) * e^(-(x-μ)²/(2σ²))
/// ```
///
/// # 示例
/// ```aether
/// Set p NormalPDF(0, 0, 1)    # 标准正态分布在0点: 0.3989
/// Set p NormalPDF(1.96, 0, 1) # 在1.96点: 0.0584
/// Set p NormalPDF(10, 10, 2)  # μ=10, σ=2: 0.1995
/// ```
pub fn normal_pdf(args: &[Value]) -> Result<Value, RuntimeError> {
    let (x, mean, std) = match args.len() {
        1 => match &args[0] {
            Value::Number(x) => (*x, 0.0, 1.0),
            _ => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Number".to_string(),
                    got: format!("{:?}", args[0]),
                })
            }
        },
        3 => match (&args[0], &args[1], &args[2]) {
            (Value::Number(x), Value::Number(m), Value::Number(s)) => {
                if *s <= 0.0 {
                    return Err(RuntimeError::InvalidOperation(format!(
                        "Standard deviation must be positive, got {}",
                        s
                    )));
                }
                (*x, *m, *s)
            }
            _ => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Number, Number, Number".to_string(),
                    got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
                })
            }
        },
        n => {
            return Err(RuntimeError::WrongArity {
                expected: 1,
                got: n,
            })
        }
    };

    let z = (x - mean) / std;
    let coefficient = 1.0 / (std * (2.0 * consts::PI).sqrt());
    let exponent = -0.5 * z * z;
    let pdf = coefficient * exponent.exp();

    Ok(Value::Number(pdf))
}

/// 正态分布的累积分布函数 (CDF)
///
/// # 功能
/// 计算正态分布的累积概率 P(X ≤ x)。
///
/// # 参数
/// - `x`: Number - 计算点
/// - `mean`: Number - 均值 μ（可选，默认0）
/// - `std`: Number - 标准差 σ（可选，默认1）
///
/// # 返回值
/// Number - 累积概率，范围 [0, 1]
///
/// # 公式
/// ```
/// CDF(x) = 0.5 * [1 + erf((x-μ)/(σ√2))]
/// ```
///
/// # 示例
/// ```aether
/// Set p NormalCDF(0, 0, 1)    # 50% (中位数)
/// Set p NormalCDF(1.96, 0, 1) # 97.5% (95%置信区间上界)
/// Set p NormalCDF(-1.96, 0, 1)# 2.5% (95%置信区间下界)
/// ```
pub fn normal_cdf(args: &[Value]) -> Result<Value, RuntimeError> {
    let (x, mean, std) = match args.len() {
        1 => match &args[0] {
            Value::Number(x) => (*x, 0.0, 1.0),
            _ => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Number".to_string(),
                    got: format!("{:?}", args[0]),
                })
            }
        },
        3 => match (&args[0], &args[1], &args[2]) {
            (Value::Number(x), Value::Number(m), Value::Number(s)) => {
                if *s <= 0.0 {
                    return Err(RuntimeError::InvalidOperation(format!(
                        "Standard deviation must be positive, got {}",
                        s
                    )));
                }
                (*x, *m, *s)
            }
            _ => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Number, Number, Number".to_string(),
                    got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
                })
            }
        },
        n => {
            return Err(RuntimeError::WrongArity {
                expected: 1,
                got: n,
            })
        }
    };

    let z = (x - mean) / (std * 2.0_f64.sqrt());
    let erf_result = erf(&[Value::Number(z)])?;

    match erf_result {
        Value::Number(erf_val) => {
            let cdf = 0.5 * (1.0 + erf_val);
            Ok(Value::Number(cdf))
        }
        _ => unreachable!(),
    }
}

/// 泊松分布的概率质量函数 (PMF)
///
/// # 功能
/// 计算泊松分布在指定点的概率。
///
/// # 参数
/// - `k`: Number - 事件发生次数（非负整数）
/// - `lambda`: Number - 平均发生率 λ（必须 > 0）
///
/// # 返回值
/// Number - 概率值
///
/// # 公式
/// ```
/// P(X = k) = (λ^k * e^(-λ)) / k!
/// ```
///
/// # 应用
/// - 单位时间内事件发生的次数
/// - 稀有事件的概率模型
///
/// # 示例
/// ```aether
/// Set p PoissonPMF(3, 2.5)    # λ=2.5时，恰好3次的概率
/// Set p PoissonPMF(0, 1)      # 平均1次，0次发生的概率: e^(-1)
/// Set p PoissonPMF(5, 5)      # λ=k=5: 0.1755
/// ```
pub fn poisson_pmf(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(k), Value::Number(lambda)) => {
            if *k < 0.0 || k.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "k must be a non-negative integer, got {}",
                    k
                )));
            }

            if *lambda <= 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Lambda must be positive, got {}",
                    lambda
                )));
            }

            // Calculate k! using factorial function
            let fact_result = factorial(&[Value::Number(*k)])?;
            let k_factorial = match fact_result {
                Value::Number(f) => f,
                _ => unreachable!(),
            };

            // P(X = k) = (λ^k * e^(-λ)) / k!
            let numerator = lambda.powf(*k) * (-lambda).exp();
            let pmf = numerator / k_factorial;

            Ok(Value::Number(pmf))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

// ============================================================================
// 带精度计算函数
// ============================================================================

/// 四舍五入到指定小数位数
///
/// # 功能
/// 将数字四舍五入到指定的小数位数。
///
/// # 参数
/// - `x`: Number - 输入数字
/// - `digits`: Number - 小数位数（非负整数）
///
/// # 返回值
/// Number - 四舍五入后的数字
///
/// # 示例
/// ```aether
/// Set a RoundTo(3.14159, 2)       # 3.14
/// Set b RoundTo(123.456, 1)       # 123.5
/// Set c RoundTo(0.666666, 4)      # 0.6667
/// ```
pub fn round_to(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(x), Value::Number(digits)) => {
            if *digits < 0.0 || digits.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Digits must be a non-negative integer, got {}",
                    digits
                )));
            }

            let multiplier = 10_f64.powi(*digits as i32);
            let result = (x * multiplier).round() / multiplier;
            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 带精度加法
///
/// # 功能
/// 先将两个数字按指定精度四舍五入，然后进行加法运算。
///
/// # 参数
/// - `a`: Number - 第一个加数
/// - `b`: Number - 第二个加数
/// - `precision`: Number - 精度（小数位数）
///
/// # 返回值
/// Number - 结果（按精度四舍五入）
///
/// # 示例
/// ```aether
/// Set result AddWithPrecision(0.1, 0.2, 2)    # 0.30
/// Set result AddWithPrecision(1.235, 2.346, 2) # 3.58
/// ```
pub fn add_with_precision(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::Number(a), Value::Number(b), Value::Number(precision)) => {
            if *precision < 0.0 || precision.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Precision must be a non-negative integer, got {}",
                    precision
                )));
            }

            let multiplier = 10_f64.powi(*precision as i32);
            let a_rounded = (a * multiplier).round() / multiplier;
            let b_rounded = (b * multiplier).round() / multiplier;
            let result = ((a_rounded + b_rounded) * multiplier).round() / multiplier;

            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number, Number".to_string(),
            got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
        }),
    }
}

/// 带精度减法
///
/// # 功能
/// 先将两个数字按指定精度四舍五入，然后进行减法运算。
///
/// # 参数
/// - `a`: Number - 被减数
/// - `b`: Number - 减数
/// - `precision`: Number - 精度（小数位数）
///
/// # 返回值
/// Number - 结果（按精度四舍五入）
///
/// # 示例
/// ```aether
/// Set result SubWithPrecision(1.5, 0.3, 1)    # 1.2
/// Set result SubWithPrecision(5.678, 2.345, 2) # 3.33
/// ```
pub fn sub_with_precision(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::Number(a), Value::Number(b), Value::Number(precision)) => {
            if *precision < 0.0 || precision.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Precision must be a non-negative integer, got {}",
                    precision
                )));
            }

            let multiplier = 10_f64.powi(*precision as i32);
            let a_rounded = (a * multiplier).round() / multiplier;
            let b_rounded = (b * multiplier).round() / multiplier;
            let result = ((a_rounded - b_rounded) * multiplier).round() / multiplier;

            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number, Number".to_string(),
            got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
        }),
    }
}

/// 带精度乘法
///
/// # 功能
/// 先将两个数字按指定精度四舍五入，然后进行乘法运算。
///
/// # 参数
/// - `a`: Number - 第一个乘数
/// - `b`: Number - 第二个乘数
/// - `precision`: Number - 精度（小数位数）
///
/// # 返回值
/// Number - 结果（按精度四舍五入）
///
/// # 示例
/// ```aether
/// Set result MulWithPrecision(0.1, 0.2, 3)    # 0.02
/// Set result MulWithPrecision(3.456, 2.5, 2)  # 8.64
/// ```
pub fn mul_with_precision(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::Number(a), Value::Number(b), Value::Number(precision)) => {
            if *precision < 0.0 || precision.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Precision must be a non-negative integer, got {}",
                    precision
                )));
            }

            let multiplier = 10_f64.powi(*precision as i32);
            let a_rounded = (a * multiplier).round() / multiplier;
            let b_rounded = (b * multiplier).round() / multiplier;
            let result = ((a_rounded * b_rounded) * multiplier).round() / multiplier;

            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number, Number".to_string(),
            got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
        }),
    }
}

/// 带精度除法
///
/// # 功能
/// 先将两个数字按指定精度四舍五入，然后进行除法运算。
///
/// # 参数
/// - `a`: Number - 被除数
/// - `b`: Number - 除数（不能为0）
/// - `precision`: Number - 精度（小数位数）
///
/// # 返回值
/// Number - 结果（按精度四舍五入）
///
/// # 示例
/// ```aether
/// Set result DivWithPrecision(1.0, 3.0, 2)    # 0.33
/// Set result DivWithPrecision(10.0, 3.0, 4)   # 3.3333
/// ```
pub fn div_with_precision(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::Number(a), Value::Number(b), Value::Number(precision)) => {
            if *precision < 0.0 || precision.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Precision must be a non-negative integer, got {}",
                    precision
                )));
            }

            if *b == 0.0 {
                return Err(RuntimeError::InvalidOperation(
                    "Division by zero".to_string(),
                ));
            }

            let multiplier = 10_f64.powi(*precision as i32);
            let a_rounded = (a * multiplier).round() / multiplier;
            let b_rounded = (b * multiplier).round() / multiplier;
            let result = ((a_rounded / b_rounded) * multiplier).round() / multiplier;

            Ok(Value::Number(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, Number, Number".to_string(),
            got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
        }),
    }
}

/// 设置全局计算精度
///
/// # 功能
/// 对数组中的所有数字应用指定精度的四舍五入。
///
/// # 参数
/// - `array`: Array - 数字数组
/// - `precision`: Number - 精度（小数位数）
///
/// # 返回值
/// Array - 应用精度后的数组
///
/// # 示例
/// ```aether
/// Set nums [3.14159, 2.71828, 1.41421]
/// Set rounded SetPrecision(nums, 2)   # [3.14, 2.72, 1.41]
/// ```
pub fn set_precision(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::Number(precision)) => {
            if *precision < 0.0 || precision.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Precision must be a non-negative integer, got {}",
                    precision
                )));
            }

            let multiplier = 10_f64.powi(*precision as i32);
            let mut result = Vec::new();

            for val in arr {
                match val {
                    Value::Number(n) => {
                        let rounded = (n * multiplier).round() / multiplier;
                        result.push(Value::Number(rounded));
                    }
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        })
                    }
                }
            }

            Ok(Value::Array(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}
