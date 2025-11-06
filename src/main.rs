use aether::Aether;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // 脚本模式：运行文件
        run_file(&args[1]);
    } else {
        // REPL 交互模式
        run_repl();
    }
}

/// 运行 Aether 脚本文件
fn run_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(code) => {
            // 作为独立语言使用时，默认启用所有IO权限
            let mut engine = Aether::with_all_permissions();
            match engine.eval(&code) {
                Ok(result) => {
                    // 只在有显式输出时打印
                    if result != aether::Value::Null {
                        println!("{}", result);
                    }
                }
                Err(e) => {
                    eprintln!("错误: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("无法读取文件 '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}

/// 启动 REPL 交互模式
fn run_repl() {
    println!("Aether REPL v0.1.0");
    println!("输入 'exit' 或 'quit' 退出");
    println!("输入 'help' 查看帮助");
    println!();

    // REPL模式也默认启用所有IO权限
    let mut engine = Aether::with_all_permissions();
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
                        eprintln!("错误: {}", e);
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
    println!("  Print \"Hello\"     # 打印输出");
    println!();
    println!("数据结构:");
    println!("  [1, 2, 3]         # 数组");
    println!("  {{a: 1, b: 2}}      # 字典");
    println!();
    println!("控制流:");
    println!("  If (X > 0) {{      # 条件判断");
    println!("    Print \"正数\"");
    println!("  }}");
    println!();
    println!("  For I In Range 5 {{  # 循环");
    println!("    Print I");
    println!("  }}");
    println!();
    println!("内置函数:");
    println!("  Len [1,2,3]       # 长度");
    println!("  Map F [1,2,3]     # 映射");
    println!("  Filter F [1,2,3]  # 过滤");
    println!("  Sum [1,2,3]       # 求和");
    println!();
    println!("数学函数:");
    println!("  Sin 1.57          # 正弦");
    println!("  Sqrt 16           # 平方根");
    println!("  Pow 2 10          # 幂运算");
    println!();
    println!("统计函数:");
    println!("  Mean [1,2,3,4]    # 平均值");
    println!("  StdDev [1,2,3,4]  # 标准差");
    println!("  LinearRegression [[1,2],[2,4],[3,6]]  # 线性回归");
    println!();
    println!("REPL 命令:");
    println!("  help              # 显示此帮助");
    println!("  exit, quit        # 退出 REPL");
    println!();
}
