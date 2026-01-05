extern crate cbindgen;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir).join("bindings").join("aether.h");

    // Create bindings directory if it doesn't exist
    std::fs::create_dir_all(PathBuf::from(&crate_dir).join("bindings")).ok();

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_cpp_compat(true)
        .with_include_guard("AETHER_H")
        .with_documentation(true)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(output_file);

    println!("cargo:rerun-if-changed=src/ffi.rs");

    // 验证标准库文件
    validate_stdlib(&crate_dir);
}

/// 验证标准库文件的语法正确性
fn validate_stdlib(crate_dir: &str) {
    println!("cargo:warning=检查所有内置标准库...");

    let stdlib_dir = PathBuf::from(crate_dir).join("stdlib");

    if !stdlib_dir.exists() {
        println!(
            "cargo:warning=Standard library directory not found: {:?}",
            stdlib_dir
        );
        return;
    }

    // 读取所有 .aether 文件
    let mut file_count = 0;
    let mut aether_files = Vec::new();

    match fs::read_dir(&stdlib_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();

                // 只检查 .aether 文件，跳过 examples 和 README
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("aether") {
                    file_count += 1;

                    // 告诉 Cargo 监控这个文件的变化
                    println!("cargo:rerun-if-changed={}", path.display());

                    aether_files.push(path);
                }
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to read stdlib directory: {}", e);
            return;
        }
    }

    if file_count == 0 {
        println!("cargo:warning=No .aether files found in stdlib directory");
        return;
    }

    // 简单的语法检查：确保文件可以被读取，并进行基本的语法模式检查
    let mut all_valid = true;

    for path in &aether_files {
        match validate_aether_file_basic(path) {
            Ok(()) => {
                println!(
                    "cargo:warning=✓ {}",
                    path.file_name().unwrap().to_string_lossy()
                );
            }
            Err(e) => {
                println!(
                    "cargo:warning=✗ {}: {}",
                    path.file_name().unwrap().to_string_lossy(),
                    e
                );
                all_valid = false;
            }
        }
    }

    if all_valid {
        println!("cargo:warning=共 {} 个标准库文件检查成功！", file_count);
    } else {
        println!("cargo:warning=ERROR: Some standard library files have potential issues!");
        println!("cargo:warning=Run 'cargo test' to verify standard library functionality.");
    }
}

/// 对 Aether 文件进行基本的语法检查
fn validate_aether_file_basic(path: &PathBuf) -> Result<(), String> {
    // 读取文件内容
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    if content.is_empty() {
        return Err("File is empty".to_string());
    }

    // 基本的语法模式检查 - 跳过字符串内的内容
    let lines: Vec<&str> = content.lines().collect();
    let mut has_unclosed_braces = 0;
    let mut has_unclosed_brackets = 0;
    let mut has_unclosed_parens = 0;

    for (line_num, line) in lines.iter().enumerate() {
        let line = line.trim();

        // 跳过注释和空行
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }

        // 检查括号匹配 - 忽略字符串内的字符
        let mut in_string = false;
        let mut escape_next = false;
        let mut in_multiline_string = false;

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            // 检查多行字符串开始 """
            if !in_string
                && i + 2 < chars.len()
                && chars[i] == '"'
                && chars[i + 1] == '"'
                && chars[i + 2] == '"'
            {
                in_multiline_string = !in_multiline_string;
                i += 3;
                continue;
            }

            if in_multiline_string {
                i += 1;
                continue;
            }

            if escape_next {
                escape_next = false;
                i += 1;
                continue;
            }

            match ch {
                '\\' if in_string => {
                    escape_next = true;
                }
                '"' => {
                    in_string = !in_string;
                }
                '{' if !in_string => has_unclosed_braces += 1,
                '}' if !in_string => has_unclosed_braces -= 1,
                '[' if !in_string => has_unclosed_brackets += 1,
                ']' if !in_string => has_unclosed_brackets -= 1,
                '(' if !in_string => has_unclosed_parens += 1,
                ')' if !in_string => has_unclosed_parens -= 1,
                _ => {}
            }

            i += 1;
        }

        // 检查是否有负数的括号（关闭多于打开）
        if has_unclosed_braces < 0 {
            return Err(format!(
                "Line {}: Unmatched closing brace '}}' ",
                line_num + 1
            ));
        }
        if has_unclosed_brackets < 0 {
            return Err(format!(
                "Line {}: Unmatched closing bracket ']'",
                line_num + 1
            ));
        }
        if has_unclosed_parens < 0 {
            return Err(format!(
                "Line {}: Unmatched closing parenthesis ')'",
                line_num + 1
            ));
        }
    }

    // 检查是否有未关闭的括号
    if has_unclosed_braces > 0 {
        return Err(format!(
            "Unclosed braces: {} '{{' without matching '}}'",
            has_unclosed_braces
        ));
    }
    if has_unclosed_brackets > 0 {
        return Err(format!(
            "Unclosed brackets: {} '[' without matching ']'",
            has_unclosed_brackets
        ));
    }
    if has_unclosed_parens > 0 {
        return Err(format!(
            "Unclosed parentheses: {} '(' without matching ')'",
            has_unclosed_parens
        ));
    }

    Ok(())
}
