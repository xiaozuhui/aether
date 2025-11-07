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
pub mod environment;
pub mod evaluator;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod stdlib;
pub mod token;
pub mod value;

// FFI and language bindings
pub mod ffi;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export commonly used types
pub use ast::{Expr, Program, Stmt};
pub use builtins::{BuiltInRegistry, IOPermissions};
pub use cache::{ASTCache, CacheStats};
pub use environment::Environment;
pub use evaluator::{EvalResult, Evaluator, RuntimeError};
pub use lexer::Lexer;
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

    /// Evaluate Aether code and return the result
    pub fn eval(&mut self, code: &str) -> Result<Value, String> {
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
