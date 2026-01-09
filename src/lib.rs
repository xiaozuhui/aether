//! Aether - A lightweight, embeddable domain-specific language
//!
//! This crate provides a complete implementation of the Aether language,
//! including lexer, parser, evaluator, and standard library.
//!
//! # Quick Start
//!
//! ## As a DSL (Embedded in Your Application)
//!
//! When embedding Aether as a DSL, IO operations are **disabled by default** for security:
//!
//! ```
//! use aether::Aether;
//!
//! // Default: IO disabled (safe for user scripts)
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
//! Enable IO only when needed:
//!
//! ```
//! use aether::{Aether, IOPermissions};
//!
//! // Enable only filesystem
//! let mut perms = IOPermissions::default();
//! perms.filesystem_enabled = true;
//! let mut engine = Aether::with_permissions(perms);
//!
//! // Or enable all IO
//! let mut engine = Aether::with_all_permissions();
//! ```
//!
//! ## High-Performance Engine Modes (New!)
//!
//! For **high-frequency, large-scale DSL execution**, Aether provides three optimized engine modes:
//!
//! ### 1. GlobalEngine - Global Singleton (Best for Single-Thread)
//!
//! ```rust
//! use aether::engine::GlobalEngine;
//!
//! // Execute with isolated environment (variables cleared each time)
//! let result = GlobalEngine::eval_isolated("Set X 10\n(X + 20)").unwrap();
//! println!("Result: {}", result);
//!
//! // Benefits:
//! // - ✅ Maximum performance (engine created only once)
//! // - ✅ AST cache accumulates (up to 142x speedup!)
//! // - ✅ Environment isolation (variables cleared between calls)
//! // - ⚠️ Single-threaded (uses Mutex)
//! ```
//!
//! ### 2. EnginePool - Engine Pool (Best for Multi-Thread)
//!
//! ```rust
//! use aether::engine::EnginePool;
//! use std::thread;
//!
//! // Create pool once (size = 2-4x CPU cores recommended)
//! let pool = EnginePool::new(8);
//!
//! // Use across threads
//! let handles: Vec<_> = (0..4).map(|i| {
//!     let pool = pool.clone();
//!     thread::spawn(move || {
//!         let mut engine = pool.acquire(); // Auto-acquire
//!         let code = format!("Set X {}\n(X * 2)", i);
//!         engine.eval(&code)
//!     }) // Auto-return on scope exit
//! }).collect();
//!
//! // Benefits:
//! // - ✅ Multi-thread safe (lock-free queue)
//! // - ✅ RAII pattern (auto-return to pool)
//! // - ✅ Environment isolation (cleared on acquire)
//! // - ✅ AST cache per engine
//! ```
//!
//! ### 3. ScopedEngine - Closure Style (Best for Simplicity)
//!
//! ```rust
//! use aether::engine::ScopedEngine;
//!
//! // Closure style (like Py3o)
//! let result = ScopedEngine::with(|engine| {
//!     engine.eval("Set X 10")?;
//!     engine.eval("(X + 20)")
//! }).unwrap();
//!
//! // Or simplified version
//! let result = ScopedEngine::eval("Set X 10\n(X + 20)").unwrap();
//!
//! // Benefits:
//! // - ✅ Complete isolation (new engine each time)
//! // - ✅ Clean API (auto lifetime management)
//! // - ⚠️ Lower performance (no cache reuse)
//! ```
//!
//! ### Mode Comparison
//!
//! | Feature | GlobalEngine | EnginePool | ScopedEngine |
//! |---------|-------------|------------|--------------|
//! | Performance | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
//! | Multi-thread | ❌ | ✅ | ✅ |
//! | Isolation | ✅ | ✅ | ✅ |
//! | AST Cache | ✅ | ✅ | ❌ |
//! | Use Case | Single-thread high-freq | Multi-thread | Occasional |
//!
//! ### Selective Standard Library Loading (Recommended for DSL)
//!
//! For better performance, load only the stdlib modules you need:
//!
//! ```
//! use aether::Aether;
//!
//! // Load only string and array utilities
//! let mut engine = Aether::new()
//!     .with_stdlib_string_utils()
//!     .unwrap()
//!     .with_stdlib_array_utils()
//!     .unwrap();
//!
//! // Or load data structures
//! let mut engine2 = Aether::new()
//!     .with_stdlib_set()
//!     .unwrap()
//!     .with_stdlib_queue()
//!     .unwrap()
//!     .with_stdlib_stack()
//!     .unwrap();
//!
//! // Available modules:
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
//! ## As a Standalone Language (Command-Line Tool)
//!
//! The `aether` command-line tool automatically enables all IO permissions,
//! allowing scripts to freely use file and network operations:
//!
//! ```bash
//! # All IO operations work in CLI mode
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

