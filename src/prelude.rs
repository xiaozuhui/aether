// Re-exports of commonly used public types.
// Kept in a separate module to keep lib.rs smaller.

pub use crate::ast::{Expr, Program, Stmt};
pub use crate::builtins::{BuiltInRegistry, IOPermissions};
pub use crate::cache::{ASTCache, CacheStats};
pub use crate::environment::Environment;
pub use crate::evaluator::{ErrorReport, EvalResult, Evaluator, RuntimeError};
pub use crate::lexer::Lexer;
pub use crate::module_system::{DisabledModuleResolver, FileSystemModuleResolver, ModuleResolver};
pub use crate::optimizer::Optimizer;
pub use crate::parser::{ParseError, Parser};
pub use crate::runtime::{
    ExecutionLimitError, ExecutionLimits, TraceEntry, TraceFilter, TraceLevel, TraceStats,
};
pub use crate::sandbox::{
    ExecutionMetrics, MetricsCollector, MetricsSnapshot, ModuleCacheManager, ModuleCacheStats,
    ModuleMetrics, PathRestriction, PathValidationError, PathValidator, SandboxConfig,
    SandboxPolicy, ScopedValidator,
};
pub use crate::token::Token;
pub use crate::value::Value;
