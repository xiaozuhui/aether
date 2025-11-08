# 变更日志 (Changelog)

记录 Aether 项目的所有重要变更和里程碑。

---

## [Unreleased]

### 新增功能 ✨

#### 标准库数据结构扩展 � (NEW)

Aether 标准库新增 5 个数据结构和算法库，120+ 个新函数！

**新增库**：

- **Set（集合）**：`stdlib/set.aether` - 集合运算、去重、关系判断
- **Queue（队列）**：`stdlib/queue.aether` - FIFO 队列、批量操作
- **Stack（栈）**：`stdlib/stack.aether` - LIFO 栈、栈操作、表达式求值
- **Heap（堆）**：`stdlib/heap.aether` - 最小堆/最大堆、优先级队列、Top K
- **Sorting（排序）**：`stdlib/sorting.aether` - 9 种排序算法（快排、归并、堆排序等）

**新增示例**：

- `stdlib/examples/set_demo.aether` - 集合操作示例
- `stdlib/examples/queue_demo.aether` - 队列应用示例
- `stdlib/examples/stack_demo.aether` - 栈应用示例（括号匹配、表达式求值）
- `stdlib/examples/heap_demo.aether` - 堆应用示例（优先级队列）
- `stdlib/examples/sorting_demo.aether` - 排序算法对比
- `stdlib/examples/data_structures_demo.aether` - 8 个综合应用场景

**特性**：

- 纯函数式、不可变数据结构
- 统一的命名规范
- 详细的文档和注释
- 实际应用场景演示

**文档**：

- 查看 `docs/STDLIB_EXTENSION_SUMMARY.md` 了解详情
- 更新了 `stdlib/README.md` 包含所有新库

#### 类和接口设计方案 🎨 (DESIGN)

为 Aether 引入面向对象特性制定了详细设计方案！

**设计文档**：`docs/CLASS_INTERFACE_DESIGN.md`

**核心设计**：

- 基于字典的轻量级类实现
- 单继承 + 多接口
- 简洁的语法，与 Aether 风格一致
- 渐进式引入，保持向后兼容

**计划特性**：

- 类定义（Class）和实例化
- 继承（Extends）和方法重写
- 接口（Interface）和实现（Implements）
- 静态方法、访问控制、属性

**实施路线图**：分 5 个阶段，预计 11-15 周完成

#### 大整数支持 🔢

Aether 现在支持任意精度的大整数运算！

- **自动检测**：超过 15 位的整数自动使用 BigInt 进行精确计算
- **精确运算**：加法、减法、乘法、除法都支持任意大的整数
- **无缝集成**：大整数作为分数（Fraction）内部表示，与现有功能完全兼容
- **性能优化**：小整数（≤15位）仍使用快速的浮点运算

**示例**：

```aether
Set A 3284628396498263948629734587234583548273548253487325
Set B 4728364875283754872534781253784527635487235478923587423
Set RESULT (A * B)
PRINTLN(RESULT)
// 输出精确结果，无精度损失
```

**文档**：

- `docs/BIGINT_GUIDE.md` - 大整数使用指南
- `tests/bigint_tests.rs` - 完整的单元测试

#### 报表生成模块 📊 (NEW)

添加了完整的报表生成和文档处理功能模块：

**已实现功能 (3个)**

- `FORMAT_NUMBER(number, decimals, use_separator)` - 数字格式化（支持千分位分隔符）
- `FORMAT_CURRENCY(amount, symbol, decimals)` - 货币格式化
- `FORMAT_PERCENT(number, decimals)` - 百分比格式化

**规划功能 (67个接口已定义)**

- Excel 操作：读取、写入、格式化、图表（45个函数）
- Word 文档：创建、编辑、模板（9个函数）
- PDF 生成：创建、添加内容（5个函数）
- 数据处理：透视表、分组、聚合（8个函数）

**文档**

- `docs/REPORT_GUIDE.md` - 完整用户指南（510行）
- `docs/REPORT_IMPLEMENTATION_PLAN.md` - 技术实现计划（360行）
- `docs/REPORT_QUICKSTART.md` - 快速开始指南（300行）
- `docs/REPORT_SUMMARY.md` - 功能总结（350行）
- `examples/report_demo.aether` - 演示示例（180行）

**下一步**

- 添加 Excel 读写功能（需要 calamine 和 rust_xlsxwriter）
- 实现 Word 文档生成（需要 docx-rs）
- 添加日期格式化（需要 chrono）

#### IO功能与安全控制 🔒

新增文件系统和网络IO功能，**默认禁用以确保安全性**：

**文件系统函数 (7个)**

- READ_FILE, WRITE_FILE, APPEND_FILE, DELETE_FILE
- FILE_EXISTS, LIST_DIR, CREATE_DIR

**网络函数 (4个)**

- HTTP_GET, HTTP_POST, HTTP_PUT, HTTP_DELETE

**安全特性**

- `IOPermissions` 配置结构：控制IO权限
- 默认禁用所有IO操作（安全第一）
- 可选择性启用文件系统或网络权限
- 支持 `Aether::with_permissions()` 自定义配置
- 支持 `Aether::with_all_permissions()` 启用所有IO

使用示例：

```rust
use aether::{Aether, IOPermissions};

// 默认：IO禁用（安全模式）
let mut engine = Aether::new();

// 选择性启用文件系统
let mut perms = IOPermissions::default();
perms.filesystem_enabled = true;
let mut engine = Aether::with_permissions(perms);

// 启用所有IO
let mut engine = Aether::with_all_permissions();
```

#### 薪酬计算模块 💰

新增完整的薪酬计算模块，共78个函数，涵盖：

