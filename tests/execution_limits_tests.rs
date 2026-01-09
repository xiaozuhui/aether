//! 执行限制功能的集成测试
//!
//! 测试步数限制、递归深度限制、执行超时等

use aether::{Aether, ExecutionLimits};

#[test]
fn test_step_limit_prevents_infinite_loop() {
    // 创建步数限制为 10 步的引擎
    let limits = ExecutionLimits {
        max_steps: Some(10),
        max_recursion_depth: None,
        max_duration_ms: None,
        max_memory_bytes: None,
    };

    let mut engine = Aether::new().with_limits(limits);

    // 创建一个会无限循环的代码
    let code = r#"
        Set I 0
        While (I < 10000) {
            Set I 0
        }
    "#;

    let result = engine.eval(code);
    assert!(result.is_err(), "Should fail due to step limit");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("step limit exceeded") || err.contains("Step limit exceeded"),
        "Error should mention step limit: {}",
        err
    );
    assert!(
        err.contains("10"),
        "Error should contain the limit: {}",
        err
    );
}

#[test]
fn test_recursion_depth_limit() {
    // 创建递归深度限制为 5 层的引擎
    let limits = ExecutionLimits {
        max_steps: None,
        max_recursion_depth: Some(5),
        max_duration_ms: None,
        max_memory_bytes: None,
    };

    let mut engine = Aether::new().with_limits(limits);

    // 创建一个递归函数
    let code = r#"
        Func RECURSIVE(N) {
            If (N < 1) {
                N
            } Else {
                RECURSIVE((N - 1))
            }
        }

        RECURSIVE(10)
    "#;

    let result = engine.eval(code);
    assert!(result.is_err(), "Should fail due to recursion depth limit");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("Recursion depth limit exceeded"),
        "Error should mention recursion depth: {}",
        err
    );
}

#[test]
fn test_execution_timeout() {
    // 创建执行超时为 100ms 的引擎
    let limits = ExecutionLimits {
        max_steps: None,
        max_recursion_depth: None,
        max_duration_ms: Some(100),
        max_memory_bytes: None,
    };

    let mut engine = Aether::new().with_limits(limits);

    // 创建一个会长时间运行的代码（循环计算）
    let code = r#"
        Set SUM 0
        Set I 0
        While (I < 100000) {
            Set SUM (SUM + I)
            Set I (I + 1)
        }
        SUM
    "#;

    let result = engine.eval(code);
    // 可能会因为步数限制先触发，也可能因为超时触发
    // 只要有一个限制生效就行
    assert!(result.is_err(), "Should fail due to some limit");

    let err = result.unwrap_err().to_string();
    println!("Error: {}", err);
    // 检查是否是限制错误
    assert!(
        err.contains("limit exceeded") || err.contains("Execution limit") || err.contains("step limit"),
        "Error should be about limits: {}",
        err
    );
}

#[test]
fn test_custom_limits() {
    // 测试自定义限制配置
    let limits = ExecutionLimits {
        max_steps: Some(1000),
        max_recursion_depth: Some(50),
        max_duration_ms: Some(5000),
        max_memory_bytes: None,
    };

    let engine = Aether::new().with_limits(limits);
    assert_eq!(engine.limits().max_steps, Some(1000));
    assert_eq!(engine.limits().max_recursion_depth, Some(50));
    assert_eq!(engine.limits().max_duration_ms, Some(5000));
}

#[test]
fn test_set_limits() {
    // 测试动态设置限制
    let mut engine = Aether::new();

    // 初始限制
    let limits = ExecutionLimits {
        max_steps: Some(100),
        max_recursion_depth: None,
        max_duration_ms: None,
        max_memory_bytes: None,
    };

    engine.set_limits(limits.clone());
    assert_eq!(engine.limits().max_steps, Some(100));

    // 修改限制
    let new_limits = ExecutionLimits {
        max_steps: Some(200),
        max_recursion_depth: Some(10),
        max_duration_ms: None,
        max_memory_bytes: None,
    };

    engine.set_limits(new_limits);
    assert_eq!(engine.limits().max_steps, Some(200));
    assert_eq!(engine.limits().max_recursion_depth, Some(10));
}

#[test]
fn test_no_limits() {
    // 测试无限制配置
    let limits = ExecutionLimits::unrestricted();
    let mut engine = Aether::new().with_limits(limits);

    // 正常执行的代码
    let code = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;

    let result = engine.eval(code);
    assert!(result.is_ok(), "Should succeed without limits");
    assert_eq!(result.unwrap().to_string(), "30");
}

#[test]
fn test_execution_limits_default() {
    // 测试默认限制
    let limits = ExecutionLimits::default();
    assert_eq!(limits.max_steps, Some(1_000_000));
    assert_eq!(limits.max_recursion_depth, Some(1000));
    assert_eq!(limits.max_duration_ms, Some(30_000));
}

#[test]
fn test_execution_limits_strict() {
    // 测试严格限制
    let limits = ExecutionLimits::strict();
    assert_eq!(limits.max_steps, Some(100_000));
    assert_eq!(limits.max_recursion_depth, Some(100));
    assert_eq!(limits.max_duration_ms, Some(5_000));
}

#[test]
fn test_execution_limits_lenient() {
    // 测试宽松限制
    let limits = ExecutionLimits::lenient();
    assert_eq!(limits.max_steps, Some(10_000_000));
    assert_eq!(limits.max_recursion_depth, Some(5000));
    assert_eq!(limits.max_duration_ms, Some(300_000));
}

#[test]
fn test_normal_execution_with_limits() {
    // 测试在限制内正常执行
    let limits = ExecutionLimits {
        max_steps: Some(1000),
        max_recursion_depth: Some(100),
        max_duration_ms: Some(5000),
        max_memory_bytes: None,
    };

    let mut engine = Aether::new().with_limits(limits);

    // 正常执行的代码
    let code = r#"
        Func ADD(A, B) {
            Return (A + B)
        }

        Set RESULT ADD(10, 20)
        RESULT
    "#;

    let result = engine.eval(code);
    if let Err(ref e) = result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok(), "Should succeed within limits");
    assert_eq!(result.unwrap().to_string(), "30");
}

#[test]
fn test_step_counter_with_simple_code() {
    // 测试步数计数器对简单代码的计数
    let limits = ExecutionLimits {
        max_steps: Some(5), // 只允许 5 步
        max_recursion_depth: None,
        max_duration_ms: None,
        max_memory_bytes: None,
    };

    let mut engine = Aether::new().with_limits(limits);

    // 这段代码有 3 条语句，应该在限制内
    let code = r#"
        Set A 1
        Set B 2
        (A + B)
    "#;

    let result = engine.eval(code);
    assert!(result.is_ok(), "Should succeed within step limit");
    assert_eq!(result.unwrap().to_string(), "3");

    // 这段代码有 6 条语句，应该超出限制
    let code2 = r#"
        Set A 1
        Set B 2
        Set C 3
        Set D 4
        Set E 5
        Set F 6
    "#;

    let result2 = engine.eval(code2);
    assert!(result2.is_err(), "Should fail due to step limit");
}
