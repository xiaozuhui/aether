use aether::Aether;
use std::env;
use std::fs;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // 检查各种命令行标志
        let use_stdlib = !args.contains(&"--no-stdlib".to_string());
        let show_ast = args.contains(&"--ast".to_string());
        let check_only = args.contains(&"--check".to_string());
        let debug_mode = args.contains(&"--debug".to_string());
        let ouroboros = args.contains(&"--ouroboros".to_string())
            || args.contains(&"--py-to-aether".to_string());
        let run_after = args.contains(&"--run".to_string());
        let show_help = args.contains(&"--help".to_string()) || args.contains(&"-h".to_string());

        if show_help {
            print_cli_help();
            return;
        }

        if run_after && !ouroboros {
            eprintln!("错误: --run 只能与 --ouroboros 一起使用（用于 Python → Aether 转译后运行）");
            eprintln!("提示: 运行 Aether 脚本请直接使用: aether <脚本文件>");
            eprintln!("使用 --help 查看帮助");
            std::process::exit(2);
        }

        // 获取脚本文件名
        // - 默认：第一个非 flag 参数
        // - ouroboros：允许用 "-" 表示 stdin
        let script_file = args.iter().skip(1).find(|arg| {
            if arg.starts_with("--") {
                return false;
            }
            if ouroboros && arg.as_str() == "-" {
                return true;
            }
            !arg.starts_with('-')
        });

        if let Some(file) = script_file {
            let file = file.as_str();
            if ouroboros {
                ouroboros_transpile_python(file, run_after, use_stdlib, debug_mode);
            } else if check_only {
                check_file(file);
            } else if show_ast {
                show_ast_for_file(file);
            } else {
                run_file(file, use_stdlib, debug_mode);
            }
        } else {
            eprintln!("错误: 未指定脚本文件");
            eprintln!("使用 --help 查看帮助");
            std::process::exit(1);
        }
    } else {
        // REPL 交互模式
        run_repl();
    }
}

/// 打印命令行帮助
fn print_cli_help() {
    println!("Aether 语言解释器 v0.2.0");
    println!();
    println!("用法:");
    println!("  aether [选项] <脚本文件>");
    println!("  aether                    # 启动 REPL 交互模式");
    println!();
    println!("选项:");
    println!("  -h, --help               显示此帮助信息");
    println!("  --check                  只检查语法，不执行代码");
    println!("  --ast                    显示抽象语法树 (AST)");
    println!("  --debug                  启用调试模式（显示求值过程）");
    println!("  --no-stdlib              不自动加载标准库");
    println!("  --ouroboros              将 Python 代码转译为 Aether（输出到 stdout）");
    println!("  --run                    配合 --ouroboros：转译后直接运行");
    println!();
    println!("示例:");
    println!("  aether script.aether              # 运行脚本");
    println!("  aether --check script.aether      # 检查语法");
    println!("  aether --ast script.aether        # 查看 AST");
    println!("  aether --debug script.aether      # 调试模式运行");
    println!("  aether --no-stdlib script.aether  # 不加载标准库");
    println!("  aether --ouroboros demo.py > demo.aether      # Python → Aether");
    println!("  aether --ouroboros --run demo.py              # 转译后直接运行");
    println!("  cat demo.py | aether --ouroboros - > demo.aether  # 从 stdin 转译");
    println!();
}

/// Ouroboros：将 Python 源码转译为 Aether。
///
/// - filename = "-" 时从 stdin 读取
/// - 默认输出转译后的 Aether 到 stdout
/// - 若 run_after=true，则转译后直接执行（stdout 输出执行结果/脚本输出）
/// - 诊断信息输出到 stderr；如果有错误则退出码=1
fn ouroboros_transpile_python(
    filename: &str,
    run_after: bool,
    load_stdlib: bool,
    debug_mode: bool,
) {
    use aether::pytranspile::{Severity, TranspileOptions, python_to_aether};

    let source = if filename == "-" {
        let mut buf = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buf) {
            eprintln!("✗ 无法从 stdin 读取: {}", e);
            std::process::exit(1);
        }
        buf
    } else {
        match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("✗ 无法读取文件 '{}': {}", filename, e);
                std::process::exit(1);
            }
        }
    };

    let opts = TranspileOptions::default();
    let res = python_to_aether(&source, &opts);

    // 打印诊断信息（warning 也会打印）。
    for d in &res.diagnostics.0 {
        let level = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        if d.span.line > 0 {
            eprintln!(
                "[{level}] {}: {} (line {}, col {})",
                d.code, d.message, d.span.line, d.span.col
            );
        } else {
            eprintln!("[{level}] {}: {}", d.code, d.message);
        }
    }

    if res.diagnostics.has_errors() || res.aether.is_none() {
        std::process::exit(1);
    }

    let code = res.aether.unwrap();

    if !run_after {
        print!("{}", code);
        if !code.ends_with('\n') {
            println!();
        }
        return;
    }

    // 转译后直接运行：行为尽量与 run_file 对齐。
    let mut engine = if load_stdlib {
        match Aether::with_stdlib() {
            Ok(engine) => engine,
            Err(e) => {
                eprintln!("警告: 标准库加载失败: {}", e);
                eprintln!("继续运行但不加载标准库...");
                Aether::with_all_permissions()
            }
        }
    } else {
        Aether::with_all_permissions()
    };

    if debug_mode {
        println!("=== Ouroboros 调试模式 ===");
        println!("Python 输入: {}", filename);
        println!(
            "标准库: {}",
            if load_stdlib {
                "已加载"
            } else {
                "未加载"
            }
        );
        println!();
    }

    match engine.eval(&code) {
        Ok(result) => {
            if debug_mode {
                println!("=== 执行结果 ===");
            }
            if result != aether::Value::Null {
                println!("{}", result);
            }
            if debug_mode {
                println!("\n=== 执行完成 ===");
            }
        }
        Err(e) => {
            eprintln!("✗ 运行时错误:");
            print_detailed_error(&code, &e.to_string());
            std::process::exit(1);
        }
    }
}

