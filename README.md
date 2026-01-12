# Aether

<div align="center">

## 轻量级、可嵌入的领域特定语言 (DSL)

[![Crates.io](https://img.shields.io/crates/v/aether.svg)](https://crates.io/crates/aether-azathoth)
[![Documentation](https://docs.rs/aether/badge.svg)](https://docs.rs/aether-azathoth/latest/aether/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)

**高性能 · 易集成 · 跨平台 · 安全优先**

</div>

---

## 📋 目录

- [概述](#-概述)
- [快速开始](#-快速开始)
- [语言特性](#-语言特性)
- [安全模型](#-安全模型)
- [性能优化](#-性能优化)
- [语言绑定](#-语言绑定)
- [许可证](#-许可证)

---

## 🎯 概述

Aether 是一个现代化、轻量级的脚本语言，设计用于嵌入到 Rust、Go 和 TypeScript 应用程序中。

### 核心特性

- 🚀 **高性能**: 基于 Rust，带 AST 缓存和常量折叠优化
- 🔌 **易于集成**: 简单的 API，支持 Rust/Go/TypeScript
- 🌍 **跨平台**: x86_64、ARM64、WebAssembly
- ✨ **现代特性**: Generator、惰性求值、闭包
- 📝 **简洁语法**: 易学易读，UPPER_SNAKE_CASE 命名
- 🔒 **安全优先**: 库模式默认禁用 IO，CLI 模式自动启用

### 标准库 (200+ 函数)

- **基础**: I/O、类型转换、字符串/数组/字典操作
- **文件系统**: READ_FILE, WRITE_FILE, LIST_DIR, CREATE_DIR 等
- **网络**: HTTP_GET, HTTP_POST, HTTP_PUT, HTTP_DELETE
- **数学**: 线性代数、统计、概率分布、矩阵运算
- **精确计算**: 分数运算、固定精度金融计算
- **薪资计算**: 工资、加班费、个税、社保（78个函数）
- **报表生成**: Excel 创建/写入/保存、数据格式化（部分功能规划中）

---

## 🚀 快速开始

### 安装

```bash
# Rust 库
cargo add aether

# 命令行工具
cargo install aether

# Go
go get github.com/yourusername/aether-go

# TypeScript/aether
npm install @yourusername/aether
```

### Hello World

**命令行 (IO 自动启用):**

```bash
# 创建 hello.aether
echo 'PRINTLN("Hello, Aether!")' > hello.aether

# 运行
aether hello.aether
```

### Python → Aether（CLI 转译）

当你希望把现有的 Python 逻辑迁移到 Aether（并继续运行在 Aether 引擎里）时，可以直接用 CLI 转译。

注意：默认会拒绝 `numpy`、文件/网络 IO、以及 `print/input`（更适合 DSL 场景的安全默认值）。

```bash
# demo.py
cat > demo.py <<'PY'
x = [1, 2, 3]
y = {"a": 1, "b": 2}
z = y["a"] + 12.34
z
PY

# 转译为 Aether（输出到 stdout）
aether --ouroboros demo.py > demo.aether

# 查看结果
cat demo.aether

# 也支持从 stdin 读取（用 '-' 表示 stdin）
cat demo.py | aether --ouroboros - > demo.aether

# 转译后直接运行
aether --ouroboros --run demo.py
```

如果你要在 Rust 服务端把 Python 代码作为输入，然后先转成 Aether 再执行，可以直接调用转译 API：

> 说明：本仓库当前默认已开启 `pytranspile` feature。若你的依赖显式关掉了默认 feature，请在 `Cargo.toml` 开启 `features = ["pytranspile"]`。

```rust
use aether::{Aether, Value};
use aether::pytranspile::{python_to_aether, TranspileOptions};

fn main() -> Result<(), String> {
    let py = r#"
z = 1 + 2
z
"#;

    let res = python_to_aether(py, &TranspileOptions::default());
    if res.diagnostics.has_errors() {
        return Err(format!("{}", res.diagnostics));
    }

    let code = res.aether.unwrap();

    // DSL 默认建议用 Aether::new()（IO 默认禁用）
    let mut engine = Aether::new();
    let v = engine.eval(&code)?;
    if v != Value::Null {
        println!("{}", v);
    }
    Ok(())
}
```

**Rust 嵌入 (默认安全):**

```rust
use aether::Aether;

fn main() {
    let mut engine = Aether::new(); // IO 默认禁用
    
    let result = engine.eval(r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#).unwrap();
    
    println!("结果: {}", result); // 输出: 30
}
```

**启用 IO (可选):**

```rust
use aether::{Aether, IOPermissions};

// 完全启用 IO
let mut engine = Aether::with_all_permissions();

// 或仅启用文件系统
let permissions = IOPermissions {
    filesystem_enabled: true,
    network_enabled: false,
};
let mut engine = Aether::with_permissions(permissions);

engine.eval(r#"
    WRITE_FILE("output.txt", "Hello!")
    PRINTLN(READ_FILE("output.txt"))
"#).unwrap();
```

### 无 IO 调试：TRACE（推荐用于 DSL）

在 DSL 场景下通常会禁用 IO（不能 `PRINT/PRINTLN/INPUT`），但你仍然可以通过 `TRACE(...)` **安全记录调试信息**：

- `TRACE(...)` 不会写 stdout / 文件 / 网络
- 它只会把信息追加到引擎的**内存缓冲区**
- 宿主（Rust）可通过 `take_trace()` 读取并自行输出/写日志

补充：

- 每条 trace 会自动带递增序号前缀：`#1 ...`, `#2 ...`
- 可选标签：`TRACE("label", x, y)` 会记录为 `[#N] [label] x y`
- 缓冲区有上限（默认 1024 条）；超出会丢弃最旧条目

```aether
Set X [1, 2, 3]
Set Y {"a": 12}
Set Z (Y["a"] + 3)

TRACE("X=" + TO_STRING(X))
TRACE({"y": Y, "z": Z})

Z
```

Rust 侧读取 trace：

```rust
use aether::Aether;

fn main() -> Result<(), String> {
    let mut engine = Aether::new(); // DSL 模式：默认无 IO

    let v = engine.eval(r#"
        Set X [1, 2, 3]
        TRACE("hello")
        TRACE(X)
        42
    "#)?;

    let trace = engine.take_trace();
    // 这里由宿主决定如何处理（打印/结构化日志/埋点）
    // e.g. ["#1 hello", "#2 [dbg] 1 2", ...]
    println!("trace={:?}", trace);
    println!("result={}", v);
    Ok(())
}

```

### 宿主注入与隔离执行（推荐 DSL / 模块化 B 方案）

`Import/Export` 的运行时模块系统已实现，但 **DSL 场景默认禁用导入**（安全优先）。
在 DSL 工程里仍更推荐 **B 方案**：由宿主统一管理“模块/函数库”（例如从数据库取出 Aether 函数定义），在每次执行前注入。

为支持“像 PyO3 一样把数据/函数放到 globals，然后执行脚本，并且执行完自动清空不污染”，引擎提供：

- `engine.set_global(name, Value)`：直接注入 Rust 侧数据（无需 `eval`）
- `engine.with_isolated_scope(|engine| ...)`：闭包作用域，闭包结束后自动丢弃本次注入/定义
- `engine.reset_env()`：强制清空整个环境（会清掉通过 `eval` 加载的 stdlib/函数）

### 文件模块（Import/Export，通用语言/CLI 场景）

在通用语言/CLI 场景下，你可以使用 `Import/Export` 把代码拆成多个 `.aether` 文件。

- DSL（`Aether::new()`）默认 resolver 为禁用：脚本里 `Import` 会报错
- CLI 会显式启用文件系统 resolver，并自动以“脚本所在目录”作为相对导入的 base
- 别名关键字为 `As`（也兼容旧写法 `as`）
- 命名空间导入：`Import M From "./math"` 会把模块导出绑定为一个 Dict 到 `M`（可用 `M["ADD"]` 访问）
- 具名导入推荐使用 `{}`：例如 `Import {ADD} From "./math"`（避免与命名空间导入歧义）

最小示例见：

- [examples/module_import/main.aether](examples/module_import/main.aether)
- [examples/module_import/math.aether](examples/module_import/math.aether)

命令行运行：

```bash
aether examples/module_import/main.aether
```

Rust 侧以文件方式执行（方案1：`eval_file` 只管理 base_dir，上层显式启用 resolver）：

```rust
use aether::{Aether, FileSystemModuleResolver};

fn main() -> Result<(), String> {
    let mut engine = Aether::new();
    engine.set_module_resolver(Box::new(FileSystemModuleResolver::default()));
    engine.eval_file("examples/module_import/main.aether")?;
    Ok(())
}
```

更多设计与规划：

- [docs/MODULE_SYSTEM_DESIGN.md](docs/MODULE_SYSTEM_DESIGN.md)
- [docs/ROADMAP.md](docs/ROADMAP.md)

最小示例：Rust 数据 + DB 函数 + 脚本（闭包结束自动清理）：

```rust
use aether::{Aether, Value};
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let mut engine = Aether::new(); // DSL：默认无 IO

    // 模拟从 DB 取出来的一堆 Aether 函数定义
    let db_funcs: Vec<String> = vec![
        r#"Func ADD_TAX (amount, rate) { Return (amount * (1 + rate)) }"#.to_string(),
        r#"Func APPLY_DISCOUNT (subtotal, coupon) { Return (subtotal - coupon) }"#.to_string(),
    ];

    let script = r#"
Set net APPLY_DISCOUNT(INPUT[\"subtotal\"], INPUT[\"coupon\"])
ADD_TAX(net, RATE)
"#;

    let out = engine.with_isolated_scope(|engine| {
        // 注入 Rust 数据（不用 eval）
        engine.set_global("RATE", Value::Number(0.08));

        let mut input = HashMap::new();
        input.insert("subtotal".to_string(), Value::Number(1000.0));
        input.insert("coupon".to_string(), Value::Number(50.0));
        engine.set_global("INPUT", Value::Dict(input));

        // 注入 DB 函数（逐条 eval）
        for f in &db_funcs {
            engine.eval(f)?;
        }

        // 执行脚本
        engine.eval(script)
    })?;

    println!("out={}", out);
    Ok(())
}
```

---

## 📚 语言特性

### 1. 基础语法

```aether
// 变量 (必须 UPPER_SNAKE_CASE)
Set COUNT 10
Set MESSAGE "Hello, Aether"
Set NUMBERS [1, 2, 3, 4, 5]
Set USER {"name": "Alice", "age": 30}

// 函数
Func ADD (A, B) {
    Return (A + B)
}

Set RESULT ADD(5, 3)
PRINTLN("5 + 3 =", RESULT)
```

### 2. 控制流

```aether
// If-Else
Func ABS (X) {
    If (X < 0) {
        Return (0 - X)
    } Else {
        Return X
    }
}

// For 循环
For I In RANGE(0, 5) {
    PRINTLN("数字:", I)
}

// While 循环
Set I 0
While (I < 5) {
    PRINTLN(I)
    Set I (I + 1)
}
```

### 3. Generator (惰性序列)

```aether
Generator FIBONACCI (LIMIT) {
    Set A 0
    Set B 1
    Set COUNT 0
    
    While (COUNT < LIMIT) {
        Yield A
        Set NEXT (A + B)
        Set A B
        Set B NEXT
        Set COUNT (COUNT + 1)
    }
}

// 使用
For NUM In FIBONACCI(10) {
    PRINTLN(NUM)
}
```

### 4. 惰性求值

```aether
// 延迟计算，仅在需要时执行
Lazy EXPENSIVE_DATA (
    PRINTLN("正在加载大数据集...")
    Return READ_FILE("big_data.json")
)

// 数据仅在访问时加载
If (NEEDS_ANALYSIS) {
    Set DATA EXPENSIVE_DATA  // 此时才执行
    PROCESS(DATA)
}
```

### 5. 精确和精度算术

```aether
// 问题：浮点精度
Set A 0.1
Set B 0.2
PRINTLN(A + B)  // 可能显示: 0.30000000000000004

// 解决方案：分数运算（精确）
Set FA TO_FRACTION(0.1)
Set FB TO_FRACTION(0.2)
Set FC FRAC_ADD(FA, FB)
PRINTLN(FC)           // 显示: 3/10
PRINTLN(TO_FLOAT(FC)) // 显示: 0.3

// 金融计算（固定精度）
Set PRICE1 19.99
Set PRICE2 29.99
Set TOTAL ADD_WITH_PRECISION(PRICE1, PRICE2, 2)
PRINTLN(TOTAL)  // 显示: 49.98

Set TAX MUL_WITH_PRECISION(TOTAL, 0.08, 2)
PRINTLN(TAX)    // 显示: 4.00
```

### 6. 文件系统操作

```aether
// CLI 模式：自动工作
WRITE_FILE("data.txt", "Hello, World!")
Set CONTENT READ_FILE("data.txt")
PRINTLN(CONTENT)

If FILE_EXISTS("data.txt") {
    PRINTLN("文件存在!")
    DELETE_FILE("data.txt")
}

// 目录操作
CREATE_DIR("output")
Set FILES LIST_DIR(".")
For FILE In FILES {
    PRINTLN(FILE)
}
```

### 7. 网络操作

```aether
// HTTP GET
Set RESPONSE HTTP_GET("https://api.github.com")
PRINTLN(RESPONSE)

// HTTP POST
Set DATA '{"name": "test"}'
Set RESULT HTTP_POST(
    "https://api.example.com/data",
    DATA,
    "application/json"
)
PRINTLN(RESULT)
```

### 8. 报表生成 (🆕)

```aether
// Excel 操作
Set WORKBOOK EXCEL_CREATE()
EXCEL_WRITE_CELL(WORKBOOK, "Sheet1", 0, 0, "姓名")
EXCEL_WRITE_CELL(WORKBOOK, "Sheet1", 0, 1, "销售额")
EXCEL_WRITE_CELL(WORKBOOK, "Sheet1", 0, 2, "完成率")
EXCEL_WRITE_CELL(WORKBOOK, "Sheet1", 1, 0, "张三")
EXCEL_WRITE_CELL(WORKBOOK, "Sheet1", 1, 1, 120000)
EXCEL_WRITE_CELL(WORKBOOK, "Sheet1", 1, 2, 0.95)
EXCEL_SAVE(WORKBOOK, "report.xlsx")

// 数据格式化
Set AMOUNT 1234567.89
PRINTLN(FORMAT_NUMBER(AMOUNT, 2))         // "1,234,567.89"
PRINTLN(FORMAT_CURRENCY(AMOUNT, "¥", 2)) // "¥1,234,567.89"
PRINTLN(FORMAT_PERCENT(0.1234, 2))        // "12.34%"
```

### 9. 错误处理

```aether
// 错误示例
Set myVar 10
// ❌ 错误: 变量名必须使用全大写字母和下划线
// 正确: Set MY_VAR 10

Set RESULT (X + Y
// ❌ 错误: Parse error at line 1, column 18: Expected RightParen

// 正确
Set RESULT (X + Y)
```

---

## 🔒 安全模型

### CLI 模式 vs 库模式

| 模式 | IO 状态 | 使用场景 |
|------|---------|----------|
| CLI | 默认启用 | 直接运行脚本，用户明确信任 |
| 库 | 默认禁用 | 嵌入应用，脚本可能不可信 |

### 权限控制

```rust
use aether::{Aether, IOPermissions};

// 1. 无 IO（最安全，默认）
let mut engine = Aether::new();

// 2. 仅文件系统
let permissions = IOPermissions {
    filesystem_enabled: true,
    network_enabled: false,
};
let mut engine = Aether::with_permissions(permissions);

// 3. 完全权限
let mut engine = Aether::with_all_permissions();
```

### 命名约定强制

所有变量、函数、参数必须使用 `UPPER_SNAKE_CASE`：

```aether
// ✅ 正确
Set MY_VARIABLE 10
Func CALCULATE_TOTAL (PRICE, TAX_RATE) { }

// ❌ 错误
Set myVariable 10      // 会报错
Func calculateTotal () // 会报错
```

---

## ⚡ 性能优化

Aether 引入了多项性能优化：

### 1. AST 缓存 (50-140x 加速)

自动缓存已解析的代码，避免重复解析：

```rust
let mut engine = Aether::new();
let code = "Set X 10\n(X + 20)";

// 第一次：解析 + 执行
engine.eval(code)?; // ~400µs

// 第二次：缓存命中 + 执行
engine.eval(code)?; // ~2.8µs (142x 快!)

// 查看缓存统计
println!("{}", engine.cache_stats());
// 输出: 命中率: 50.0%, 加速比: 142x
```

### 2. 常量折叠

编译时计算常量表达式：

```aether
// 优化前
Set X (2 + 3 * 4)

// 优化后（自动）
Set X 14
```

### 3. 死代码消除

移除永不执行的代码：

```aether
// 优化前
While False {
    PRINTLN("永远不执行")
}

// 优化后（自动删除整个循环）
```

### 4. 环境管理优化

- HashMap 预分配容量
- 热路径/冷路径分离
- 环境对象池复用
- **结果**: 变量访问快 10-15%

### 自定义优化选项

```rust
let mut engine = Aether::new();

// 控制优化
engine.set_optimization(
    true,  // 常量折叠
    true,  // 死代码消除
    false  // 尾递归优化（部分完成）
);
```

### 性能测试

```bash
# 运行基准测试
cargo bench

# 快速测试
cargo bench -- --sample-size 10

# 对比优化效果
cargo bench -- --save-baseline before
# 进行优化...
cargo bench -- --baseline before
```

---

## 🔗 语言绑定

### Go

Aether 提供完整的 Go 语言绑定,支持线程安全、变量操作、追踪调试等高级功能。

```bash
# 安装
go get github.com/xiaozuhui/aether/bindings/go@latest
```

```go
package main

import (
    "fmt"
    aether "github.com/xiaozuhui/aether/bindings/go"
)

func main() {
    engine := aether.New()
    defer engine.Close()

    // 从 Go 设置变量
    engine.SetGlobal("name", "Alice")

    result, err := engine.Eval(`
        TRACE_DEBUG("api", "Processing")
        ("Hello, " + name)
    `)

    if err != nil {
        fmt.Println("Error:", err)
        return
    }

    fmt.Println("Result:", result) // Hello, Alice

    // 获取追踪
    traces, _ := engine.TakeTrace()
    for _, trace := range traces {
        fmt.Println(trace)
    }
}
```

**特性:**
- ✅ 线程安全,支持并发
- ✅ 变量操作 (SetGlobal/GetGlobal)
- ✅ 追踪与调试 (TRACE/TakeTrace)
- ✅ 执行限制与缓存控制

详细文档: [bindings/go/README.md](bindings/go/README.md)

### TypeScript/aether

```typescript
import { Aether } from '@yourusername/aether';

async function main() {
    const engine = new Aether();
    await engine.init();
    
    const result = engine.eval(`
        Set X 10
        Set Y 20
        Return (X + Y)
    `);
    
    console.log('Result:', result); // 30
}

main();
```

---

## 🛠️ 开发与测试

### 构建

```bash
# 克隆仓库
git clone https://github.com/yourusername/aether.git
cd aether

# 构建
cargo build --release

# 运行测试
cargo test

# 运行所有测试（包括集成测试）
cargo test --all
```

### 测试覆盖

- ✅ **100+ 测试**（单元/集成/脚本测试）
- ✅ 完整的解释器测试（Lexer, Parser, Evaluator）
- ✅ 所有内置函数测试
- ✅ 错误处理和命名约定测试
- ✅ 性能基准测试

### 基准测试

```bash
# 运行所有基准测试
cargo bench

# 查看结果
open target/criterion/report/index.html

# 快速基准测试
./scripts/bench.sh quick

# 特定类别
./scripts/bench.sh arithmetic
```

**基准覆盖：**

- 算术运算、变量操作、函数调用
- 控制流、数据结构、解析性能
- 不同程序规模（小/中/大型）

---

## 📖 内置函数速查

### I/O 操作

```aether
PRINT, PRINTLN, INPUT
```

### 文件系统

```aether
READ_FILE, WRITE_FILE, APPEND_FILE
DELETE_FILE, FILE_EXISTS, CREATE_DIR
LIST_DIR, DELETE_DIR, FILE_SIZE
```

### 网络

```aether
HTTP_GET, HTTP_POST, HTTP_PUT, HTTP_DELETE
```

### 报表生成 (🆕)

```aether
// 说明：部分 EXCEL_* / FORMAT_DATE 当前版本为占位符（调用会返回“尚未实现”）

// Excel
EXCEL_CREATE, EXCEL_WRITE_CELL, EXCEL_SAVE
EXCEL_WRITE_ROW, EXCEL_WRITE_COLUMN, EXCEL_WRITE_TABLE
EXCEL_READ_SHEET, EXCEL_READ_CELL, EXCEL_READ_RANGE, EXCEL_GET_SHEETS

// 数据格式化
FORMAT_NUMBER, FORMAT_CURRENCY, FORMAT_PERCENT
FORMAT_DATE
```

### 类型转换

```aether
TO_STRING, TO_NUMBER, TYPE_OF
TO_ARRAY, TO_DICT, IS_NULL
```

### 数组操作

```aether
PUSH, POP, SHIFT, UNSHIFT
MAP, FILTER, REDUCE, SORT
FIND, INCLUDES, JOIN, SLICE
```

### 字符串操作

```aether
LEN, SPLIT, TRIM, UPPER, LOWER
REPLACE, SUBSTRING, STARTS_WITH, ENDS_WITH
```

### 数学函数

```aether
ABS, SQRT, POW, SIN, COS, TAN
MIN, MAX, SUM, AVG, MEDIAN
STDEV, VARIANCE, CORRELATION
LINEAR_REGRESSION, MATRIX_INVERSE
```

### 精确计算

```aether
TO_FRACTION, FRAC_ADD, FRAC_SUB
FRAC_MUL, FRAC_DIV, TO_FLOAT
ADD_WITH_PRECISION, SUB_WITH_PRECISION
MUL_WITH_PRECISION, DIV_WITH_PRECISION
```

### 薪资计算 (78 个函数)

```aether
// 基本工资
HOURLY_TO_DAILY, DAILY_TO_MONTHLY
MONTHLY_TO_ANNUAL, ANNUAL_TO_MONTHLY

// 加班费
CALC_WEEKDAY_OVERTIME  // 1.5x
CALC_WEEKEND_OVERTIME  // 2x
CALC_HOLIDAY_OVERTIME  // 3x

// 个税
CALC_PERSONAL_TAX      // 7级累进
CALC_BONUS_TAX         // 年终奖税

// 社保
CALC_SOCIAL_INSURANCE
CALC_HOUSING_FUND
```

---

## 🎯 开发状态

### 当前版本: v0.4.3

**已完成：**

- ✅ 完整的解释器 (Lexer, Parser, Evaluator)
- ✅ 190+ 内置函数
- ✅ 增强的错误报告
- ✅ 严格的命名约定
- ✅ AST 缓存和性能优化
- ✅ 100+ 测试（持续维护）
- ✅ python转译
- ✅ 无IO Trace
- ✅ 实现注入、Import和Export

**计划中：**

- 🔄 完整的尾递归优化
- 🔄 JIT 编译器
- 🔄 绑定golang环境
- 🔄 绑定typescript环境
- 🔄 Python 绑定
- 🔄 试算 - 在内部变量不确定的情况下，通过自动赋值为0或""来让代码跑通，用于代码初期简单测试

---

## 📄 许可证

根据 Apache License 2.0 许可（[LICENSE-APACHE](LICENSE-APACHE) 或 <http://www.apache.org/licenses/LICENSE-2.0）。>

---

## 🙏 致谢

Aether 的灵感来自：

- [Lua](https://www.lua.org/) - 可嵌入的脚本语言
- [Rhai](https://rhai.rs/) - Rust 的嵌入式脚本
- [Crafting Interpreters](https://craftinginterpreters.com/) - 关于解释器设计的优秀书籍

---

## 📬 联系方式

- GitHub Issues: [提交问题](https://github.com/xiaozuhui/aether/issues)
- Email: [邮箱](xiaozuhui@outlook.com)

---

<div align="center">

**由 Aether 贡献者用 ❤️ 制作**

[⬆ 返回顶部](#aether)

</div>
