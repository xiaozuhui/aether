//! 运行时限制和能力
//!
//! 本模块提供执行限制、调试器和 TRACE 系统等运行时能力。

pub mod limits;
pub mod trace;

pub use limits::{ExecutionLimitError, ExecutionLimits};
pub use trace::{TraceEntry, TraceFilter, TraceLevel, TraceStats};
