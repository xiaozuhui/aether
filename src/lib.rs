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

// 重新导出常用类型
pub use ast::{Expr, Program, Stmt};
pub use builtins::{BuiltInRegistry, IOPermissions};
pub use cache::{ASTCache, CacheStats};
pub use environment::Environment;
pub use evaluator::{ErrorReport, EvalResult, Evaluator, RuntimeError};
pub use lexer::Lexer;
pub use module_system::{DisabledModuleResolver, FileSystemModuleResolver, ModuleResolver};
pub use optimizer::Optimizer;
pub use parser::{ParseError, Parser};
pub use runtime::{
    ExecutionLimitError, ExecutionLimits, TraceEntry, TraceFilter, TraceLevel, TraceStats,
};
pub use sandbox::{
    ExecutionMetrics, MetricsCollector, MetricsSnapshot, ModuleCacheManager, ModuleCacheStats,
    ModuleMetrics, PathRestriction, PathValidationError, PathValidator, SandboxConfig,
    SandboxPolicy, ScopedValidator,
};
pub use token::Token;
pub use value::Value;

/// 主要的 Aether 引擎结构体
pub struct Aether {
    evaluator: Evaluator,
    cache: ASTCache,
    optimizer: Optimizer,
}

impl Aether {
    /// 创建新的 Aether 引擎实例
    ///
    /// **用于 DSL 嵌入**：IO 操作默认禁用以确保安全性。
    /// 使用 `with_permissions()` 或 `with_all_permissions()` 来启用 IO。
    ///
    /// **用于 CLI 使用**：命令行工具默认使用 `with_all_permissions()`。
    pub fn new() -> Self {
        Self::with_permissions(IOPermissions::default())
    }

    /// 使用自定义 IO 权限创建新的 Aether 引擎
    pub fn with_permissions(permissions: IOPermissions) -> Self {
        Aether {
            evaluator: Evaluator::with_permissions(permissions),
            cache: ASTCache::new(),
            optimizer: Optimizer::new(),
        }
    }

    /// 创建启用所有 IO 权限的新 Aether 引擎
    pub fn with_all_permissions() -> Self {
        Self::with_permissions(IOPermissions::allow_all())
    }

    /// 创建预加载标准库的新 Aether 引擎
    ///
    /// 这将创建一个具有所有权限的引擎，并自动加载
    /// 所有标准库模块（string_utils、array_utils、validation、datetime、testing）。
    pub fn with_stdlib() -> Result<Self, String> {
        let mut engine = Self::with_all_permissions();
        stdlib::preload_stdlib(&mut engine)?;
        Ok(engine)
    }

    /// 加载特定的标准库模块
    ///
    /// 可用模块："string_utils"、"array_utils"、"validation"、"datetime"、"testing"
    pub fn load_stdlib_module(&mut self, module_name: &str) -> Result<(), String> {
        if let Some(code) = stdlib::get_module(module_name) {
            self.eval(code)?;
            Ok(())
        } else {
            Err(format!("Unknown stdlib module: {}", module_name))
        }
    }

    /// 加载所有标准库模块
    pub fn load_all_stdlib(&mut self) -> Result<(), String> {
        stdlib::preload_stdlib(self)
    }

    // ============================================================
    // 执行限制
    // ============================================================

    /// 使用执行限制创建新的 Aether 引擎
    pub fn with_limits(mut self, limits: ExecutionLimits) -> Self {
        self.evaluator.set_limits(limits);
        self
    }

    /// 设置执行限制
    pub fn set_limits(&mut self, limits: ExecutionLimits) {
        self.evaluator.set_limits(limits);
    }

    /// 获取当前执行限制
    pub fn limits(&self) -> &ExecutionLimits {
        self.evaluator.limits()
    }

    // ============================================================
    // 可链式调用的 stdlib 模块加载方法
    // ============================================================

