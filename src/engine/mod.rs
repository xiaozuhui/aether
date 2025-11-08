//! Aether Engine - 高性能引擎管理模块
//!
//! 提供三种独立的引擎模式，适用于不同场景：
//!
//! ## 1. GlobalEngine - 全局单例模式
//!
//! **适用场景**：单线程、高频调用、需要最大化性能
//!
//! ```rust
//! use aether::engine::GlobalEngine;
//!
//! // 使用全局单例（隔离环境，但保留AST缓存）
//! let result = GlobalEngine::eval_isolated("Set X 10\n(X + 20)")?;
//! println!("Result: {}", result);
//! ```
//!
//! **特点**：
//! - ✅ 性能最优（只创建一次引擎）
//! - ✅ AST缓存效果最佳（可达142x加速）
//! - ✅ 环境隔离（每次eval前清空变量）
//! - ⚠️ 需要线程同步（Mutex）
//!
//! ## 2. PooledEngine - 引擎池模式
//!
//! **适用场景**：单线程内需要多个引擎实例、避免频繁创建开销
//!
//! ```rust
//! use aether::engine::EnginePool;
//!
//! // 创建引擎池（建议4-8个）
//! let mut pool = EnginePool::new(4);
//!
//! // 多次使用（复用引擎实例）
//! for i in 0..100 {
//!     let mut engine = pool.acquire(); // 自动获取
//!     let code = format!("Set X {}\n(X * 2)", i);
//!     engine.eval(&code)?;
//! } // 作用域结束自动归还
//! ```
//!
//! **特点**：
//! - ✅ 复用引擎实例（避免频繁创建）
//! - ✅ 自动归还（RAII模式）
//! - ✅ 环境隔离（每次获取前清空）
//! - ⚠️ 线程局部（每个线程独立池）
//!
//! ## 3. ScopedEngine - 闭包模式
//!
//! **适用场景**：临时执行、需要完全隔离、偶尔使用
//!
//! ```rust
//! use aether::engine::ScopedEngine;
//!
//! // 使用闭包（每次都是新引擎）
//! let result = ScopedEngine::with(|engine| {
//!     engine.eval("Set X 10")?;
//!     engine.eval("(X + 20)")
//! })?;
//! ```
//!
//! **特点**：
//! - ✅ 完全隔离（每次新建引擎）
//! - ✅ API简洁（类似Py3o）
//! - ⚠️ 性能较低（无法利用缓存）
//!
//! ## 模式对比
//!
//! | 特性 | GlobalEngine | PooledEngine | ScopedEngine |
//! |------|-------------|--------------|--------------|
//! | 性能 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
//! | 多引擎 | ❌ | ✅ | ❌ |
//! | 环境隔离 | ✅ | ✅ | ✅ |
//! | AST缓存 | ✅ | ✅ | ❌ |
//! | 内存占用 | 低 | 中 | 低 |
//! | 使用场景 | 单线程高频 | 避免频繁创建 | 临时执行 |

pub mod global;
pub mod pool;
pub mod scoped;

pub use global::GlobalEngine;
pub use pool::{EnginePool, PooledEngine};
pub use scoped::ScopedEngine;
