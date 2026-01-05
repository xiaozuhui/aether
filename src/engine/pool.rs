//! 引擎池模式
//!
//! 使用线程局部存储管理引擎池，每个线程维护独立的引擎池。
//! 适合单线程内高频调用场景，需要比 GlobalEngine 更多的引擎实例。
//!
//! **注意**：由于 Aether 使用 `Rc`（非线程安全），引擎池是线程局部的。
//! 每个线程有独立的引擎池，线程间不共享。

use crate::{Aether, Value};

/// 线程局部引擎池
///
/// # 使用场景
///
/// - ✅ 单线程内需要多个引擎实例
/// - ✅ 避免频繁创建引擎的开销
/// - ✅ 需要环境隔离的高频调用
/// - ⚠️ 每个线程有独立的池（不跨线程共享）
///
/// # 特点
///
/// - **线程局部**：每个线程有独立的引擎池
/// - **自动管理**：RAII模式，使用完自动归还
/// - **环境隔离**：每次获取前清空变量
/// - **AST缓存**：每个引擎独立维护缓存
///
/// # 示例
///
/// ```rust
/// use aether::engine::EnginePool;
///
/// // 创建线程局部引擎池
/// let pool = EnginePool::new(4);
///
/// // 使用引擎
/// {
///     let mut engine = pool.acquire();
///     let result = engine.eval("Set X 10\n(X + 20)").unwrap();
///     println!("Result: {}", result);
/// } // engine 自动归还到池中
///
/// // 多次使用（复用引擎实例）
/// for i in 0..100 {
///     let mut engine = pool.acquire();
///     let code = format!("Set X {}\n(X * 2)", i);
///     engine.eval(&code).unwrap();
/// }
/// ```
///
/// # 与 GlobalEngine 对比
///
/// - **GlobalEngine**: 单个引擎实例，适合简单场景
/// - **EnginePool**: 多个引擎实例，避免频繁创建开销
///
/// 如果你只需要一个引擎实例，使用 `GlobalEngine` 更简单。
pub struct EnginePool {
    engines: Vec<Aether>,
    available: Vec<bool>,
}

impl EnginePool {
    /// 创建新的引擎池
    ///
    /// # 参数
    ///
    /// - `capacity`: 池大小
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aether::engine::EnginePool;
    ///
    /// // 创建容量为4的引擎池
    /// let pool = EnginePool::new(4);
    /// ```
    pub fn new(capacity: usize) -> Self {
        let mut engines = Vec::with_capacity(capacity);
        let available = vec![true; capacity];

        // 预创建引擎实例
        for _ in 0..capacity {
            engines.push(Aether::new());
        }

        Self { engines, available }
    }

    /// 从池中获取引擎（自动归还）
    ///
    /// 如果池中没有可用引擎，会创建临时引擎。
    /// 返回的 `PooledEngine` 会在作用域结束时自动归还到池中。
    ///
    /// # 环境隔离
    ///
    /// 每次获取前自动清空环境变量，保证隔离性。
    pub fn acquire(&mut self) -> PooledEngine {
        // 查找可用引擎
        for (i, &is_available) in self.available.iter().enumerate() {
            if is_available {
                self.available[i] = false;
                let mut engine = std::mem::take(&mut self.engines[i]);

                // 重置环境（保证隔离性）
                engine.evaluator.reset_env();

                return PooledEngine {
                    engine: Some(engine),
                    pool_index: Some(i),
                    pool: self as *mut Self,
                };
            }
        }

        // 池中无可用引擎，创建临时引擎
        let engine = Aether::new();
        PooledEngine {
            engine: Some(engine),
            pool_index: None,
            pool: std::ptr::null_mut(),
        }
    }

    /// 归还引擎到池中
    fn return_engine(&mut self, index: usize, engine: Aether) {
        self.engines[index] = engine;
        self.available[index] = true;
    }

    /// 获取池的容量
    pub fn capacity(&self) -> usize {
        self.engines.len()
    }

    /// 获取池中当前可用的引擎数量
    pub fn available(&self) -> usize {
        self.available.iter().filter(|&&x| x).count()
    }
}

/// 自动归还的引擎（RAII模式）
///
/// 当此对象离开作用域时，引擎会自动归还到池中。
pub struct PooledEngine {
    engine: Option<Aether>,
    pool_index: Option<usize>,
    pool: *mut EnginePool,
}

impl PooledEngine {
    /// 执行 Aether 代码
    ///
    /// # 参数
    ///
    /// - `code`: 要执行的代码
    ///
    /// # 返回
    ///
    /// - `Ok(Value)`: 执行结果
    /// - `Err(String)`: 错误信息
    pub fn eval(&mut self, code: &str) -> Result<Value, String> {
        self.engine.as_mut().unwrap().eval(code)
    }

    /// 获取AST缓存统计信息
    pub fn cache_stats(&self) -> crate::cache::CacheStats {
        self.engine.as_ref().unwrap().cache_stats()
    }

    /// 清空AST缓存
    pub fn clear_cache(&mut self) {
        self.engine.as_mut().unwrap().clear_cache();
    }

    /// 配置优化选项
    pub fn set_optimization(
        &mut self,
        constant_folding: bool,
        dead_code: bool,
        tail_recursion: bool,
    ) {
        self.engine
            .as_mut()
            .unwrap()
            .set_optimization(constant_folding, dead_code, tail_recursion);
    }

    /// 异步执行 Aether 代码（requires "async" feature）
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use aether::engine::EnginePool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut pool = EnginePool::new(4);
    ///     let mut engine = pool.acquire();
    ///     let result = engine.eval_async("Set X 10\n(X + 20)").await.unwrap();
    ///     println!("Result: {}", result);
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn eval_async(&mut self, code: &str) -> Result<Value, String> {
        tokio::task::yield_now().await;
        self.eval(code)
    }
}

impl Drop for PooledEngine {
    fn drop(&mut self) {
        if let Some(engine) = self.engine.take()
            && let Some(index) = self.pool_index
        {
            // 归还到池中
            unsafe {
                if !self.pool.is_null() {
                    (*self.pool).return_engine(index, engine);
                }
            }
        }
        // 否则是临时引擎，直接丢弃
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_single_thread() {
        let mut pool = EnginePool::new(2);

        // 第一次获取
        {
            let mut engine = pool.acquire();
            let result = engine.eval("Set X 10\n(X + 20)").unwrap();
            assert_eq!(result.to_string(), "30");
        }

        // 第二次获取（环境应该是干净的）
        {
            let mut engine = pool.acquire();
            let result = engine.eval("X");
            assert!(result.is_err()); // X 不应该存在
        }
    }

    #[test]
    fn test_pool_multiple_acquire() {
        let mut pool = EnginePool::new(2);

        // 多次获取和使用
        for i in 0..10 {
            let mut engine = pool.acquire();
            let code = format!("Set X {}\n(X * 2)", i);
            let result = engine.eval(&code).unwrap();
            assert_eq!(result.to_string(), format!("{}", i * 2));
        }
    }

    #[test]
    fn test_pool_auto_return() {
        let mut pool = EnginePool::new(2);

        assert_eq!(pool.available(), 2);

        let _engine1 = pool.acquire();
        assert_eq!(pool.available(), 1);

        {
            let _engine2 = pool.acquire();
            assert_eq!(pool.available(), 0);
        } // engine2 归还

        assert_eq!(pool.available(), 1);
    }
}
