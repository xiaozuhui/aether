// src/builtins/payroll/tax.rs
//! 个人所得税计算函数

use crate::builtins::util::get_number;
use crate::evaluator::RuntimeError;
use crate::value::Value;

/// 中国个人所得税税率表（2019年起）
/// 级数 | 累计应纳税所得额 | 税率 | 速算扣除数
/// 1    | 不超过36000      | 3%   | 0
/// 2    | 36000-144000     | 10%  | 2520
/// 3    | 144000-300000    | 20%  | 16920
/// 4    | 300000-420000    | 25%  | 31920
/// 5    | 420000-660000    | 30%  | 52920
/// 6    | 660000-960000    | 35%  | 85920
/// 7    | 超过960000       | 45%  | 181920
///
/// 计算个人所得税（按年度累计）
///
/// # 参数
/// - 应纳税所得额（累计）
///
/// # 返回
/// 个人所得税
pub fn calc_personal_tax(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let taxable_income = get_number(&args[0])?;
    Ok(Value::Number(annual_tax(taxable_income)))
}

/// 个人所得税**年度累计**税率表（综合所得，7 档超额累进）。
///
/// 口径说明：这是年度表；月度应纳税所得额（5000 元/月起征）不能
/// 直接套用本表——先用 [`calc_taxable_income`] 按月预扣，年度汇算时
/// 再累计套用本表。`CALC_GROSS_FROM_NET` 的反推也使用本表。
fn annual_tax(taxable_income: f64) -> f64 {
    if taxable_income <= 0.0 {
        return 0.0;
    }
    let tax = if taxable_income <= 36000.0 {
        taxable_income * 0.03
    } else if taxable_income <= 144000.0 {
        taxable_income * 0.10 - 2520.0
    } else if taxable_income <= 300000.0 {
        taxable_income * 0.20 - 16920.0
    } else if taxable_income <= 420000.0 {
        taxable_income * 0.25 - 31920.0
    } else if taxable_income <= 660000.0 {
        taxable_income * 0.30 - 52920.0
    } else if taxable_income <= 960000.0 {
        taxable_income * 0.35 - 85920.0
    } else {
        taxable_income * 0.45 - 181920.0
    };
    tax.max(0.0)
}

/// 计算应纳税所得额
///
/// # 参数
/// - 应发工资
/// - 社保
/// - 公积金
/// - 专项附加扣除（默认0）
///
/// # 返回
/// 应纳税所得额
pub fn calc_taxable_income(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    let gross_salary = get_number(&args[0])?;
    let social_insurance = get_number(&args[1])?;
    let housing_fund = get_number(&args[2])?;
    let special_deduction = if args.len() > 3 {
        get_number(&args[3])?
    } else {
        0.0
    };

    // 应纳税所得额 = 应发工资 - 社保 - 公积金 - 起征点(5000) - 专项附加扣除
    let taxable = gross_salary - social_insurance - housing_fund - 5000.0 - special_deduction;

    Ok(Value::Number(taxable.max(0.0)))
}

/// 计算年终奖个税（单独计税）
///
/// # 参数
/// - 年终奖金额
///
/// # 返回
/// 年终奖个税
pub fn calc_annual_bonus_tax(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let bonus = get_number(&args[0])?;

    if bonus <= 0.0 {
        return Ok(Value::Number(0.0));
    }

    // 年终奖除以12，找到适用税率
    let monthly_avg = bonus / 12.0;

    let (rate, deduction) = if monthly_avg <= 3000.0 {
        (0.03, 0.0)
    } else if monthly_avg <= 12000.0 {
        (0.10, 210.0)
    } else if monthly_avg <= 25000.0 {
        (0.20, 1410.0)
    } else if monthly_avg <= 35000.0 {
        (0.25, 2660.0)
    } else if monthly_avg <= 55000.0 {
        (0.30, 4410.0)
    } else if monthly_avg <= 80000.0 {
        (0.35, 7160.0)
    } else {
        (0.45, 15160.0)
    };

    // 单独计税：税额 = 全额 × 税率 − 速算扣除数 × 12
    // （速算扣除数是月度表的值，对应全年需乘 12）
    let tax = bonus * rate - deduction * 12.0;
    Ok(Value::Number(tax.max(0.0)))
}

/// 计算实际税率
///
/// # 参数
/// - 税额
/// - 总收入
///
/// # 返回
/// 实际税率（百分比）
pub fn calc_effective_tax_rate(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let tax = get_number(&args[0])?;
    let total_income = get_number(&args[1])?;

    if total_income <= 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "总收入必须大于0".to_string(),
        ));
    }

    let rate = (tax / total_income) * 100.0;
    Ok(Value::Number(rate))
}

/// 从税后工资反推应发工资（简化版）
///
/// # 参数
/// - 税后工资
/// - 社保
/// - 公积金
///
/// # 返回
/// 税后反推税前（单调二分逼近）。
///
/// # 参数
/// - 税后净额
/// - 社保
/// - 公积金
///
/// # 返回
/// 税前应发工资
///
/// 口径：net = gross − ss − hf − annual_tax(max(gross − ss − hf − 5000, 0))。
/// 右端对 gross 严格单调递增（边际税率 ≤ 45% < 100%），二分必然收敛，
/// 100 次迭代精度远高于 0.01 元。
pub fn calc_gross_from_net(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    let net = get_number(&args[0])?;
    let social_insurance = get_number(&args[1])?;
    let housing_fund = get_number(&args[2])?;
    let deductions = social_insurance + housing_fund;

    // 税后净额关于税前严格单调递增，二分求根
    let net_of = |gross: f64| gross - deductions - annual_tax(gross - deductions - 5000.0);

    // 下界：无税时的税前（f(lo) ≤ net）
    let mut lo = (net + deductions).max(0.0);
    // 上界：税率最高 45%，净额 ≥ 55%×应纳税部分，此上界必使 f(hi) ≥ net
    let mut hi = (net + deductions + 5000.0) / 0.55 + 5000.0;
    if hi < lo {
        hi = lo;
    }
    for _ in 0..100 {
        let mid = (lo + hi) / 2.0;
        if net_of(mid) < net {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Ok(Value::Number((lo + hi) / 2.0))
}

/// 计算年度汇算清缴退税
///
/// # 参数
/// - 年度累计已缴税额
/// - 年度应缴税额
///
/// # 返回
/// 应退税额（正数为退税，负数为补税）
pub fn calc_tax_refund(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let paid_tax = get_number(&args[0])?;
    let due_tax = get_number(&args[1])?;

    // 退税额 = 已缴税额 - 应缴税额
    Ok(Value::Number(paid_tax - due_tax))
}
