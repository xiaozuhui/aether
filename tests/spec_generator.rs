//! BDD 规格：Generator / Yield（首次触发急切收集语义）。
//!
//! 功能点与边界（已冻结，用户选择「首次触发急切收集」）：
//! 1. `Generator NAME(params) { ... Yield expr ... }` 定义生成器函数；
//!    `Set G NAME(args)` 调用它得到生成器实例。
//! 2. **第一次 `NEXT(G)` 时完整执行函数体一次**，所有 Yield 值按序
//!    收集进内部缓冲；之后的 NEXT 逐个弹出；耗尽后 NEXT 返回 Null，
//!    `DONE(G)` 返回 true。
//! 3. 函数体内的副作用（TRACE/PRINT 等）在首次 NEXT 时**恰好发生一次**，
//!    之后再次 NEXT 不重复执行。
//! 4. `For X In G { ... }` 可直接迭代生成器直到耗尽。
//! 5. 生成器值的克隆**共享消费状态**（类似 Python 迭代器语义）：
//!    `Set G2 G` 后 NEXT(G) 与 NEXT(G2) 消费同一序列。
//! 6. 生成器外的顶层 `Yield` 是错误。
//! 7. 无限生成器（While True 内 Yield）在首次 NEXT 时触发步数上限
//!    报错，而不是永久挂起。

mod spec_common;

use aether::{Aether, ExecutionLimits, Value};
use spec_common::{assert_bool, eval_err, eval_ok};

/// 三值生成器脚本，供多个用例复用。
const THREE_YIELDS: &str = r#"
    Generator COUNT() {
        Yield 10
        Yield 20
        Yield 30
    }
"#;

/// NEXT 按定义顺序返回每个 Yield 的值。
#[test]
fn next_returns_yields_in_order() {
    let v = eval_ok(&format!(
        "{THREE_YIELDS}{}",
        r#"
        Set G COUNT()
        Set A NEXT(G)
        Set B NEXT(G)
        Set C NEXT(G)
        [A, B, C]
        "#
    ));
    match v {
        Value::Array(a) => {
            spec_common::assert_number(&a[0], 10.0);
            spec_common::assert_number(&a[1], 20.0);
            spec_common::assert_number(&a[2], 30.0);
        }
        other => panic!("预期 Array，实际 {}", other.type_name()),
    }
}

/// 耗尽后 NEXT 返回 Null（而非错误），DONE 变为 true。
#[test]
fn exhausted_generator_returns_null_and_is_done() {
    let v = eval_ok(&format!(
        "{THREE_YIELDS}{}",
        r#"
        Set G COUNT()
        Set A NEXT(G)
        Set B NEXT(G)
        Set C NEXT(G)
        Set D NEXT(G)
        Set E NEXT(G)
        [D, E, DONE(G)]
        "#
    ));
    match v {
        Value::Array(a) => {
            assert!(matches!(a[0], Value::Null), "耗尽后 NEXT 应返回 Null");
            assert!(matches!(a[1], Value::Null), "再次 NEXT 仍应返回 Null");
            assert_bool(&a[2], true);
        }
        other => panic!("预期 Array，实际 {}", other.type_name()),
    }
}

/// DONE 状态翻转：未开始 → false；消费中 → false；耗尽 → true。
#[test]
fn done_flag_transitions() {
    let v = eval_ok(&format!(
        "{THREE_YIELDS}{}",
        r#"
        Set G COUNT()
        Set D0 DONE(G)
        Set A NEXT(G)
        Set D1 DONE(G)
        Set B NEXT(G)
        Set C NEXT(G)
        Set D2 DONE(G)
        [D0, D1, D2]
        "#
    ));
    match v {
        Value::Array(a) => {
            assert_bool(&a[0], false);
            assert_bool(&a[1], false);
            assert_bool(&a[2], true);
        }
        other => panic!("预期 Array，实际 {}", other.type_name()),
    }
}

/// For X In G 直接迭代生成器直到耗尽。
#[test]
fn for_in_iterates_generator() {
    let v = eval_ok(&format!(
        "{THREE_YIELDS}{}",
        r#"
        Set G COUNT()
        Set TOTAL 0
        For X In G {
            Set TOTAL (TOTAL + X)
        }
        TOTAL
        "#
    ));
    spec_common::assert_number(&v, 60.0);
}

