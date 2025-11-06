// src/builtins/payroll/attendance.rs
//! 考勤相关计算函数

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

/// 计算出勤率
///
/// # 参数
/// - 实际出勤天数
/// - 应出勤天数
///
/// # 返回
/// 出勤率（0-1之间的小数）
pub fn calc_attendance_rate(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let actual_days = get_number(&args[0])?;
    let required_days = get_number(&args[1])?;

    if required_days <= 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "应出勤天数必须大于0".to_string(),
        ));
    }

    let rate = (actual_days / required_days).min(1.0).max(0.0);
    Ok(Value::Number(rate))
}

/// 计算迟到扣款
///
/// # 参数
/// - 迟到次数
/// - 单次扣款金额
///
/// # 返回
/// 迟到扣款总额
pub fn calc_late_deduction(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let late_count = get_number(&args[0])?;
    let deduction_per_time = get_number(&args[1])?;

    Ok(Value::Number(late_count * deduction_per_time))
}

/// 计算早退扣款
///
/// # 参数
/// - 早退次数
/// - 单次扣款金额
///
/// # 返回
/// 早退扣款总额
pub fn calc_early_leave_deduction(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let early_leave_count = get_number(&args[0])?;
    let deduction_per_time = get_number(&args[1])?;

    Ok(Value::Number(early_leave_count * deduction_per_time))
}

/// 计算缺勤扣款
///
/// # 参数
/// - 缺勤天数
/// - 日薪
///
/// # 返回
/// 缺勤扣款
pub fn calc_absent_deduction(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let absent_days = get_number(&args[0])?;
    let daily_pay = get_number(&args[1])?;

    Ok(Value::Number(absent_days * daily_pay))
}

/// 计算请假扣款
///
/// # 参数
/// - 请假天数
/// - 日薪
/// - 扣款比例（默认1.0，即全额扣款；年假可能是0）
///
/// # 返回
/// 请假扣款
pub fn calc_leave_deduction(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let leave_days = get_number(&args[0])?;
    let daily_pay = get_number(&args[1])?;
    let deduction_ratio = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        1.0 // 默认全额扣款
    };

    Ok(Value::Number(leave_days * daily_pay * deduction_ratio))
}

/// 计算病假工资
///
/// # 参数
/// - 基本工资
/// - 病假天数
/// - 工龄（年）
///
/// # 返回
/// 病假期间工资
///
/// # 说明
/// 根据工龄，病假工资为基本工资的60%-100%
pub fn calc_sick_leave_pay(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    let base_salary = get_number(&args[0])?;
    let sick_days = get_number(&args[1])?;
    let seniority = get_number(&args[2])?;

    // 根据工龄确定病假工资比例
    let ratio = if seniority < 2.0 {
        0.60 // 不足2年，60%
    } else if seniority < 4.0 {
        0.70 // 2-4年，70%
    } else if seniority < 6.0 {
        0.80 // 4-6年，80%
    } else if seniority < 8.0 {
        0.90 // 6-8年，90%
    } else {
        1.00 // 8年以上，100%
    };

    // 计算日薪并按比例支付
    let daily_pay = base_salary / 21.75;
    Ok(Value::Number(daily_pay * sick_days * ratio))
}

/// 计算无薪假扣款
///
/// # 参数
/// - 基本工资
/// - 无薪假天数
///
/// # 返回
/// 扣款金额
pub fn calc_unpaid_leave_deduction(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let base_salary = get_number(&args[0])?;
    let unpaid_days = get_number(&args[1])?;

    // 按日薪扣款
    let daily_pay = base_salary / 21.75;
    Ok(Value::Number(daily_pay * unpaid_days))
}
