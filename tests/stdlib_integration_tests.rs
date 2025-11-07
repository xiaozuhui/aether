// tests/stdlib_integration_tests.rs
// 标准库集成测试

use aether::Aether;

#[test]
fn test_stdlib_embedded() {
    // 验证标准库代码已嵌入
    assert!(aether::stdlib::STRING_UTILS.len() > 0);
    assert!(aether::stdlib::ARRAY_UTILS.len() > 0);
    assert!(aether::stdlib::VALIDATION.len() > 0);
    assert!(aether::stdlib::DATETIME.len() > 0);
    assert!(aether::stdlib::TESTING.len() > 0);
}

#[test]
fn test_load_stdlib_module() {
    let mut engine = Aether::new();

    // 加载字符串工具模块
    engine
        .load_stdlib_module("string_utils")
        .expect("Failed to load string_utils");

    // 测试 STR_TRIM 函数
    let result = engine
        .eval(r#"STR_TRIM("  hello  ")"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "hello");
}

#[test]
fn test_load_all_stdlib() {
    let mut engine = Aether::new();

    // 加载所有标准库
    engine.load_all_stdlib().expect("Failed to load stdlib");

    // 测试各个模块的函数

    // 1. 字符串工具
    let result = engine
        .eval(r#"STR_TO_UPPER("hello")"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "HELLO");

    // 2. 数组工具
    let result = engine
        .eval(r#"ARR_SUM([1, 2, 3, 4, 5])"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "15");

    // 3. 验证库
    let result = engine
        .eval(r#"VALIDATE_EMAIL("test@example.com")"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "true");

    // 4. 日期时间
    let result = engine
        .eval(r#"DT_IS_LEAP_YEAR(2024)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_with_stdlib_constructor() {
    // 使用便捷构造函数
    let mut engine = Aether::with_stdlib().expect("Failed to create engine with stdlib");

    // 验证标准库已加载
    let result = engine
        .eval(r#"STR_REPEAT("*", 5)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "*****");
}

#[test]
fn test_string_utils_functions() {
    let mut engine = Aether::with_stdlib().expect("Failed to create engine");

    // 测试字符串分割
    let code = r#"
        Set PARTS STR_SPLIT("a,b,c", ",")
        ArrLen(PARTS)
    "#;
    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "3");

    // 测试字符串连接
    let code = r#"STR_JOIN(["hello", "world"], " ")"#;
    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "hello world");
}

#[test]
fn test_array_utils_functions() {
    let mut engine = Aether::with_stdlib().expect("Failed to create engine");

    // 测试数组去重
    let code = r#"
        Set ARR [1, 2, 2, 3, 3, 3]
        Set UNIQUE ARR_UNIQUE(ARR)
        ArrLen(UNIQUE)
    "#;
    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "3");

    // 测试数组最大值
    let code = r#"ARR_MAX([1, 5, 3, 9, 2])"#;
    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "9");

    // 测试数组平均值
    let code = r#"ARR_AVERAGE([10, 20, 30, 40, 50])"#;
    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "30");
}

#[test]
fn test_validation_functions() {
    let mut engine = Aether::with_stdlib().expect("Failed to create engine");

    // 测试邮箱验证
    let result = engine
        .eval(r#"VALIDATE_EMAIL("user@example.com")"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "true");

    let result = engine
        .eval(r#"VALIDATE_EMAIL("invalid")"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "false");

    // 测试范围验证
    let result = engine
        .eval(r#"VALIDATE_RANGE(50, 0, 100)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "true");

    let result = engine
        .eval(r#"VALIDATE_RANGE(150, 0, 100)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "false");
}

#[test]
fn test_datetime_functions() {
    let mut engine = Aether::with_stdlib().expect("Failed to create engine");

    // 测试闰年判断
    let result = engine
        .eval(r#"DT_IS_LEAP_YEAR(2024)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "true");

    let result = engine
        .eval(r#"DT_IS_LEAP_YEAR(2023)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "false");

    // 测试月份天数
    let result = engine
        .eval(r#"DT_DAYS_IN_MONTH(2024, 2)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "29");

    // 测试日期格式化
    let result = engine
        .eval(r#"DT_FORMAT_DATE(2024, 12, 25)"#)
        .expect("Failed to eval");
    assert_eq!(result.to_string(), "2024-12-25");
}

#[test]
fn test_testing_framework() {
    let mut engine = Aether::with_stdlib().expect("Failed to create engine");

    // 测试 Mock 对象创建
    let code = r#"
        Set MOCK MOCK_CREATE()
        MOCK_WAS_CALLED(MOCK)
    "#;
    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "false");

    // 测试数据生成
    let code = r#"
        Set ARR TEST_DATA_INT_ARRAY(5, 10)
        ArrLen(ARR)
    "#;
    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_complex_stdlib_usage() {
    let mut engine = Aether::with_stdlib().expect("Failed to create engine");

    // 综合测试：数据清洗和验证
    let code = r#"
        // 清洗邮箱列表
        Set EMAILS ["  user1@example.com  ", "invalid", "user2@test.com", "  "]
        Set CLEANED []
        
        Set I 0
        While (I < ArrLen(EMAILS)) {
            Set EMAIL ArrGet(EMAILS, I)
            Set EMAIL STR_TRIM(EMAIL)
            
            If (StrLen(EMAIL) > 0) {
                Set IS_VALID VALIDATE_EMAIL(EMAIL)
                If (IS_VALID) {
                    Set CLEANED ArrPush(CLEANED, EMAIL)
                }
            }
            
            Set I (I + 1)
        }
        
        ArrLen(CLEANED)
    "#;

    let result = engine.eval(code).expect("Failed to eval");
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_selective_module_loading() {
    let mut engine = Aether::new();

    // 只加载字符串和数组工具
    engine
        .load_stdlib_module("string_utils")
        .expect("Failed to load");
    engine
        .load_stdlib_module("array_utils")
        .expect("Failed to load");

    // 这些应该工作
    let _ = engine.eval(r#"STR_TRIM("  hi  ")"#).expect("Should work");
    let _ = engine.eval(r#"ARR_SUM([1, 2, 3])"#).expect("Should work");

    // 这个应该失败（没有加载验证库）
    let result = engine.eval(r#"VALIDATE_EMAIL("test@test.com")"#);
    assert!(
        result.is_err(),
        "Should fail because validation module not loaded"
    );
}

#[test]
fn test_stdlib_get_module() {
    // 测试模块获取 API
    assert!(aether::stdlib::get_module("string_utils").is_some());
    assert!(aether::stdlib::get_module("array_utils").is_some());
    assert!(aether::stdlib::get_module("validation").is_some());
    assert!(aether::stdlib::get_module("datetime").is_some());
    assert!(aether::stdlib::get_module("testing").is_some());
    assert!(aether::stdlib::get_module("nonexistent").is_none());
}
