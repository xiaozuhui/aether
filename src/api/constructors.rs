use super::Aether;
use crate::builtins::IOPermissions;
use crate::evaluator::Evaluator;
use crate::optimizer::Optimizer;
use crate::stdlib;

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
            cache: crate::cache::ASTCache::new(),
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
}

impl Default for Aether {
    fn default() -> Self {
        Self::new()
    }
}
