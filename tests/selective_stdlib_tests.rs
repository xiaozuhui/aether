// tests/selective_stdlib_tests.rs
//! 测试选择性加载标准库的功能

use aether::Aether;

#[test]
fn test_selective_string_utils() {
    // 只加载 string_utils 模块
    let mut engine = Aether::new()
        .with_stdlib_string_utils()
        .expect("Failed to load string_utils");

    // 测试 string_utils 中的函数可用
    let code = r#"
        Set TEXT "hello world"
        Set UPPER (STR_TO_UPPER(TEXT))
        UPPER
    "#;

    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "HELLO WORLD");
}

#[test]
fn test_selective_array_utils() {
    // 只加载 array_utils 模块
    let mut engine = Aether::new()
        .with_stdlib_array_utils()
        .expect("Failed to load array_utils");

    // 测试 array_utils 中的函数可用
    let code = r#"
        Set ARR [1, 2, 3, 2, 1, 3]
        Set UNIQUE (ARR_UNIQUE(ARR))
        UNIQUE
    "#;

    let result = engine.eval(code).expect("Failed to eval");
    // 结果应该是去重后的数组
    let result_str = result.to_string();
    assert!(result_str.contains("1") && result_str.contains("2") && result_str.contains("3"));
}

#[test]
fn test_chained_loading() {
    // 链式加载多个模块
    let mut engine = Aether::new()
        .with_stdlib_string_utils()
        .expect("Failed to load string_utils")
        .with_stdlib_array_utils()
        .expect("Failed to load array_utils")
        .with_stdlib_validation()
        .expect("Failed to load validation");

    // 测试多个模块的函数都可用
    let code = r#"
        Set TEXT "test@example.com"
        Set IS_EMAIL (VALIDATE_EMAIL(TEXT))
        IS_EMAIL
    "#;

    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_selective_vs_full() {
    // 测试选择性加载和完整加载的功能一致性

    // 完整加载
    let mut engine_full = Aether::with_stdlib().expect("Failed to load full stdlib");

    // 选择性加载
    let mut engine_selective = Aether::new()
        .with_stdlib_string_utils()
        .expect("Failed to load string_utils");

    let code = r#"
        Set TEXT "hello"
        (STR_TO_UPPER(TEXT))
    "#;

    let result_full = engine_full.eval(code).expect("Failed with full stdlib");
    let result_selective = engine_selective
        .eval(code)
        .expect("Failed with selective stdlib");

    assert_eq!(result_full.to_string(), result_selective.to_string());
}

#[test]
fn test_json_module() {
    // 测试 JSON 模块加载
    let mut engine = Aether::new()
        .with_stdlib_json()
        .expect("Failed to load json module");

    let code = r#"
        Set OBJ {"name":"Alice", "age": 30}
        Set JSON_STR (JSON_STRINGIFY(OBJ))
        Set PRETTY (JSON_PRETTY_DEFAULT(JSON_STR))
        PRETTY
    "#;

    let result = engine.eval(code).expect("Failed to eval");
    let result_str = result.to_string();

    // JSON 字符串应该包含这些内容
    assert!(result_str.contains("name") || result_str.contains("Alice"));
}

#[test]
fn test_data_structures() {
    // 测试数据结构模块
    let mut engine = Aether::new()
        .with_stdlib_set()
        .expect("Failed to load set")
        .with_stdlib_queue()
        .expect("Failed to load queue")
        .with_stdlib_stack()
        .expect("Failed to load stack");

    // 测试 Set
    let code_set = r#"
        Set MY_SET (SET_NEW())
        Set MY_SET (SET_ADD(MY_SET, 1))
        Set MY_SET (SET_ADD(MY_SET, 2))
        (SET_CONTAINS(MY_SET, 1))
    "#;

    let result = engine.eval(code_set).expect("Failed to eval set");
    assert_eq!(result.to_string(), "true");

    // 测试 Stack
    let code_stack = r#"
        Set MY_STACK (STACK_NEW())
        Set MY_STACK (STACK_PUSH(MY_STACK, 10))
        Set MY_STACK (STACK_PUSH(MY_STACK, 20))
        (STACK_SIZE(MY_STACK))
    "#;

    let result = engine.eval(code_stack).expect("Failed to eval stack");
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_functional_module() {
    // 测试函数式编程模块
    let mut engine = Aether::new()
        .with_stdlib_functional()
        .expect("Failed to load functional");

    let code = r#"
        Set NUMBERS [1, 2, 3, 4, 5]
        Func GREATER_THAN_THREE(X) {
            Return (X > 3)
        }
        Set RESULT (FIND(NUMBERS, GREATER_THAN_THREE))
        RESULT
    "#;

    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_datetime_module() {
    // 测试日期时间模块
    let mut engine = Aether::new()
        .with_stdlib_datetime()
        .expect("Failed to load datetime");

    let code = r#"
        Set IS_VALID (DT_IS_VALID_DATE(2024, 11, 8))
        IS_VALID
    "#;

    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "true");
}
