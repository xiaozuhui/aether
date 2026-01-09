//! 结构化 TRACE 事件演示
//!
//! 演示如何使用新的级别化 TRACE API

use aether::{Aether, TraceFilter, TraceLevel};

fn main() {
    let mut engine = Aether::new();

    // 演示代码
    let code = r#"
        # 调试级别
        TRACE_DEBUG("calculation", "intermediate_result", 42)

        # 信息级别
        TRACE_INFO("user_action", "login", 12345)
        TRACE_INFO("api_call", "GET /api/users", 200)

        # 警告级别
        TRACE_WARN("performance", "slow_query", 5000)
        TRACE_WARN("cache", "miss_rate", 0.15)

        # 错误级别
        TRACE_ERROR("database", "connection_failed", "timeout")
        TRACE_ERROR("validation", "invalid_input", "user_id")

        # 旧 API 继续工作
        TRACE("legacy_trace", "still_works")
        TRACE(1, 2, 3)

        # 条件 TRACE
        Set RESPONSE_TIME 250
        If (RESPONSE_TIME > 200) {
            TRACE_WARN("api", "slow_response", RESPONSE_TIME)
        } Else {
            TRACE_INFO("api", "normal_response", RESPONSE_TIME)
        }
    "#;

    // 执行代码
    let result = engine.eval(code);
    if let Err(e) = result {
        eprintln!("执行错误: {}", e);
        return;
    }

    println!("=== 所有 TRACE 记录 (格式化) ===\n");
    let formatted_traces = engine.take_trace();
    for trace in &formatted_traces {
        println!("{}", trace);
    }

    println!("\n=== 结构化 TRACE 统计 ===\n");
    let stats = engine.trace_stats();
    println!("总记录数: {}", stats.total_entries);
    println!("缓冲区大小: {}", stats.buffer_size);
    println!("缓冲区已满: {}", stats.buffer_full);
    println!("\n按级别统计:");

    let mut levels: Vec<_> = stats.by_level.iter().collect();
    levels.sort_by_key(|(k, _)| **k);
    for (level, count) in levels {
        println!("  {}: {}", level, count);
    }

    println!("\n按类别统计:");
    let mut categories: Vec<_> = stats.by_category.iter().collect();
    categories.sort_by_key(|(k, _)| k.as_str());
    for (category, count) in categories {
        println!("  {}: {}", category, count);
    }

    println!("\n=== 过滤 Error 级别的记录 ===\n");
    let error_traces = engine.trace_by_level(TraceLevel::Error);
    for trace in &error_traces {
        println!("  [{}] {}: {:?}", trace.level, trace.category, trace.values);
    }

    println!("\n=== 过滤 api 类别的记录 ===\n");
    let api_traces = engine.trace_by_category("api");
    for trace in &api_traces {
        println!("  [{}] {}: {:?}", trace.level, trace.category, trace.values);
    }

    println!("\n=== 过滤 Warn 及以上级别 ===\n");
    let filter = TraceFilter::new().with_min_level(TraceLevel::Warn);
    let warn_and_above = engine.trace_filter(&filter);
    for trace in &warn_and_above {
        println!("  [{}] {}: {:?}", trace.level, trace.category, trace.values);
    }
}
