use crate::cli::error_context;
use aether::Aether;
use std::io::{self, Write};

pub fn run_repl() {
    println!("Aether REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("输入 'exit' 或 'quit' 退出");
    println!("输入 'help' 查看帮助");
    println!("输入 ':load stdlib' 加载标准库");
    println!();

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

                match engine.eval(input) {
                    Ok(result) => {
                        if result != aether::Value::Null {
                            println!("{}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("✗ {}", e);
                        if let Some((line, col)) = error_context::extract_line_column(&e.to_string()) {
                            error_context::print_source_context(input, line, col);
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
