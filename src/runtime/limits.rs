//! 执行限制配置和错误类型
//!
//! 提供执行资源限制，防止恶意或错误代码耗尽系统资源。

use std::fmt;

/// 执行限制配置
///
/// 用于控制脚本执行的资源消耗，包括步数、递归深度、执行时长和内存使用。
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionLimits {
    /// 最大执行步数（指令计数）
    /// None 表示无限制
    pub max_steps: Option<usize>,

    /// 最大递归深度（调用栈深度）
    /// None 表示无限制
    pub max_recursion_depth: Option<usize>,

    /// 最大执行时长（毫秒）
    /// None 表示无限制
    pub max_duration_ms: Option<u64>,

    /// 最大内存分配（字节）
    /// None 表示无限制（暂未实现，预留）
    pub max_memory_bytes: Option<usize>,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self {
            max_steps: Some(1_000_000),      // 默认100万步
            max_recursion_depth: Some(1000), // 默认1000层
            max_duration_ms: Some(30_000),   // 默认30秒
            max_memory_bytes: None,
        }
    }
}

impl ExecutionLimits {
    /// 创建无限制的配置
    pub fn unrestricted() -> Self {
        Self {
            max_steps: None,
            max_recursion_depth: None,
            max_duration_ms: None,
            max_memory_bytes: None,
        }
    }

    /// 创建严格限制的配置（用于 DSL 安全模式）
    pub fn strict() -> Self {
        Self {
            max_steps: Some(100_000),       // 10万步
            max_recursion_depth: Some(100), // 100层
            max_duration_ms: Some(5_000),   // 5秒
            max_memory_bytes: None,
        }
    }

    /// 创建宽松限制的配置（用于 CLI 模式）
    pub fn lenient() -> Self {
        Self {
            max_steps: Some(10_000_000),     // 1000万步
            max_recursion_depth: Some(5000), // 5000层
            max_duration_ms: Some(300_000),  // 5分钟
            max_memory_bytes: None,
        }
    }
}

/// 执行限制错误
///
/// 当脚本超出配置的资源限制时返回此错误。
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionLimitError {
    /// 步数限制超出
    StepLimitExceeded { steps: usize, limit: usize },

    /// 递归深度限制超出
    RecursionDepthExceeded { depth: usize, limit: usize },

    /// 执行时长超出
    DurationExceeded { duration_ms: u64, limit: u64 },

    /// 内存限制超出（暂未实现）
    MemoryLimitExceeded { bytes: usize, limit: usize },
}

impl fmt::Display for ExecutionLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionLimitError::StepLimitExceeded { steps, limit } => write!(
                f,
                "Execution step limit exceeded: {} steps (limit: {})",
                steps, limit
            ),
            ExecutionLimitError::RecursionDepthExceeded { depth, limit } => write!(
                f,
                "Recursion depth limit exceeded: {} levels (limit: {})",
                depth, limit
            ),
            ExecutionLimitError::DurationExceeded { duration_ms, limit } => write!(
                f,
                "Execution duration limit exceeded: {} ms (limit: {} ms)",
                duration_ms, limit
            ),
            ExecutionLimitError::MemoryLimitExceeded { bytes, limit } => write!(
                f,
                "Memory limit exceeded: {} bytes (limit: {} bytes)",
                bytes, limit
            ),
        }
    }
}

impl std::error::Error for ExecutionLimitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let limits = ExecutionLimits::default();
        assert_eq!(limits.max_steps, Some(1_000_000));
        assert_eq!(limits.max_recursion_depth, Some(1000));
        assert_eq!(limits.max_duration_ms, Some(30_000));
        assert_eq!(limits.max_memory_bytes, None);
    }

    #[test]
    fn test_unrestricted_limits() {
        let limits = ExecutionLimits::unrestricted();
        assert_eq!(limits.max_steps, None);
        assert_eq!(limits.max_recursion_depth, None);
        assert_eq!(limits.max_duration_ms, None);
        assert_eq!(limits.max_memory_bytes, None);
    }

    #[test]
    fn test_strict_limits() {
        let limits = ExecutionLimits::strict();
        assert_eq!(limits.max_steps, Some(100_000));
        assert_eq!(limits.max_recursion_depth, Some(100));
        assert_eq!(limits.max_duration_ms, Some(5_000));
        assert_eq!(limits.max_memory_bytes, None);
    }

    #[test]
    fn test_lenient_limits() {
        let limits = ExecutionLimits::lenient();
        assert_eq!(limits.max_steps, Some(10_000_000));
        assert_eq!(limits.max_recursion_depth, Some(5000));
        assert_eq!(limits.max_duration_ms, Some(300_000));
        assert_eq!(limits.max_memory_bytes, None);
    }

    #[test]
    fn test_error_display() {
        let err = ExecutionLimitError::StepLimitExceeded {
            steps: 1000,
            limit: 100,
        };
        assert!(err.to_string().contains("1000"));
        assert!(err.to_string().contains("100"));

        let err = ExecutionLimitError::RecursionDepthExceeded {
            depth: 500,
            limit: 100,
        };
        assert!(err.to_string().contains("500"));
        assert!(err.to_string().contains("100"));

        let err = ExecutionLimitError::DurationExceeded {
            duration_ms: 5000,
            limit: 1000,
        };
        assert!(err.to_string().contains("5000"));
        assert!(err.to_string().contains("1000"));
    }
}