/// 检查文件语法
fn check_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(code) => {
            use aether::{Lexer, Parser};

            println!("正在检查 '{}'...", filename);

            // 词法分析
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

            // 语法分析
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
                    print_detailed_error(&code, &e.to_string());
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

/// 显示文件的 AST
fn show_ast_for_file(filename: &str) {
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
                    print_detailed_error(&code, &e.to_string());
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

/// 运行 Aether 脚本文件
fn run_file(filename: &str, load_stdlib: bool, debug_mode: bool) {
    match fs::read_to_string(filename) {
        Ok(code) => {
            // 作为独立语言使用时，默认启用所有IO权限
            let mut engine = if load_stdlib {
                match Aether::with_stdlib() {
                    Ok(engine) => engine,
                    Err(e) => {
                        eprintln!("警告: 标准库加载失败: {}", e);
                        eprintln!("继续运行但不加载标准库...");
                        Aether::with_all_permissions()
                    }
                }
            } else {
                Aether::with_all_permissions()
            };

            if debug_mode {
                println!("=== 调试模式 ===");
                println!("文件: {}", filename);
                println!(
                    "标准库: {}",
                    if load_stdlib {
                        "已加载"
                    } else {
                        "未加载"
                    }
                );
                println!();
            }

            match engine.eval(&code) {
                Ok(result) => {
                    if debug_mode {
                        println!("=== 执行结果 ===");
                    }
                    // 只在有显式输出时打印
                    if result != aether::Value::Null {
                        println!("{}", result);
                    }
                    if debug_mode {
                        println!("\n=== 执行完成 ===");
                    }
                }
                Err(e) => {
                    eprintln!("✗ 运行时错误:");
                    print_detailed_error(&code, &e.to_string());
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

/// 打印详细的错误信息，包含源代码上下文
fn print_detailed_error(source: &str, error_msg: &str) {
    eprintln!("{}", error_msg);

    // 尝试提取行号和列号
    if let Some(line_col) = extract_line_column(error_msg) {
        let (line, col) = line_col;
        print_source_context(source, line, col);
    }
}

/// 从错误消息中提取行号和列号
fn extract_line_column(error_msg: &str) -> Option<(usize, usize)> {
    // 查找 "line X, column Y" 模式
    if let Some(line_start) = error_msg.find("line ")
        && let Some(line_end) = error_msg[line_start..].find(',')
    {
        let line_str = &error_msg[line_start + 5..line_start + line_end];
        if let Ok(line) = line_str.trim().parse::<usize>()
            && let Some(col_start) = error_msg.find("column ")
        {
            let col_str = &error_msg[col_start + 7..];
            // 找到第一个非数字字符
            let col_end = col_str
                .find(|c: char| !c.is_numeric())
                .unwrap_or(col_str.len());
            if let Ok(col) = col_str[..col_end].trim().parse::<usize>() {
                return Some((line, col));
            }
        }
    }
    None
}

/// 打印源代码上下文
fn print_source_context(source: &str, error_line: usize, error_col: usize) {
    let lines: Vec<&str> = source.lines().collect();

    if error_line == 0 || error_line > lines.len() {
        return;
    }

    eprintln!();
    eprintln!("源代码位置:");

    // 显示前一行（如果存在）
    if error_line > 1 {
        eprintln!("{:4} | {}", error_line - 1, lines[error_line - 2]);
    }

    // 显示错误行
    eprintln!("{:4} | {}", error_line, lines[error_line - 1]);

    // 显示错误位置指示器
    let indent = format!("{:4} | ", error_line);
    let pointer = " ".repeat(error_col.saturating_sub(1)) + "^";
    eprintln!("{}{}", indent, pointer);

    // 显示后一行（如果存在）
    if error_line < lines.len() {
        eprintln!("{:4} | {}", error_line + 1, lines[error_line]);
    }
    eprintln!();
}

/// 启动 REPL 交互模式
fn run_repl() {
    println!("Aether REPL v0.1.0");
    println!("输入 'exit' 或 'quit' 退出");
    println!("输入 'help' 查看帮助");
    println!("输入 ':load stdlib' 加载标准库");
    println!();

    // REPL模式也默认启用所有IO权限
    let mut engine = Aether::with_all_permissions();
    let mut stdlib_loaded = false;
    let mut line_number = 1;

    loop {
        print!("aether[{}]> ", line_number);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                // 处理特殊命令
                match input {
                    "exit" | "quit" => {
                        println!("再见！");
                        break;
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    ":load stdlib" => {
                        if stdlib_loaded {
                            println!("标准库已经加载过了");
                        } else {
                            match engine.load_all_stdlib() {
                                Ok(_) => {
                                    println!("✓ 标准库加载成功");
                                    stdlib_loaded = true;
                                }
                                Err(e) => {
                                    eprintln!("✗ 标准库加载失败: {}", e);
                                }
                            }
                        }
                        continue;
                    }
                    cmd if cmd.starts_with(":load ") => {
                        let module = cmd.strip_prefix(":load ").unwrap().trim();
                        match engine.load_stdlib_module(module) {
                            Ok(_) => println!("✓ 模块 '{}' 加载成功", module),
                            Err(e) => eprintln!("✗ 模块加载失败: {}", e),
                        }
                        continue;
                    }
                    "" => continue,
                    _ => {}
                }

                // 执行代码
                match engine.eval(input) {
                    Ok(result) => {
                        if result != aether::Value::Null {
                            println!("{}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("✗ {}", e);
                        // 在 REPL 中也显示详细错误信息
                        if let Some((line, col)) = extract_line_column(&e.to_string()) {
                            print_source_context(input, line, col);
                        }
                    }
                }

                line_number += 1;
            }
            Err(e) => {
                eprintln!("读取输入错误: {}", e);
                break;
            }
        }
    }
}

/// 打印帮助信息
fn print_help() {
    println!("Aether 语言帮助:");
    println!();
    println!("基本语法:");
    println!("  Set X 10          # 定义变量");
    println!("  (X + 5)           # 表达式求值");
    println!("  Println \"Hello\"   # 打印输出");
    println!();
    println!("Lambda 表达式:");
    println!("  Lambda X -> X * 2           # 单参数");
    println!("  Lambda (X, Y) -> X + Y      # 多参数");
    println!("  Func(X) {{ Return (X * 2) }}  # 块语法");
    println!();
    println!("多行字符串:");
    println!("  \"\"\"多行");
    println!("  字符串");
    println!("  内容\"\"\"");
    println!();
    println!("数据结构:");
    println!("  [1, 2, 3]         # 数组");
    println!("  [[1, 2], [3, 4]]  # 嵌套数组");
    println!("  {{a: 1, b: 2}}      # 字典");
    println!("  {{a: {{b: 1}}}}       # 嵌套字典");
    println!();
    println!("控制流:");
    println!("  If (X > 0) {{      # 条件判断");
    println!("    Println \"正数\"");
    println!("  }}");
    println!();
    println!("  While (I < 10) {{ # 循环");
    println!("    Set I (I + 1)");
    println!("  }}");
    println!();
    println!("标准库 (使用 :load stdlib 加载):");
    println!("  STR_TRIM(str)            # 字符串修剪");
    println!("  ARR_UNIQUE(arr)          # 数组去重");
    println!("  SET_FROM_ARRAY(arr)      # 创建集合");
    println!("  QUEUE_NEW()              # 创建队列");
    println!("  STACK_NEW()              # 创建栈");
    println!("  MIN_HEAP_NEW()           # 创建最小堆");
    println!("  QUICK_SORT(arr)          # 快速排序");
    println!();
    println!("REPL 命令:");
    println!("  help                     # 显示此帮助");
    println!("  :load stdlib             # 加载所有标准库");
    println!("  :load string_utils       # 加载字符串工具库");
    println!("  :load array_utils        # 加载数组工具库");
    println!("  :load set                # 加载集合库");
    println!("  :load queue              # 加载队列库");
    println!("  :load stack              # 加载栈库");
    println!("  :load heap               # 加载堆库");
    println!("  :load sorting            # 加载排序算法库");
    println!("  :load validation         # 加载验证库");
    println!("  :load datetime           # 加载日期时间库");
    println!("  :load testing            # 加载测试框架");
    println!("  exit, quit               # 退出 REPL");
    println!();
}
