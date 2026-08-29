//! Aether - 一个轻量级、可嵌入的领域特定语言
//!
//! 这个 crate 提供了 Aether 语言的完整实现，
//! 包括词法分析器、解析器、求值器和标准库。
//!
//! # 快速开始
//!
//! ## 作为 DSL（嵌入到您的应用程序中）
//!
//! 当将 Aether 作为 DSL 嵌入时，IO 操作**默认禁用**以确保安全性：
//!
//! ```
//! use aether::Aether;
//!
//! // 默认：IO 禁用（对用户脚本安全）
//! let mut engine = Aether::new();
//! let code = r#"
//!     Set X 10
//!     Set Y 20
//!     (X + Y)
//! "#;
//!
//! match engine.eval(code) {
//!     Ok(result) => println!("Result: {}", result),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! 仅在需要时启用 IO：
//!
//! ```
//! use aether::{Aether, IOPermissions};
//!
//! // 仅启用文件系统
//! let mut perms = IOPermissions::default();
//! perms.filesystem_enabled = true;
//! let mut engine = Aether::with_permissions(perms);
//!
//! // 或启用所有 IO
//! let mut engine = Aether::with_all_permissions();
//! ```
//!
//! ## 高性能引擎模式（新增！）
//!
//! 对于**高频、大规模 DSL 执行**，Aether 提供了三种优化的引擎模式：
//!
//! ### 1. GlobalEngine - 全局单例（最适合单线程）
//!
//! ```rust
//! use aether::engine::GlobalEngine;
//!
//! // 使用隔离环境执行（每次清除变量）
//! let result = GlobalEngine::eval_isolated("Set X 10\n(X + 20)").unwrap();
//! println!("Result: {}", result);
//!
//! // 优势：
//! // - ✅ 最大性能（引擎仅创建一次）
//! // - ✅ AST 缓存累积（高达 142 倍加速！）
//! // - ✅ 环境隔离（每次调用清除变量）
//! // - ⚠️ 线程局部（thread_local，每个线程持有独立实例，不跨线程共享）
//! ```
//!
//! ### 2. EnginePool - 引擎池（单线程内多实例）
//!
//! ```rust
//! use aether::engine::EnginePool;
//!
//! // 一次性创建池（避免频繁创建引擎的开销）
//! let pool = EnginePool::new(4);
//!
//! // 借出 → 使用 → 作用域结束自动归还（RAII）
//! {
//!     let mut engine = pool.acquire();
//!     let result = engine.eval("Set X 10\n(X + 20)").unwrap();
//! }
//!
//! // acquire 只需 &self，可同时持有多个句柄
//! let e1 = pool.acquire();
//! let e2 = pool.acquire();
//!
//! // 优势：
//! // - ✅ RAII 模式（自动归还池）
//! // - ✅ 环境隔离（获取时清除变量）
//! // - ✅ 每个引擎独立维护 AST 缓存
//! // - ⚠️ 线程局部（Aether 使用 Rc，非 Send；跨线程请为每个线程建独立的池）
//! ```
//!
//! ### 3. ScopedEngine - 闭包风格（最适合简单性）
//!
//! ```rust
//! use aether::engine::ScopedEngine;
//!
//! // 闭包风格（类似 Py3o）
//! let result = ScopedEngine::with(|engine| {
//!     engine.eval("Set X 10")?;
//!     engine.eval("(X + 20)")
//! }).unwrap();
//!
//! // 或简化版本
//! let result = ScopedEngine::eval("Set X 10\n(X + 20)").unwrap();
//!
//! // 优势：
//! // - ✅ 完全隔离（每次新建引擎）
//! // - ✅ 简洁 API（自动生命周期管理）
//! // - ⚠️ 较低性能（无缓存重用）
//! ```
//!
//! ### 模式对比
//!
//! | 特性 | GlobalEngine | EnginePool | ScopedEngine |
//! |---------|-------------|------------|--------------|
//! | 性能 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
//! | 跨线程共享单引擎 | ❌ | ❌ | ❌ |
//! | 隔离 | ✅ | ✅ | ✅ |
//! | AST 缓存 | ✅ | ✅ | ❌ |
//! | 使用场景 | 单线程高频 | 单线程多实例 | 偶尔使用 |
//!
//! 三种模式的引擎都不能跨线程共享（`Aether` 持有 `Rc`，非 `Send`）。
//! 多线程场景请为每个线程创建独立的 `GlobalEngine`（thread_local 自动如此）
//! 或独立的 `EnginePool`。
//!
//! ### 选择性标准库加载（推荐用于 DSL）
//!
//! 为获得更好性能，仅加载您需要的 stdlib 模块：
//!
//! ```
//! use aether::Aether;
//!
//! // 按模块名选择性加载（返回 Result，未知模块名报错）
//! let mut engine = Aether::new()
//!     .with_stdlib_module("string_utils").unwrap()
//!     .with_stdlib_module("array_utils").unwrap();
//!
//! // 或一次性加载全部标准库
//! let mut full = Aether::with_stdlib().unwrap();
//!
//! // 可用模块名：string_utils, array_utils, functional, validation, datetime,
//! // testing, set, queue, stack, heap, sorting, json, csv, regex_utils,
//! // text_template, cli_utils
//! ```
//!
//! ## 作为独立语言（命令行工具）
//!
//! `aether` 命令行工具自动启用所有 IO 权限，
//! 允许脚本自由使用文件和网络操作：
//!
//! ```bash
//! # 在 CLI 模式下，所有 IO 操作都有效
//! aether script.aether
//! ```

pub mod ast;
pub mod builtins;
pub mod cache;
pub mod debugger;
pub mod engine;
pub mod environment;
pub mod evaluator;
pub mod lexer;
pub mod module_system;
pub mod numeric;
pub mod optimizer;
pub mod parser;
pub mod runtime;
pub mod stdlib;
pub mod token;
pub mod value;

mod api;
mod prelude;

pub use api::Aether;
pub use prelude::*;
