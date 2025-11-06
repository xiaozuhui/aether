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
pub mod environment;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod value;

// Re-export commonly used types
pub use ast::{Expr, Program, Stmt};
pub use builtins::{BuiltInRegistry, IOPermissions};
pub use environment::Environment;
pub use evaluator::{EvalResult, Evaluator, RuntimeError};
pub use lexer::Lexer;
pub use parser::{ParseError, Parser};
pub use token::Token;
pub use value::Value;

/// Main Aether engine struct
pub struct Aether {
    evaluator: Evaluator,
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
        }
    }

    /// Create a new Aether engine with all IO permissions enabled
    pub fn with_all_permissions() -> Self {
        Self::with_permissions(IOPermissions::allow_all())
    }

    /// Evaluate Aether code and return the result
    pub fn eval(&mut self, code: &str) -> Result<Value, String> {
        // Parse the code
        let mut parser = Parser::new(code);
        let program = parser
            .parse_program()
            .map_err(|e| format!("Parse error: {}", e))?;

        // Evaluate the program
        self.evaluator
            .eval_program(&program)
            .map_err(|e| format!("Runtime error: {}", e))
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
}
