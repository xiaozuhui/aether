//! BDD 规格：文档 / 示例 / 帮助与实现的一致性（防漂移）。
//!
//! 功能点与边界（已冻结）：
//! 1. **HELP() 覆盖全部注册函数**：内置函数列表不得漏掉任何已注册
//!    名字（旧实现只覆盖约 46/201，缺 MAP/FILTER/REDUCE/JOIN 和全部
//!    payroll 函数）。
//! 2. **rustdoc 中的 ```aether 代码块必须可解析**：文档示例使用真实
//!    语法（UPPER_SNAKE 标识符、// 注释、带括号调用）。旧文档大量
//!    使用 `#` 注释和驼峰命名——语言根本不支持，纯属误导。
//! 3. **examples/ 下全部脚本可解析；不含 INPUT 的脚本可完整运行**
//!    （旧有 4 个示例因语法漂移无法运行）。
//! 4. **PAYROLL_GUIDE 中反引号引用的内置函数名必须真实注册**
//!    （旧文档引用了 4 个不存在的函数）。

use aether::{Aether, BuiltInRegistry, FileSystemModuleResolver, IOPermissions, Parser, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// HELP() 的输出必须包含每一个已注册的内置函数名。
#[test]
fn help_lists_every_registered_function() {
    // 与 Aether::new() 相同的权限配置（默认禁用 IO），保证名字集合一致
    let registry = BuiltInRegistry::new();
    let names = registry.names();
    assert!(!names.is_empty(), "注册表不应为空（测试前提）");

    let mut engine = Aether::new();
    let help = match engine.eval("HELP()") {
        Ok(v) => v.to_string(),
        Err(e) => panic!("HELP() 求值失败: {e}"),
    };
    for name in names {
        assert!(help.contains(&name), "HELP() 输出缺少已注册函数 {name}");
    }
}

/// 从 Rust 源码中提取 ```aether 围栏代码块（剥离 /// 前缀）。
fn extract_aether_blocks(source: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut inside = false;
    for raw in source.lines() {
        let trimmed = raw.trim_start();
        let stripped = trimmed
            .strip_prefix("///")
            .map(|s| s.trim_start())
            .unwrap_or(raw);
        if !inside {
            if stripped.starts_with("```aether") {
                inside = true;
                current.clear();
            }
        } else if stripped.trim_start().starts_with("```") {
            blocks.push(current.join("\n"));
            inside = false;
        } else {
            current.push(stripped.to_string());
        }
    }
    blocks
}

/// 递归收集 src/ 下的全部 .rs 文件。
fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// rustdoc 中的所有 ```aether 示例块必须能被真实解析器解析。
#[test]
fn rustdoc_aether_snippets_parse() {
    let mut files = Vec::new();
    collect_rs_files(Path::new("src"), &mut files);
    assert!(!files.is_empty(), "应能找到 src/ 下的源文件（测试前提）");

    let mut checked = 0usize;
    for file in &files {
        let source = fs::read_to_string(file).expect("读取源文件失败");
        for block in extract_aether_blocks(&source) {
            let result = Parser::new(&block).parse_program();
            assert!(
                result.is_ok(),
                "文档示例无法解析：{} 中的块:\n{block}\n错误: {:?}",
                file.display(),
                result.err()
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "应提取到至少一个 ```aether 代码块（测试前提）");
}

/// 列出某目录下（不递归）的全部 .aether 文件。
fn aether_files(dir: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).expect("目录应存在").flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "aether") {
            out.push(path);
        }
    }
    out.sort();
    out
}

/// 以 CLI 等价配置运行一个示例脚本（全 IO 权限 + 预载标准库；
/// 含 Import 的脚本装配文件系统解析器）。
fn run_example(path: &Path) -> Result<Value, String> {
    let src = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut engine = Aether::with_all_permissions();
    engine
        .load_all_stdlib()
        .map_err(|e| format!("预载标准库失败: {e}"))?;
    if src.contains("Import") {
        engine.set_module_resolver(Box::new(FileSystemModuleResolver::default()));
    }
    engine.eval_file(path)
}

/// examples/ 下全部脚本可解析；不含 INPUT 的脚本可完整运行。
#[test]
fn examples_parse_and_run() {
    let files = aether_files("examples");
    assert!(!files.is_empty(), "examples/ 应有 .aether 文件（测试前提）");

    for path in &files {
        let src = fs::read_to_string(path).expect("读取示例失败");
        // 1) 语法层：每个示例都必须能通过真实解析器
        let parsed = Parser::new(&src).parse_program();
        assert!(
            parsed.is_ok(),
            "示例无法解析: {}\n错误: {:?}",
            path.display(),
            parsed.err()
        );
        // 2) 执行层：不依赖交互输入（INPUT）的示例必须能完整运行
        if !src.contains("INPUT(") {
            let run = run_example(path);
            assert!(
                run.is_ok(),
                "示例运行失败: {} => {:?}",
                path.display(),
                run.err()
            );
        }
    }
}

/// 从 Markdown 中提取反引号包裹的 UPPER_SNAKE 词元。
fn backtick_upper_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find('`') {
        let after = &rest[start + 1..];
        if let Some(end) = after.find('`') {
            let token = &after[..end];
            let ok = token.len() >= 4
                && token.starts_with(|c: char| c.is_ascii_uppercase())
                && token
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
            if ok {
                tokens.push(token.to_string());
            }
            rest = &after[end + 1..];
        } else {
            break;
        }
    }
    tokens
}

/// PAYROLL_GUIDE 中反引号引用的内置函数名必须真实注册。
#[test]
fn payroll_guide_references_only_registered_functions() {
    let guide = fs::read_to_string("docs/PAYROLL_GUIDE.md").expect("读取指南失败");
    let tokens = backtick_upper_tokens(&guide);
    assert!(!tokens.is_empty(), "指南应包含反引号引用（测试前提）");

    // 用全权限注册表对账（文件类函数仅在启用时注册）
    let registry = BuiltInRegistry::with_permissions(IOPermissions::allow_all());
    let names = registry.names();

    for token in tokens {
        assert!(
            names.contains(&token),
            "PAYROLL_GUIDE 引用了未注册的函数 {token}"
        );
    }
}
