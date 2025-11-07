# Aether 基准测试

本目录包含 Aether 解释器的性能基准测试套件。

## 测试覆盖范围

基准测试覆盖了以下方面：

### 1. 算术运算 (Arithmetic)

- 简单加法运算
- 复杂表达式计算
- 嵌套算术运算

### 2. 变量操作 (Variables)

- 变量赋值
- 变量读取
- 多变量操作

### 3. 函数调用 (Functions)

- 内置函数调用
- 用户自定义函数调用
- 递归函数（斐波那契数列）

### 4. 控制流 (Control Flow)

- If 条件语句
- While 循环
- For 循环

### 5. 数组操作 (Arrays)

- 数组创建
- 数组访问
- 数组迭代
- 数组映射（Map）

### 6. 字典操作 (Dictionaries)

- 字典创建
- 字典访问
- 字典修改

### 7. 字符串操作 (Strings)

- 字符串拼接
- 字符串长度计算
- 字符串分割

### 8. 精确数学运算 (Precision)

- 有理数运算
- 大整数运算

### 9. 程序规模测试 (Program Sizes)

- 小型程序（<10 行）
- 中型程序（10-30 行）
- 大型程序（>30 行）

### 10. 解析性能 (Parsing)

- 词法分析（Lexer）
- 语法分析（Parser）
- 完整的解析与求值

### 11. 递归性能 (Fibonacci)

- 不同输入规模的斐波那契数列计算

### 12. 工资计算模块 (Payroll)

- 基本工资计算示例

## 运行基准测试

### 运行所有基准测试

```bash
cargo bench
```

### 运行特定的基准测试组

```bash
# 只运行算术运算测试
cargo bench --bench aether_benchmarks -- arithmetic

# 只运行函数调用测试
cargo bench --bench aether_benchmarks -- functions

# 只运行数组操作测试
cargo bench --bench aether_benchmarks -- arrays
```

### 运行特定的单个测试

```bash
# 运行简单加法测试
cargo bench --bench aether_benchmarks -- simple_addition

# 运行递归函数测试
cargo bench --bench aether_benchmarks -- recursive_function
```

### 查看详细输出

```bash
# 显示更详细的统计信息
cargo bench -- --verbose

# 保存基准测试结果
cargo bench -- --save-baseline my_baseline
```

### 比较基准测试结果

```bash
# 先运行并保存基线
cargo bench -- --save-baseline before

# 进行代码修改后，与基线比较
cargo bench -- --baseline before
```

## 查看测试报告

运行基准测试后，Criterion 会生成 HTML 报告：

```bash
# 报告位置
target/criterion/report/index.html
```

在浏览器中打开该文件可以查看详细的性能图表和统计数据。

## 性能优化建议

基于基准测试结果，你可以：

1. **识别瓶颈**：找出执行时间最长的操作
2. **对比优化**：修改代码后对比性能变化
3. **回归测试**：确保优化没有引入性能退化
4. **持续监控**：将基准测试集成到 CI/CD 流程中

## 自定义基准测试

如果需要添加新的基准测试，可以编辑 `aether_benchmarks.rs` 文件：

```rust
fn bench_my_feature(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_feature");
    
    group.bench_function("test_name", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("your code here")).unwrap();
        });
    });
    
    group.finish();
}

// 然后添加到 criterion_group! 宏中
criterion_group!(
    benches,
    // ... 其他测试 ...
    bench_my_feature
);
```

## 性能指标说明

Criterion 提供的关键指标：

- **time**: 平均执行时间
- **Lower bound / Upper bound**: 95% 置信区间
- **R² (R-squared)**: 回归拟合度（越接近 1.0 越好）
- **Mean**: 平均值
- **Std. Dev.**: 标准差
- **Median**: 中位数
- **MAD (Median Absolute Deviation)**: 中位数绝对偏差

## 注意事项

1. 运行基准测试时建议关闭其他高负载应用，以获得更准确的结果
2. 基准测试在 release 模式下运行，以反映真实性能
3. 某些测试（如大规模递归）可能需要较长时间完成
4. 确保系统处于稳定状态，避免温度限制等因素影响结果

## 持续集成

可以将基准测试添加到 CI 流程中：

```yaml
# .github/workflows/benchmark.yml 示例
name: Benchmark

on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run benchmarks
        run: cargo bench
```

## 疑难解答

### 如果遇到编译错误

```bash
# 清理并重新构建
cargo clean
cargo bench
```

### 如果测试运行时间过长

可以减少样本数量（仅用于快速测试）：

```bash
cargo bench -- --sample-size 10
```

## 更多资源

- [Criterion.rs 文档](https://github.com/bheisler/criterion.rs)
- [Rust 性能优化指南](https://nnethercote.github.io/perf-book/)
