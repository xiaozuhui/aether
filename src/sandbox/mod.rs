//! 沙箱模块：提供安全隔离和权限控制能力
//!
//! 本模块实现了 Aether DSL 的安全护城河，包括：
//! - 路径安全验证（防止路径遍历攻击）
//! - 模块缓存生命周期管理
//! - 沙箱配置统一
//! - 可观测性指标收集

pub mod config;
pub mod context;
pub mod metrics;
pub mod module_cache;
pub mod path_validator;

pub use config::{SandboxConfig, SandboxPolicy};
pub use context::{get_filesystem_validator, ScopedValidator, set_filesystem_validator};
pub use metrics::{ExecutionMetrics, MetricsCollector, MetricsSnapshot, ModuleMetrics};
pub use module_cache::{ModuleCacheManager, ModuleCacheStats};
pub use path_validator::{PathValidationError, PathRestriction, PathValidator};
