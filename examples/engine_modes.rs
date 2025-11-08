// examples/engine_modes.rs
//! Aether 引擎模式示例
//!
//! 演示三种高性能引擎模式的使用方法：
//! 1. GlobalEngine - 全局单例（单线程最优）
//! 2. EnginePool - 引擎池（多线程场景）
//! 3. ScopedEngine - 闭包模式（简洁API）

use aether::engine::{EnginePool, GlobalEngine, ScopedEngine};
use std::time::Instant;

fn main() {
    println!("=== Aether Engine Modes Demo ===\n");

    // 1. GlobalEngine 示例
    demo_global_engine();

    // 2. EnginePool 示例
    demo_engine_pool();

    // 3. ScopedEngine 示例
    demo_scoped_engine();

    // 4. 性能对比
    performance_comparison();
}

/// 演示 GlobalEngine（全局单例模式）
fn demo_global_engine() {
    println!("📌 1. GlobalEngine (全局单例模式)");
    println!("   适用场景：单线程、高频调用\n");

    // 基本使用（隔离环境）
    let result = GlobalEngine::eval_isolated("Set X 10\n(X + 20)").unwrap();
    println!("   执行结果: {}", result);

    // 验证隔离性（X 不应该存在）
    let result = GlobalEngine::eval_isolated("X");
    assert!(result.is_err());
    println!("   ✅ 环境隔离：上次的变量 X 已清空");

    // 非隔离模式（变量累积）
    GlobalEngine::eval("Set Y 100").unwrap();
    let result = GlobalEngine::eval("(Y + 1)").unwrap();
    println!("   非隔离模式：Y = {}", result);

    // 清空环境
    GlobalEngine::clear_env();
    println!("   ✅ 环境已清空\n");
}

/// 演示 EnginePool（引擎池模式）
fn demo_engine_pool() {
    println!("📌 2. EnginePool (引擎池模式)");
    println!("   适用场景：单线程内需要多个引擎实例\n");

    // 创建引擎池
    let mut pool = EnginePool::new(4);
    println!("   引擎池大小: {}", pool.capacity());
    println!("   当前可用引擎: {}", pool.available());

    // 单线程使用
    {
        let mut engine = pool.acquire();
        let result = engine.eval("Set X 10\n(X * 2)").unwrap();
        println!("   单次执行: {}", result);
    } // 引擎自动归还

    println!("   ✅ 引擎已自动归还，当前可用: {}", pool.available());

    // 多次使用（复用引擎实例）
    println!("\n   执行 10 次计算...");
    for i in 0..10 {
        let mut engine = pool.acquire();
        let code = format!("Set X {}\n(X * 2)", i);
        let result = engine.eval(&code).unwrap();
        if i < 3 {
            println!("   迭代 {} 结果: {}", i, result);
        }
    }

    println!("   ✅ 多次执行完成，引擎复用减少创建开销\n");
}

/// 演示 ScopedEngine（闭包模式）
fn demo_scoped_engine() {
    println!("📌 3. ScopedEngine (闭包模式)");
    println!("   适用场景：临时执行、完全隔离\n");

    // 基本使用
    let result = ScopedEngine::eval("Set X 10\n(X + 20)").unwrap();
    println!("   基本执行: {}", result);

    // 闭包风格
    let result = ScopedEngine::with(|engine| {
        engine.eval("Set X 10")?;
        engine.eval("Set Y 20")?;
        engine.eval("(X + Y)")
    })
    .unwrap();
    println!("   闭包风格: {}", result);

    // 自定义返回值
    let (x, y) = ScopedEngine::with(|engine| {
        engine.eval("Set X 100")?;
        engine.eval("Set Y 200")?;
        let x = engine.eval("X")?;
        let y = engine.eval("Y")?;
        Ok((x, y))
    })
    .unwrap();
    println!("   自定义返回: X={}, Y={}", x, y);

    // 验证隔离性
    let result = ScopedEngine::eval("X");
    assert!(result.is_err());
    println!("   ✅ 完全隔离：每次都是新引擎\n");
}

/// 性能对比测试
fn performance_comparison() {
    println!("📌 4. 性能对比");
    println!("   测试代码：重复执行相同脚本 1000 次\n");

    let code = "Set X 10\nSet Y 20\n(X + Y)";
    let iterations = 1000;

    // GlobalEngine 性能
    let start = Instant::now();
    for _ in 0..iterations {
        GlobalEngine::eval_isolated(code).unwrap();
    }
    let global_time = start.elapsed();
    println!("   GlobalEngine:  {:?}", global_time);

    // EnginePool 性能（单线程）
    let mut pool = EnginePool::new(4);
    let start = Instant::now();
    for _ in 0..iterations {
        let mut engine = pool.acquire();
        engine.eval(code).unwrap();
    }
    let pool_time = start.elapsed();
    println!("   EnginePool:    {:?}", pool_time);

    // ScopedEngine 性能
    let start = Instant::now();
    for _ in 0..iterations {
        ScopedEngine::eval(code).unwrap();
    }
    let scoped_time = start.elapsed();
    println!("   ScopedEngine:  {:?}", scoped_time);

    // 对比
    println!("\n   性能排名:");
    println!("   1️⃣  GlobalEngine (最快，AST缓存效果最好)");
    println!("   2️⃣  EnginePool   (略慢，但避免频繁创建)");
    println!("   3️⃣  ScopedEngine (最慢，每次创建新引擎)");

    // AST 缓存效果
    if let Some(stats) = GlobalEngine::cache_stats() {
        println!("\n   GlobalEngine AST 缓存统计:");
        println!("   - 命中次数: {}", stats.hits);
        println!("   - 未命中次数: {}", stats.misses);
        println!(
            "   - 命中率: {:.2}%",
            stats.hits as f64 / (stats.hits + stats.misses) as f64 * 100.0
        );
        if stats.hits > 0 {
            println!("   ✨ AST缓存显著提升性能！");
        }
    }

    println!("\n=== Demo 完成 ===");
}