// FFI and language bindings
pub mod ffi;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod pytranspile;

// Re-export commonly used types
pub use ast::{Expr, Program, Stmt};
pub use builtins::{BuiltInRegistry, IOPermissions};
pub use cache::{ASTCache, CacheStats};
pub use environment::Environment;
pub use evaluator::{ErrorReport, EvalResult, Evaluator, RuntimeError};
pub use lexer::Lexer;
pub use sandbox::{SandboxConfig, SandboxPolicy, PathValidator, PathRestriction, PathValidationError, ScopedValidator, ModuleCacheManager, ModuleCacheStats, MetricsCollector, MetricsSnapshot, ExecutionMetrics, ModuleMetrics};
pub use runtime::{ExecutionLimits, ExecutionLimitError, TraceEntry, TraceFilter, TraceLevel, TraceStats};
pub use module_system::{DisabledModuleResolver, FileSystemModuleResolver, ModuleResolver};
pub use optimizer::Optimizer;
pub use parser::{ParseError, Parser};
pub use token::Token;
pub use value::Value;

/// Main Aether engine struct
pub struct Aether {
    evaluator: Evaluator,
    cache: ASTCache,
    optimizer: Optimizer,
}

impl Aether {
    /// Create a new Aether engine instance
    ///
    /// **For DSL embedding**: IO operations are disabled by default for security.
    /// Use `with_permissions()` or `with_all_permissions()` to enable IO.
    ///
    /// **For CLI usage**: The command-line tool uses `with_all_permissions()` by default.
    pub fn new() -> Self {
        Self::with_permissions(IOPermissions::default())
    }

    /// Create a new Aether engine with custom IO permissions
    pub fn with_permissions(permissions: IOPermissions) -> Self {
        Aether {
            evaluator: Evaluator::with_permissions(permissions),
            cache: ASTCache::new(),
            optimizer: Optimizer::new(),
        }
    }

    /// Create a new Aether engine with all IO permissions enabled
    pub fn with_all_permissions() -> Self {
        Self::with_permissions(IOPermissions::allow_all())
    }

    /// Create a new Aether engine with standard library preloaded
    ///
    /// This creates an engine with all permissions and automatically loads
    /// all standard library modules (string_utils, array_utils, validation, datetime, testing).
    pub fn with_stdlib() -> Result<Self, String> {
        let mut engine = Self::with_all_permissions();
        stdlib::preload_stdlib(&mut engine)?;
        Ok(engine)
    }

    /// Load a specific standard library module
    ///
    /// Available modules: "string_utils", "array_utils", "validation", "datetime", "testing"
    pub fn load_stdlib_module(&mut self, module_name: &str) -> Result<(), String> {
        if let Some(code) = stdlib::get_module(module_name) {
            self.eval(code)?;
            Ok(())
        } else {
            Err(format!("Unknown stdlib module: {}", module_name))
        }
    }

    /// Load all standard library modules
    pub fn load_all_stdlib(&mut self) -> Result<(), String> {
        stdlib::preload_stdlib(self)
    }

    // ============================================================
    // Execution Limits
    // ============================================================

    /// Create a new Aether engine with execution limits
    pub fn with_limits(mut self, limits: ExecutionLimits) -> Self {
        self.evaluator.set_limits(limits);
        self
    }

    /// Set execution limits
    pub fn set_limits(&mut self, limits: ExecutionLimits) {
        self.evaluator.set_limits(limits);
    }

