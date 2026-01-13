use aether::{Aether, Value};

#[test]
fn test_aether_creation() {
    let _engine = Aether::new();
}

#[test]
fn test_cache_usage() {
    let mut engine = Aether::new();
    let code = "Set X 10\nX";

    // 第一次执行会解析
    let result1 = engine.eval(code).unwrap();
    assert_eq!(result1, Value::Number(10.0));

    // 第二次执行应该使用缓存
    let result2 = engine.eval(code).unwrap();
    assert_eq!(result2, Value::Number(10.0));

    // 检查缓存统计
    let stats = engine.cache_stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 1);
}
