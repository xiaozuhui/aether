// src/builtins/payroll/allowance.rs
//! 津贴补贴计算函数

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

/// 计算餐补
///
/// # 参数
/// - 标准餐补（每天）
/// - 出勤天数
///
/// # 返回
/// 餐补总额
pub fn calc_meal_allowance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let daily_allowance = get_number(&args[0])?;
    let attendance_days = get_number(&args[1])?;

    Ok(Value::Number(daily_allowance * attendance_days))
}

/// 计算交通补贴
///
/// # 参数
/// - 标准交通补贴（每天）
/// - 出勤天数
///
/// # 返回
/// 交通补贴总额
pub fn calc_transport_allowance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let daily_allowance = get_number(&args[0])?;
    let attendance_days = get_number(&args[1])?;

    Ok(Value::Number(daily_allowance * attendance_days))
}

/// 计算通讯补贴
///
/// # 参数
/// - 月度通讯补贴
/// - 工作天数
/// - 标准天数（默认21.75）
///
/// # 返回
/// 按比例的通讯补贴
pub fn calc_communication_allowance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_allowance = get_number(&args[0])?;
    let worked_days = get_number(&args[1])?;
    let standard_days = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        21.75
    };

    Ok(Value::Number(
        monthly_allowance * worked_days / standard_days,
    ))
}

/// 计算住房补贴
///
/// # 参数
/// - 月度住房补贴
/// - 工作月数（可以是小数，如入职半月为0.5）
///
/// # 返回
/// 按比例的住房补贴
pub fn calc_housing_allowance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_allowance = get_number(&args[0])?;
    let worked_months = get_number(&args[1])?;

    Ok(Value::Number(monthly_allowance * worked_months))
}

/// 计算高温补贴
///
/// # 参数
/// - 标准高温补贴（每天）
/// - 高温天数
///
/// # 返回
/// 高温补贴总额
pub fn calc_high_temp_allowance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let daily_allowance = get_number(&args[0])?;
    let high_temp_days = get_number(&args[1])?;

    Ok(Value::Number(daily_allowance * high_temp_days))
}

/// 计算夜班补贴
///
/// # 参数
/// - 标准夜班补贴（每晚）
/// - 夜班次数
///
/// # 返回
/// 夜班补贴总额
pub fn calc_night_shift_allowance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let per_shift_allowance = get_number(&args[0])?;
    let night_shifts = get_number(&args[1])?;

    Ok(Value::Number(per_shift_allowance * night_shifts))
}

/// 计算岗位津贴
///
/// # 参数
/// - 月度岗位津贴
/// - 工作月数（可以是小数）
///
/// # 返回
/// 按比例的岗位津贴
pub fn calc_position_allowance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_allowance = get_number(&args[0])?;
    let worked_months = get_number(&args[1])?;

    Ok(Value::Number(monthly_allowance * worked_months))
}
