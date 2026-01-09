//! 结构化 TRACE 事件的集成测试
//!
//! 测试 TRACE_DEBUG, TRACE_INFO, TRACE_WARN, TRACE_ERROR 等功能

use aether::{Aether, TraceFilter, TraceLevel};

#[test]
fn test_trace_debug() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_DEBUG("test_category", "debug message", 42)
    "#;

    let result = engine.eval(code);
    assert!(result.is_ok());

    // 获取所有 trace 记录
    let traces = engine.trace_records();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].level, TraceLevel::Debug);
    assert_eq!(traces[0].category, "test_category");
    assert_eq!(traces[0].values.len(), 2);
}

#[test]
fn test_trace_info() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_INFO("user_action", "login", 12345)
    "#;

    let result = engine.eval(code);
    assert!(result.is_ok());

    let traces = engine.trace_records();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].level, TraceLevel::Info);
    assert_eq!(traces[0].category, "user_action");
}

#[test]
fn test_trace_warn() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_WARN("api_call", "slow_response", 5000)
    "#;

    let result = engine.eval(code);
    assert!(result.is_ok());

    let traces = engine.trace_records();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].level, TraceLevel::Warn);
    assert_eq!(traces[0].category, "api_call");
}

#[test]
fn test_trace_error() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_ERROR("database", "connection_failed", "timeout")
    "#;

    let result = engine.eval(code);
    assert!(result.is_ok());

    let traces = engine.trace_records();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].level, TraceLevel::Error);
    assert_eq!(traces[0].category, "database");
}

#[test]
fn test_trace_multiple_levels() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_DEBUG("debug", "debug message")
        TRACE_INFO("info", "info message")
        TRACE_WARN("warn", "warn message")
        TRACE_ERROR("error", "error message")
    "#;

    let result = engine.eval(code);
    assert!(result.is_ok());

    let traces = engine.trace_records();
    assert_eq!(traces.len(), 4);

    assert_eq!(traces[0].level, TraceLevel::Debug);
    assert_eq!(traces[1].level, TraceLevel::Info);
    assert_eq!(traces[2].level, TraceLevel::Warn);
    assert_eq!(traces[3].level, TraceLevel::Error);
}

#[test]
fn test_trace_filter_by_level() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_DEBUG("test", "debug")
        TRACE_INFO("test", "info")
        TRACE_WARN("test", "warn")
        TRACE_ERROR("test", "error")
    "#;

    engine.eval(code).unwrap();

    // 过滤 Error 级别
    let error_traces = engine.trace_by_level(TraceLevel::Error);
    assert_eq!(error_traces.len(), 1);
    assert_eq!(error_traces[0].category, "test");

    // 过滤 Warn 级别（精确匹配）
    let warn_traces = engine.trace_by_level(TraceLevel::Warn);
    assert_eq!(warn_traces.len(), 1); // 只有 Warn

    // 使用 TraceFilter 来过滤 Warn 及以上级别
    let filter = TraceFilter::new().with_min_level(TraceLevel::Warn);
    let warn_and_above = engine.trace_filter(&filter);
    assert_eq!(warn_and_above.len(), 2); // Warn 和 Error
}

#[test]
fn test_trace_filter_by_category() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_INFO("api", "request1")
        TRACE_INFO("database", "query1")
        TRACE_INFO("api", "request2")
        TRACE_INFO("database", "query2")
    "#;

    engine.eval(code).unwrap();

    let api_traces = engine.trace_by_category("api");
    assert_eq!(api_traces.len(), 2);

    let db_traces = engine.trace_by_category("database");
    assert_eq!(db_traces.len(), 2);
}

#[test]
fn test_trace_filter_combined() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_INFO("api", "normal_request")
        TRACE_WARN("api", "slow_request")
        TRACE_ERROR("database", "connection_failed")
        TRACE_DEBUG("api", "debug_info")
    "#;

    engine.eval(code).unwrap();

    // 组合过滤：api 类别的 Warn 及以上级别
    let filter = TraceFilter::new()
        .with_min_level(TraceLevel::Warn)
        .with_category("api".to_string());

    let filtered = engine.trace_filter(&filter);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].category, "api");
    assert_eq!(filtered[0].level, TraceLevel::Warn);
}

