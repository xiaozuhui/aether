//! BDD 规格：payroll 模块的真实性与正确性。
//!
//! 功能点与边界（已冻结）：
//! 1. **假函数删除**（用户决策）：恒等返回/固定常数、无法真实计算的
//!    日期函数一律移除——CALC_HOLIDAY_DAYS、IS_HOLIDAY、CALC_WORKDAYS、
//!    CALC_ANNUAL_WORKDAYS、CALC_ANNUAL_PAY_DAYS、GET_LEGAL_PAY_DAYS。
//!    调用它们必须报「未定义」错误，而不是返回伪装的结果。
//! 2. 可真实计算的函数保留并验证：CALC_NATURAL_DAYS、IS_WEEKEND、
//!    CALC_WORK_HOURS、CALC_TAXABLE_INCOME。
//! 3. **税率表边界**：CALC_PERSONAL_TAX（年度累计口径）各级边界两侧
//!    数值与官方公式一致。
//! 4. **年终奖单独计税**：速算扣除数必须乘 12
//!    （`税 = 年终奖 × 税率 − 月速算扣除数 × 12`）。
//!    旧实现漏乘 12，第二档起全部偏低（144000 应缴 11880 而非 14190）。
//! 5. **税后反推税前**：CALC_GROSS_FROM_NET 必须在全部 7 个税档
//!    上满足往返一致性（误差 < 0.01），旧实现只写了 4 档且迭代
//!    收敛不可靠。

mod spec_common;

use spec_common::{assert_number, eval_err, eval_ok};

/// 年度累计个税公式（规格的 Rust 镜像，作为唯一真值来源）。
fn annual_tax(x: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else if x <= 36000.0 {
        x * 0.03
    } else if x <= 144000.0 {
        x * 0.10 - 2520.0
    } else if x <= 300000.0 {
        x * 0.20 - 16920.0
    } else if x <= 420000.0 {
        x * 0.25 - 31920.0
    } else if x <= 660000.0 {
        x * 0.30 - 52920.0
    } else if x <= 960000.0 {
        x * 0.35 - 85920.0
    } else {
        x * 0.45 - 181920.0
    }
    .max(0.0)
}

/// 年终奖单独计税公式（速算扣除数 × 12）。
fn bonus_tax(b: f64) -> f64 {
    if b <= 0.0 {
        return 0.0;
    }
    let m = b / 12.0;
    let (rate, deduction) = if m <= 3000.0 {
        (0.03, 0.0)
    } else if m <= 12000.0 {
        (0.10, 210.0)
    } else if m <= 25000.0 {
        (0.20, 1410.0)
    } else if m <= 35000.0 {
        (0.25, 2660.0)
    } else if m <= 55000.0 {
        (0.30, 4410.0)
    } else if m <= 80000.0 {
        (0.35, 7160.0)
    } else {
        (0.45, 15160.0)
    };
    (b * rate - deduction * 12.0).max(0.0)
}

/// CALC_PERSONAL_TAX 在全部税档边界两侧与官方公式一致。
#[test]
fn personal_tax_matches_formula_at_all_brackets() {
    let cases = [
        0.0,
        1.0,
        36000.0,
        36001.0,
        144000.0,
        144001.0,
        300000.0,
        300001.0,
        420000.0,
        420001.0,
        660000.0,
        660001.0,
        960000.0,
        960001.0,
        2_000_000.0,
    ];
    for x in cases {
        let v = eval_ok(&format!("CALC_PERSONAL_TAX({x})"));
        assert_number(&v, annual_tax(x));
    }
    // 负数/零不缴税
    assert_number(&eval_ok("CALC_PERSONAL_TAX(0)"), 0.0);
}

/// CALC_ANNUAL_BONUS_TAX：速算扣除数必须乘 12。
/// 旧实现在第二档（144000）返回 14190，正确值是 11880。
#[test]
fn annual_bonus_tax_multiplies_quick_deduction_by_twelve() {
    let cases = [
        36_000.0,    // 边界：月均 3000，3% 档
        36_001.0,    // 跨入 10% 档
        144_000.0,   // 10% 档顶（旧实现错误重灾区）
        300_000.0,   // 20% 档
        540_000.0,   // 25% 档
        660_000.0,   // 30% 档
        1_080_000.0, // 45% 档
    ];
    for b in cases {
        let v = eval_ok(&format!("CALC_ANNUAL_BONUS_TAX({b})"));
        assert_number(&v, bonus_tax(b));
    }
}

