//! BDD 规格：调试器条件断点（真实现）。
//!
//! 功能点与边界（已冻结）：
//! 1. 条件断点（BreakpointType::Conditional）在**位置命中**后，
//!    还要用**当前环境**求值条件表达式，仅当结果为真才暂停。
//!    旧行为：condition 字段从未被求值，与普通行断点完全相同。
//! 2. 条件为假 → 不暂停，继续执行。
//! 3. 条件表达式解析失败 → 视为不暂停（安全降级，不中断执行）。
//!
//! 测试沿用 debugger_tests.rs 的范式：attach_debugger 挂自定义钩子，
//! 钩子记录暂停时的行号后以 Continue 模式恢复，不经过 stdin 交互。

use aether::debugger::{BreakpointType, DebuggerState, ExecutionMode};
use aether::{Aether, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// 目标脚本：第 4 行（Set TOTAL）是 For 循环体，执行 4 次，
/// 循环变量 I 依次为 1、2、3、4（注意 r#" 字符串首换行占第 1 行，
/// 第 3 行是 For 语句本身，只在循环开始前执行一次且 I 尚未定义）。
const LOOP_SCRIPT: &str = r#"
Set TOTAL 0
For I In [1, 2, 3, 4] {
    Set TOTAL (TOTAL + I)
}
TOTAL
"#;

/// 构造挂了条件断点（第 3 行）的调试引擎，paused 记录每次暂停的行号。
fn conditional_engine(condition: &str, paused: Rc<RefCell<Vec<usize>>>) -> Aether {
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state
        .borrow_mut()
        .set_breakpoint(BreakpointType::Conditional {
            file: "test.aether".to_string(),
            line: 4,
            condition: condition.to_string(),
        });

    let mut engine = Aether::new();
    engine.set_source_file("test.aether".to_string());
    engine.attach_debugger(
        state.clone(),
        Box::new(move |ev: &mut aether::evaluator::Evaluator| {
            paused.borrow_mut().push(ev.get_current_line());
            // 恢复执行（Continue），返回 false 表示不退出
            state
                .borrow_mut()
                .set_execution_mode(ExecutionMode::Continue);
            false
        }),
    );
    engine
}

/// 条件为真才暂停：断点行执行 4 次（I=1..4），仅 I==3 时暂停一次。
#[test]
fn conditional_breakpoint_pauses_only_when_condition_is_true() {
    let paused = Rc::new(RefCell::new(Vec::new()));
    let mut engine = conditional_engine("I == 3", Rc::clone(&paused));

    let result = engine.eval(LOOP_SCRIPT).expect("求值失败");
    if let Value::Number(n) = result {
        assert_eq!(n, 10.0, "1+2+3+4 应为 10");
    } else {
        panic!("预期 Number，实际 {result:?}");
    }
    assert_eq!(*paused.borrow(), vec![4], "应只在 I==3 的那次命中时暂停");
}

/// 条件恒为假：位置命中 4 次但从不暂停，脚本正常跑完。
#[test]
fn conditional_breakpoint_with_false_condition_never_pauses() {
    let paused = Rc::new(RefCell::new(Vec::new()));
    let mut engine = conditional_engine("I == 99", Rc::clone(&paused));

    let result = engine.eval(LOOP_SCRIPT).expect("求值失败");
    if let Value::Number(n) = result {
        assert_eq!(n, 10.0);
    } else {
        panic!("预期 Number，实际 {result:?}");
    }
    assert!(paused.borrow().is_empty(), "条件恒假时不应有任何暂停");
}

/// 条件表达式非法（解析失败）：安全降级为不暂停，不影响执行结果。
#[test]
fn conditional_breakpoint_with_invalid_condition_does_not_pause() {
    let paused = Rc::new(RefCell::new(Vec::new()));
    let mut engine = conditional_engine("I ==", Rc::clone(&paused));

    let result = engine.eval(LOOP_SCRIPT).expect("非法条件不应中断执行");
    if let Value::Number(n) = result {
        assert_eq!(n, 10.0);
    } else {
        panic!("预期 Number，实际 {result:?}");
    }
    assert!(paused.borrow().is_empty(), "非法条件应视为不暂停");
}