#[test]
fn test_trace_backward_compatibility() {
    let mut engine = Aether::new();

    // 旧 API 继续工作
    let code = r#"
        TRACE("simple_trace")
        TRACE("label", 1, 2, 3)
    "#;

    engine.eval(code).unwrap();

    // take_trace() 应该返回格式化的字符串
    let traces = engine.take_trace();
    assert_eq!(traces.len(), 2);
    assert!(traces[0].contains("simple_trace"));
    assert!(traces[1].contains("[label]"));

    // trace_records() 应该返回空（旧 API 不生成结构化条目）
    let structured = engine.trace_records();
    assert_eq!(structured.len(), 0);
}

#[test]
fn test_trace_stats() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_DEBUG("debug", "msg1")
        TRACE_DEBUG("debug", "msg2")
        TRACE_INFO("info", "msg3")
        TRACE_WARN("warn", "msg4")
        TRACE_ERROR("error", "msg5")
    "#;

    engine.eval(code).unwrap();

    let stats = engine.trace_stats();
    assert_eq!(stats.total_entries, 5);

    // 检查级别统计
    assert_eq!(*stats.by_level.get(&TraceLevel::Debug).unwrap(), 2);
    assert_eq!(*stats.by_level.get(&TraceLevel::Info).unwrap(), 1);
    assert_eq!(*stats.by_level.get(&TraceLevel::Warn).unwrap(), 1);
    assert_eq!(*stats.by_level.get(&TraceLevel::Error).unwrap(), 1);

    // 检查类别统计
    assert_eq!(*stats.by_category.get("debug").unwrap(), 2);
    assert_eq!(*stats.by_category.get("info").unwrap(), 1);
    assert_eq!(*stats.by_category.get("warn").unwrap(), 1);
    assert_eq!(*stats.by_category.get("error").unwrap(), 1);
}

#[test]
fn test_trace_clear() {
    let mut engine = Aether::new();

    let code = r#"
        TRACE_INFO("test", "message1")
        TRACE_WARN("test", "message2")
    "#;

    engine.eval(code).unwrap();
    assert_eq!(engine.trace_records().len(), 2);

    // 清空
    engine.clear_trace();
    assert_eq!(engine.trace_records().len(), 0);

    // 再次添加应该重新开始
    engine.eval("TRACE_ERROR(\"test\", \"message3\")").unwrap();
    assert_eq!(engine.trace_records().len(), 1);
}

#[test]
fn test_trace_with_expressions() {
    let mut engine = Aether::new();

    let code = r#"
        Set X 42
        Set Y "hello"
        TRACE_INFO("calc", X, Y)
    "#;

    engine.eval(code).unwrap();

    let traces = engine.trace_records();
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].values.len(), 2);
    assert_eq!(traces[0].values[0].to_string(), "42");
    assert_eq!(traces[0].values[1].to_string(), "hello");
}

#[test]
fn test_trace_entry_format() {
    use aether::TraceEntry;

    let entry = TraceEntry::new(TraceLevel::Info, "test_category".to_string(), vec![])
        .with_label("test_label".to_string());

    let formatted = entry.format();
    assert!(formatted.contains("[INFO:test_category:test_label]"));
}

#[test]
fn test_trace_level_ordering() {
    assert!(TraceLevel::Debug < TraceLevel::Info);
    assert!(TraceLevel::Info < TraceLevel::Warn);
    assert!(TraceLevel::Warn < TraceLevel::Error);
}

#[test]
fn test_trace_buffer_ring_behavior() {
    let mut engine = Aether::new();

    // 生成超过缓冲区大小的 trace (1024)
    let mut code = String::new();
    for i in 1..=1100 {
        code.push_str(&format!("TRACE_INFO(\"test\", {})\n", i));
    }

    engine.eval(&code).unwrap();

    let traces = engine.trace_records();
    // 应该保留最多 1024 条
    assert_eq!(traces.len(), 1024);

    let stats = engine.trace_stats();
    assert!(stats.buffer_full);
}

#[test]
fn test_trace_error_on_invalid_args() {
    let mut engine = Aether::new();

    // 缺少参数
    let result = engine.eval("TRACE_INFO()");
    assert!(result.is_err());

    // category 不是字符串
    let result = engine.eval("TRACE_INFO(123, \"value\")");
    assert!(result.is_err());
}
