# Aether 用户指南

完整的 Aether 编程语言参考手册，包含所有内置函数和使用示例。

## 目录

- [快速开始](#快速开始)
  - [命令行工具](#命令行工具)
  - [作为库使用](#作为库使用)
- [语言基础](#语言基础)
- [内置函数参考](#内置函数参考)
  - [I/O 函数](#io-函数) (3个)
  - [类型函数](#类型函数) (4个)
  - [数组函数](#数组函数) (13个)
  - [字符串函数](#字符串函数) (9个)
  - [字典函数](#字典函数) (4个)
  - [数学函数](#数学函数) (95个)
- [高级特性](#高级特性)
- [示例程序](#示例程序)

---

## 快速开始

### 命令行工具

Aether 可以作为独立的命令行工具使用：

#### 运行脚本文件

```bash
# 运行 Aether 脚本
aether my_script.aether

# 运行示例
aether examples/stats_demo.aether
```

#### 交互式 REPL

```bash
# 启动交互模式（无参数）
aether

# 然后输入 Aether 代码：
aether[1]> Set X 10
aether[2]> Set Y 20
aether[3]> (X + Y)
30
aether[4]> help      # 显示帮助
aether[5]> exit      # 退出 REPL
```

### 作为库使用

#### Rust

```rust
use aether::Aether;

fn main() {
    let mut engine = Aether::new();
    let code = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;
    
    match engine.eval(code) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => eprintln!("错误: {}", e),
    }
}
```

### Hello World

```aether
Println("Hello, World!")
```

### 基本计算

```aether
Set x 10
Set y 20
Set sum x + y
Println("Sum: " + ToString(sum))
```

### 函数定义

```aether
Function greet(name)
    Return "Hello, " + name + "!"
EndFunction

Println(greet("Aether"))
```

---

## 语言基础

### 数据类型

- **Number**: 浮点数 `42`, `3.14`
- **String**: 字符串 `"hello"`, `'world'`
- **Boolean**: 布尔值 `True`, `False`
- **Null**: 空值 `Null`
- **Array**: 数组 `[1, 2, 3]`
- **Dict**: 字典 `{"name": "Alice", "age": 30}`

### 变量声明

```aether
Set x 10
Set name "Alice"
Set items [1, 2, 3]
```

### 控制流

```aether
# If-Else
If x > 10
    Println("大于10")
Else
    Println("小于等于10")
EndIf

# For 循环
For i In Range(1, 10)
    Println(i)
EndFor

# While 循环
Set i 0
While i < 10
    Println(i)
    Set i i + 1
EndWhile
```

---

## 内置函数参考

### I/O 函数

#### Print(value)

打印值到标准输出，不添加换行符。

**参数**: `value` - 任意类型

**返回值**: Null

**示例**:

```aether
Print("Hello")
Print(42)
```

#### Println(value)

打印值到标准输出，并添加换行符。

**参数**: `value` - 任意类型

**返回值**: Null

**示例**:

```aether
Println("Hello, World!")
```

#### Input(prompt)

显示提示信息并读取用户输入。

**参数**: `prompt` - String，提示信息

**返回值**: String，用户输入的文本

**示例**:

```aether
Set name Input("请输入姓名: ")
Println("你好, " + name)
```

---

### 类型函数

#### TypeOf(value)

获取值的类型名称。

**返回值**: "Number", "String", "Boolean", "Null", "Array", "Dict", "Function" 等

**示例**:

```aether
Println(TypeOf(42))        # Number
Println(TypeOf("hello"))   # String
```

#### ToString(value)

将值转换为字符串。

**示例**:

```aether
Set num 42
Set str ToString(num)
Println(str)  # "42"
```

#### ToNumber(value)

将字符串转换为数字。

**示例**:

```aether
Set str "123"
Set num ToNumber(str)
Println(num + 10)  # 133
```

#### Len(collection)

获取集合的长度。

**参数**: Array, String, 或 Dict

**返回值**: Number

**示例**:

```aether
Println(Len([1, 2, 3]))     # 3
Println(Len("hello"))       # 5
Println(Len({"a": 1}))      # 1
```

---

### 数组函数

#### Range(start, end, [step])

生成数字序列。

**参数**:

- `start` - 起始值（包含）
- `end` - 结束值（不包含）
- `step` - 步长（可选，默认1）

**示例**:

```aether
Set nums Range(0, 10)        # [0,1,2,3,4,5,6,7,8,9]
Set evens Range(0, 10, 2)    # [0,2,4,6,8]
```

#### Push(array, value)

在数组末尾添加元素。

**返回值**: 修改后的数组

**示例**:

```aether
Set arr [1, 2, 3]
Push(arr, 4)
Println(arr)  # [1, 2, 3, 4]
```

#### Pop(array)

移除并返回数组最后一个元素。

**示例**:

```aether
Set arr [1, 2, 3]
Set last Pop(arr)
Println(last)  # 3
Println(arr)   # [1, 2]
```

#### Join(array, separator)

将数组元素连接为字符串。

**示例**:

```aether
Set arr ["a", "b", "c"]
Set str Join(arr, "-")
Println(str)  # "a-b-c"
```

#### Reverse(array)

反转数组。

**示例**:

```aether
Set arr [1, 2, 3]
Set rev Reverse(arr)
Println(rev)  # [3, 2, 1]
```

#### Sort(array)

对数组排序。

**示例**:

```aether
Set arr [3, 1, 2]
Set sorted Sort(arr)
Println(sorted)  # [1, 2, 3]
```

#### Sum(array)

计算数组所有数字的和。

**示例**:

```aether
Set arr [1, 2, 3, 4, 5]
Println(Sum(arr))  # 15
```

#### Map(array, function)

对数组每个元素应用函数。

**示例**:

```aether
Set arr [1, 2, 3]
Set doubled Map(arr, Function(x) Return x * 2 EndFunction)
Println(doubled)  # [2, 4, 6]
```

#### Filter(array, predicate)

筛选满足条件的元素。

**示例**:

```aether
Set arr [1, 2, 3, 4, 5]
Set evens Filter(arr, Function(x) Return x % 2 == 0 EndFunction)
Println(evens)  # [2, 4]
```

#### Reduce(array, function, initial)

累积计算数组值，回调可选第三参数为索引。

**示例**:

```aether
Set arr [1, 2, 3, 4]
# 基本用法（2 参回调）
Set sum Reduce(arr, Function(acc, x) Return acc + x EndFunction, 0)  # 10

# 带索引（3 参回调）
Set weighted Reduce(arr, Function(acc, x, i) Return acc + x * i EndFunction, 0)  # 20
```

---

### 字符串函数

#### Split(string, delimiter)

按分隔符分割字符串。

**示例**:

```aether
Set str "a,b,c"
Set parts Split(str, ",")
Println(parts)  # ["a", "b", "c"]
```

#### Upper(string)

转换为大写。

**示例**:

```aether
Println(Upper("hello"))  # "HELLO"
```

#### Lower(string)

转换为小写。

**示例**:

```aether
Println(Lower("HELLO"))  # "hello"
```

#### Trim(string)

移除首尾空白字符。

**示例**:

```aether
Println(Trim("  hello  "))  # "hello"
```

#### Contains(string, substring)

检查是否包含子串。

**示例**:

```aether
Println(Contains("hello world", "world"))  # True
```

#### StartsWith(string, prefix)

检查是否以指定前缀开始。

**示例**:

```aether
Println(StartsWith("hello", "he"))  # True
```

#### EndsWith(string, suffix)

检查是否以指定后缀结束。

**示例**:

```aether
Println(EndsWith("hello", "lo"))  # True
```

#### Replace(string, old, new)

替换字符串中的子串。

**示例**:

```aether
Set str "hello world"
Println(Replace(str, "world", "Aether"))  # "hello Aether"
```

#### Repeat(string, count)

重复字符串指定次数。

**示例**:

```aether
Println(Repeat("ha", 3))  # "hahaha"
```

---

### 字典函数

#### Keys(dict)

获取字典的所有键。

**示例**:

```aether
Set d {"a": 1, "b": 2}
Println(Keys(d))  # ["a", "b"]
```

#### Values(dict)

获取字典的所有值。

**示例**:

```aether
Set d {"a": 1, "b": 2}
Println(Values(d))  # [1, 2]
```

#### Has(dict, key)

检查字典是否包含指定键。

**示例**:

```aether
Set d {"name": "Alice"}
Println(Has(d, "name"))  # True
Println(Has(d, "age"))   # False
```

#### Merge(dict1, dict2)

合并两个字典。

**示例**:

```aether
Set d1 {"a": 1}
Set d2 {"b": 2}
Set merged Merge(d1, d2)
Println(merged)  # {"a": 1, "b": 2}
```

---

### 数学函数

Aether 提供了 **95 个数学函数**，涵盖基础运算、三角函数、统计学、线性代数和概率论。

#### 基础数学 (6个)

**Abs(x)** - 绝对值

```aether
Println(Abs(-5))  # 5
```

**Floor(x)** - 向下取整

```aether
Println(Floor(3.7))  # 3
```

**Ceil(x)** - 向上取整

```aether
Println(Ceil(3.2))  # 4
```

**Round(x)** - 四舍五入

```aether
Println(Round(3.5))  # 4
```

**Min(a, b)** - 最小值

```aether
Println(Min(5, 10))  # 5
```

**Max(a, b)** - 最大值

```aether
Println(Max(5, 10))  # 10
```

#### 三角函数 (9个)

**Sin(x)**, **Cos(x)**, **Tan(x)** - 基本三角函数（弧度制）

```aether
Set pi PI()
Println(Sin(pi / 2))  # 1.0
Println(Cos(0))       # 1.0
```

**Asin(x)**, **Acos(x)**, **Atan(x)** - 反三角函数

```aether
Println(Asin(1))      # π/2
```

**Atan2(y, x)** - 双参数反正切

```aether
Println(Atan2(1, 1))  # π/4
```

**Sinh(x)**, **Cosh(x)**, **Tanh(x)** - 双曲三角函数

```aether
Println(Sinh(0))      # 0
```

#### 对数与指数 (9个)

**Sqrt(x)** - 平方根

```aether
Println(Sqrt(16))  # 4
```

**Pow(x, y)** - x的y次方

```aether
Println(Pow(2, 10))  # 1024
```

**Exp(x)** - e^x

```aether
Println(Exp(1))  # e ≈ 2.718
```

**Ln(x)** - 自然对数

```aether
Println(Ln(E()))  # 1
```

**Log(x)**, **Log2(x)** - 常用对数和二进制对数

```aether
Println(Log(100))   # 2
Println(Log2(8))    # 3
```

#### 统计函数 (7个)

**Mean(array)** - 平均值

```aether
Set data [1, 2, 3, 4, 5]
Println(Mean(data))  # 3
```

**Median(array)** - 中位数

```aether
Set data [1, 2, 3, 4, 5]
Println(Median(data))  # 3
```

**Variance(array)** - 方差

```aether
Set data [1, 2, 3, 4, 5]
Println(Variance(data))  # 2
```

**Std(array)** - 标准差

```aether
Set data [1, 2, 3, 4, 5]
Println(Std(data))  # 1.414...
```

**Quantile(array, q)** - 分位数

```aether
Set data [1, 2, 3, 4, 5]
Println(Quantile(data, 0.5))  # 中位数: 3
```

#### 线性回归与概率分布 (5个)

**LinearRegression(x, y)** - 简单线性回归

```aether
Set x [1, 2, 3, 4, 5]
Set y [3, 5, 7, 9, 11]
Set model LinearRegression(x, y)
Set slope model[0]      # 2.0
Set intercept model[1]  # 1.0
Set r2 model[2]         # 1.0 (完美拟合)
```

**NormalPDF(x, [mean, std])** - 正态分布概率密度函数

```aether
# 标准正态分布
Set pdf NormalPDF(0)  # 0.3989

# 自定义分布 (μ=100, σ=15)
Set pdf_custom NormalPDF(120, 100, 15)
```

**NormalCDF(x, [mean, std])** - 正态分布累积分布函数

```aether
Set cdf NormalCDF(0)          # 0.5 (中位数)
Set cdf_95 NormalCDF(1.96)    # 0.975 (95%置信区间)
```

**PoissonPMF(k, lambda)** - 泊松分布概率质量函数

```aether
# 平均每小时3次事件
Set p0 PoissonPMF(0, 3)  # 0.0498 (无事件概率)
Set p3 PoissonPMF(3, 3)  # 0.224 (最可能)
```

#### 向量运算 (5个)

**Dot(v1, v2)** - 点积

```aether
Set v1 [1, 2, 3]
Set v2 [4, 5, 6]
Println(Dot(v1, v2))  # 32
```

**Norm(vector)** - 欧几里得范数

```aether
Set v [3, 4]
Println(Norm(v))  # 5
```

**Cross(v1, v2)** - 叉积（仅3D向量）

```aether
Set v1 [1, 0, 0]
Set v2 [0, 1, 0]
Println(Cross(v1, v2))  # [0, 0, 1]
```

**Distance(v1, v2)** - 欧几里得距离

```aether
Set p1 [0, 0]
Set p2 [3, 4]
Println(Distance(p1, p2))  # 5
```

**Normalize(vector)** - 归一化向量

```aether
Set v [3, 4]
Println(Normalize(v))  # [0.6, 0.8]
```

#### 矩阵运算 (5个)

**Matmul(m1, m2)** - 矩阵乘法

```aether
Set A [[1, 2], [3, 4]]
Set B [[5, 6], [7, 8]]
Set C Matmul(A, B)  # [[19, 22], [43, 50]]
```

**Transpose(matrix)** - 矩阵转置

```aether
Set M [[1, 2], [3, 4]]
Set MT Transpose(M)  # [[1, 3], [2, 4]]
```

**Determinant(matrix)** - 行列式（支持任意大小方阵）

```aether
Set M [[1, 2], [3, 4]]
Println(Determinant(M))  # -2

# 4x4 矩阵
Set M4 [[1,2,3,4], [2,1,2,3], [3,2,1,2], [4,3,2,1]]
Println(Determinant(M4))  # -20
```

**Inverse(matrix)** - 矩阵求逆

```aether
Set A [[4, 7], [2, 6]]
Set invA Inverse(A)
Set I Matmul(A, invA)  # [[1, 0], [0, 1]] (单位矩阵)
```

#### 数学常数 (4个)

**PI()** - 圆周率 π ≈ 3.14159

```aether
Set circumference 2 * PI() * radius
```

**E()** - 自然常数 e ≈ 2.71828

```aether
Set growth E() * rate
```

**TAU()** - τ = 2π ≈ 6.28318

```aether
Set full_circle TAU()
```

**PHI()** - 黄金比例 φ ≈ 1.61803

```aether
Set golden PHI()
```

---

## 高级特性

### 生成器 (Generators)

生成器用于创建惰性序列。

```aether
Generator fibonacci(n)
    Set a 0
    Set b 1
    Set i 0
    While i < n
        Yield a
        Set temp a
        Set a b
        Set b temp + b
        Set i i + 1
    EndWhile
EndGenerator

For num In fibonacci(10)
    Println(num)
EndFor
```

### 惰性求值 (Lazy Evaluation)

延迟计算直到需要时才执行。

```aether
Lazy expensive_calculation
    # 只在需要时执行
    Return heavy_computation()
EndLazy

# 稍后使用
Set result Force(expensive_calculation)
```

### 闭包 (Closures)

函数可以捕获外部作用域的变量。

```aether
Function make_counter()
    Set count 0
    Function increment()
        Set count count + 1
        Return count
    EndFunction
    Return increment
EndFunction

Set counter make_counter()
Println(counter())  # 1
Println(counter())  # 2
```

---

## 示例程序

### 数据分析示例

```aether
# 销售数据分析
Set months [1, 2, 3, 4, 5, 6]
Set sales [120, 135, 158, 172, 195, 210]

# 计算统计信息
Set avg Mean(sales)
Set std_dev Std(sales)

Println("平均销售额: " + ToString(avg))
Println("标准差: " + ToString(std_dev))

# 线性回归预测
Set model LinearRegression(months, sales)
Set slope model[0]
Set intercept model[1]
Set r2 model[2]

Println("趋势: 销量 = " + ToString(slope) + " * 月份 + " + ToString(intercept))
Println("拟合优度 R²: " + ToString(r2))

# 预测第7个月
Set month7 slope * 7 + intercept
Println("预测第7个月销量: " + ToString(month7))
```

### 质量控制示例

```aether
# 产品重量分布分析 (μ=500g, σ=5g)
Set spec_lower 490
Set spec_upper 510

# 计算合格率
Set prob_lower NormalCDF(spec_lower, 500, 5)
Set prob_upper NormalCDF(spec_upper, 500, 5)
Set pass_rate prob_upper - prob_lower

Println("合格率: " + ToString(pass_rate * 100) + "%")

# 不合格率
Set reject_rate 1 - pass_rate
Println("不合格率: " + ToString(reject_rate * 100) + "%")
```

### 矩阵计算示例

```aether
# 解线性方程组: Ax = b
# 2x + y = 5
# x + 3y = 7
# 解: x = 2, y = 1

Set A [[2, 1], [1, 3]]
Set b [[5], [7]]

Set invA Inverse(A)
Set x Matmul(invA, b)

Println("解: x = " + ToString(x))

# 验证
Set result Matmul(A, x)
Println("验证 Ax = " + ToString(result))
```

---

## 性能提示

1. **数组操作**: 尽量使用内置函数（Map, Filter, Reduce）而不是手写循环
2. **字符串拼接**: 大量拼接时使用数组 + Join 效率更高
3. **矩阵运算**: 对大矩阵（n>10）的行列式和求逆操作较慢
4. **生成器**: 处理大序列时使用生成器节省内存

---

## 错误处理

Aether 会在运行时检测以下错误：

- **类型错误**: 操作不支持的类型
- **除零错误**: 除法或模运算分母为零
- **索引错误**: 数组或字符串索引越界
- **未定义变量**: 使用未声明的变量
- **参数错误**: 函数参数数量或类型不匹配

---

## 下一步

- 查看 [示例程序](../examples/) 了解更多用法
- 阅读 [开发文档](../DEVELOPMENT.md) 了解如何扩展 Aether
- 访问 [变更日志](../CHANGELOG.md) 查看版本历史

---

**版本**: 0.1.0  
**最后更新**: 2025年11月6日  
**函数总数**: 95个
