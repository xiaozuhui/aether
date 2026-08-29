//! BDD 规格：AST 缓存正确性与引擎池行为。
//!
//! 功能点与边界（已冻结）：
//! 1. **LRU 淘汰**：容量满时淘汰「最近最少使用」的条目（get 命中
//!    会刷新新鲜度）。旧实现注释写着 FIFO，实际用 HashMap 迭代序
//!    取前 N 个删除——淘汰顺序是随机的。
//! 2. **碰撞安全**：命中必须确认源码完全一致（哈希仅作加速索引），
//!    不同源码不得互相污染。旧实现纯 u64 哈希键，理论碰撞会返回
//!    错误的 AST。
//! 3. 引擎池：借出/归还计数、环境隔离（归还能拿回干净环境）、
//!    并发句柄、每引擎独立缓存（既有行为锁定）。

use aether::{ASTCache, Parser};

/// 同一源码第二次求值命中缓存。
#[test]
fn cache_hits_same_source() {
    let mut cache = ASTCache::new();
    let prog = Parser::new("Set X 1").parse_program().expect("解析失败");
    cache.insert("Set X 1", prog);
    assert!(cache.get("Set X 1").is_some(), "同源码应命中缓存");
    let stats = cache.stats();
    assert_eq!(stats.hits, 1);
}

/// 容量 2：get(A) 刷新 A 的新鲜度后插入 C，应淘汰 B（LRU）而非 A。
/// 旧实现按 HashMap 随机序删除，本用例大概率失败。
#[test]
fn cache_evicts_least_recently_used() {
    let mut cache = ASTCache::with_capacity(2);
    cache.insert("Set A 1", Parser::new("Set A 1").parse_program().unwrap());
    cache.insert("Set B 1", Parser::new("Set B 1").parse_program().unwrap());
    // 触碰 A：A 成为最近使用
    assert!(cache.get("Set A 1").is_some());
    // 插入 C 超容量：B 是最近最少使用，应被淘汰
    cache.insert("Set C 1", Parser::new("Set C 1").parse_program().unwrap());
    assert!(cache.get("Set B 1").is_none(), "LRU 应淘汰 B");
    assert!(cache.get("Set A 1").is_some(), "A 被触碰过，不应被淘汰");
    assert!(cache.get("Set C 1").is_some());
}

/// 不同源码各自取回自己的 AST，互不污染（碰撞安全的行为面）。
#[test]
fn cache_keeps_distinct_sources_independent() {
    let mut cache = ASTCache::new();
    let p1 = Parser::new("Set X 1").parse_program().unwrap();
    let p2 = Parser::new("Set X 1\nSet Y 2").parse_program().unwrap();
    // parse_program 返回语句向量 Vec<Located>，长度即可区分两份 AST
    let (n1, n2) = (p1.len(), p2.len());
    assert_ne!(n1, n2, "测试前提：两段源码的语句数应不同");
    cache.insert("Set X 1", p1);
    cache.insert("Set X 1\nSet Y 2", p2);
    assert_eq!(cache.get("Set X 1").unwrap().len(), n1);
    assert_eq!(cache.get("Set X 1\nSet Y 2").unwrap().len(), n2);
}

/// clear 之后全部未命中、计数器从零重新开始。
#[test]
fn cache_clear_resets() {
    let mut cache = ASTCache::new();
    cache.insert("Set A 1", Parser::new("Set A 1").parse_program().unwrap());
    cache.clear();
    let stats = cache.stats();
    assert_eq!((stats.hits, stats.misses), (0, 0), "clear 应清零计数器");
    assert!(cache.get("Set A 1").is_none());
    // clear 之后的 get 是一次新的未命中，应被计入
    let stats = cache.stats();
    assert_eq!((stats.hits, stats.misses), (0, 1));
}

/// 引擎池：归还后的引擎环境是干净的（隔离性）。
#[test]
fn pool_returns_clean_environment() {
    let pool = aether::engine::EnginePool::new(2);
    {
        let mut engine = pool.acquire();
        engine.eval("Set X 10").expect("求值失败");
    }
    let mut engine = pool.acquire();
    assert!(engine.eval("X").is_err(), "归还再借出的引擎不应残留变量 X");
}

/// 引擎池：并发持有多个句柄，逐个归还后可用数恢复。
#[test]
fn pool_supports_concurrent_handles() {
    let pool = aether::engine::EnginePool::new(2);
    assert_eq!(pool.available(), 2);

    let first = pool.acquire();
    assert_eq!(pool.available(), 1);
    {
        let _second = pool.acquire();
        assert_eq!(pool.available(), 0);
    } // second 归还
    assert_eq!(pool.available(), 1);
    drop(first);
    assert_eq!(pool.available(), 2);
}

/// 引擎池：同时持有的两个引擎各自维护独立的 AST 缓存统计。
/// （注意：归还后再借出会拿回**同一个**引擎实例——缓存延续正是
/// 池化复用的意义，因此本用例必须并发持有两个引擎来验证独立性。）
#[test]
fn pool_engines_have_independent_caches() {
    let pool = aether::engine::EnginePool::new(2);
    let mut a = pool.acquire();
    a.eval("Set X 1").expect("求值失败");
    a.eval("Set X 1").expect("求值失败"); // 引擎 a 缓存命中一次
    let stats_a = a.cache_stats();
    assert!(stats_a.hits >= 1, "引擎 a 应有缓存命中");

    let mut b = pool.acquire(); // 第二个引擎实例，缓存从零开始
    // 同一代码对引擎 b 而言是首次
    b.eval("Set X 1").expect("求值失败");
    let stats_b = b.cache_stats();
    assert_eq!(stats_b.hits, 0, "引擎 b 的缓存应独立于引擎 a");
}
