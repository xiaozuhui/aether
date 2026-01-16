pub fn print_cli_help() {
    println!("Aether 语言解释器 v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("用法:");
    println!("  aether [选项] <脚本文件>");
    println!("  aether                    # 启动 REPL 交互模式");
    println!();
    println!("选项:");
    println!("  -h, --help               显示此帮助信息");
    println!("  --check                  只检查语法，不执行代码");
    println!("  --ast                    显示抽象语法树 (AST)");
    println!("  --debug                  启用调试模式（打印额外运行信息）");
    println!("  --debugger               启动交互式调试器 (类似GDB)");
    println!("  --metrics                执行后打印性能指标（耗时/缓存/trace 统计）");
    println!("  --metrics-json           以 JSON 输出结果 + 性能指标（机器可读）");
    println!("  --metrics-json-pretty    以格式化 JSON 输出结果 + 性能指标（机器可读）");
    println!("  --no-stdlib              不自动加载标准库");
    println!("  --json-error             出错时输出结构化 JSON 错误（写到 stderr）");
    println!("  --trace                  执行后打印 TRACE 缓冲区内容");
    println!("  --trace-stats            执行后打印 TRACE 统计信息");
    println!("  --trace-buffer-size <N>  设置 TRACE 缓冲区容量（条目数）");
    println!();
    println!("示例:");
    println!("  aether script.aether                                   # 运行脚本");
    println!("  aether --check script.aether                           # 检查语法");
    println!("  aether --ast script.aether                             # 查看 AST");
    println!("  aether --debug script.aether                           # 调试模式运行");
    println!("  aether --debugger script.aether                        # 启动调试器");
    println!("  aether --metrics script.aether                         # 运行并打印性能指标");
    println!(
        "  aether --metrics-json script.aether                    # JSON 输出（含结果与指标）"
    );
    println!("  aether --metrics-json-pretty script.aether             # 格式化 JSON 输出（含结果与指标）");
    println!("  aether --trace script.aether                           # 运行并打印 TRACE");
    println!("  aether --trace --trace-stats script.aether             # 运行并打印 TRACE + 统计");
    println!("  aether --trace-buffer-size 4096 --trace script.aether  # 调大缓冲区后打印 TRACE");
    println!("  aether --no-stdlib script.aether                       # 不加载标准库");
    println!();
}
