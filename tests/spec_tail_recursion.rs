//! BDD 规格：尾递归优化的语义正确性。
//!
//! 功能点与边界（已冻结）：
//! 1. **观测等价**：优化开启与关闭执行同一程序，结果值与 TRACE
//!    副作用序列必须完全一致（优化不得改变可观测行为）。
//! 2. **死代码不得复活**：尾调用 Return 之后的语句在原语义中
//!    永不执行（Return 即退出函数体），优化后同样不得执行。
//!    旧实现的转换把这类语句搬进循环体，导致每轮迭代都执行。
//! 3. **非尾 Return 正确退出**：循环化之后普通 Return 仍要带值
//!    退出函数。
//! 4. **循环体内的尾调用不再转换**（检测范围收窄到非循环位置），
//!    保持解释执行，结果必须仍然正确。
//! 5. 可优化的深尾递归（10 万层）不触发递归深度限制（栈安全）。
//!
//! 副作用观测手段：TRACE 进入字符串缓冲（take_trace），
//! TRACE_INFO 进入结构化缓冲（trace_by_category）。

use aether::{Aether, ExecutionLimits, Value};

/// 以指定优化配置运行脚本，返回（结果, TRACE 字符串缓冲）。
fn run(code: &str, optimize: bool) -> (Result<Value, String>, Vec<String>) {
    let mut engine = Aether::new();
    engine.set_optimization(optimize, optimize, optimize);
    let result = engine.eval(code);
    let traces = engine.take_trace();
    (result, traces)
}

/// 断言优化开/关的观测行为完全一致。
///
/// 未优化路径的 DSL 递归每层展开多个 Rust 帧（debug 构建实测
/// 2MB 测试线程约 35 层即临界），整个差分在 16MB 大栈线程内运行；
/// `Value` 含 `Rc` 非 Send，线程内完成比对、只回传差异描述。
fn assert_equivalent(code: &str) {
    let code_for_thread = code.to_string();
    let mismatches = std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            let code = code_for_thread;
            let (off_result, off_traces) = run(&code, false);
            let (on_result, on_traces) = run(&code, true);
            let fmt = |r: &Result<Value, String>| {
                r.as_ref().map(|v| v.to_string()).map_err(|e| e.clone())
            };
            let mut problems = Vec::new();
            if fmt(&off_result) != fmt(&on_result) {
                problems.push(format!(
                    "结果不一致: 优化关={:?} 优化开={:?}",
                    fmt(&off_result),
                    fmt(&on_result)
                ));
            }
            if off_traces != on_traces {
                problems.push(format!(
                    "TRACE 序列不一致:\n  优化关={off_traces:?}\n  优化开={on_traces:?}"
                ));
            }
            problems
        })
        .expect("创建大栈线程失败")
        .join()
        .expect("差分线程不应 panic");
    assert!(mismatches.is_empty(), "{code}\n{}", mismatches.join("\n"));
}

/// 统计 TRACE 缓冲中含指定标记的条数。
fn count_tag(traces: &[String], tag: &str) -> usize {
    traces.iter().filter(|t| t.contains(tag)).count()
}

/// 经典尾递归差分电池：阶乘 / 累加 / 斐波那契 / 最大公约数。
#[test]
fn differential_classic_tail_recursion() {
    assert_equivalent(
        r#"
        Func FACTORIAL(N, ACC) {
            If (N <= 1) { Return ACC }
            Return FACTORIAL(N - 1, ACC * N)
        }
        FACTORIAL(10, 1)
        "#,
    );
    // 注意：差分在大栈线程内运行（见 assert_equivalent），未优化路径
    // 可以安全跑到 40 层；十万层深递归由
    // deep_tail_recursion_is_stack_safe_when_optimized 单独覆盖（仅优化开）。
    assert_equivalent(
        r#"
        Func SUM_TO_N(N, ACC) {
            If (N <= 0) { Return ACC }
            Return SUM_TO_N(N - 1, ACC + N)
        }
        SUM_TO_N(40, 0)
        "#,
    );
    assert_equivalent(
        r#"
        Func FIB(N, A, B) {
            If (N == 0) { Return A }
            Return FIB(N - 1, B, A + B)
        }
        FIB(30, 0, 1)
        "#,
    );
    assert_equivalent(
        r#"
        Func GCD(A, B) {
            If (B == 0) { Return A }
            Return GCD(B, A % B)
        }
        GCD(1071, 462)
        "#,
    );
}

