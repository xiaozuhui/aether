//! BDD 规格：数值精确性（TO_FRACTION 重建 / 混合运算提升 / 大数乘法）。
//!
//! 功能点与边界（已冻结）：
//! 1. `f64` → Fraction 的转换必须能从浮点舍入噪声中**还原本意分数**：
//!    `TO_FRACTION(1/3)` 得到 1/3，而不是 0.333... 的精确十进制展开
//!    （333.../10^16）。实现采用连分数渐近分数重建：返回第一个满足
//!    「浮点往返严格相等」的最简分数。
//! 2. 科学计数法字面量（1e-7、1.5e3）必须正确转换（旧实现走
//!    `format!("{}")` 找小数点，遇到 'e' 直接退化为 `as i64` → 0）。
//! 3. 混合运算规则：**只要操作数里有 Fraction，就把 Number 提升为
//!    Fraction 做精确计算**；纯 Number（整数/小数）运算保持 f64。
//!    （用户决策：`1/3` 是 f64，但 `0.5 + TO_FRACTION(1/3)` 必须是 5/6。）
//! 4. Number × Fraction 的整数 Number 不得经 `as i64` 转换（会在
//!    ±9.2e18 之外饱和截断），必须经十进制字符串直达 BigInt。

mod spec_common;

use aether::Value;
use spec_common::{assert_bool, assert_fraction, assert_number, eval_ok};

/// `1/3` 在 Aether 中是 f64 除法（0.3333333333333333），
/// TO_FRACTION 必须把它还原为最简分数 1/3。
#[test]
fn to_fraction_recovers_one_third_from_float_division() {
    let v = eval_ok("TO_FRACTION(1/3)");
    assert_fraction(&v, "1", "3");
}

/// 0.1 的 f64 值有舍入误差，但 TO_FRACTION 应还原为 1/10。
#[test]
fn to_fraction_recovers_one_tenth() {
    let v = eval_ok("TO_FRACTION(0.1)");
    assert_fraction(&v, "1", "10");
}

/// 科学计数法小数：旧实现 format!("{}") 得到 "1e-7"，找不到小数点，
/// 走 `as i64` 分支得到 0/1。期望 1/10000000。
#[test]
fn to_fraction_handles_small_scientific_notation() {
    let v = eval_ok("TO_FRACTION(1e-7)");
    assert_fraction(&v, "1", "10000000");
}

/// 科学计数法大数：1.5e3 = 1500，应走整数快路径得到 1500/1。
#[test]
fn to_fraction_handles_large_scientific_notation() {
    let v = eval_ok("TO_FRACTION(1.5e3)");
    assert_fraction(&v, "1500", "1");
}

/// 普通小数：2.5 → 5/2。
#[test]
fn to_fraction_plain_decimal() {
    let v = eval_ok("TO_FRACTION(2.5)");
    assert_fraction(&v, "5", "2");
}

/// TO_FRACTION 作用于已经是 Fraction 的值应原样返回（幂等）。
#[test]
fn to_fraction_is_identity_on_fraction() {
    let v = eval_ok("TO_FRACTION(TO_FRACTION(1/3))");
    assert_fraction(&v, "1", "3");
}

/// 往返一致性：对一批典型值，TO_FLOAT(TO_FRACTION(x)) 必须与 x
/// **严格相等**（新语义下 Number 相等是位相等，因此这是强断言）。
#[test]
fn to_float_to_fraction_round_trip_is_exact() {
    let values = [
        "1/3",
        "0.1",
        "2.5",
        "3.14159265358979",
        "1e-7",
        "1e15",
        "999999999999999",     // 15 位，f64 精确整数
        "0.30000000000000004", // 0.1+0.2 的浮点结果
    ];
    for x in values {
        let code = format!("(TO_FLOAT(TO_FRACTION({x})) == {x})");
        let v = eval_ok(&code);
        assert_bool(&v, true);
    }
}

/// **核心回归**：Number × Fraction 的整数 Number 曾用
/// `BigInt::from(*a as i64)` 转换，1e19 超出 i64 范围被饱和为
/// i64::MAX，再乘 3 得到 27670116110564327421。
/// 期望精确结果 30000000000000000000。
#[test]
fn multiply_big_number_with_fraction_is_exact() {
    let v = eval_ok("TO_FLOAT(10000000000000000000) * TO_FRACTION(3)");
    assert_fraction(&v, "30000000000000000000", "1");
}

