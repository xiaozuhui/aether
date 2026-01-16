# Aether 调试工具和错误增强指南

## 新增的调试工具

### 1. 语法检查 (`--check`)

只检查代码的语法正确性，不执行代码。

```bash
aether --check script.aether
```

输出示例：

```
正在检查 'script.aether'...
✓ 语法检查通过
  - 45 个词法单元
  - 6 条语句
```

如果有错误，会显示详细的错误信息和源代码上下文。

### 2. AST 查看 (`--ast`)

显示代码的抽象语法树（AST），帮助理解代码的结构。

```bash
aether --ast script.aether
```

输出示例：

```
=== 抽象语法树 (AST) ===
文件: script.aether

[
    Set {
        name: "X",
        value: Number(10.0),
    },
    Set {
        name: "DOUBLE",
        value: Lambda {
            params: ["X"],
            body: [
                Return(
                    Binary {
                        left: Identifier("X"),
                        op: Multiply,
                        right: Number(2.0),
                    },
                ),
            ],
        },
    },
    ...
]

=== 共 6 条语句 ===
```

### 3. 调试模式 (`--debug`)

在调试模式下运行脚本，显示额外的执行信息。

```bash
aether --debug script.aether
```

输出示例：

```
=== 调试模式 ===
文件: script.aether
标准库: 已加载

=== 执行结果 ===
60

=== 执行完成 ===
```

说明：`--debug` 目前主要用于打印“运行元信息”（文件名、是否加载标准库、结果分段等），不会逐步打印每条语句或变量变化。

### 3.1 打印 TRACE 缓冲区 (`--trace`)

如果脚本中使用了 `TRACE(...)`（用于 DSL 安全调试的内存 trace），你可以用 `--trace` 在执行结束后把缓冲区内容打印出来：

```bash
aether --trace script.aether
```

输出示例：

```
=== 执行结果 ===
...

=== TRACE ===
#1 123
#2 [demo] hello
```

### 3.2 打印 TRACE 统计信息 (`--trace-stats`)

执行结束后输出 TRACE 的统计信息（缓冲区大小、是否丢弃过最老记录、按级别/类别计数）：

```bash
aether --trace-stats script.aether
```

输出示例：

```
=== TRACE STATS ===
buffer_size: 1024
total_entries: 12
buffer_full: false
by_level: {Info: 10, Warn: 2}
by_category: {"api": 5, "demo": 7}
```

说明：`--trace-stats` 统计的是结构化 TRACE（`TRACE_DEBUG/TRACE_INFO/TRACE_WARN/TRACE_ERROR`）。
如果你的脚本只使用 `TRACE(...)`，它会出现在 `--trace` 输出中，但不会计入按级别/类别的统计。

### 3.3 设置 TRACE 缓冲区大小 (`--trace-buffer-size <N>`)

默认 TRACE 缓冲区容量为 1024 条。如果你需要在一次运行中保留更多/更少的 trace，可调整容量：

```bash
aether --trace-buffer-size 4096 --trace script.aether
```

注意：当缓冲区满时，会自动丢弃最旧的记录。

### 4. 帮助信息 (`--help` 或 `-h`)

显示完整的命令行帮助。

```bash
aether --help
```

## 增强的错误信息

### 语法错误显示

当遇到语法错误时，Aether 现在会显示：

1. 错误描述
2. 错误所在的行号和列号
3. 源代码上下文（错误行及其前后各一行）
4. 错误位置的视觉指示器

示例：

```
✗ 语法错误:
Parse error at line 13, column 2: Expected RightParen, found Newline

源代码位置:
  12 | // 这里故意制造一个错误 - 缺少右括号
  13 | Set RESULT (X + Y
       | ^
  14 | 
```

### 运行时错误显示

运行时错误也会显示类似的上下文信息：

```
✗ 运行时错误:
Runtime error: Undefined variable: UNKNOWN_VAR

源代码位置:
   5 | Set X 10
   6 | Set Y (X + UNKNOWN_VAR)
       |            ^
   7 | PRINTLN(Y)
```

## 在构建时验证标准库

Aether 现在在编译时自动验证所有标准库文件的语法：

```
warning: aether-azathoth@0.2.0: 检查所有内置标准库...
warning: aether-azathoth@0.2.0: ✓ sorting.aether
warning: aether-azathoth@0.2.0: ✓ cli_utils.aether
warning: aether-azathoth@0.2.0: ✓ queue.aether
...
warning: aether-azathoth@0.2.0: 共 16 个标准库文件检查成功！
```

这确保了标准库代码始终是有效的。

## 命令行选项总结

```bash
# 基本运行
aether script.aether              # 运行脚本（自动加载标准库）

# 调试和分析
aether --check script.aether      # 只检查语法
aether --ast script.aether        # 显示 AST
aether --debug script.aether      # 调试模式运行
aether --trace script.aether      # 运行并打印 TRACE 缓冲区
aether --trace-stats script.aether # 运行并打印 TRACE 统计
aether --trace-buffer-size 4096 --trace script.aether # 调大 TRACE 缓冲区

# 标准库控制
aether --no-stdlib script.aether  # 不加载标准库

# 组合使用
aether --debug --no-stdlib script.aether  # 调试模式，不加载标准库

# 获取帮助
aether --help                     # 显示帮助信息
aether                            # 启动 REPL 交互模式
```

## REPL 增强

REPL 模式现在也会显示详细的错误信息：

```
aether[1]> Set X (10 + 
✗ Parse error at line 1, column 14: Expected RightParen, found EOF

源代码位置:
   1 | Set X (10 + 
       |              ^
```

## 实用技巧

### 1. 快速检查语法

在提交代码前快速检查：

```bash
aether --check *.aether
```

### 2. 调试复杂表达式

使用 AST 查看来理解复杂的嵌套表达式：

```bash
aether --ast complex_script.aether
```

### 3. 开发时使用调试模式

在开发过程中使用调试模式获取更多信息：

```bash
aether --debug my_script.aether
```

### 4. 不加载标准库进行快速测试

测试核心语法时可以跳过标准库加载：

```bash
aether --no-stdlib --check test.aether
```

## 最佳实践

1. **提交前检查**：使用 `--check` 确保代码无语法错误
2. **理解结构**：使用 `--ast` 学习和理解 Aether 的语法结构
3. **调试问题**：遇到问题时使用 `--debug` 模式获取更多信息
4. **阅读错误**：仔细阅读错误信息和源代码上下文
5. **标准库开发**：修改标准库后运行 `cargo build` 自动验证

## 未来改进

计划中的功能：

- [ ] 执行过程跟踪（每一步的变量状态）
- [ ] 性能分析工具
- [ ] 断点调试支持
- [ ] 更详细的类型信息显示
- [ ] 代码覆盖率分析
