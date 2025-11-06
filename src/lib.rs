//! Aether - A lightweight, embeddable domain-specific language
//!
//! This crate provides a complete implementation of the Aether language,
//! including lexer, parser, evaluator, and standard library.
//!
//! # Quick Start
//!
//! ```
//! use aether::Aether;
//!
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
pub use builtins::BuiltInRegistry;
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
    pub fn new() -> Self {
        Aether {
            evaluator: Evaluator::new(),
        }
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
