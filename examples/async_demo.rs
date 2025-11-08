//! Aether 异步 API 示例
//!
//! 演示如何在 async/await Rust 应用中使用 Aether DSL
//!
//! # 运行方式
//!
//! ```bash
//! cargo run --example async_demo --features async
//! ```

use aether::{
    Aether,
    engine::{EnginePool, GlobalEngine, ScopedEngine},
};

#[tokio::main]
async fn main() {
    println!("=== Aether 异步 API 示例 ===\n");

    // 示例 1: 基础 Aether 异步调用
    println!("1. 基础 Aether 异步调用:");
    basic_async_example().await;

    // 示例 2: GlobalEngine 异步调用
    println!("\n2. GlobalEngine 异步调用:");
    global_engine_async_example().await;

    // 示例 3: EnginePool 异步调用
    println!("\n3. EnginePool 异步调用:");
    engine_pool_async_example().await;

    // 示例 4: ScopedEngine 异步调用
    println!("\n4. ScopedEngine 异步调用:");
    scoped_engine_async_example().await;

    // 示例 5: 在异步 Web 服务器中使用（模拟）
    println!("\n5. 在异步 Web 服务器中使用 (模拟):");
    web_server_simulation().await;

    println!("\n=== 所有示例完成 ===");
}

/// 示例 1: 基础 Aether 异步调用
async fn basic_async_example() {
    let mut engine = Aether::new();

    // 使用 eval_async
    let result = engine
        .eval_async(
            r#"
        Set X 10
        Set Y 20
        Set Z (X + Y)
        Z
    "#,
        )
        .await
        .unwrap();

    println!("  结果: {}", result);
}

/// 示例 2: GlobalEngine 异步调用
async fn global_engine_async_example() {
    // 隔离环境执行
    let result = GlobalEngine::eval_isolated_async(
        r#"
        Set PRICE 100
        Set QUANTITY 5
        Set TOTAL (PRICE * QUANTITY)
        TOTAL
    "#,
    )
    .await
    .unwrap();

    println!("  订单总额: {}", result);

    // 非隔离执行（变量累积）
    GlobalEngine::eval_async("Set COUNTER 0").await.unwrap();
    GlobalEngine::eval_async("Set COUNTER (COUNTER + 1)")
        .await
        .unwrap();
    GlobalEngine::eval_async("Set COUNTER (COUNTER + 1)")
        .await
        .unwrap();
    let counter = GlobalEngine::eval_async("COUNTER").await.unwrap();
    println!("  计数器: {}", counter);

    GlobalEngine::clear_env();
}

/// 示例 3: EnginePool 异步调用
async fn engine_pool_async_example() {
    let mut pool = EnginePool::new(4);

    // 注意：由于 Aether 使用 Rc (非 Send)，不能直接在 tokio::spawn 中使用
    // 正确做法是在同一线程中顺序执行多个异步操作

    for i in 0..3 {
        let mut engine = pool.acquire();
        let code = format!(
            r#"
            Set X {}
            Set Y (X * 2)
            Set RESULT (X + Y)
            RESULT
        "#,
            i * 10
        );

        let result = engine.eval_async(&code).await.unwrap();
        println!("  任务 {} 结果: {}", i, result);
    }
}

/// 示例 4: ScopedEngine 异步调用
async fn scoped_engine_async_example() {
    // 简单异步执行
    let result = ScopedEngine::eval_async(
        r#"
        Set NUMBERS [1, 2, 3, 4, 5]
        Set SUM 0
        For N In NUMBERS {
            Set SUM (SUM + N)
        }
        SUM
    "#,
    )
    .await
    .unwrap();

    println!("  数组求和: {}", result);
}

/// 示例 5: 在异步 Web 服务器中使用（模拟）
async fn web_server_simulation() {
    println!("  模拟处理 HTTP 请求...");

    // 注意：由于 Aether 使用 Rc，每个请求需要在同一线程处理
    // 在真实场景中，可以为每个线程创建独立的 GlobalEngine

    for req_id in 1..=5 {
        handle_request(req_id).await;
    }
}

/// 模拟处理单个 HTTP 请求
async fn handle_request(req_id: u32) {
    // 模拟请求参数
    let user_id = req_id * 100;
    let amount = req_id * 10;

    // 使用 Aether 计算业务逻辑
    let code = format!(
        r#"
        Set USER_ID {}
        Set AMOUNT {}
        Set FEE 5
        Set TOTAL (AMOUNT + FEE)
        TOTAL
    "#,
        user_id, amount
    );

    let result = ScopedEngine::eval_async(&code).await.unwrap();

    println!("  请求 {} (用户 {}): 总金额 = {}", req_id, user_id, result);
}

/// 性能对比：同步 vs 异步
#[allow(dead_code)]
async fn performance_comparison() {
    use std::time::Instant;

    let code = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;

    // 同步版本
    let start = Instant::now();
    for _ in 0..1000 {
        let mut engine = Aether::new();
        engine.eval(code).unwrap();
    }
    let sync_duration = start.elapsed();

    // 异步版本
    let start = Instant::now();
    for _ in 0..1000 {
        let mut engine = Aether::new();
        engine.eval_async(code).await.unwrap();
    }
    let async_duration = start.elapsed();

    println!("同步版本耗时: {:?}", sync_duration);
    println!("异步版本耗时: {:?}", async_duration);
}
