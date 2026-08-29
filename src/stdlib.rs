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

/// 集合（Set）数据结构
pub const SET: &str = include_str!("../stdlib/set.aether");

/// 队列（Queue）数据结构
pub const QUEUE: &str = include_str!("../stdlib/queue.aether");

/// 栈（Stack）数据结构
pub const STACK: &str = include_str!("../stdlib/stack.aether");

/// 堆（Heap）数据结构
pub const HEAP: &str = include_str!("../stdlib/heap.aether");

/// 排序算法
pub const SORTING: &str = include_str!("../stdlib/sorting.aether");

/// JSON 处理工具
pub const JSON: &str = include_str!("../stdlib/json.aether");

/// CSV 数据处理
pub const CSV: &str = include_str!("../stdlib/csv.aether");

/// 函数式编程工具
pub const FUNCTIONAL: &str = include_str!("../stdlib/functional.aether");

/// CLI 工具库
pub const CLI_UTILS: &str = include_str!("../stdlib/cli_utils.aether");

/// 文本模板引擎
pub const TEXT_TEMPLATE: &str = include_str!("../stdlib/text_template.aether");

/// 正则风格文本处理
pub const REGEX_UTILS: &str = include_str!("../stdlib/regex_utils.aether");

/// 所有标准库模块的列表
pub const ALL_MODULES: &[(&str, &str)] = &[
    ("string_utils", STRING_UTILS),
    ("array_utils", ARRAY_UTILS),
    ("validation", VALIDATION),
    ("datetime", DATETIME),
    ("testing", TESTING),
    ("set", SET),
    ("queue", QUEUE),
    ("stack", STACK),
    ("heap", HEAP),
    ("sorting", SORTING),
    ("json", JSON),
    ("csv", CSV),
    ("functional", FUNCTIONAL),
    ("cli_utils", CLI_UTILS),
    ("text_template", TEXT_TEMPLATE),
    ("regex_utils", REGEX_UTILS),
];

/// 获取指定模块的代码（单一数据源：`ALL_MODULES` 查表）
pub fn get_module(name: &str) -> Option<&'static str> {
    ALL_MODULES
        .iter()
        .find(|(module_name, _)| *module_name == name)
        .map(|(_, code)| *code)
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