/// 生成器体内的副作用在首次 NEXT 时恰好发生一次：
/// 三次 NEXT 只产生一条 TRACE_INFO("GEN") 记录。
#[test]
fn generator_body_runs_exactly_once_at_first_next() {
    let mut engine = Aether::new();
    let result = engine
        .eval(
            r#"
            Generator ONCE() {
                TRACE_INFO("GEN", "effect")
                Yield 10
                Yield 20
            }
            Set G ONCE()
            Set A NEXT(G)
            Set B NEXT(G)
            Set C NEXT(G)
            A + B
            "#,
        )
        .expect("生成器脚本求值失败");
    spec_common::assert_number(&result, 30.0);
    // 耗尽后的第三次 NEXT 返回 Null（不报错）
    let v = engine.eval("C").expect("读取 C 失败");
    assert!(
        matches!(v, Value::Null),
        "耗尽后 NEXT 应为 Null，实际 {v:?}"
    );
    // 关键断言：副作用只发生一次（急切收集，而非每次 NEXT 重放）
    let count = engine.trace_by_category("GEN").len();
    assert_eq!(count, 1, "生成器体应只执行一次，实际 TRACE {count} 次");
}

/// 生成器函数支持参数绑定。
#[test]
fn generator_binds_parameters() {
    let v = eval_ok(
        r#"
        Generator TAKE(ITEMS) {
            For I In ITEMS {
                Yield I
            }
        }
        Set G TAKE([7, 8, 9])
        Set A NEXT(G)
        Set B NEXT(G)
        Set C NEXT(G)
        Set D NEXT(G)
        [A, B, C, D]
        "#,
    );
    match v {
        Value::Array(a) => {
            spec_common::assert_number(&a[0], 7.0);
            spec_common::assert_number(&a[1], 8.0);
            spec_common::assert_number(&a[2], 9.0);
            assert!(matches!(a[3], Value::Null), "耗尽后应返回 Null");
        }
        other => panic!("预期 Array，实际 {}", other.type_name()),
    }
}

/// 生成器值的克隆共享消费状态：`Set G2 G` 后两者消费同一序列。
#[test]
fn generator_clones_share_consumption_state() {
    let v = eval_ok(&format!(
        "{THREE_YIELDS}{}",
        r#"
        Set G COUNT()
        Set G2 G
        Set A NEXT(G)
        Set B NEXT(G2)
        [A, B]
        "#
    ));
    match v {
        Value::Array(a) => {
            spec_common::assert_number(&a[0], 10.0);
            // G2 与 G 共享序列：G 消费掉 10 后，G2 的首个 NEXT 是 20
            spec_common::assert_number(&a[1], 20.0);
        }
        other => panic!("预期 Array，实际 {}", other.type_name()),
    }
}

/// 生成器外的顶层 Yield 是错误（保持现状，锁定）。
#[test]
fn yield_outside_generator_is_error() {
    eval_err("Yield 5");
}

/// 无限生成器在首次 NEXT 时触发步数上限错误，而不是永久挂起。
/// 使用 strict 限制（10 万步）确保即使实现有缺陷也能快速失败。
/// 断言必须是「步数限制」类错误——NEXT 未实现时 NotCallable 也会
/// is_err，那样属于空转通过。
#[test]
fn infinite_generator_hits_step_limit_instead_of_hanging() {
    let mut engine = Aether::new().with_limits(ExecutionLimits::strict());
    let result = engine.eval(
        r#"
        Generator INF() {
            Set X 0
            While (True) {
                Yield X
                Set X (X + 1)
            }
        }
        Set G INF()
        NEXT(G)
        "#,
    );
    let err = result.expect_err("无限生成器应触发步数限制错误，而非成功或挂起");
    let upper = err.to_uppercase();
    assert!(
        upper.contains("STEP") || upper.contains("步") || upper.contains("LIMIT"),
        "应报步数上限错误（收集过程受 ExecutionLimits 约束），实际: {err}"
    );
}
