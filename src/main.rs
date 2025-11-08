use aether::Aether;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // 检查是否有 --stdlib 或 --no-stdlib 标志
        let use_stdlib = if args.contains(&"--no-stdlib".to_string()) {
            false
        } else {
            // 默认加载标准库
            true
        };

        // 脚本模式：运行文件
        let script_file = args
            .iter()
            .find(|arg| !arg.starts_with("--") && *arg != &args[0])
            .map(|s| s.as_str())
            .unwrap_or(&args[1]);

        run_file(script_file, use_stdlib);
    } else {
        // REPL 交互模式
        run_repl();
    }
}

/// 运行 Aether 脚本文件
fn run_file(filename: &str, load_stdlib: bool) {
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
    println!("  Println \"Hello\"   # 打印输出");
    println!();
    println!("数据结构:");
    println!("  [1, 2, 3]         # 数组");
    println!("  {{a: 1, b: 2}}      # 字典");
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
    println!("命令行选项:");
    println!("  aether script.aether     # 运行脚本 (自动加载标准库)");
    println!("  aether --no-stdlib file  # 运行脚本但不加载标准库");
    println!();
}
