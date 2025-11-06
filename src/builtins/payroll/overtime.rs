// src/builtins/payroll/overtime.rs
//! 加班费计算函数

use crate::evaluator::RuntimeError;
use crate::value::Value;

/// 辅助函数：安全地获取数字参数
fn get_number(val: &Value) -> Result<f64, RuntimeError> {
    match val {
        Value::Number(n) => Ok(*n),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", val),
        }),
    }
}

/// 计算加班费（通用）
///
/// # 参数
/// - 时薪
/// - 加班小时数
/// - 倍率（如1.5、2.0、3.0）
///
/// # 返回
/// 加班费
pub fn calc_overtime_pay(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    let hourly_rate = get_number(&args[0])?;
    let hours = get_number(&args[1])?;
    let multiplier = get_number(&args[2])?;

    if hours < 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "加班小时数不能为负数".to_string(),
        ));
    }

    Ok(Value::Number(hourly_rate * hours * multiplier))
}

/// 计算平日加班费（1.5倍）
///
/// # 参数
/// - 月薪
/// - 加班小时数
///
/// # 返回
/// 加班费
pub fn calc_weekday_overtime(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let hours = get_number(&args[1])?;

    if hours < 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "加班小时数不能为负数".to_string(),
        ));
    }

    // 计算时薪：月薪 ÷ 21.75天 ÷ 8小时
    let hourly_rate = monthly_salary / 21.75 / 8.0;

    // 平日加班费为1.5倍
    Ok(Value::Number(hourly_rate * hours * 1.5))
}

/// 计算周末加班费（2倍）
///
/// # 参数
/// - 月薪
/// - 加班小时数
///
/// # 返回
/// 加班费
pub fn calc_weekend_overtime(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let hours = get_number(&args[1])?;

    if hours < 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "加班小时数不能为负数".to_string(),
        ));
    }

    // 计算时薪：月薪 ÷ 21.75天 ÷ 8小时
    let hourly_rate = monthly_salary / 21.75 / 8.0;

    // 周末加班费为2倍
    Ok(Value::Number(hourly_rate * hours * 2.0))
}

/// 计算法定节假日加班费（3倍）
///
/// # 参数
/// - 月薪
/// - 加班小时数
///
/// # 返回
/// 加班费
pub fn calc_holiday_overtime(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let hours = get_number(&args[1])?;

    if hours < 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "加班小时数不能为负数".to_string(),
        ));
    }

    // 计算时薪：月薪 ÷ 21.75天 ÷ 8小时
    let hourly_rate = monthly_salary / 21.75 / 8.0;

    // 法定节假日加班费为3倍
    Ok(Value::Number(hourly_rate * hours * 3.0))
}

/// 汇总各类加班费
///
/// # 参数
/// - 平日加班费
/// - 周末加班费
/// - 节假日加班费
///
/// # 返回
/// 总加班费
pub fn calc_total_overtime(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    let weekday = get_number(&args[0])?;
    let weekend = get_number(&args[1])?;
    let holiday = get_number(&args[2])?;

    Ok(Value::Number(weekday + weekend + holiday))
}
