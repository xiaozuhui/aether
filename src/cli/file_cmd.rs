use crate::cli::error_context;
use std::fs;

pub fn check_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(code) => {
            use aether::{Lexer, Parser};

            println!("正在检查 '{}'...", filename);

            let mut lexer = Lexer::new(&code);
            let mut token_count = 0;
            loop {
                let token = lexer.next_token();
                token_count += 1;
                if token == aether::Token::EOF {
                    break;
                }
                if let aether::Token::Illegal(ch) = token {
                    eprintln!(
                        "✗ 词法错误: 非法字符 '{}' 在行 {}, 列 {}",
                        ch,
                        lexer.line(),
                        lexer.column()
                    );
                    std::process::exit(1);
                }
            }

            let mut parser = Parser::new(&code);
            match parser.parse_program() {
                Ok(program) => {
                    println!("✓ 语法检查通过");
                    println!("  - {} 个词法单元", token_count);
                    println!("  - {} 条语句", program.len());
                    println!();
                }
                Err(e) => {
                    eprintln!("✗ 语法错误:");
                    error_context::print_detailed_error(&code, &e.to_string());
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ 无法读取文件 '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}

pub fn show_ast_for_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(code) => {
            use aether::Parser;

            let mut parser = Parser::new(&code);
            match parser.parse_program() {
                Ok(program) => {
                    println!("=== 抽象语法树 (AST) ===");
                    println!("文件: {}", filename);
                    println!();
                    println!("{:#?}", program);
                    println!();
                    println!("=== 共 {} 条语句 ===", program.len());
                }
                Err(e) => {
                    eprintln!("✗ 解析错误:");
                    error_context::print_detailed_error(&code, &e.to_string());
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ 无法读取文件 '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}
