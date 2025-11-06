# 变更日志 (Changelog)

记录 Aether 项目的所有重要变更和里程碑。

---

## [Unreleased]

### 新增功能 ✨

#### 精确计算（分数运算）

- **TO_FRACTION(number)** - 将浮点数转换为分数
- **TO_FLOAT(fraction)** - 将分数转换回浮点数
- **SIMPLIFY(fraction)** - 简化分数到最简形式
- **FRAC_ADD(frac1, frac2)** - 分数加法
- **FRAC_SUB(frac1, frac2)** - 分数减法
- **FRAC_MUL(frac1, frac2)** - 分数乘法
- **FRAC_DIV(frac1, frac2)** - 分数除法
- **NUMERATOR(fraction)** - 获取分子
- **DENOMINATOR(fraction)** - 获取分母

#### 数论函数

- **GCD(a, b)** - 计算最大公约数
- **LCM(a, b)** - 计算最小公倍数

#### 精度计算

- **ROUND_TO(number, precision)** - 四舍五入到指定小数位
- **ADD_WITH_PRECISION(a, b, precision)** - 带精度的加法
- **SUB_WITH_PRECISION(a, b, precision)** - 带精度的减法
- **MUL_WITH_PRECISION(a, b, precision)** - 带精度的乘法
- **DIV_WITH_PRECISION(a, b, precision)** - 带精度的除法
- **SET_PRECISION(number, precision)** - 设置数字精度

#### 新增依赖

- **num-rational 0.4** - 有理数运算支持
- **num-bigint 0.4** - 大整数支持
- **num-traits 0.2** - 数值特性

### 改进 🔧

- **Value 类型扩展**: 添加 `Fraction(Ratio<BigInt>)` 变体
- **命名规范**: 所有函数名统一为 UPPER_SNAKE_CASE
- **文档完善**: 新增 `docs/PRECISION_GUIDE.md` 精度计算指南
- **示例程序**:
  - `examples/precision_simple.aether` - 基础示例
  - `examples/precision_complete.aether` - 完整示例
  - `examples/precision_demo.aether` - 综合演示
- **函数总数**: 从 95 个增加到 112 个

---

## [0.1.0] - 2025-11-06

### 新增功能 ✨

#### 高级数学函数

- **LinearRegression(x, y)** - 简单线性回归
  - 最小二乘法实现
  - 返回斜率、截距和 R² 系数
  - 用于趋势分析和预测

#### 概率分布函数

- **NormalPDF(x, [μ, σ])** - 正态分布概率密度函数
  - 支持标准正态分布和自定义分布
  - 高斯公式实现
  
- **NormalCDF(x, [μ, σ])** - 正态分布累积分布函数
  - 使用误差函数实现
  - 用于置信区间计算
  
- **PoissonPMF(k, λ)** - 泊松分布概率质量函数
  - 用于稀有事件建模
  - 直接公式实现

#### 矩阵运算增强

- **Inverse(matrix)** - 矩阵求逆
  - Gaussian-Jordan 消元法
  - 部分主元选择（数值稳定）
  - 支持任意大小方阵
  
- **Determinant(matrix)** - 扩展支持大矩阵
  - 现支持 4×4, 5×5 及更大矩阵
  - 递归余子式展开算法
  - 保持 1×1, 2×2, 3×3 直接公式

### 改进 🔧

- **文档完善**: 所有 95 个函数都有详细的中文 Rustdoc 注释
- **测试覆盖**: 新增 8 个统计/概率测试，总计 114 个测试（100%通过）
- **示例程序**: 添加 `examples/stats_demo.aether` 展示新功能
- **性能优化**: 矩阵运算数值稳定性改进

### 统计数据 📊

- **函数总数**: 95 个（90 原有 + 5 新增）
- **测试总数**: 114 个（53 单元测试 + 61 集成测试）
- **代码行数**: 约 535 行新增（math.rs）
- **测试覆盖**: 100% 通过率

---

## [早期版本] - 2024-2025

### 核心功能实现 ✅

#### 语言基础

- ✅ **词法分析器 (Lexer)** - Token 识别、注释处理
- ✅ **语法分析器 (Parser)** - 递归下降解析、Pratt 解析
- ✅ **AST 定义** - 13 种表达式、14 种语句
- ✅ **求值器 (Evaluator)** - 表达式求值、语句执行
- ✅ **环境管理** - 作用域、闭包、变量绑定

