//! BDD 规格：Lazy 延迟求值（读取即强制 + 记忆化）。
//!
//! 功能点与边界（已冻结）：
//! 1. `Lazy NAME (expr)` 定义惰性值：定义时不求值（副作用为 0）。
//! 2. **读取即强制**：任何通过标识符读取该变量的表达式都会触发求值。
//! 3. **记忆化**：首次强制后结果写回变量，后续读取不再重复求值
//!    （副作用总数保持 1）。
//! 4. 自引用（`Lazy Y (Y + 1)`）必须报「循环定义」错误，而不是死循环
//!    或原生栈溢出（实现为显式循环检测：正在求值的变量再次被读取
//!    即报错，递归深度恒为 1）。
//! 5. 重新 `Set` 同名变量直接覆盖，旧 thunk 不再被强制。

mod spec_common;

use aether::{Aether, Value};

/// 带副作用的 Lazy 定义：If 表达式块内先 TRACE 再给出值 100。
const LAZY_WITH_EFFECT: &str = r#"
    Lazy X (
        If (True) {
            TRACE_INFO("LAZY", 1)
            100
        } Else {
            0
        }
    )
"#;

/// 定义后、读取前：表达式完全不执行（副作用为 0）。
#[test]
fn lazy_is_not_evaluated_until_read() {
    let mut engine = Aether::new();
    let v = engine
        .eval(&format!("{LAZY_WITH_EFFECT}0"))
        .expect("求值失败");
    spec_common::assert_number(&v, 0.0);
    assert_eq!(
        engine.trace_by_category("LAZY").len(),
        0,
        "定义后未读取，不应产生任何副作用"
    );
}

/// 首次读取触发求值：X 参与算术即被强制。
#[test]
fn lazy_is_forced_on_first_read() {
    let mut engine = Aether::new();
    let v = engine
        .eval(&format!(
            "{LAZY_WITH_EFFECT}{}",
            r#"
            Set Y (X + 1)
            Y
            "#
        ))
        .expect("求值失败");
    spec_common::assert_number(&v, 101.0);
    assert_eq!(
        engine.trace_by_category("LAZY").len(),
        1,
        "首次读取应恰好触发一次求值"
    );
}

/// 记忆化：多次读取只求值一次。
#[test]
fn lazy_is_memoized_after_first_force() {
    let mut engine = Aether::new();
    let v = engine
        .eval(&format!(
            "{LAZY_WITH_EFFECT}{}",
            r#"
            Set A X
            Set B X
            Set C (X + X)
            [A, B, C]
            "#
        ))
        .expect("求值失败");
    match v {
        Value::Array(a) => {
            spec_common::assert_number(&a[0], 100.0);
            spec_common::assert_number(&a[1], 100.0);
            spec_common::assert_number(&a[2], 200.0);
        }
        other => panic!("预期 Array，实际 {}", other.type_name()),
    }
    assert_eq!(
        engine.trace_by_category("LAZY").len(),
        1,
        "多次读取应复用首次结果，副作用只发生一次"
    );
}

/// 自引用 Lazy 必须报「循环定义」错误（不能挂起、不能崩溃进程）。
/// 实现：evaluator 维护「正在强制求值」名单，求值期间再次读取
/// 同名变量立即报错——递归深度恒为 1，不依赖递归上限，也
/// 不会撑爆原生栈。间接循环（A 引用 B、B 引用 A）同样适用。
#[test]
fn self_referential_lazy_errors_with_recursion_limit() {
    let err = spec_common::eval_err(
        r#"
        Lazy Y (Y + 1)
        Y
        "#,
    );
    let msg = err.to_string();
    assert!(
        msg.contains("Lazy") || msg.contains("循环") || msg.contains("recursion"),
        "应报循环定义错误，实际: {msg}"
    );
}

/// 重新 Set 覆盖惰性定义：读取直接得到新值，旧 thunk 不被强制。
#[test]
fn reassignment_overrides_lazy_definition() {
    let mut engine = Aether::new();
    let v = engine
        .eval(
            r#"
            Lazy X (42)
            Set X 7
            X
            "#,
        )
        .expect("求值失败");
    spec_common::assert_number(&v, 7.0);
}

/// 纯值 Lazy 的基本路径：读取得到表达式结果。
#[test]
fn lazy_returns_expression_value() {
    let v = spec_common::eval_ok(
        r#"
        Lazy X (6 * 7)
        X
        "#,
    );
    spec_common::assert_number(&v, 42.0);
}
