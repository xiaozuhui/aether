# Aether HELP 函数使用指南

## 功能说明

在 Aether 语言中，现在可以使用 `HELP()` 函数来查看内置函数的文档。这样你就不需要查看 Rust 源代码，直接在 Aether 脚本中就能了解函数的用法。

## 使用方法

### 1. 查看所有可用函数

```aether
PRINTLN(HELP())
```

这会显示所有内置函数的分类列表，包括：

- 精确计算
- 输入输出
- 数组操作
- 字符串操作
- 数学函数（基础、三角、对数）
- 数学常数
- 统计分析
- 向量运算
- 矩阵运算
- 线性回归
- 概率分布
- 精度计算
- 类型转换
- 字典操作
- 工资计算（超过 60 个专业函数）

### 2. 查看特定函数的详细文档

```aether
PRINTLN(HELP("TO_FRACTION"))
```

这会显示该函数的：

- 函数名称
- 功能描述
- 参数列表（名称和说明）
- 返回值说明
- 使用示例

### 3. 函数名大小写不敏感

```aether
HELP("to_fraction")  // 小写
HELP("TO_FRACTION")  // 大写
HELP("To_Fraction")  // 混合
```

以上都能正常工作！

## 实际使用示例

```aether
// 场景：想进行精确的小数计算

// 1. 先看看有什么函数
PRINTLN(HELP())

// 2. 看到有 FRAC_ADD，查看详细用法
PRINTLN(HELP("FRAC_ADD"))

// 3. 实际使用
result = FRAC_ADD(0.1, 0.2)
PRINTLN(result)  // 输出: 3/10（而不是 0.30000000000000004）
```

## 已添加文档的函数

目前已经为以下类别的函数添加了完整的中文文档：

✅ **精确计算** (11个): TO_FRACTION, TO_FLOAT, SIMPLIFY, FRAC_ADD, FRAC_SUB, FRAC_MUL, FRAC_DIV, NUMERATOR, DENOMINATOR, GCD, LCM

✅ **输入输出** (3个): PRINT, PRINTLN, INPUT

✅ **数组操作** (9个): RANGE, LEN, PUSH, POP, REVERSE, SORT, SUM, MAX, MIN

✅ **字符串操作** (4个): SPLIT, UPPER, LOWER, TRIM

✅ **数学函数** (6个基础): ABS, SQRT, POW, FLOOR, CEIL, ROUND

✅ **三角函数** (3个): SIN, COS, TAN

✅ **数学常数** (2个): PI, E

✅ **统计分析** (4个): MEAN, MEDIAN, VARIANCE, STD

✅ **类型转换** (3个): TYPE, TO_STRING, TO_NUMBER

## 提示

- 如果不知道函数名，先用 `HELP()` 浏览所有函数
- 如果想查看某个函数的用法，用 `HELP("函数名")`
- 所有函数名都不区分大小写
- 可以在交互式 REPL 中使用，也可以在脚本中使用

## 更多示例

查看以下示例文件：

- `examples/test_help.aether` - HELP 函数基本测试
- `examples/demo_help.aether` - 实际使用演示
- `examples/test_help_all.aether` - 查看所有函数列表
- `examples/test_help_specific.aether` - 查看特定函数文档