#### 数据类型

- ✅ Number - 64位浮点数
- ✅ String - UTF-8 字符串
- ✅ Boolean - True/False
- ✅ Null - 空值
- ✅ Array - 动态数组
- ✅ Dict - 键值对字典
- ✅ Function - 一级函数
- ✅ Generator - 惰性序列
- ✅ Lazy - 延迟求值

#### 内置函数库 (90个)

**I/O 函数 (3个)**

- Print, Println, Input

**类型函数 (4个)**

- TypeOf, ToString, ToNumber, Len

**数组函数 (13个)**

- Range, Push, Pop, Join, Reverse, Sort, Sum, Max, Min, Map, Filter, Reduce, 等

**字符串函数 (9个)**

- Split, Upper, Lower, Trim, Contains, StartsWith, EndsWith, Replace, Repeat

**字典函数 (4个)**

- Keys, Values, Has, Merge

**数学函数 (57个)**

- 基础数学: Abs, Floor, Ceil, Round, Min, Max
- 三角函数: Sin, Cos, Tan, Asin, Acos, Atan, Atan2, Sinh, Cosh, Tanh
- 对数指数: Sqrt, Pow, Exp, Ln, Log, Log2, Exp2, Expm1, Log1p
- 特殊函数: Factorial, Gamma, Erf, Clamp, Sign, Hypot
- 统计函数: Mean, Median, Variance, Std, Quantile
- 向量运算: Dot, Norm, Cross, Distance, Normalize
- 矩阵运算: Matmul, Transpose, Determinant
- 数学常数: PI, E, TAU, PHI

### 测试框架 🧪

- **单元测试**: 53 个测试
  - Lexer: 13 个
  - Parser: 7 个
  - Evaluator: 14 个
  - Environment: 6 个
  - Value: 5 个
  - Token: 2 个
  - Aether: 1 个

- **集成测试**: 61 个测试
  - 基础函数: 31 个
  - 高级数学: 22 个
  - 统计概率: 8 个

### 文档系统 📝

- ✅ **README.md** - 项目介绍
- ✅ **docs/USER_GUIDE.md** - 完整用户指南（新建）
- ✅ **DEVELOPMENT.md** - 开发文档
- ✅ **CHANGELOG.md** - 变更日志（本文件）
- ✅ 所有函数的中文 Rustdoc 注释

### 示例程序 💡

- `examples/stats_demo.aether` - 统计和概率功能演示
- 更多示例待添加...

---

## 技术债务 & 已知限制 ⚠️

### 性能限制

- 递归行列式对大矩阵（n>10）较慢
- 矩阵运算未使用 BLAS 优化
- 未实现稀疏矩阵

### 功能限制

- 仅支持简单线性回归（无多元回归）
- 概率分布种类有限（无 t 分布、F 分布等）
- 未实现特征值分解
- 未实现傅里叶变换

### 数值限制

- 浮点数精度限制（f64）
- 无任意精度算术
- 大阶乘可能溢出

---

## 路线图 🗺️

### 短期计划 (v0.2.0)

- [ ] 特征值/特征向量计算（QR 算法）
- [ ] 傅里叶变换（DFT/FFT）
- [ ] 多元线性回归
- [ ] 更多概率分布（t 分布、卡方分布）

### 中期计划 (v0.3.0)

- [ ] 数值积分（Simpson、Romberg）
- [ ] 数值微分（有限差分）
- [ ] 插值（Lagrange、样条）
- [ ] LU 分解、QR 分解

### 长期计划 (v1.0.0)

- [ ] 优化算法（梯度下降、牛顿法）
- [ ] 神经网络基础
- [ ] 时间序列分析（ARIMA）
- [ ] BLAS/LAPACK 集成
- [ ] Go 和 TypeScript 绑定
- [ ] WASM 支持

---

## 贡献者 👥

感谢所有为 Aether 做出贡献的开发者！

---

## 版本说明

版本号格式: `MAJOR.MINOR.PATCH`

- **MAJOR**: 不兼容的 API 变更
- **MINOR**: 向后兼容的功能新增
- **PATCH**: 向后兼容的问题修正

---

_最后更新: 2025年11月6日_
