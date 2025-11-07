// examples/optimization_demo.rs
//! 演示Aether优化功能的示例

use aether::Aether;

fn main() {
    println!("=== Aether 优化功能演示 ===\n");

    // 1. AST缓存演示
    demo_ast_cache();

    // 2. 常量折叠演示
    demo_constant_folding();

    // 3. 死代码消除演示
    demo_dead_code_elimination();
}

fn demo_ast_cache() {
    println!("📦 1. AST缓存演示");
    println!("----------------------------------------");

    let mut engine = Aether::new();
    let code = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;

    // 第一次执行 - 需要解析
    let start = std::time::Instant::now();
    let result1 = engine.eval(code).unwrap();
    let time1 = start.elapsed();

    // 第二次执行 - 使用缓存
    let start = std::time::Instant::now();
    let _result2 = engine.eval(code).unwrap();
    let time2 = start.elapsed();

    println!("第一次执行: {:?} (需要解析)", time1);
    println!("第二次执行: {:?} (使用缓存)", time2);
    println!("结果: {}", result1);

    // 显示缓存统计
    let stats = engine.cache_stats();
    println!("\n缓存统计:");
    println!("  命中: {}", stats.hits);
    println!("  未命中: {}", stats.misses);
    println!("  命中率: {:.1}%", stats.hit_rate * 100.0);

    if time1 > time2 {
        let speedup = time1.as_nanos() as f64 / time2.as_nanos() as f64;
        println!("  加速比: {:.2}x", speedup);
    }

    println!();
}

fn demo_constant_folding() {
    println!("🔧 2. 常量折叠演示");
    println!("----------------------------------------");

    let mut engine = Aether::new();

    // 优化器会将 (2 + 3) * 4 折叠为 20
    let code = r#"
        Set X (2 + 3)
        Set Y (X * 4)
        Y
    "#;

    println!("代码:");
    println!("{}", code);

    let result = engine.eval(code).unwrap();
    println!("结果: {}", result);
    println!("说明: 常量表达式 (2 + 3) 在优化阶段被折叠为 5");
    println!();
}

fn demo_dead_code_elimination() {
    println!("✂️  3. 死代码消除演示");
    println!("----------------------------------------");

    let mut engine = Aether::new();

    // 使用常量比较表达式
    let code = r#"
        Set COUNTER 0
        Set RESULT COUNTER
        RESULT
    "#;

    println!("代码:");
    println!("{}", code);

    let result = engine.eval(code).unwrap();
    println!("结果: {}", result);
    println!("说明: 优化器会在编译时折叠常量表达式");
    println!("      并移除不可达的代码分支");
    println!();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_works() {
        let mut engine = Aether::new();
        let code = "Set X 10\nX";

        engine.eval(code).unwrap();
        engine.eval(code).unwrap();

        let stats = engine.cache_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }
}