/// 混合加法提升：0.5 + Fraction(1/3) 应精确计算为 Fraction 5/6，
/// 而不是退化为 f64 浮点（旧行为）。
#[test]
fn mixed_add_lifts_number_to_fraction() {
    let v = eval_ok("0.5 + TO_FRACTION(1/3)");
    assert_fraction(&v, "5", "6");
}

/// 混合减法提升：Fraction(1/4) - 0.25 精确等于 Fraction 0/1。
#[test]
fn mixed_subtract_lifts_number_to_fraction() {
    let v = eval_ok("TO_FRACTION(1/4) - 0.25");
    assert_fraction(&v, "0", "1");
}

/// 混合除法提升：0.5 / Fraction(1/4) 精确等于 Fraction 2/1。
#[test]
fn mixed_divide_lifts_number_to_fraction() {
    let v = eval_ok("0.5 / TO_FRACTION(1/4)");
    assert_fraction(&v, "2", "1");
}

/// 纯 Number 运算保持 f64（用户决策的另一半边界）：
/// `1/3` 是 f64；`4/2` 是 Number 2 而不是 Fraction。
#[test]
fn pure_number_arithmetic_stays_f64() {
    let v = eval_ok("1 / 3");
    match v {
        Value::Number(n) => assert!(
            (n - 0.3333333333333333_f64).abs() < 1e-15,
            "1/3 应为 f64，实际 {n}"
        ),
        other => panic!("预期 Number，实际 {}", other.type_name()),
    }
    let v = eval_ok("4 / 2");
    assert_number(&v, 2.0);
}

/// FRAC_DIV 除零必须报错（保护性回归）。
#[test]
fn frac_div_by_zero_errors() {
    spec_common::eval_err("FRAC_DIV(TO_FRACTION(1), TO_FRACTION(0))");
}

/// 大整数热循环不得退化为 O(n²)：整数分数（分母 1）的乘法跳过 gcd 规约。
///
/// 背景：`Ratio` 的算术运算符内置 gcd 规约，而 num-bigint 的二进制 gcd
/// 对大整数逐位移位——「累乘阶乘」这类每次乘一个小整数的循环中，大操作数
/// 的低位有大量零比特，规约会反复对整个大数移位，总代价 O(n²)：
/// 2000 次累乘（结果 5736 位）修复前需要 30 秒以上。
///
/// 修复：分母为 1 的整数乘整数直接 `Ratio::new_raw` 构造结果（分母 1 的
/// 分数恒为最简形式，无需约分）。本用例以 10 秒时限（修复后 <1 秒）守住
/// 该路径不再回归。
#[test]
fn big_integer_hot_loop_does_not_degrade_quadratically() {
    use aether::{Aether, ExecutionLimits};
    use std::time::Instant;

    let engine = Aether::new().with_limits(ExecutionLimits {
        max_steps: Some(1_000_000),
        max_recursion_depth: None,
        max_duration_ms: Some(10_000),
        max_memory_bytes: None,
    });
    let mut engine = engine;

    let code = r#"
        Set ACC 1
        Set I 1
        While (I < 2000) {
            Set I I + 1
            Set ACC ACC * I
        }
        ACC
    "#;

    let start = Instant::now();
    let v = engine
        .eval(code)
        .expect("2000 次整数累乘应在时限内完成（若超时说明 gcd 规约路径回归）");
    let elapsed = start.elapsed();

    // 2000! 是 5736 位十进制数，以首位片段核对确实算到了头
    let s = v.to_string();
    assert_eq!(s.len(), 5736, "2000! 应为 5736 位十进制数");
    assert!(
        s.starts_with("331627509245"),
        "2000! 首部应为 331627509245…"
    );

    // 带余量的绝对上限：修复后 debug 构建约百毫秒量级
    assert!(
        elapsed.as_secs() < 5,
        "2000 次整数累乘耗时 {elapsed:?}，疑似 gcd 规约路径回归"
    );
}