    /// 加载字符串工具模块（可链式调用）
    pub fn with_stdlib_string_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("string_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载数组工具模块（可链式调用）
    pub fn with_stdlib_array_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("array_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载验证模块（可链式调用）
    pub fn with_stdlib_validation(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("validation") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载日期时间模块（可链式调用）
    pub fn with_stdlib_datetime(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("datetime") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载测试框架模块（可链式调用）
    pub fn with_stdlib_testing(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("testing") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载集合数据结构模块（可链式调用）
    pub fn with_stdlib_set(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("set") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载队列数据结构模块（可链式调用）
    pub fn with_stdlib_queue(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("queue") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载栈数据结构模块（可链式调用）
    pub fn with_stdlib_stack(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("stack") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载堆数据结构模块（可链式调用）
    pub fn with_stdlib_heap(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("heap") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载排序算法模块（可链式调用）
    pub fn with_stdlib_sorting(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("sorting") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载 JSON 处理模块（可链式调用）
    pub fn with_stdlib_json(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("json") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载 CSV 处理模块（可链式调用）
    pub fn with_stdlib_csv(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("csv") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载函数式编程工具模块（可链式调用）
    pub fn with_stdlib_functional(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("functional") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载 CLI 工具模块（可链式调用）
    pub fn with_stdlib_cli_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("cli_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载文本模板引擎模块（可链式调用）
    pub fn with_stdlib_text_template(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("text_template") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载正则表达式工具模块（可链式调用）
    pub fn with_stdlib_regex_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("regex_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 求值 Aether 代码并返回结果
    pub fn eval(&mut self, code: &str) -> Result<Value, String> {
        // 在开始新的顶级求值之前清除任何之前的调用栈帧。
        self.evaluator.clear_call_stack();
        self.evaluator.reset_step_counter();

        // 尝试从缓存获取AST
        let program = if let Some(cached_program) = self.cache.get(code) {
            cached_program
        } else {
            // 解析代码
            let mut parser = Parser::new(code);
            let program = parser
                .parse_program()
                .map_err(|e| format!("Parse error: {}", e))?;

            // 优化AST
            let optimized = self.optimizer.optimize_program(&program);

            // 将优化后的结果存入缓存
            self.cache.insert(code, optimized.clone());
            optimized
        };

        // 求值程序
        self.evaluator
            .eval_program(&program)
            .map_err(|e| format!("Runtime error: {}", e))
    }

    /// 求值 Aether 代码并在失败时返回结构化的错误报告。
    ///
    /// 这适用于需要机器可读诊断的集成。
    pub fn eval_report(&mut self, code: &str) -> Result<Value, ErrorReport> {
        // 在开始新的顶级求值之前清除任何之前的调用栈帧。
        self.evaluator.clear_call_stack();
        self.evaluator.reset_step_counter();

        // 首先尝试 AST 缓存
        let program = if let Some(cached_program) = self.cache.get(code) {
            cached_program
        } else {
            let mut parser = Parser::new(code);
            let program = parser
                .parse_program()
                .map_err(|e| ErrorReport::parse_error(e.to_string()))?;

            let optimized = self.optimizer.optimize_program(&program);
            self.cache.insert(code, optimized.clone());
            optimized
        };

        self.evaluator
            .eval_program(&program)
            .map_err(|e| e.to_error_report())
    }

    /// 清空内存中的 TRACE 缓冲区。
    ///
    /// 这是为 DSL 安全调试设计的：脚本调用 `TRACE(...)` 来记录
    /// 值，宿主应用程序通过此方法带外读取它们。
    pub fn take_trace(&mut self) -> Vec<String> {
        self.evaluator.take_trace()
    }

    /// 清除 TRACE 缓冲区而不返回它。
    pub fn clear_trace(&mut self) {
        self.evaluator.clear_trace();
    }

    /// 获取所有结构化的跟踪条目
    ///
    /// 返回带有级别、类别、时间戳等的结构化跟踪条目向量。
    pub fn trace_records(&self) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_records()
    }

    /// 按级别过滤跟踪条目
    ///
    /// # 示例
    /// ```ignore
    /// let error_traces = engine.trace_by_level(crate::runtime::TraceLevel::Error);
    /// ```
    pub fn trace_by_level(
        &self,
        level: crate::runtime::TraceLevel,
    ) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_level(level)
    }

    /// 按类别过滤跟踪条目
    ///
    /// # 示例
    /// ```ignore
    /// let api_traces = engine.trace_by_category("api_call");
    /// ```
    pub fn trace_by_category(&self, category: &str) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_category(category)
    }

    /// 按标签过滤跟踪条目
    ///
    /// # 示例
    /// ```ignore
    /// let slow_traces = engine.trace_by_label("slow_request");
    /// ```
    pub fn trace_by_label(&self, label: &str) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_label(label)
    }

    /// 对跟踪条目应用复杂过滤器
    ///
    /// # 示例
    /// ```ignore
    /// use crate::runtime::{TraceFilter, TraceLevel};
    /// use std::time::Instant;
    ///
    /// let filter = TraceFilter::new()
    ///     .with_min_level(TraceLevel::Warn)
    ///     .with_category("api".to_string());
    /// let filtered = engine.trace_filter(&filter);
    /// ```
    pub fn trace_filter(
        &self,
        filter: &crate::runtime::TraceFilter,
    ) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_filter(filter)
    }

    /// 获取跟踪统计信息
    ///
    /// 返回关于跟踪条目的统计信息，包括按级别和类别的计数。
    pub fn trace_stats(&self) -> crate::runtime::TraceStats {
        self.evaluator.trace_stats()
    }

    /// 设置 TRACE 缓冲区大小
    ///
    /// 这将设置 TRACE 缓冲区可以存储的最大条目数。
    /// 如果新大小小于当前条目数，多余的条目将被从缓冲区前端移除。
    ///
    /// # 示例
    /// ```rust
    /// use aether::Aether;
    ///
    /// let mut engine = Aether::new();
    /// engine.set_trace_buffer_size(2048); // 设置为 2048 条目
    /// ```
    pub fn set_trace_buffer_size(&mut self, size: usize) {
        self.evaluator.set_trace_buffer_size(size);
    }

    /// 获取当前顶级执行的 step 计数。
    ///
    /// 该计数在每次调用 `eval(...)` / `eval_report(...)`（以及它们的文件包装器）开始时被重置。
    ///
    /// 说明：step 目前按“语句级”计数（每求值一条语句 +1）。
    pub fn step_count(&self) -> usize {
        self.evaluator.step_count()
    }

    /// 配置用于 `Import/Export` 的模块解析器。
    ///
    /// 默认情况下（DSL 嵌入），解析器出于安全考虑被禁用。
    pub fn set_module_resolver(&mut self, resolver: Box<dyn crate::module_system::ModuleResolver>) {
        self.evaluator.set_module_resolver(resolver);
    }

    /// 推送用于解析相对导入的基础目录上下文。
    ///
    /// 这通常由基于文件的运行器（CLI）在调用 `eval()` 之前使用。
    pub fn push_import_base(&mut self, module_id: String, base_dir: Option<std::path::PathBuf>) {
        self.evaluator.push_import_base(module_id, base_dir);
    }

    /// 弹出最近的基础目录上下文。
    pub fn pop_import_base(&mut self) {
        self.evaluator.pop_import_base();
    }

    /// 从文件路径求值 Aether 脚本。
    ///
    /// 这是一个便利包装器，它：
    /// - 读取文件
    /// - 推送导入基础上下文（module_id = 规范路径；base_dir = 父目录）
    /// - 求值代码
    /// - 弹出导入基础上下文
    ///
    /// 注意：这**不会**启用任何模块解析器。为了 DSL 安全性，除非您明确调用 `set_module_resolver(...)`，否则模块加载保持禁用状态。
    pub fn eval_file(&mut self, path: impl AsRef<std::path::Path>) -> Result<Value, String> {
        let path = path.as_ref();

        let code = std::fs::read_to_string(path).map_err(|e| format!("IO error: {}", e))?;

        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let base_dir = canon.parent().map(|p| p.to_path_buf());

        self.push_import_base(canon.display().to_string(), base_dir);
        let res = self.eval(&code);
        self.pop_import_base();
        res
    }

    /// 从文件路径求值 Aether 脚本，在失败时返回结构化的错误报告。
    pub fn eval_file_report(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Value, ErrorReport> {
        let path = path.as_ref();

        let code = std::fs::read_to_string(path)
            .map_err(|e| ErrorReport::io_error(format!("IO error: {e}")))?;

        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let base_dir = canon.parent().map(|p| p.to_path_buf());

        self.push_import_base(canon.display().to_string(), base_dir);
        let res = self.eval_report(&code);
        self.pop_import_base();
        res
    }

    /// 从宿主应用程序设置全局变量，而不使用 `eval()`。
    ///
    /// 当您已经有 Rust 端数据并希望将其作为 `Value` 注入脚本环境时，这很有用。
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.evaluator.set_global(name.to_string(), value);
    }

    /// 重置运行时环境（变量/函数），同时保持内置函数注册。
    ///
    /// 注意：这会清除通过 `eval()` 引入的任何内容（包括 stdlib 代码）。
    pub fn reset_env(&mut self) {
        self.evaluator.reset_env();
    }

    /// 在隔离的子作用域内运行闭包。
    ///
    /// 在闭包内注入或定义的所有变量/函数将在返回时被丢弃，而外部环境被保留。
    ///
    /// 这是为 "DSL 宿主" 场景设计的：注入 Rust 数据 + 加载每请求的
    /// Aether 函数（例如从 DB）+ 运行脚本，而不跨请求污染。
    pub fn with_isolated_scope<R>(
        &mut self,
        f: impl FnOnce(&mut Aether) -> Result<R, String>,
    ) -> Result<R, String> {
        let prev_env = self.evaluator.enter_child_scope();
        let result = f(self);
        self.evaluator.restore_env(prev_env);
        result
    }

    /// 异步求值 Aether 代码（需要 "async" 特性）
    ///
    /// 这是围绕 `eval()` 的便利包装器，在后台任务中运行。
    /// 用于将 Aether 集成到异步 Rust 应用程序中。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use aether::Aether;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut engine = Aether::new();
    ///     let result = engine.eval_async("Set X 10\n(X + 20)").await.unwrap();
    ///     println!("Result: {}", result);
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn eval_async(&mut self, code: &str) -> Result<Value, String> {
        // 由于 Aether 内部使用 Rc (非 Send)，我们在当前线程执行
        // 但通过 tokio::task::yield_now() 让出执行权，避免阻塞事件循环
        tokio::task::yield_now().await;
        self.eval(code)
    }

    /// 获取缓存统计信息
    pub fn cache_stats(&self) -> CacheStats {
        self.cache.stats()
    }

    /// 清空缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 设置优化选项
    pub fn set_optimization(
        &mut self,
        constant_folding: bool,
        dead_code: bool,
        tail_recursion: bool,
    ) {
        self.optimizer.constant_folding = constant_folding;
        self.optimizer.dead_code_elimination = dead_code;
        self.optimizer.tail_recursion = tail_recursion;
    }
}

impl Default for Aether {
    fn default() -> Self {
        Self::new()
    }
}
