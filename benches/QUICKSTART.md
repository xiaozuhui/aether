# Aether 基准测试快速上手指南

## 快速开始

### 1. 运行所有基准测试（完整模式）

```bash
cargo bench
```

这将运行所有基准测试，每个测试进行 100 次采样（默认）。完整测试可能需要 10-20 分钟。

### 2. 快速测试模式（推荐用于开发）

```bash
# 使用脚本
./scripts/bench.sh quick

# 或直接使用 cargo
cargo bench -- --sample-size 10
```

快速模式只采样 10 次，可在 2-3 分钟内完成。

### 3. 运行特定类别的测试

```bash
# 使用便捷脚本
./scripts/bench.sh arithmetic   # 算术运算
./scripts/bench.sh functions    # 函数调用
./scripts/bench.sh arrays       # 数组操作
./scripts/bench.sh parsing      # 解析性能

# 或使用 cargo 直接运行
cargo bench -- arithmetic
cargo bench -- functions
```

### 4. 查看测试结果

基准测试完成后，在浏览器中打开：

```bash
./scripts/bench.sh report

# 或手动打开
open target/criterion/report/index.html
```

## 性能对比工作流

### 场景：优化代码并对比性能

```bash
# 1. 保存优化前的基线
cargo bench -- --save-baseline before

# 2. 修改代码进行优化
# ... 编辑代码 ...

# 3. 运行测试并与基线对比
cargo bench -- --baseline before
```

Criterion 会自动显示性能变化百分比。

## 常见测试示例

### 测试简单表达式性能

```bash
cargo bench -- simple_addition
```

**预期结果**：~400-600 纳秒

### 测试递归函数性能

```bash
cargo bench -- recursive_function
```

**预期结果**：~5-10 微秒（Fib(10)）

### 测试不同规模的斐波那契

```bash
cargo bench -- fibonacci
```

这将测试 Fib(5)、Fib(10)、Fib(15) 的性能对比。

### 测试解析性能

```bash
cargo bench -- parsing
```

分别测试：

- 词法分析（Lexer）
- 语法分析（Parser）
- 完整的解析和执行

## 理解测试结果

### 输出示例

```
arithmetic/simple_addition
                        time:   [426.04 ns 429.40 ns 435.75 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high severe
```

**解读**：

- **time**: 中间值是平均执行时间（429.40 ns）
- **[lower, mean, upper]**: 95% 置信区间
- **outliers**: 异常值数量（通常可以忽略）

### 性能比较输出

```
                        time:   [429.40 ns 432.15 ns 435.89 ns]
                        change: [-5.2345% -3.1234% -1.0123%] (p = 0.00 < 0.05)
                        Performance has improved.
```

**解读**：

- **change**: 相对于基线的性能变化
- **negative change**: 性能提升（时间减少）
- **positive change**: 性能下降（时间增加）
- **p < 0.05**: 变化具有统计学意义

## 添加自定义基准测试

编辑 `benches/aether_benchmarks.rs`：

```rust
fn bench_my_feature(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_feature");
    
    group.bench_function("test_case_1", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("your aether code")).unwrap();
        });
    });
    
    group.finish();
}

// 添加到测试组
criterion_group!(
    benches,
    bench_arithmetic,
    bench_my_feature  // 新增
);
```

## 性能优化检查清单

使用基准测试时，关注这些方面：

### ✅ 识别性能瓶颈

```bash
# 运行所有测试，找出最慢的操作
cargo bench | grep "time:"
```

### ✅ 迭代优化

1. 运行基准测试并保存基线
2. 进行代码优化
3. 重新运行测试对比
4. 重复直到达到目标

### ✅ 回归测试

在 CI/CD 中运行基准测试，确保性能不会退化：

```bash
# 在 CI 中运行
cargo bench --no-fail-fast
```

## 性能参考值

基于典型开发机器（M2 Mac）的参考值：

| 操作类型 | 预期时间 | 说明 |
|---------|---------|------|
| 简单算术 | ~400-600 ns | `(1 + 2)` |
| 复杂表达式 | ~1-2 μs | 多层嵌套运算 |
| 变量赋值 | ~300-500 ns | `Set X 10` |
| 函数调用 | ~1-3 μs | 用户定义函数 |
| 数组创建 | ~500-800 ns | 5个元素 |
| 递归函数 | ~5-10 μs | Fib(10) |
| While循环 | ~2-4 μs | 10次迭代 |
| 解析小程序 | ~5-10 μs | 5-10行代码 |

**注意**：实际性能会因硬件而异。

## 故障排查

### 问题：测试时间过长

**解决方案**：使用快速模式

```bash
cargo bench -- --sample-size 10
```

### 问题：结果不稳定

**解决方案**：

1. 关闭其他高负载应用
2. 确保电源接通（笔记本电脑）
3. 等待系统温度稳定
4. 增加样本数量

```bash
cargo bench -- --sample-size 200
```

### 问题：想要更详细的统计信息

**解决方案**：使用 verbose 模式

```bash
cargo bench -- --verbose
```

## 有用的命令汇总

```bash
# 运行所有测试（完整）
cargo bench

# 快速测试
cargo bench -- --sample-size 10

# 测试特定类别
cargo bench -- arithmetic

# 测试特定单项
cargo bench -- simple_addition

# 保存基线
cargo bench -- --save-baseline my_baseline

# 与基线对比
cargo bench -- --baseline my_baseline

# 查看帮助
cargo bench -- --help

# 清理测试结果
rm -rf target/criterion

# 使用脚本（推荐）
./scripts/bench.sh help
```

## 集成到开发流程

### 日常开发

```bash
# 1. 开发新功能前记录基线
./scripts/bench.sh save before_feature_x

# 2. 开发完成后对比
./scripts/bench.sh compare before_feature_x

# 3. 如果性能下降，使用快速模式迭代优化
./scripts/bench.sh quick
```

### PR 审查

```bash
# 审查者可以运行快速测试验证性能
./scripts/bench.sh quick

# 或运行特定相关的测试
./scripts/bench.sh arithmetic  # 如果改动涉及算术运算
```

## 下一步

- 阅读 [Criterion.rs 用户指南](https://bheisler.github.io/criterion.rs/book/)
- 查看 `benches/README.md` 了解更多详情
- 探索 HTML 报告中的图表和详细统计数据