**基本工资计算 (7个)**

- CALC_HOURLY_PAY, CALC_DAILY_PAY, CALC_MONTHLY_FROM_HOURLY
- CALC_ANNUAL_SALARY, CALC_BASE_SALARY
- CALC_GROSS_SALARY, CALC_NET_SALARY

**加班费计算 (5个)**

- CALC_OVERTIME_PAY, CALC_WEEKDAY_OVERTIME (1.5倍)
- CALC_WEEKEND_OVERTIME (2倍), CALC_HOLIDAY_OVERTIME (3倍)
- CALC_TOTAL_OVERTIME

**个人所得税 (6个)**

- CALC_PERSONAL_TAX, CALC_TAXABLE_INCOME
- CALC_ANNUAL_BONUS_TAX, CALC_EFFECTIVE_TAX_RATE
- CALC_GROSS_FROM_NET, CALC_TAX_REFUND

**社保公积金 (10个)**

- CALC_PENSION_INSURANCE, CALC_MEDICAL_INSURANCE
- CALC_UNEMPLOYMENT_INSURANCE, CALC_HOUSING_FUND
- CALC_SOCIAL_INSURANCE, ADJUST_SOCIAL_BASE
- CALC_INJURY_INSURANCE, CALC_MATERNITY_INSURANCE
- CALC_SOCIAL_BASE_LOWER, CALC_SOCIAL_BASE_UPPER

**考勤扣款 (7个)**

- CALC_ATTENDANCE_RATE, CALC_LATE_DEDUCTION
- CALC_EARLY_LEAVE_DEDUCTION, CALC_ABSENT_DEDUCTION
- CALC_LEAVE_DEDUCTION, CALC_SICK_LEAVE_PAY
- CALC_UNPAID_LEAVE_DEDUCTION

**奖金计算 (6个)**

- CALC_PERFORMANCE_PAY, CALC_ANNUAL_BONUS
- CALC_ATTENDANCE_BONUS, CALC_SALES_COMMISSION
- CALC_PROJECT_BONUS, CALC_13TH_SALARY

**津贴补贴 (7个)**

- CALC_MEAL_ALLOWANCE, CALC_TRANSPORT_ALLOWANCE
- CALC_COMMUNICATION_ALLOWANCE, CALC_HOUSING_ALLOWANCE
- CALC_HIGH_TEMP_ALLOWANCE, CALC_NIGHT_SHIFT_ALLOWANCE
- CALC_POSITION_ALLOWANCE

**薪资折算转换 (12个)**

- ANNUAL_TO_MONTHLY, MONTHLY_TO_ANNUAL
- DAILY_TO_MONTHLY, MONTHLY_TO_DAILY
- HOURLY_TO_MONTHLY, MONTHLY_TO_HOURLY
- PRORATE_BY_NATURAL_DAYS, PRORATE_BY_LEGAL_DAYS (21.75天)
- PRORATE_BY_WORKDAYS
- CALC_ONBOARDING_SALARY, CALC_RESIGNATION_SALARY
- CALC_14TH_SALARY

**日期时间计算 (12个)**

- CALC_NATURAL_DAYS, GET_LEGAL_PAY_DAYS (21.75天)
- CALC_WORKDAYS, CALC_WEEKEND_DAYS, CALC_HOLIDAY_DAYS
- IS_WORKDAY, IS_WEEKEND, IS_HOLIDAY
- CALC_WORK_HOURS, CALC_MONTHLY_WORK_HOURS
- CALC_ANNUAL_WORKDAYS, CALC_ANNUAL_PAY_DAYS

**统计分析 (6个)**

- CALC_SALARY_AVERAGE, CALC_SALARY_MEDIAN
- CALC_SALARY_RANGE, CALC_PERCENTILE
- CALC_SALARY_STD_DEV, CALC_SALARY_DISTRIBUTION

#### 增强错误报告系统

- **精确定位**：所有语法错误现在都会显示行号和列号
- **详细描述**：提供具体的错误原因和上下文信息
- **命名规范强制**：所有变量名、函数名和参数名必须使用 UPPER_SNAKE_CASE
- **关键字保护**：保留 PascalCase 用于关键字（如 Function、Let、If 等）

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

#### 增强的错误报告系统

- **位置信息**: 所有解析错误现在包含行号和列号
- **详细错误消息**: 提供清晰的错误原因和建议
- **错误类型**:
  - `ParseError::UnexpectedToken` - 包含期望和实际的标记，以及位置
  - `ParseError::InvalidIdentifier` - 标识符命名规范错误
  - `ParseError::InvalidExpression` - 表达式语法错误
  - `ParseError::UnexpectedEOF` - 文件意外结束

#### 强制命名规范

- **UPPER_SNAKE_CASE**: 变量名、函数名、参数名必须全部使用大写字母和下划线
- **编译时检查**: 在解析阶段就检查命名规范，立即报错
- **清晰提示**: 错误信息包含正确的命名示例

### 改进 🔧

- **Value 类型扩展**: 添加 `Fraction(Ratio<BigInt>)` 变体
- **词法分析器**: 添加行号和列号跟踪
- **解析器**: 增强的位置跟踪和错误处理
- **文档完善**:
  - `docs/PRECISION_GUIDE.md` - 精度计算指南
  - `docs/ERROR_REPORTING.md` - 错误报告和命名规范指南
- **示例程序**:
  - `examples/precision_simple.aether` - 基础示例
  - `examples/precision_complete.aether` - 完整示例
  - `examples/precision_demo.aether` - 综合演示
  - `examples/test_error_reporting.aether` - 错误报告测试
  - `examples/test_correct_naming.aether` - 正确命名示例
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
