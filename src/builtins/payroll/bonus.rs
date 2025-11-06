// src/builtins/payroll/bonus.rs
//! 绩效与奖金计算函数

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

/// 计算绩效工资
///
/// # 参数
/// - 基本工资
/// - 绩效系数（如0.8, 1.0, 1.2）
///
/// # 返回
/// 绩效工资
pub fn calc_performance_pay(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let base_salary = get_number(&args[0])?;
    let performance_coefficient = get_number(&args[1])?;

    Ok(Value::Number(base_salary * performance_coefficient))
}

/// 计算年终奖
///
/// # 参数
/// - 月薪
/// - 月数（默认12个月）
/// - 绩效系数（默认1.0）
///
/// # 返回
/// 年终奖金额
pub fn calc_annual_bonus(args: &[Value]) -> Result<Value, RuntimeError> {
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
    let performance_coefficient = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        1.0
    };

    Ok(Value::Number(
        monthly_salary * months * performance_coefficient,
    ))
}

/// 计算全勤奖
///
/// # 参数
/// - 基础奖金
/// - 实际出勤率
/// - 全勤阈值（默认1.0，即100%）
///
/// # 返回
/// 全勤奖（达到阈值才发放）
pub fn calc_attendance_bonus(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let bonus = get_number(&args[0])?;
    let attendance_rate = get_number(&args[1])?;
    let threshold = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        1.0 // 默认100%全勤
    };

    if attendance_rate >= threshold {
        Ok(Value::Number(bonus))
    } else {
        Ok(Value::Number(0.0))
    }
}

/// 计算销售提成
///
/// # 参数
/// - 销售额
/// - 提成比例
/// - 基础门槛（默认0）
///
/// # 返回
/// 销售提成
pub fn calc_sales_commission(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let sales_amount = get_number(&args[0])?;
    let commission_rate = get_number(&args[1])?;
    let threshold = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        0.0
    };

    let commissionable = (sales_amount - threshold).max(0.0);
    Ok(Value::Number(commissionable * commission_rate))
}

/// 计算项目奖金
///
/// # 参数
/// - 项目总金额
/// - 个人分配比例
///
/// # 返回
/// 个人项目奖金
pub fn calc_project_bonus(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let project_amount = get_number(&args[0])?;
    let allocation_ratio = get_number(&args[1])?;

    Ok(Value::Number(project_amount * allocation_ratio))
}

/// 计算13薪
///
/// # 参数
/// - 月薪
/// - 实际工作月数
///
/// # 返回
/// 13薪金额（按比例）
pub fn calc_13th_salary(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let monthly_salary = get_number(&args[0])?;
    let worked_months = get_number(&args[1])?;

    // 按实际工作月数比例发放
    let ratio = (worked_months / 12.0).min(1.0);
    Ok(Value::Number(monthly_salary * ratio))
}
