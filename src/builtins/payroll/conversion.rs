// src/builtins/payroll/conversion.rs
//! 薪资转换和折算函数

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


/// 年薪转月薪
pub fn annual_to_monthly(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }
    let annual = get_number(&args[0])?;
    Ok(Value::Number(annual / 12.0))
}

/// 月薪转年薪
pub fn monthly_to_annual(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }
    let monthly = get_number(&args[0])?;
    Ok(Value::Number(monthly * 12.0))
}

/// 日薪转月薪（按21.75天）
pub fn daily_to_monthly(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }
    let daily = get_number(&args[0])?;
    Ok(Value::Number(daily * 21.75))
}

/// 月薪转日薪（按21.75天）
pub fn monthly_to_daily(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }
    let monthly = get_number(&args[0])?;
    Ok(Value::Number(monthly / 21.75))
}

/// 时薪转月薪（按标准工时）
///
/// # 参数
/// - 时薪
/// - 每日工时（默认8小时）
/// - 每月工作天数（默认21.75天）
pub fn hourly_to_monthly(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }
    let hourly = get_number(&args[0])?;
    let daily_hours = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        8.0
    };
    let monthly_days = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        21.75
    };
    Ok(Value::Number(hourly * daily_hours * monthly_days))
}

/// 月薪转时薪（按标准工时）
///
/// # 参数
/// - 月薪
/// - 每日工时（默认8小时）
/// - 每月工作天数（默认21.75天）
pub fn monthly_to_hourly(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }
    let monthly = get_number(&args[0])?;
    let daily_hours = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        8.0
    };
    let monthly_days = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        21.75
    };
    Ok(Value::Number(monthly / (daily_hours * monthly_days)))
}

/// 按自然天折算月薪
///
/// # 参数
/// - 月薪
/// - 实际工作天数
/// - 当月总天数（如28/29/30/31）
pub fn prorate_by_natural_days(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }
    let monthly_salary = get_number(&args[0])?;
    let worked_days = get_number(&args[1])?;
    let total_days = get_number(&args[2])?;

    Ok(Value::Number(monthly_salary * worked_days / total_days))
}

/// 按21.75天折算月薪
///
/// # 参数
/// - 月薪
/// - 实际工作天数
pub fn prorate_by_legal_days(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let monthly_salary = get_number(&args[0])?;
    let worked_days = get_number(&args[1])?;

    Ok(Value::Number(monthly_salary * worked_days / 21.75))
}

/// 按工作日折算月薪
///
/// # 参数
/// - 月薪
/// - 实际工作天数
/// - 当月工作日总天数（扣除周末和节假日）
pub fn prorate_by_workdays(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }
    let monthly_salary = get_number(&args[0])?;
    let worked_days = get_number(&args[1])?;
    let total_workdays = get_number(&args[2])?;

    Ok(Value::Number(monthly_salary * worked_days / total_workdays))
}

/// 入职当月薪资折算
///
/// # 参数
/// - 月薪
/// - 入职日期（当月的第几天）
/// - 当月总天数
/// - 折算方式：0=自然天，1=21.75天，2=工作日
/// - 当月工作日总数（折算方式为2时必需）
pub fn calc_onboarding_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 4 {
        return Err(RuntimeError::WrongArity {
            expected: 4,
            got: args.len(),
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let onboarding_day = get_number(&args[1])?;
    let total_days = get_number(&args[2])?;
    let calc_type = get_number(&args[3])? as i32;

    let worked_days = total_days - onboarding_day + 1.0;

    match calc_type {
        0 => {
            // 按自然天
            Ok(Value::Number(monthly_salary * worked_days / total_days))
        }
        1 => {
            // 按21.75天
            Ok(Value::Number(monthly_salary * worked_days / 21.75))
        }
        2 => {
            // 按工作日
            if args.len() < 5 {
                return Err(RuntimeError::InvalidOperation(
                    "按工作日折算需要提供当月工作日总数".to_string(),
                ));
            }
            let total_workdays = get_number(&args[4])?;
            Ok(Value::Number(monthly_salary * worked_days / total_workdays))
        }
        _ => Err(RuntimeError::InvalidOperation(
            "折算方式必须是0（自然天）、1（21.75天）或2（工作日）".to_string(),
        )),
    }
}

/// 离职当月薪资折算
///
/// # 参数
/// - 月薪
/// - 离职日期（当月的第几天，含当日）
/// - 当月总天数
/// - 折算方式：0=自然天，1=21.75天，2=工作日
/// - 当月工作日总数（折算方式为2时必需）
pub fn calc_resignation_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 4 {
        return Err(RuntimeError::WrongArity {
            expected: 4,
            got: args.len(),
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let resignation_day = get_number(&args[1])?;
    let total_days = get_number(&args[2])?;
    let calc_type = get_number(&args[3])? as i32;

    let worked_days = resignation_day;

    match calc_type {
        0 => {
            // 按自然天
            Ok(Value::Number(monthly_salary * worked_days / total_days))
        }
        1 => {
            // 按21.75天
            Ok(Value::Number(monthly_salary * worked_days / 21.75))
        }
        2 => {
            // 按工作日
            if args.len() < 5 {
                return Err(RuntimeError::InvalidOperation(
                    "按工作日折算需要提供当月工作日总数".to_string(),
                ));
            }
            let total_workdays = get_number(&args[4])?;
            Ok(Value::Number(monthly_salary * worked_days / total_workdays))
        }
        _ => Err(RuntimeError::InvalidOperation(
            "折算方式必须是0（自然天）、1（21.75天）或2（工作日）".to_string(),
        )),
    }
}

/// 计算14薪
///
/// # 参数
/// - 月薪
/// - 实际工作月数
pub fn calc_14th_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let worked_months = get_number(&args[1])?;

    // 14薪 = 2个月工资，按实际工作月数比例发放
    let ratio = (worked_months / 12.0).min(1.0);
    Ok(Value::Number(monthly_salary * 2.0 * ratio))
}
