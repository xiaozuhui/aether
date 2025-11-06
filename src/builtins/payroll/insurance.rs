// src/builtins/payroll/insurance.rs
//! 社保公积金计算函数

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

/// 计算养老保险
///
/// # 参数
/// - 缴费基数
/// - 个人比例（默认0.08，即8%）
///
/// # 返回
/// 养老保险个人缴纳金额
pub fn calc_pension_insurance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let base = get_number(&args[0])?;
    let rate = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.08 // 默认8%
    };

    Ok(Value::Number(base * rate))
}

/// 计算医疗保险
///
/// # 参数
/// - 缴费基数
/// - 个人比例（默认0.02，即2%）
///
/// # 返回
/// 医疗保险个人缴纳金额
pub fn calc_medical_insurance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let base = get_number(&args[0])?;
    let rate = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.02 // 默认2%
    };

    Ok(Value::Number(base * rate))
}

/// 计算失业保险
///
/// # 参数
/// - 缴费基数
/// - 个人比例（默认0.005，即0.5%）
///
/// # 返回
/// 失业保险个人缴纳金额
pub fn calc_unemployment_insurance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let base = get_number(&args[0])?;
    let rate = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.005 // 默认0.5%
    };

    Ok(Value::Number(base * rate))
}

/// 计算住房公积金
///
/// # 参数
/// - 缴费基数
/// - 个人比例（默认0.12，即12%）
///
/// # 返回
/// 公积金个人缴纳金额
pub fn calc_housing_fund(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let base = get_number(&args[0])?;
    let rate = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.12 // 默认12%
    };

    Ok(Value::Number(base * rate))
}

/// 计算社保公积金总额（养老+医疗+失业+公积金）
///
/// # 参数
/// - 缴费基数
/// - 养老保险比例（可选，默认0.08）
/// - 医疗保险比例（可选，默认0.02）
/// - 失业保险比例（可选，默认0.005）
/// - 公积金比例（可选，默认0.12）
///
/// # 返回
/// 社保公积金总额
pub fn calc_social_insurance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let base = get_number(&args[0])?;

    // 使用默认比例或自定义比例
    let pension_rate = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.08 // 8%
    };

    let medical_rate = if args.len() > 2 {
        get_number(&args[2])?
    } else {
        0.02 // 2%
    };

    let unemployment_rate = if args.len() > 3 {
        get_number(&args[3])?
    } else {
        0.005 // 0.5%
    };

    let housing_rate = if args.len() > 4 {
        get_number(&args[4])?
    } else {
        0.12 // 12%
    };

    let total = base * (pension_rate + medical_rate + unemployment_rate + housing_rate);
    Ok(Value::Number(total))
}

/// 汇总五险一金个人缴纳总额
///
/// # 参数
/// - 社保总额
/// - 公积金
///
/// # 返回
/// 五险一金总扣除额
pub fn calc_total_insurance_fund(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let social_insurance = get_number(&args[0])?;
    let housing_fund = get_number(&args[1])?;

    Ok(Value::Number(social_insurance + housing_fund))
}

/// 调整社保缴费基数（限制在上下限之间）
///
/// # 参数
/// - 实际工资
/// - 基数下限
/// - 基数上限
///
/// # 返回
/// 调整后的缴费基数
pub fn adjust_social_base(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    let salary = get_number(&args[0])?;
    let lower_limit = get_number(&args[1])?;
    let upper_limit = get_number(&args[2])?;

    let adjusted = if salary < lower_limit {
        lower_limit
    } else if salary > upper_limit {
        upper_limit
    } else {
        salary
    };

    Ok(Value::Number(adjusted))
}

/// 计算社保基数下限
///
/// # 参数
/// - 社会平均工资
/// - 下限比例（默认0.6，即60%）
///
/// # 返回
/// 社保基数下限
pub fn calc_social_base_lower(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let avg_salary = get_number(&args[0])?;
    let ratio = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.6
    };

    Ok(Value::Number(avg_salary * ratio))
}

/// 计算社保基数上限
///
/// # 参数
/// - 社会平均工资
/// - 上限倍数（默认3倍）
///
/// # 返回
/// 社保基数上限
pub fn calc_social_base_upper(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let avg_salary = get_number(&args[0])?;
    let multiplier = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        3.0
    };

    Ok(Value::Number(avg_salary * multiplier))
}

/// 计算工伤保险（个人不缴纳，企业缴纳，此函数供完整性）
///
/// # 参数
/// - 社保基数
/// - 企业缴纳比例（默认0.2%，0.002）
///
/// # 返回
/// 工伤保险金额
pub fn calc_injury_insurance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let base = get_number(&args[0])?;
    let rate = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.002 // 0.2%
    };

    Ok(Value::Number(base * rate))
}

/// 计算生育保险（个人不缴纳，企业缴纳，此函数供完整性）
///
/// # 参数
/// - 社保基数
/// - 企业缴纳比例（默认0.8%，0.008）
///
/// # 返回
/// 生育保险金额
pub fn calc_maternity_insurance(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let base = get_number(&args[0])?;
    let rate = if args.len() > 1 {
        get_number(&args[1])?
    } else {
        0.008 // 0.8%
    };

    Ok(Value::Number(base * rate))
}
