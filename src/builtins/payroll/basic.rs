// src/builtins/payroll/basic.rs
//! 基础薪酬计算函数

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

/// 计算时薪
///
/// # 参数
/// - 月薪
/// - 月工作小时数（默认174小时）
///
/// # 返回
/// 时薪
pub fn calc_hourly_pay(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let monthly_hours = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        174.0 // 默认月工作小时数: 21.75天 * 8小时
    };

    if monthly_hours <= 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "月工作小时数必须大于0".to_string(),
        ));
    }

    Ok(Value::Number(monthly_salary / monthly_hours))
}

/// 计算日薪
///
/// # 参数
/// - 月薪
/// - 月工作天数（默认21.75天）
///
/// # 返回
/// 日薪
pub fn calc_daily_pay(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let monthly_days = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        21.75 // 法定计薪天数
    };

    if monthly_days <= 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "月工作天数必须大于0".to_string(),
        ));
    }

    Ok(Value::Number(monthly_salary / monthly_days))
}

/// 根据时薪计算月薪
///
/// # 参数
/// - 时薪
/// - 月工作小时数（默认174小时）
///
/// # 返回
/// 月薪
pub fn calc_monthly_from_hourly(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let hourly_rate = get_number(&args[0])?;
    let monthly_hours = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        174.0
    };

    Ok(Value::Number(hourly_rate * monthly_hours))
}

/// 计算年薪
///
/// # 参数
/// - 月薪
/// - 月数（默认12个月）
///
/// # 返回
/// 年薪
pub fn calc_annual_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let months = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        12.0
    };

    Ok(Value::Number(monthly_salary * months))
}

/// 根据出勤率计算基本工资
///
/// # 参数
/// - 基本工资
/// - 实际出勤天数
/// - 应出勤天数
///
/// # 返回
/// 实际基本工资
pub fn calc_base_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    let base_salary = get_number(&args[0])?;
    let actual_days = get_number(&args[1])?;
    let required_days = get_number(&args[2])?;

    if required_days <= 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "应出勤天数必须大于0".to_string(),
        ));
    }

    let attendance_rate = actual_days / required_days;
    Ok(Value::Number(base_salary * attendance_rate))
}

/// 计算应发工资
///
/// # 参数
/// - 基本工资
/// - 加班费
/// - 奖金
/// - 补贴
///
/// # 返回
/// 应发工资总额
pub fn calc_gross_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 4 {
        return Err(RuntimeError::WrongArity {
            expected: 4,
            got: args.len(),
        });
    }

    let base = get_number(&args[0])?;
    let overtime = get_number(&args[1])?;
    let bonus = get_number(&args[2])?;
    let allowance = get_number(&args[3])?;

    Ok(Value::Number(base + overtime + bonus + allowance))
}

/// 计算实发工资
///
/// # 参数
/// - 应发工资
/// - 社保
/// - 公积金
/// - 个税
/// - 其他扣除
///
/// # 返回
/// 实发工资
pub fn calc_net_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 5 {
        return Err(RuntimeError::WrongArity {
            expected: 5,
            got: args.len(),
        });
    }

    let gross = get_number(&args[0])?;
    let social_insurance = get_number(&args[1])?;
    let housing_fund = get_number(&args[2])?;
    let tax = get_number(&args[3])?;
    let other_deductions = get_number(&args[4])?;

    let net = gross - social_insurance - housing_fund - tax - other_deductions;
    Ok(Value::Number(net.max(0.0)))
}
