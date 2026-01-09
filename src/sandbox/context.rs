//! 沙箱上下文（线程局部存储）
//!
//! 使用线程局部存储来传递 PathValidator 给内置函数，避免修改函数签名。

use super::PathValidator;
use std::cell::RefCell;

// 线程局部的沙箱上下文
thread_local! {
    static FILESYSTEM_VALIDATOR: RefCell<Option<PathValidator>> = const { RefCell::new(None) };
}

/// 设置文件系统路径验证器（线程局部）
pub fn set_filesystem_validator(validator: Option<PathValidator>) {
    FILESYSTEM_VALIDATOR.with(|v| *v.borrow_mut() = validator);
}

/// 获取文件系统路径验证器（线程局部）
pub fn get_filesystem_validator() -> Option<PathValidator> {
    FILESYSTEM_VALIDATOR.with(|v| v.borrow().clone())
}

/// 在作用域内设置验证器（RAII 模式）
pub struct ScopedValidator {
    _private: (),
}

impl ScopedValidator {
    /// 设置验证器并在作用域结束时自动恢复
    pub fn set(validator: PathValidator) -> Self {
        set_filesystem_validator(Some(validator));
        Self { _private: () }
    }
}

impl Drop for ScopedValidator {
    fn drop(&mut self) {
        set_filesystem_validator(None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_context() {
        let validator = PathValidator::with_root_dir(std::path::PathBuf::from("/safe"));

        // 设置验证器
        set_filesystem_validator(Some(validator.clone()));

        // 获取验证器
        let retrieved = get_filesystem_validator().unwrap();
        assert_eq!(
            retrieved.restriction().root_dir,
            validator.restriction().root_dir
        );

        // 清除验证器
        set_filesystem_validator(None);
        assert!(get_filesystem_validator().is_none());
    }

    #[test]
    fn test_scoped_validator() {
        assert!(get_filesystem_validator().is_none());

        {
            let validator = PathValidator::with_root_dir(std::path::PathBuf::from("/safe"));
            let _scope = ScopedValidator::set(validator);

            // 在作用域内验证器存在
            assert!(get_filesystem_validator().is_some());
        }

        // 作用域结束后验证器自动清除
        assert!(get_filesystem_validator().is_none());
    }
}
