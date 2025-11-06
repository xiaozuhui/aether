// src/builtins/payroll/statistics.rs
//! 薪资统计分析函数

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

/// 计算平均薪资
///
/// # 参数
/// - 薪资数组（通过多个参数传入）
///
/// # 返回
/// 平均值
pub fn calc_salary_average(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let mut sum = 0.0;
    for arg in args {
        sum += get_number(arg)?;
    }

    Ok(Value::Number(sum / args.len() as f64))
}

/// 计算薪资中位数
///
/// # 参数
/// - 薪资数组（通过多个参数传入）
///
/// # 返回
/// 中位数
pub fn calc_salary_median(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let mut salaries: Vec<f64> = Vec::new();
    for arg in args {
        salaries.push(get_number(arg)?);
    }
    salaries.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = salaries.len();
    let median = if len % 2 == 0 {
        (salaries[len / 2 - 1] + salaries[len / 2]) / 2.0
    } else {
        salaries[len / 2]
    };

    Ok(Value::Number(median))
}

/// 计算薪资范围（最大值 - 最小值）
///
/// # 参数
/// - 薪资数组（通过多个参数传入）
///
/// # 返回
/// 范围值
pub fn calc_salary_range(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    for arg in args {
        let val = get_number(arg)?;
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    Ok(Value::Number(max - min))
}

/// 计算百分位数
///
/// # 参数
/// - 百分位（如25, 50, 75, 90）
/// - 薪资数组（通过后续参数传入）
///
/// # 返回
/// 指定百分位的薪资值
pub fn calc_percentile(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let percentile = get_number(&args[0])?;
    if percentile < 0.0 || percentile > 100.0 {
        return Err(RuntimeError::InvalidOperation(
            "百分位必须在0-100之间".to_string(),
        ));
    }

    let mut salaries: Vec<f64> = Vec::new();
    for i in 1..args.len() {
        salaries.push(get_number(&args[i])?);
    }
    salaries.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (percentile / 100.0 * (salaries.len() - 1) as f64).round() as usize;
    Ok(Value::Number(salaries[index]))
}

/// 计算标准差
///
/// # 参数
/// - 薪资数组（通过多个参数传入）
///
/// # 返回
/// 标准差
pub fn calc_salary_std_dev(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    // 计算平均值
    let mut sum = 0.0;
    let mut salaries: Vec<f64> = Vec::new();
    for arg in args {
        let val = get_number(arg)?;
        salaries.push(val);
        sum += val;
    }
    let mean = sum / salaries.len() as f64;

    // 计算方差
    let mut variance_sum = 0.0;
    for salary in &salaries {
        let diff = salary - mean;
        variance_sum += diff * diff;
    }
    let variance = variance_sum / salaries.len() as f64;

    // 返回标准差
    Ok(Value::Number(variance.sqrt()))
}

/// 计算薪资分布
///
/// # 参数
/// - 区间大小（如5000）
/// - 薪资数组（通过后续参数传入）
///
/// # 返回
/// 返回各区间的数量（简化版本，返回区间数）
pub fn calc_salary_distribution(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let bin_size = get_number(&args[0])?;
    if bin_size <= 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "区间大小必须大于0".to_string(),
        ));
    }

    let mut salaries: Vec<f64> = Vec::new();
    for i in 1..args.len() {
        salaries.push(get_number(&args[i])?);
    }

    if salaries.is_empty() {
        return Ok(Value::Number(0.0));
    }

    // 找出最大值和最小值
    let mut min = salaries[0];
    let mut max = salaries[0];
    for &salary in &salaries {
        if salary < min {
            min = salary;
        }
        if salary > max {
            max = salary;
        }
    }

    // 计算需要的区间数
    let num_bins = ((max - min) / bin_size).ceil() + 1.0;

    Ok(Value::Number(num_bins))
}
