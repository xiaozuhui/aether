//! 沙箱配置
//!
//! 提供统一的沙箱配置入口，简化权限和安全管理。

use crate::builtins::IOPermissions;
use std::collections::HashSet;
use std::path::PathBuf;

use super::path_validator::PathRestriction;

/// 沙箱策略类型
#[derive(Debug, Clone, PartialEq)]
pub enum SandboxPolicy {
    /// 完全禁用（默认）
    Disabled,
    /// 仅允许只读访问
    ReadOnly,
    /// 完全访问
    FullAccess,
}

/// 统一的沙箱配置
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// IO 权限（保留向后兼容）
    pub io_permissions: IOPermissions,

    /// 文件系统沙箱策略
    pub filesystem_policy: SandboxPolicy,

    /// 文件系统路径限制
    pub filesystem_restriction: Option<PathRestriction>,

    /// 模块系统沙箱策略
    pub module_policy: SandboxPolicy,

    /// 模块路径限制
    pub module_restriction: Option<PathRestriction>,

    /// 是否启用可观测性指标收集
    pub enable_metrics: bool,

    /// 最大模块缓存数量（0 = 不限制）
    pub max_module_cache_size: usize,

    /// 模块缓存 TTL（秒，0 = 永不过期）
    pub module_cache_ttl_secs: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            io_permissions: IOPermissions::default(),
            filesystem_policy: SandboxPolicy::Disabled,
            filesystem_restriction: None,
            module_policy: SandboxPolicy::Disabled,
            module_restriction: None,
            enable_metrics: false,
            max_module_cache_size: 100,
            module_cache_ttl_secs: 0,
        }
    }
}

impl SandboxConfig {
    /// 创建 DSL 安全默认配置（禁用所有 IO）
    pub fn dsl_safe() -> Self {
        Self::default()
    }

    /// 创建 CLI 完全访问配置
    pub fn cli_full_access() -> Self {
        Self {
            io_permissions: IOPermissions::allow_all(),
            filesystem_policy: SandboxPolicy::FullAccess,
            module_policy: SandboxPolicy::FullAccess,
            enable_metrics: true,
            ..Default::default()
        }
    }

    /// 创建受限沙箱配置（仅允许 root_dir 内访问）
    pub fn sandboxed(root_dir: PathBuf) -> Self {
        // 创建文件扩展名白名单（仅允许 .aether 文件）
        let mut allowed_extensions = HashSet::new();
        allowed_extensions.insert("aether".to_string());

        Self {
            io_permissions: IOPermissions {
                filesystem_enabled: true,
                network_enabled: false,
            },
            filesystem_policy: SandboxPolicy::ReadOnly,
            filesystem_restriction: Some(PathRestriction {
                root_dir: root_dir.clone(),
                allow_absolute: false,
                allow_parent_traversal: false,
                allowed_extensions: None,
            }),
            module_policy: SandboxPolicy::ReadOnly,
            module_restriction: Some(PathRestriction {
                root_dir,
                allow_absolute: false,
                allow_parent_traversal: false,
                allowed_extensions: Some(allowed_extensions),
            }),
            enable_metrics: true,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert_eq!(config.filesystem_policy, SandboxPolicy::Disabled);
        assert_eq!(config.module_policy, SandboxPolicy::Disabled);
        assert!(!config.enable_metrics);
    }

    #[test]
    fn test_sandbox_config_dsl_safe() {
        let config = SandboxConfig::dsl_safe();
        assert_eq!(config.filesystem_policy, SandboxPolicy::Disabled);
        assert_eq!(config.module_policy, SandboxPolicy::Disabled);
        assert!(!config.io_permissions.filesystem_enabled);
        assert!(!config.io_permissions.network_enabled);
    }

    #[test]
    fn test_sandbox_config_cli_full_access() {
        let config = SandboxConfig::cli_full_access();
        assert_eq!(config.filesystem_policy, SandboxPolicy::FullAccess);
        assert_eq!(config.module_policy, SandboxPolicy::FullAccess);
        assert!(config.io_permissions.filesystem_enabled);
        assert!(config.io_permissions.network_enabled);
        assert!(config.enable_metrics);
    }

    #[test]
    fn test_sandbox_config_sandboxed() {
        let root = PathBuf::from("/safe/dir");
        let config = SandboxConfig::sandboxed(root.clone());

        assert_eq!(config.filesystem_policy, SandboxPolicy::ReadOnly);
        assert_eq!(config.module_policy, SandboxPolicy::ReadOnly);
        assert!(config.io_permissions.filesystem_enabled);
        assert!(!config.io_permissions.network_enabled);

        // 检查路径限制
        assert!(config.filesystem_restriction.is_some());
        assert!(config.module_restriction.is_some());

        let fs_restriction = config.filesystem_restriction.as_ref().unwrap();
        assert!(!fs_restriction.allow_absolute);
        assert!(!fs_restriction.allow_parent_traversal);

        let module_restriction = config.module_restriction.as_ref().unwrap();
        assert!(module_restriction.allowed_extensions.is_some());
        let extensions = module_restriction.allowed_extensions.as_ref().unwrap();
        assert!(extensions.contains("aether"));
    }
}