/// **核心回归**：尾调用 Return 之后的语句在优化后必须永不执行。
/// 旧实现把它们搬进循环体，`TRACE("DEAD", N)` 会在每轮迭代执行
/// （N=3 时打印 3 次）。
#[test]
fn dead_code_after_tail_return_never_runs() {
    let code = r#"
        Func F(N) {
            TRACE("ALIVE", N)
            If (N <= 0) {
                Return 0
            }
            Return F(N - 1)
            TRACE("DEAD", N)
        }
        F(3)
    "#;
    let (result, traces) = run(code, true);
    let v = result.expect("优化后求值失败");
    if let Value::Number(n) = v {
        assert_eq!(n, 0.0);
    } else {
        panic!("预期 Number，实际 {v:?}");
    }
    // 未优化时 ALIVE 出现 4 次（N=3,2,1,0），DEAD 0 次——优化后必须一致
    assert_eq!(count_tag(&traces, "ALIVE"), 4);
    assert_eq!(
        count_tag(&traces, "DEAD"),
        0,
        "尾调用 Return 后的死代码被执行了"
    );
}

/// If 表达式分支中的尾调用：分支之后的语句同样不得执行。
/// 旧实现：If 表达式求值完成后落空继续执行 `TRACE("AFTER", N)`。
#[test]
fn statements_after_if_expression_tail_call_are_skipped() {
    let code = r#"
        Func H(N) {
            Set R (If (N <= 0) { Return 0 } Else { Return H(N - 1) })
            TRACE("AFTER", N)
            R
        }
        H(2)
    "#;
    let (result, traces) = run(code, true);
    let v = result.expect("优化后求值失败");
    if let Value::Number(n) = v {
        assert_eq!(n, 0.0);
    } else {
        panic!("预期 Number，实际 {v:?}");
    }
    assert_eq!(
        count_tag(&traces, "AFTER"),
        0,
        "If 分支尾调用后的语句被执行了"
    );
    // 对照：未优化时行为一致
    let (_, off_traces) = run(code, false);
    assert_eq!(count_tag(&off_traces, "AFTER"), 0);
}

/// 非尾 Return：条件满足时带值退出函数（优化后同样正确）。
#[test]
fn non_tail_return_exits_with_value() {
    let code = r#"
        Func G(N) {
            If (N >= 3) {
                Return 99
            }
            Return G(N + 1)
        }
        G(0)
    "#;
    let (result, _) = run(code, true);
    let v = result.expect("优化后求值失败");
    if let Value::Number(n) = v {
        assert_eq!(n, 99.0);
    } else {
        panic!("预期 Number，实际 {v:?}");
    }
}

/// 循环体内的尾调用不再被转换（检测收窄），但结果必须与未优化一致。
#[test]
fn tail_call_inside_loop_is_not_transformed_but_correct() {
    assert_equivalent(
        r#"
        Func W(N) {
            While (N < 5) {
                Return W(N + 1)
            }
            Return N
        }
        W(0)
        "#,
    );
    let (result, _) = run(
        r#"
        Func W(N) {
            While (N < 5) {
                Return W(N + 1)
            }
            Return N
        }
        W(0)
        "#,
        true,
    );
    let v = result.expect("优化后求值失败");
    if let Value::Number(n) = v {
        assert_eq!(n, 5.0);
    } else {
        panic!("预期 Number，实际 {v:?}");
    }
}

/// 每轮迭代的 TRACE 序列在优化开/关之间必须一致（副作用顺序等价）。
#[test]
fn trace_sequence_is_equivalent() {
    assert_equivalent(
        r#"
        Func F(N, ACC) {
            TRACE("STEP", N)
            If (N <= 1) { Return ACC }
            Return F(N - 1, ACC * N)
        }
        F(5, 1)
        "#,
    );
}

/// 深尾递归（10 万层）：优化后不触发递归深度限制（栈安全）。
/// 只测优化开启——未优化时解释器按默认 1000 层限制直接报错，
/// 这属于预期行为，不参与本断言。
#[test]
fn deep_tail_recursion_is_stack_safe_when_optimized() {
    let limits = ExecutionLimits {
        max_steps: None,
        max_recursion_depth: Some(1000),
        max_duration_ms: Some(30_000),
        max_memory_bytes: None,
    };
    let mut engine = Aether::new().with_limits(limits);
    engine.set_optimization(true, true, true);
    let result = engine.eval(
        r#"
        Func SUM_TO_N(N, ACC) {
            If (N <= 0) { Return ACC }
            Return SUM_TO_N(N - 1, ACC + N)
        }
        SUM_TO_N(100000, 0)
        "#,
    );
    let v = result.expect("10 万层尾递归应被循环化，不应触发递归限制");
    if let Value::Number(n) = v {
        assert!((n - 5000050000.0).abs() < 1e-6, "实际 {n}");
    } else {
        panic!("预期 Number，实际 {v:?}");
    }
}
