// src/stdlib.rs
//! Aether Standard Library
//!
//! 内置的标准库，在编译时嵌入二进制文件中

/// 字符串工具库
pub const STRING_UTILS: &str = include_str!("../stdlib/string_utils.aether");

/// 数组工具库
pub const ARRAY_UTILS: &str = include_str!("../stdlib/array_utils.aether");

/// 数据验证库
pub const VALIDATION: &str = include_str!("../stdlib/validation.aether");

/// 日期时间库
pub const DATETIME: &str = include_str!("../stdlib/datetime.aether");

/// 测试框架
pub const TESTING: &str = include_str!("../stdlib/testing.aether");

/// 所有标准库模块的列表
pub const ALL_MODULES: &[(&str, &str)] = &[
    ("string_utils", STRING_UTILS),
    ("array_utils", ARRAY_UTILS),
    ("validation", VALIDATION),
    ("datetime", DATETIME),
    ("testing", TESTING),
];

/// 获取指定模块的代码
pub fn get_module(name: &str) -> Option<&'static str> {
    match name {
        "string_utils" => Some(STRING_UTILS),
        "array_utils" => Some(ARRAY_UTILS),
        "validation" => Some(VALIDATION),
        "datetime" => Some(DATETIME),
        "testing" => Some(TESTING),
        _ => None,
    }
}

/// 获取所有标准库代码（合并为一个字符串）
pub fn get_all_stdlib() -> String {
    let mut result = String::new();
    result.push_str("// Aether Standard Library - Auto-loaded\n\n");

    for (name, code) in ALL_MODULES {
        result.push_str(&format!("// ========== {} ==========\n", name));
        result.push_str(code);
        result.push_str("\n\n");
    }

    result
}

/// 标准库预加载器
///
/// 用于在 Aether 引擎初始化时加载标准库
pub fn preload_stdlib(engine: &mut crate::Aether) -> Result<(), String> {
    for (name, code) in ALL_MODULES {
        engine
            .eval(code)
            .map_err(|e| format!("Failed to load stdlib module '{}': {}", name, e))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exists() {
        assert!(STRING_UTILS.len() > 0);
        assert!(ARRAY_UTILS.len() > 0);
        assert!(VALIDATION.len() > 0);
        assert!(DATETIME.len() > 0);
        assert!(TESTING.len() > 0);
    }

    #[test]
    fn test_get_module() {
        assert!(get_module("string_utils").is_some());
        assert!(get_module("array_utils").is_some());
        assert!(get_module("unknown").is_none());
    }

    #[test]
    fn test_all_modules_count() {
        assert_eq!(ALL_MODULES.len(), 5);
    }
}