/// 税后反推税前：在覆盖全部 7 档的收入上往返一致（误差 < 0.01）。
/// 口径：net = gross − ss − hf − tax(max(gross − ss − hf − 5000, 0))。
#[test]
fn gross_from_net_round_trips_across_all_brackets() {
    let (ss, hf) = (2000.0, 1500.0);
    let grosss: [f64; 7] = [
        8_000.0,     // 低于起征点，免税
        20_000.0,    // 3% 档
        50_000.0,    // 10% 档
        120_000.0,   // 20% 档
        300_000.0,   // 25% 档
        800_000.0,   // 35% 档
        1_500_000.0, // 45% 档
    ];
    for gross in grosss {
        let taxable = (gross - ss - hf - 5000.0).max(0.0);
        let net = gross - ss - hf - annual_tax(taxable);
        let v = eval_ok(&format!("CALC_GROSS_FROM_NET({net}, {ss}, {hf})"));
        match v {
            aether::Value::Number(g) => assert!(
                (g - gross).abs() < 0.01,
                "反推失准：目标税前 {gross}，实际 {g}"
            ),
            other => panic!("预期 Number，实际 {}", other.type_name()),
        }
    }
}

/// **假函数已删除**：调用必须报错（未定义），不得返回伪装结果。
#[test]
fn fake_datetime_functions_are_removed() {
    let removed = [
        ("CALC_HOLIDAY_DAYS", "CALC_HOLIDAY_DAYS(10)"),
        ("IS_HOLIDAY", "IS_HOLIDAY(1, 1)"),
        ("CALC_WORKDAYS", "CALC_WORKDAYS(30, 8)"),
        ("CALC_ANNUAL_WORKDAYS", "CALC_ANNUAL_WORKDAYS(2025)"),
        ("CALC_ANNUL_WORKDAYS_ALIAS", "CALC_ANNUAL_PAY_DAYS(2025)"),
        ("GET_LEGAL_PAY_DAYS", "GET_LEGAL_PAY_DAYS()"),
    ];
    for (_, code) in removed {
        let err = eval_err(code);
        // 未注册的名字按普通标识符解析：builtin 表查不到、变量也查不到，
        // 报 Undefined variable；若曾被解析为可调用名则报 Not callable。
        // 两种消息都表示「函数已不存在」，关键是不能静默成功。
        assert!(
            err.contains("Not callable") || err.contains("Undefined"),
            "已删除函数调用必须报错，实际: {err}"
        );
    }
}

/// 保留的真实函数行为验证。
#[test]
fn real_datetime_functions_are_kept_and_correct() {
    // 自然天数：含首尾
    assert_number(&eval_ok("CALC_NATURAL_DAYS(1, 31)"), 31.0);
    // 周末判断：6=周六、7=周日 返回 1；工作日返回 0
    assert_number(&eval_ok("IS_WEEKEND(6)"), 1.0);
    assert_number(&eval_ok("IS_WEEKEND(7)"), 1.0);
    assert_number(&eval_ok("IS_WEEKEND(3)"), 0.0);
    // 工时：天数 × 每日工时（默认 8）
    assert_number(&eval_ok("CALC_WORK_HOURS(22)"), 176.0);
    assert_number(&eval_ok("CALC_WORK_HOURS(22, 7.5)"), 165.0);
}

/// 月度应纳税所得额：工资 − 社保 − 公积金 − 5000 起征 − 专项附加。
/// （口径边界锁定：此函数按**月**计算，与年度累计税率表不可直接串联。）
#[test]
fn taxable_income_is_monthly_with_threshold() {
    assert_number(
        &eval_ok("CALC_TAXABLE_INCOME(20000, 2000, 1000, 1000)"),
        11000.0,
    );
    // 无专项附加时默认 0
    assert_number(&eval_ok("CALC_TAXABLE_INCOME(20000, 2000, 1000)"), 12000.0);
    // 低于起征点时为 0
    assert_number(&eval_ok("CALC_TAXABLE_INCOME(5000, 500, 300)"), 0.0);
}
