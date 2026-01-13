use aether::{ASTCache, Program};

#[test]
fn test_cache_basic() {
    let mut cache = ASTCache::new();
    let code = "Set X 10";

    // 第一次获取应该失败
    assert!(cache.get(code).is_none());
    assert_eq!(cache.stats().misses, 1);

    // 插入后应该能获取
    let program: Program = vec![];
    cache.insert(code, program.clone());
    assert!(cache.get(code).is_some());
    assert_eq!(cache.stats().hits, 1);
}

#[test]
fn test_cache_capacity() {
    let mut cache = ASTCache::with_capacity(5);
    let program: Program = vec![];

    // 插入超过容量的条目
    for i in 0..10 {
        let code = format!("Set X {}", i);
        cache.insert(&code, program.clone());
    }

    // 缓存大小应该被限制
    assert!(cache.stats().size <= 5);
}
