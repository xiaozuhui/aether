use aether::Aether;

pub fn print_metrics_json(
    engine: &Aether,
    elapsed: std::time::Duration,
    cache_before: aether::CacheStats,
    result: aether::Value,
    pretty: bool,
) {
    let cache_after = engine.cache_stats();
    let trace_stats = engine.trace_stats();

    let payload = serde_json::json!({
        "ok": true,
        "result": if result == aether::Value::Null { serde_json::Value::Null } else { serde_json::Value::String(result.to_string()) },
        "metrics": {
            "wall_time_ms": elapsed.as_millis(),
            "step_count": engine.step_count(),
            "ast_cache": {
                "before": cache_before,
                "after": cache_after
            },
            "structured_trace": trace_stats
        }
    });

    print_json(payload, pretty);
}

pub fn print_json(payload: serde_json::Value, pretty: bool) {
    let s = if pretty {
        serde_json::to_string_pretty(&payload)
    } else {
        serde_json::to_string(&payload)
    }
    .unwrap_or_else(|_| "{}".to_string());

    println!("{}", s);
}

pub fn print_metrics(
    elapsed: std::time::Duration,
    cache_before: &aether::CacheStats,
    cache_after: &aether::CacheStats,
    trace_stats: &aether::TraceStats,
    step_count: usize,
) {
    println!("=== METRICS ===");
    println!("wall_time_ms: {}", elapsed.as_millis());
    println!("step_count: {}", step_count);

    println!(
        "ast_cache: size {}/{} -> {}/{}, hits {} -> {}, misses {} -> {}, hit_rate {:.2}% -> {:.2}%",
        cache_before.size,
        cache_before.max_size,
        cache_after.size,
        cache_after.max_size,
        cache_before.hits,
        cache_after.hits,
        cache_before.misses,
        cache_after.misses,
        cache_before.hit_rate * 100.0,
        cache_after.hit_rate * 100.0
    );

    println!(
        "structured_trace: total_entries={}, buffer_size={}, buffer_full={}",
        trace_stats.total_entries, trace_stats.buffer_size, trace_stats.buffer_full
    );
    println!();
}
