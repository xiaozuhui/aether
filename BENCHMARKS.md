# Aether 基准测试

本项目已经集成了全面的性能基准测试系统，使用 Criterion.rs 框架。

## 快速开始

```bash
# 运行所有基准测试（完整模式，约 10-20 分钟）
cargo bench

# 快速测试（减少样本，约 2-3 分钟）
cargo bench -- --sample-size 10

# 使用便捷脚本
./scripts/bench.sh quick
```

## 测试覆盖范围

### 1. 算术运算 (Arithmetic)

- 简单加法
- 复杂表达式
- 嵌套算术运算

### 2. 变量操作 (Variables)

- 变量赋值
- 变量读取
- 多变量操作

### 3. 函数 (Functions)

- 内置函数调用 (PRINTLN)
- 用户自定义函数
- 递归函数 (斐波那契数列)

### 4. 控制流 (Control Flow)

- If/Else 条件语句
- While 循环
- For 循环

### 5. 数组 (Arrays)

- 数组创建
- 数组访问
- 数组迭代
- 数组映射操作

### 6. 字典 (Dictionaries)

- 字典创建
- 字典访问
- 字典更新

### 7. 字符串 (Strings)

- 字符串拼接
- 字符串长度
- 字符串分割

### 8. 精确数学 (Precision)

- 有理数运算
- 大整数运算

### 9. 程序规模 (Program Sizes)

- 小型程序 (<10 行代码)
- 中型程序 (10-30 行代码)
- 大型程序 (>30 行代码)

### 10. 解析性能 (Parsing)

- 词法分析 (Lexer)
- 语法分析 (Parser)
- 完整解析与求值

### 11. 斐波那契规模测试 (Fibonacci)

- Fib(5)
- Fib(10)
- Fib(15)

### 12. 工资计算 (Payroll)

- 基本工资计算示例

## 典型性能指标

基于 M2 Mac 的参考值（--sample-size 10）：

| 测试项 | 平均时间 | 说明 |
|--------|----------|------|
| 简单加法 | ~430-460 ns | `(1 + 2)` |
| 复杂表达式 | ~1.0 µs | `((10 + 20) * 3 - 5) / 2` |
| 嵌套算术 | ~1.7 µs | 多层括号运算 |
| 变量赋值 | ~520-540 ns | `Set X 100` |
| 变量读取 | ~290 ns | 读取已赋值变量 |
| 多变量 | ~2.2 µs | 3个变量的赋值和计算 |
| 内置函数 | ~1.6-2.0 µs | PRINTLN |
| 用户函数 | ~1.4 µs | 简单加法函数 |
| 递归函数 | ~250 µs | Fib(10) |
| If 语句 | ~2.2 µs | 条件判断 |
| While 循环 | ~5.7 µs | 10次迭代 |
| For 循环 | ~4.2 µs | 10次迭代 |
| 数组创建 | ~820 ns | 5个元素 |
| 数组访问 | ~630 ns | 索引访问 |
| 数组迭代 | ~5.3 µs | 10个元素 |

**注意**：实际性能因硬件配置而异。

## 使用方法

### 运行特定测试

```bash
# 只测试算术运算
cargo bench -- arithmetic

# 只测试函数调用
cargo bench -- functions

# 只测试数组操作
cargo bench -- arrays
```

### 性能对比工作流

```bash
# 1. 保存当前性能基线
cargo bench -- --save-baseline before

# 2. 修改代码进行优化
# ... 编辑代码 ...

# 3. 与基线对比
cargo bench -- --baseline before
```

Criterion 会自动计算并显示性能变化百分比。

### 查看详细报告

```bash
# 在浏览器中打开 HTML 报告
open target/criterion/report/index.html

# 或使用脚本
./scripts/bench.sh report
```

## 便捷脚本

项目提供了 `scripts/bench.sh` 脚本来简化操作：

```bash
./scripts/bench.sh                  # 运行所有测试
./scripts/bench.sh quick            # 快速模式
./scripts/bench.sh arithmetic       # 特定类别
./scripts/bench.sh save baseline1   # 保存基线
./scripts/bench.sh compare baseline1 # 对比基线
./scripts/bench.sh report           # 打开报告
./scripts/bench.sh clean            # 清理结果
./scripts/bench.sh help             # 查看帮助
```

## 持续集成

可以将基准测试集成到 CI/CD 流程中。参考 `benches/github-actions-example.yml`。

## 更多文档

- [详细使用指南](benches/README.md)
- [快速上手](benches/QUICKSTART.md)
- [GitHub Actions 配置示例](benches/github-actions-example.yml)

## 注意事项

1. **环境稳定性**：运行基准测试时，关闭其他高负载应用以获得准确结果
2. **电源管理**：笔记本电脑请连接电源，避免省电模式影响性能
3. **样本数量**：默认 100 个样本，可通过 `--sample-size` 调整
4. **预热时间**：Criterion 会自动进行预热，确保结果稳定
5. **输出控制**：内置函数（如 PRINTLN）会产生大量输出，这是正常现象

## 故障排查

### 编译错误

```bash
cargo clean
cargo bench
```

### 测试超时

```bash
# 使用快速模式
cargo bench -- --sample-size 10
```

### 查看详细错误

```bash
# 使用 verbose 模式
cargo bench -- --verbose
```

## 性能优化建议

基于基准测试结果，可以：

1. **识别瓶颈**：查找执行时间最长的操作
2. **迭代优化**：小步快跑，每次优化后运行测试验证
3. **回归检测**：确保优化不会引入性能退化
4. **持续监控**：在 CI 中运行，跟踪性能趋势

---

**提示**：首次运行基准测试可能需要较长时间编译依赖，后续运行会快很多。
