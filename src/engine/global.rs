//! 全局单例引擎模式
//!
//! 使用线程局部存储（thread_local）创建引擎单例，适合单线程高频调用场景。
//! 每次执行前清空环境变量以保证隔离性，但保留AST缓存以优化性能。
//!
//! **注意**：由于使用 thread_local，每个线程有独立的引擎实例。
//! 如需多线程共享引擎池，请使用 `EnginePool`。

use crate::{Aether, Value};
use std::cell::RefCell;

thread_local! {
    /// 线程局部 Aether 引擎单例
    ///
    /// **线程安全**：每个线程有独立的引擎实例
    ///
    /// **性能优化**：
    /// - 每个线程只创建一次引擎实例
    /// - AST 缓存在多次调用间累积（可达142x加速）
    /// - 内置函数注册表复用
    static THREAD_LOCAL_AETHER: RefCell<Aether> = RefCell::new(Aether::new());
}

/// 全局单例引擎
///
/// # 使用场景
///
/// - ✅ 单线程应用
/// - ✅ 高频率DSL执行（如配置解析、规则引擎）
/// - ✅ 需要最大化性能
/// - ❌ 多线程并发（会有锁竞争，请使用 EnginePool）
///
/// # 隔离性保证
///
/// - 每次 `eval_isolated()` 前清空环境变量
/// - 不同执行间的变量不会互相影响
/// - AST缓存跨执行保留（性能优化）
///
/// # 示例
///
/// ```rust
/// use aether::engine::GlobalEngine;
///
/// // 执行代码（隔离环境）
/// let result = GlobalEngine::eval_isolated("Set X 10\n(X + 20)").unwrap();
/// assert_eq!(result.to_string(), "30");
///
/// // 再次执行（上次的X不存在，环境已清空）
/// let result2 = GlobalEngine::eval_isolated("(X + 1)"); // 错误：X未定义
/// assert!(result2.is_err());
///
/// // 如果需要保留变量（不隔离），使用 eval()
/// GlobalEngine::eval("Set Y 100").unwrap();
/// let result3 = GlobalEngine::eval("(Y + 1)").unwrap();
/// assert_eq!(result3.to_string(), "101");
/// ```
///
/// # 性能提示
///
/// 对于重复执行相同代码的场景，性能提升显著：
///
/// ```rust
/// use aether::engine::GlobalEngine;
///
/// let code = "Set X 10\n(X * 2)";
///
/// // 第一次：解析 + 缓存 + 执行
/// GlobalEngine::eval_isolated(code).unwrap();
///
/// // 后续执行：直接从缓存读取AST（142x faster！）
/// for _ in 0..1000 {
///     GlobalEngine::eval_isolated(code).unwrap();
/// }
/// ```
pub struct GlobalEngine;

impl GlobalEngine {
    /// 使用全局引擎执行代码（隔离环境）
    ///
    /// 每次执行前清空环境变量，确保不同执行间的隔离性。
    /// AST缓存保留，性能最优。
    ///
    /// # 参数
    ///
    /// - `code`: 要执行的Aether代码
    ///
    /// # 返回
    ///
    /// - `Ok(Value)`: 执行结果
    /// - `Err(String)`: 错误信息
    ///
    /// # 线程安全
    ///
    /// 每个线程有独立的引擎实例，无需担心线程安全问题。
    pub fn eval_isolated(code: &str) -> Result<Value, String> {
        THREAD_LOCAL_AETHER.with(|engine| {
            let mut engine = engine.borrow_mut();

            // 重置环境（保证隔离性）
            engine.evaluator.reset_env();

            // 执行代码（使用缓存）
            engine.eval(code)
        })
    }

    /// 使用全局引擎执行代码（非隔离，变量会累积）
    ///
    /// **警告**：此方法不会清空环境，变量会在多次调用间保留。
    /// 仅在明确需要变量累积时使用。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aether::engine::GlobalEngine;
    ///
    /// // 第一次设置变量
    /// GlobalEngine::eval("Set X 10").unwrap();
    ///
    /// // 第二次可以使用X（变量保留）
    /// let result = GlobalEngine::eval("(X + 20)").unwrap();
    /// assert_eq!(result.to_string(), "30");
    ///
    /// // 记得清空（如果不再需要）
    /// GlobalEngine::clear_env();
    /// ```
    pub fn eval(code: &str) -> Result<Value, String> {
        THREAD_LOCAL_AETHER.with(|engine| engine.borrow_mut().eval(code))
    }

    /// 清空全局引擎的环境变量
    ///
    /// 用于手动清理 `eval()` 累积的变量。
    /// `eval_isolated()` 会自动清空，无需调用此方法。
    pub fn clear_env() {
        THREAD_LOCAL_AETHER.with(|engine| {
            engine.borrow_mut().evaluator.reset_env();
        });
    }

    /// 清空全局引擎的AST缓存
    ///
    /// 如果执行了大量不同的代码，缓存可能占用内存。
    /// 定期清理可以释放内存。
    ///
    /// **注意**：清理后性能会下降，直到缓存重新建立。
    pub fn clear_cache() {
        THREAD_LOCAL_AETHER.with(|engine| {
            engine.borrow_mut().clear_cache();
        });
    }

    /// 获取AST缓存统计信息
    ///
    /// 返回缓存命中率、命中次数、未命中次数等信息。
    pub fn cache_stats() -> Option<crate::cache::CacheStats> {
        THREAD_LOCAL_AETHER.with(|engine| Some(engine.borrow().cache_stats()))
    }

    /// 配置优化选项
    ///
    /// # 参数
    ///
    /// - `constant_folding`: 常量折叠优化
    /// - `dead_code`: 死代码消除
    /// - `tail_recursion`: 尾递归优化
    pub fn set_optimization(constant_folding: bool, dead_code: bool, tail_recursion: bool) {
        THREAD_LOCAL_AETHER.with(|engine| {
            engine
                .borrow_mut()
                .set_optimization(constant_folding, dead_code, tail_recursion);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_engine_isolated() {
        // 第一次执行
        let result = GlobalEngine::eval_isolated("Set X 10\n(X + 20)").unwrap();
        assert_eq!(result.to_string(), "30");

        // 第二次执行，X应该不存在（环境已清空）
        let result = GlobalEngine::eval_isolated("X");
        assert!(result.is_err());
    }

    #[test]
    fn test_global_engine_non_isolated() {
        // 清空环境
        GlobalEngine::clear_env();

        // 设置变量
        GlobalEngine::eval("Set Y 100").unwrap();

        // 变量应该保留
        let result = GlobalEngine::eval("(Y + 1)").unwrap();
        assert_eq!(result.to_string(), "101");

        // 清空
        GlobalEngine::clear_env();
    }

    #[test]
    fn test_global_engine_cache() {
        let code = "Set X 10\n(X * 2)";

        // 第一次执行
        GlobalEngine::eval_isolated(code).unwrap();

        // 获取缓存统计
        let stats = GlobalEngine::cache_stats().unwrap();
        assert!(stats.hits + stats.misses > 0);
    }
}
