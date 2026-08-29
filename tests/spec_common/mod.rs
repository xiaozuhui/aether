//! BDD 规格测试的公共辅助函数。
//!
//! 这些规格测试（tests/spec_*.rs）遵循 BDD 流程：先按冻结的功能点与边界
//! 写下预期行为（多数在修复前应当失败），再实现代码使其转绿。
#![allow(dead_code)]

use aether::{Aether, Value};

/// 在默认权限的全新引擎上求值一段脚本。
pub fn eval(code: &str) -> Result<Value, String> {
    let mut engine = Aether::new();
    engine.eval(code)
}

/// 断言脚本求值**成功**，返回结果值（失败则 panic 并带错误信息）。
pub fn eval_ok(code: &str) -> Value {
    eval(code).unwrap_or_else(|e| panic!("预期求值成功，实际得到错误: {e}"))
}

/// 断言脚本求值**失败**，返回错误信息。
pub fn eval_err(code: &str) -> String {
    eval(code).expect_err("预期求值失败，实际得到成功")
}

/// 断言结果是一个数值，且在 1e-6 容差内等于 expected。
///
/// 注意：失败信息只带类型名，不做 `{:?}` 深度格式化——
/// Lazy 值持有环境引用（环），Debug 递归会栈溢出。
pub fn assert_number(v: &Value, expected: f64) {
    match v {
        Value::Number(n) => assert!((n - expected).abs() < 1e-6, "预期数值 {expected}，实际 {n}"),
        other => panic!("预期 Number，实际 {}", other.type_name()),
    }
}

/// 断言结果是布尔值且等于 expected。
pub fn assert_bool(v: &Value, expected: bool) {
    match v {
        Value::Boolean(b) => assert_eq!(*b, expected, "预期布尔 {expected}，实际 {b}"),
        other => panic!("预期 Boolean，实际 {}", other.type_name()),
    }
}

/// 断言结果是字符串且等于 expected。
pub fn assert_str(v: &Value, expected: &str) {
    match v {
        Value::String(s) => assert_eq!(s, expected),
        other => panic!("预期 String，实际 {}", other.type_name()),
    }
}

/// 断言结果是 Fraction，且分子/分母**按 BigInt 十进制字符串精确相等**。
///
/// 数值正确性是本项目的核心目标，因此分数断言不经过 f64（会引入精度噪声），
/// 而是直接比较 BigInt 的字符串形式。
pub fn assert_fraction(v: &Value, numer: &str, denom: &str) {
    match v {
        Value::Fraction(f) => {
            assert_eq!(f.numer().to_string(), numer, "分子不符");
            assert_eq!(f.denom().to_string(), denom, "分母不符");
        }
        other => panic!("预期 Fraction，实际 {}", other.type_name()),
    }
}