    /// Get current execution limits
    pub fn limits(&self) -> &ExecutionLimits {
        self.evaluator.limits()
    }

    // ============================================================
    // Chainable stdlib module loading methods
    // ============================================================

    /// Load string utilities module (chainable)
    pub fn with_stdlib_string_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("string_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load array utilities module (chainable)
    pub fn with_stdlib_array_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("array_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load validation module (chainable)
    pub fn with_stdlib_validation(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("validation") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load datetime module (chainable)
    pub fn with_stdlib_datetime(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("datetime") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load testing framework module (chainable)
    pub fn with_stdlib_testing(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("testing") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load set data structure module (chainable)
    pub fn with_stdlib_set(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("set") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load queue data structure module (chainable)
    pub fn with_stdlib_queue(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("queue") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load stack data structure module (chainable)
    pub fn with_stdlib_stack(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("stack") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load heap data structure module (chainable)
    pub fn with_stdlib_heap(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("heap") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load sorting algorithms module (chainable)
    pub fn with_stdlib_sorting(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("sorting") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load JSON processing module (chainable)
    pub fn with_stdlib_json(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("json") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load CSV processing module (chainable)
    pub fn with_stdlib_csv(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("csv") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load functional programming utilities module (chainable)
    pub fn with_stdlib_functional(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("functional") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load CLI utilities module (chainable)
    pub fn with_stdlib_cli_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("cli_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load text template engine module (chainable)
    pub fn with_stdlib_text_template(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("text_template") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Load regex utilities module (chainable)
    pub fn with_stdlib_regex_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("regex_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// Evaluate Aether code and return the result
    pub fn eval(&mut self, code: &str) -> Result<Value, String> {
        // Clear any previous call stack frames before starting a new top-level evaluation.
        self.evaluator.clear_call_stack();

        // 尝试从缓存获取AST
        let program = if let Some(cached_program) = self.cache.get(code) {
            cached_program
        } else {
            // Parse the code
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

        // Evaluate the program
        self.evaluator
            .eval_program(&program)
            .map_err(|e| format!("Runtime error: {}", e))
    }

    /// Evaluate Aether code and return a structured error report on failure.
    ///
    /// This is intended for integrations that need machine-readable diagnostics.
    pub fn eval_report(&mut self, code: &str) -> Result<Value, ErrorReport> {
        // Clear any previous call stack frames before starting a new top-level evaluation.
        self.evaluator.clear_call_stack();

        // Try AST cache first
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

    /// Drain the in-memory TRACE buffer.
    ///
    /// This is designed for DSL-safe debugging: scripts call `TRACE(...)` to record
    /// values, and the host application reads them out-of-band via this method.
    pub fn take_trace(&mut self) -> Vec<String> {
        self.evaluator.take_trace()
    }

    /// Clear the TRACE buffer without returning it.
    pub fn clear_trace(&mut self) {
        self.evaluator.clear_trace();
    }

    /// Get all structured trace entries (Stage 3.2)
    ///
    /// Returns a vector of structured trace entries with levels, categories, timestamps, etc.
    pub fn trace_records(&self) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_records()
    }

    /// Filter trace entries by level (Stage 3.2)
    ///
    /// # Example
    /// ```ignore
    /// let error_traces = engine.trace_by_level(crate::runtime::TraceLevel::Error);
    /// ```
    pub fn trace_by_level(&self, level: crate::runtime::TraceLevel) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_level(level)
    }

    /// Filter trace entries by category (Stage 3.2)
    ///
    /// # Example
    /// ```ignore
    /// let api_traces = engine.trace_by_category("api_call");
    /// ```
    pub fn trace_by_category(&self, category: &str) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_category(category)
    }

    /// Filter trace entries by label (Stage 3.2)
    ///
    /// # Example
    /// ```ignore
    /// let slow_traces = engine.trace_by_label("slow_request");
    /// ```
    pub fn trace_by_label(&self, label: &str) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_label(label)
    }

    /// Apply complex filter to trace entries (Stage 3.2)
    ///
    /// # Example
    /// ```ignore
    /// use crate::runtime::{TraceFilter, TraceLevel};
    /// use std::time::Instant;
    ///
    /// let filter = TraceFilter::new()
    ///     .with_min_level(TraceLevel::Warn)
    ///     .with_category("api".to_string());
    /// let filtered = engine.trace_filter(&filter);
    /// ```
    pub fn trace_filter(&self, filter: &crate::runtime::TraceFilter) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_filter(filter)
    }

    /// Get trace statistics (Stage 3.2)
    ///
    /// Returns statistics about trace entries, including counts by level and category.
    pub fn trace_stats(&self) -> crate::runtime::TraceStats {
        self.evaluator.trace_stats()
    }

    /// Set TRACE buffer size (Stage 3.2)
    ///
    /// Note: This method is a placeholder for future implementation.
    /// Currently, the buffer size is fixed at 1024 entries.
    #[allow(dead_code)]
    pub fn set_trace_buffer_size(&mut self, _size: usize) {
        // TODO: Implement configurable buffer size
        // For now, buffer size is fixed at TRACE_MAX_ENTRIES (1024)
    }

    /// Configure the module resolver used for `Import/Export`.
    ///
    /// By default (DSL embedding), the resolver is disabled for safety.
    pub fn set_module_resolver(&mut self, resolver: Box<dyn crate::module_system::ModuleResolver>) {
        self.evaluator.set_module_resolver(resolver);
    }

    /// Push a base directory context for resolving relative imports.
    ///
    /// This is typically used by a file-based runner (CLI) before calling `eval()`.
    pub fn push_import_base(&mut self, module_id: String, base_dir: Option<std::path::PathBuf>) {
        self.evaluator.push_import_base(module_id, base_dir);
    }

    /// Pop the most recent base directory context.
    pub fn pop_import_base(&mut self) {
        self.evaluator.pop_import_base();
    }

    /// Evaluate an Aether script from a file path.
    ///
    /// This is a convenience wrapper that:
    /// - reads the file
    /// - pushes an import base context (module_id = canonical path; base_dir = parent dir)
    /// - evaluates the code
    /// - pops the import base context
    ///
    /// Note: this does **not** enable any module resolver. For DSL safety, module loading
    /// remains disabled unless you explicitly call `set_module_resolver(...)`.
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

    /// Evaluate an Aether script from a file path, returning a structured error report on failure.
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

    /// Set a global variable from the host application without using `eval()`.
    ///
    /// This is useful when you already have Rust-side data and want to inject it
    /// as `Value` into the script environment.
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.evaluator.set_global(name.to_string(), value);
    }

    /// Reset the runtime environment (variables/functions) while keeping built-ins registered.
    ///
    /// Note: this clears anything that was introduced via `eval()` (including stdlib code).
    pub fn reset_env(&mut self) {
        self.evaluator.reset_env();
    }

    /// Run a closure inside an isolated child scope.
    ///
    /// All variables/functions you inject or define inside the closure will be dropped
    /// when it returns, while the outer environment is preserved.
    ///
    /// This is designed for the "DSL host" scenario: inject Rust data + load per-request
    /// Aether functions (e.g. from DB) + run the script, without cross-request pollution.
    pub fn with_isolated_scope<R>(
        &mut self,
        f: impl FnOnce(&mut Aether) -> Result<R, String>,
    ) -> Result<R, String> {
        let prev_env = self.evaluator.enter_child_scope();
        let result = f(self);
        self.evaluator.restore_env(prev_env);
        result
    }

    /// Evaluate Aether code asynchronously (requires "async" feature)
    ///
    /// This is a convenience wrapper around `eval()` that runs in a background task.
    /// Useful for integrating Aether into async Rust applications.
    ///
    /// # Example
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aether_creation() {
        let _engine = Aether::new();
    }

    #[test]
    fn test_cache_usage() {
        let mut engine = Aether::new();
        let code = "Set X 10\nX";

        // 第一次执行会解析
        let result1 = engine.eval(code).unwrap();
        assert_eq!(result1, Value::Number(10.0));

        // 第二次执行应该使用缓存
        let result2 = engine.eval(code).unwrap();
        assert_eq!(result2, Value::Number(10.0));

        // 检查缓存统计
        let stats = engine.cache_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }
}
