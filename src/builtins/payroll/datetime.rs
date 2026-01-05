// src/builtins/payroll/datetime.rs
//! 日期时间计算函数

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

/// 计算自然天数（包含周末和节假日）
///
/// # 参数
/// - 起始日（如1号）
/// - 结束日（如31号）
///
/// # 返回
/// 自然天数（含首尾）
pub fn calc_natural_days(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let start_day = get_number(&args[0])?;
    let end_day = get_number(&args[1])?;

    Ok(Value::Number(end_day - start_day + 1.0))
}

/// 获取月度计薪天数（21.75天）
///
/// # 返回
/// 固定返回21.75
pub fn get_legal_pay_days(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Number(21.75))
}

/// 计算工作日天数（扣除周末）
///
/// # 参数
/// - 总天数
/// - 周末天数
///
/// # 返回
/// 工作日天数
pub fn calc_workdays(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let total_days = get_number(&args[0])?;
    let weekend_days = get_number(&args[1])?;

    Ok(Value::Number(total_days - weekend_days))
}

/// 计算周末天数
///
/// # 参数
/// - 总天数
/// - 起始星期几（1=周一，7=周日）
///
/// # 返回
/// 周末天数（周六+周日）
pub fn calc_weekend_days(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let total_days = get_number(&args[0])? as i32;
    let start_weekday = get_number(&args[1])? as i32;

    let mut weekend_count = 0;
    for i in 0..total_days {
        let weekday = ((start_weekday - 1 + i) % 7) + 1;
        if weekday == 6 || weekday == 7 {
            // 周六或周日
            weekend_count += 1;
        }
    }

    Ok(Value::Number(weekend_count as f64))
}

/// 计算法定节假日天数
///
/// # 参数
/// - 当月节假日数组（作为字符串参数传入，实际使用时需解析）
///
/// # 返回
/// 节假日天数
pub fn calc_holiday_days(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    // 简化实现：直接返回传入的天数
    let holiday_days = get_number(&args[0])?;
    Ok(Value::Number(holiday_days))
}

/// 判断是否为工作日
///
/// # 参数
/// - 星期几（1=周一，7=周日）
/// - 是否为节假日（0=否，1=是）
///
/// # 返回
/// 1=工作日，0=非工作日
pub fn is_workday(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let weekday = get_number(&args[0])? as i32;
    let is_holiday = get_number(&args[1])? as i32;

    let result = if is_holiday == 1 {
        0 // 节假日不是工作日
    } else if (1..=5).contains(&weekday) {
        1 // 周一到周五是工作日
    } else {
        0 // 周末不是工作日
    };

    Ok(Value::Number(result as f64))
}

/// 判断是否为周末
///
/// # 参数
/// - 星期几（1=周一，7=周日）
///
/// # 返回
/// 1=周末，0=非周末
pub fn is_weekend(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let weekday = get_number(&args[0])? as i32;
    let result = if weekday == 6 || weekday == 7 { 1 } else { 0 };

    Ok(Value::Number(result as f64))
}

/// 判断是否为法定节假日
///
/// # 参数
/// - 日期（当月第几天）
/// - 节假日列表（以数组形式传入）
///
/// # 返回
/// 1=节假日，0=非节假日
pub fn is_holiday(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let _date = get_number(&args[0])? as i32;

    // 简化实现：假设第二个参数是节假日数量
    // 实际使用时可能需要更复杂的逻辑
    let holiday_indicator = get_number(&args[1])? as i32;

    Ok(Value::Number(holiday_indicator as f64))
}

/// 计算标准工作时长（小时）
///
/// # 参数
/// - 工作天数
/// - 每日工时（默认8小时）
///
/// # 返回
/// 总工作小时数
pub fn calc_work_hours(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let work_days = get_number(&args[0])?;
    let daily_hours = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        8.0
    };

    Ok(Value::Number(work_days * daily_hours))
}

/// 计算月度标准工作小时数
///
/// # 参数
/// - 月度工作天数（默认21.75天）
/// - 每日工时（默认8小时）
///
/// # 返回
/// 月度标准工作小时数（默认174小时）
pub fn calc_monthly_work_hours(args: &[Value]) -> Result<Value, RuntimeError> {
    let monthly_days = if args.is_empty() {
        21.75
    } else {
        get_number(&args[0])?
    };

    let daily_hours = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        8.0
    };

    Ok(Value::Number(monthly_days * daily_hours))
}

/// 计算年度工作天数
///
/// # 返回
/// 固定返回 (365 - 104) = 261天（扣除52周的周末）
pub fn calc_annual_workdays(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Number(261.0))
}

/// 计算年度计薪天数
///
/// # 返回
/// 固定返回 261天（年度工作日）
pub fn calc_annual_pay_days(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Number(261.0))
}
