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
//! // - ⚠️ 单线程（使用 Mutex）
//! ```
//!
//! ### 2. EnginePool - 引擎池（最适合多线程）
//!
//! ```rust
//! use aether::engine::EnginePool;
//! use std::thread;
//!
//! // 一次性创建池（大小 = 推荐 2-4 倍 CPU 核心数）
//! let pool = EnginePool::new(8);
//!
//! // 跨线程使用
//! let handles: Vec<_> = (0..4).map(|i| {
//!     let pool = pool.clone();
//!     thread::spawn(move || {
//!         let mut engine = pool.acquire(); // 自动获取
//!         let code = format!("Set X {}\n(X * 2)", i);
//!         engine.eval(&code)
//!     }) // 作用域退出时自动返回
//! }).collect();
//!
//! // 优势：
//! // - ✅ 多线程安全（无锁队列）
//! // - ✅ RAII 模式（自动返回池）
//! // - ✅ 环境隔离（获取时清除）
//! // - ✅ 每个引擎的 AST 缓存
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
//! | 多线程 | ❌ | ✅ | ✅ |
//! | 隔离 | ✅ | ✅ | ✅ |
//! | AST 缓存 | ✅ | ✅ | ❌ |
//! | 使用场景 | 单线程高频 | 多线程 | 偶尔使用 |
//!
//! ### 选择性标准库加载（推荐用于 DSL）
//!
//! 为获得更好性能，仅加载您需要的 stdlib 模块：
//!
//! ```
//! use aether::Aether;
//!
//! // 仅加载字符串和数组工具
//! let mut engine = Aether::new()
//!     .with_stdlib_string_utils()
//!     .unwrap()
//!     .with_stdlib_array_utils()
//!     .unwrap();
//!
//! // 或加载数据结构
//! let mut engine2 = Aether::new()
//!     .with_stdlib_set()
//!     .unwrap()
//!     .with_stdlib_queue()
//!     .unwrap()
//!     .with_stdlib_stack()
//!     .unwrap();
//!
//! // 可用模块：
//! // - with_stdlib_string_utils()
//! // - with_stdlib_array_utils()
//! // - with_stdlib_validation()
//! // - with_stdlib_datetime()
//! // - with_stdlib_testing()
//! // - with_stdlib_set()
//! // - with_stdlib_queue()
//! // - with_stdlib_stack()
//! // - with_stdlib_heap()
//! // - with_stdlib_sorting()
//! // - with_stdlib_json()
//! // - with_stdlib_csv()
//! // - with_stdlib_functional()
//! // - with_stdlib_cli_utils()
//! // - with_stdlib_text_template()
//! // - with_stdlib_regex_utils()
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
pub mod optimizer;
pub mod parser;
pub mod runtime;
pub mod sandbox;
pub mod stdlib;
pub mod token;
pub mod value;

// FFI 和语言绑定
pub mod ffi;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

mod api;
mod prelude;

pub use api::Aether;
pub use prelude::*;
