# Aether

<div align="center">

## 轻量级、可嵌入的领域特定语言

[![Crates.io](https://img.shields.io/crates/v/aether.svg)](https://crates.io/crates/aether-azathoth)
[![Documentation](https://docs.rs/aether/badge.svg)](https://docs.rs/aether-azathoth/latest/aether/)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

**高性能 · 易集成 · 跨平台 · 安全优先**

</div>

---

## 📋 目录

- [概述](#-概述)
- [快速开始](#-快速开始)
- [语言特性](#-语言特性)
- [大整数支持](#-大整数支持)
- [安全模型](#-安全模型)
- [性能优化](#-性能优化)
- [许可证](#-许可证)

---

## 🎯 概述

Aether 是一个现代化、轻量级的脚本语言，设计用于嵌入到 Rust 应用程序中。

### 核心特性

- 🚀 **高性能**: 基于 Rust，带 AST 缓存和常量折叠优化
- 🔌 **易于集成**: 简单的 Rust API
- 🌍 **跨平台**: x86_64、ARM64
- ✨ **现代特性**: 闭包、Lambda 表达式、模块系统 (Import/Export)
- 📝 **简洁语法**: 易学易读，UPPER_SNAKE_CASE 命名
- 🔒 **安全优先**: 库模式默认禁用 IO，CLI 模式自动启用

### 内置函数库 (150 个内置函数)

- **基础**: I/O、类型转换、字符串/数组/字典操作
- **文件系统**: READ_FILE, WRITE_FILE, LIST_DIR, CREATE_DIR 等
- **网络**: HTTP_GET, HTTP_POST, HTTP_PUT, HTTP_DELETE
- **数学**: 线性代数、统计、概率分布、矩阵运算
- **精确计算**: 分数运算、固定精度金融计算
- **薪资计算**: 工资、加班费、个税、社保（78个函数）
- **Excel 公式兼容（规划中）**: 未来将支持公式转写/兼容，不再内置 Excel 文件读写

---

## 🚀 快速开始

### 安装

```bash
# Rust 库
cargo add aether

# 命令行工具
cargo install aether
```

### Hello World

**命令行 (IO 自动启用):**

```bash
# 创建 hello.aether
echo 'PRINTLN("Hello, Aether!")' > hello.aether

# 运行
aether hello.aether
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

### 3. 闭包

内层函数可以捕获外层作用域的变量（读取）：

```aether
Func MAKE_ADDER (BASE) {
    Func ADD (X) {
        Return (BASE + X)   // 捕获外层的 BASE
    }
    Return ADD
}

Set ADD5 MAKE_ADDER(5)
PRINTLN(ADD5(10))   // 15
```

> 注意：闭包目前只支持**读取**捕获的变量。在内层函数里对捕获变量重新 `Set` 不会影响外层作用域。

### 4. Lambda 表达式

`MAP`/`FILTER`/`REDUCE` 等高阶函数可以直接使用 `Func (参数) { ... }` 内联写法：

```aether
Set DOUBLED MAP([1, 2, 3], Func (X) { Return (X * 2) })
PRINTLN(DOUBLED)   // [2, 4, 6]

Set SUM REDUCE([1, 2, 3, 4], Func (ACC, X) { Return (ACC + X) }, 0)
PRINTLN(SUM)       // 10
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

### 8. 错误处理

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

## 🔢 大整数支持

Aether 支持任意精度的大整数运算。当整数超过 15 位时，系统会自动切换到精确计算模式，避免浮点数精度损失：

- **小整数（≤15位）**：使用高效的浮点数运算（f64）
- **大整数（>15位）**：自动切换到任意精度的 BigInt 运算

### 基本运算

```aether
Set A 3284628396498263948629734587234583548273548253487325
Set B 4728364875283754872534781253784527635487235478923587423
PRINTLN(A * B)
// 输出: 15530921538361993565152129229913877304236184424817572492058487603003384389356972658598499493820859259913475

Set C 999999999999999999999999999999
PRINTLN(C + 1)   // 1000000000000000000000000000000
PRINTLN(C - 1)   // 999999999999999999999999999998
PRINTLN(C / 2)   // 精确分数结果
```

加减乘除都保持精确；除法返回精确的分数结果。

### 位运算

支持位运算符 `&`、`|`、`^`、`<<`、`>>`，优先级采用 C 风格（移位高于加减，按位与/或/异或低于相等比较）：

```aether
PRINTLN(12 & 10)    // 8
PRINTLN(12 | 3)     // 15
PRINTLN(12 ^ 10)    // 6
PRINTLN(1 << 10)    // 1024

// 大整数同样支持
Set BIG 123456789012345678901234567890
PRINTLN(BIG >> 60)  // 107081695084
PRINTLN(BIG & 255)  // 210
```

注意：位运算要求整数操作数。`Number` 必须是整数值（`1.5 & 1` 报错），`Fraction` 的分母必须为 1。

### 科学计数法

正指数会构造**精确值**（超过阈值时自动成为大整数）：

```aether
PRINTLN(1e30)     // 1000000000000000000000000000000（精确大整数）
PRINTLN(1.5e3)    // 1500
PRINTLN(3e+2)     // 300
```

负指数降级为 f64 浮点数（可能损失精度，需要精确值请用 `TO_FRACTION`）：

```aether
PRINTLN(2e-2)     // 0.02
```

### 配置大整数阈值

默认超过 15 位的整数字面量切换为 BigInteger，可通过 API 或 CLI 调整：

```rust
let mut engine = Aether::new();
engine.set_bigint_threshold(3);   // 超过 3 位即用 BigInt
println!("{}", engine.bigint_threshold());
```

```bash
aether --bigint-threshold 3 script.aether
```

注意：AST 缓存按源码文本命中，修改阈值后已缓存的程序不会重新解析。

### 实现细节与性能

- 大整数在内部表示为 `Fraction(Ratio<BigInt>)`（分母为 1 的分数），与 `TO_FRACTION`、`FRAC_*` 等分数函数完全兼容
- 大整数字面量在**解析期**一次性构造 `BigInt`，重复求值不会重复解析字符串
- 两个大整数字面量的 `+`、`-`、`*` 会在优化期**常量折叠**
- `Number` 与 `Fraction` 混合运算时，整数值 f64 通过十进制字符串精确提升，避免 `as i64` 在超出 i64 范围（约 9.2e18）时静默截断
- 小整数仍然使用快速的浮点运算，只有大整数才触发 BigInt 计算

限制：大整数仅用于整数运算，浮点数仍使用 f64；极大的数字可能消耗大量内存；BigInt 运算比浮点数慢，但保证精确。

### 与其他语言对比

```python
# Python
>>> 3284628396498263948629734587234583548273548253487325 * 4728364875283754872534781253784527635487235478923587423
15530921538361993565152129229913877304236184424817572492058487603003384389356972658598499493820859259913475
```

```lisp
;; Common Lisp
(* 3284628396498263948629734587234583548273548253487325 4728364875283754872534781253784527635487235478923587423)
; => 15530921538361993565152129229913877304236184424817572492058487603003384389356972658598499493820859259913475
```

```aether
// Aether
Set A 3284628396498263948629734587234583548273548253487325
Set B 4728364875283754872534781253784527635487235478923587423
PRINTLN(A * B)
// 输出: 15530921538361993565152129229913877304236184424817572492058487603003384389356972658598499493820859259913475
```

Aether 提供了开箱即用的大整数支持。运行 `cargo test --test bigint_tests` 可验证大整数功能。

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

### 5. 引擎模式（Rust 嵌入高频执行）

在 Rust 应用中反复、大量执行 DSL 时，每次 `Aether::new()` 都要重建求值器、内置函数注册表和 AST 缓存。`aether::engine` 模块提供三种引擎模式应对高频调用：

#### GlobalEngine —— 线程局部单例（推荐）

```rust
use aether::engine::GlobalEngine;

// 隔离环境（推荐）：每次执行前清空变量，AST 缓存跨执行保留
let result = GlobalEngine::eval_isolated("Set X 10\n(X + 20)")?; // 30

// 非隔离：变量跨调用保留
GlobalEngine::eval("Set Y 100")?;
GlobalEngine::eval("(Y + 1)")?; // 101

GlobalEngine::clear_env();   // 手动清空环境
GlobalEngine::clear_cache(); // 手动清空 AST 缓存
```

每个线程持有独立实例（`thread_local`），线程间不共享状态。另有 `cache_stats()`、`set_optimization()` 可用。

#### EnginePool —— 引擎池（RAII 自动归还）

```rust
use aether::engine::EnginePool;

let mut pool = EnginePool::new(4);

for i in 0..100 {
    let mut engine = pool.acquire(); // 获取前自动清空环境
    let code = format!("Set X {}\n(X * 2)", i);
    engine.eval(&code)?;
} // 离开作用域自动归还；池满时 acquire 会创建临时引擎
```

#### ScopedEngine —— 闭包模式（完全隔离）

```rust
use aether::engine::ScopedEngine;

// 闭包风格：可执行多段代码并返回任意类型
let (x, y) = ScopedEngine::with(|engine| {
    engine.eval("Set X 10")?;
    engine.eval("Set Y 20")?;
    let x = engine.eval("X")?;
    let y = engine.eval("Y")?;
    Ok((x, y))
})?;

// 简化版：单次执行
let result = ScopedEngine::eval("Set X 10\n(X + 20)")?; // 30

// 信任来源的脚本可启用全部 IO 权限
let result = ScopedEngine::eval_with_all_permissions(code)?;
```

#### 模式对比

| 模式 | 环境隔离方式 | AST 缓存 | 适用场景 |
|------|------------|---------|---------|
| GlobalEngine | 执行前 `reset_env()` | ✅ 跨执行累积 | 单线程高频调用（配置解析、规则引擎） |
| EnginePool | `acquire()` 时 `reset_env()` | ✅ 每引擎独立 | 单线程内需多个引擎实例 |
| ScopedEngine | 每次新建引擎 | ❌ | 临时执行、偶尔使用、极简 API |

三种模式均为线程局部设计（Aether 内部使用 `Rc`，非 `Send`），隔离性由 `engine` 模块的单元测试保证。启用 `async` 特性后三种模式均有对应的异步变体。

#### 实测性能

Apple M2、release 构建下重复执行 `"Set X 10\nSet Y 20\n(X + Y)"`，预热后每轮 10000 次迭代、共 5 轮取中位数：

| 模式 | 单次执行耗时 | 相对性能 |
|------|-----------|---------|
| `GlobalEngine::eval_isolated` | ~22µs | 基准（AST 缓存稳态命中率 ~100%） |
| `EnginePool` acquire+eval | ~22µs | 持平 |
| `ScopedEngine::eval` | ~39µs | 1.8x 慢（每次新建引擎，无法复用缓存） |

完整演示：`cargo run --example engine_modes`

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

---

## 📚 更多文档

### 用户指南

- [调试指南](docs/DEBUG_GUIDE.md) - 调试工具、错误追踪和排错技巧
- [安全沙箱指南](docs/SANDBOX_GUIDE.md) - 权限控制、IO限制和安全最佳实践

### 专题指南

- [薪酬计算指南](docs/PAYROLL_GUIDE.md) - 工资、加班费、个税、社保计算(78个函数)
- [精确计算指南](docs/PRECISION_GUIDE.md) - 分数运算、固定精度金融计算

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

## 📖 语言参考

### 语法要点

- **块结构**：统一使用大括号 `{ }`，不支持 `EndIf`/`EndFunction` 等结束关键字
- **注释**：`//` 单行注释、`/* ... */` 块注释（不支持 `#`）
- **命名**：变量名和函数名必须使用 `UPPER_SNAKE_CASE`（函数**参数**允许小写）
- **内置函数**：全部为 `UPPER_SNAKE_CASE`，如 `PRINTLN`、`TO_STRING`、`LINEAR_REGRESSION`
- **条件表达式**：`If`/`While` 的条件需用括号包裹，如 `If (X > 10) { ... }`

```aether
// 数据类型
Set NUM 42            // Number
Set PI_TEXT "3.14"    // String
Set FLAG True         // Boolean
Set NOTHING Null      // Null
Set LIST [1, 2, 3]    // Array
Set USER {"name": "Alice", "age": 30}  // Dict

// 函数定义与调用
Func GREET (name) {
    Return ("Hello, " + name + "!")
}
PRINTLN(GREET("Aether"))

// 控制流
If (NUM > 10) {
    PRINTLN("大于10")
} Elif (NUM == 10) {
    PRINTLN("等于10")
} Else {
    PRINTLN("小于10")
}

For I In RANGE(1, 10) {
    PRINTLN(I)
}

Set I 0
While (I < 10) {
    PRINTLN(I)
    Set I (I + 1)
}
```

### 命令行工具

```bash
aether my_script.aether        # 运行脚本
aether                         # 无参数启动交互式 REPL
```

常用选项：

| 选项 | 说明 |
| ------ | ------ |
| `--check` | 只检查语法，不执行 |
| `--ast` | 显示抽象语法树 (AST) |
| `--debug` | 调试模式（打印额外运行信息） |
| `--debugger` | 启动交互式调试器（类似 GDB） |
| `--metrics` / `--metrics-json[-pretty]` | 执行后输出性能指标 |
| `--trace` / `--trace-stats` | 打印 TRACE 缓冲区内容/统计 |
| `--trace-buffer-size <N>` | 设置 TRACE 缓冲区容量 |
| `--json-error` | 出错时输出结构化 JSON 错误 |
| `--no-stdlib` | 不自动加载标准库 |

REPL 提示符为 `aether[N]>`，输入 `help` 查看帮助，`exit` 或 `quit` 退出：

```text
aether[1]> Set X 10
aether[2]> Set Y 20
aether[3]> (X + Y)
30
```

### 内置函数一览（共 150 个）

以下函数名均为实际注册名，可直接调用。

#### I/O 与调试（8 个）

```aether
PRINT, PRINTLN, INPUT
TRACE, TRACE_DEBUG, TRACE_INFO, TRACE_WARN, TRACE_ERROR
```

#### 类型（4 个）

```aether
LEN, TYPE, TO_STRING, TO_NUMBER
```

- `LEN(x)` - Array/String/Dict 的长度
- `TYPE(x)` - 类型名称："Number"、"String"、"Boolean"、"Null"、"Array"、"Dict"、"Function" 等
- `TO_NUMBER("123")` - 字符串转数字

#### 数组（10 个）

```aether
RANGE, PUSH, POP, MAP, FILTER, REDUCE
JOIN, REVERSE, SORT, SUM
```

- `RANGE(start, end, [step])` - 生成数字序列（不包含 end）
- `PUSH(arr, v)` - 末尾添加元素（原地修改）
- `MAP(arr, Func (X) { ... })` - 对每个元素应用函数
- `FILTER(arr, Func (X) { ... })` - 保留使谓词为 True 的元素
- `REDUCE(arr, Func (ACC, X) { ... }, init)` - 累积计算
- `JOIN(arr, sep)` - 连接为字符串

#### 字符串（13 个）

```aether
SPLIT, UPPER, LOWER, TRIM, CONTAINS
STARTS_WITH, ENDS_WITH, REPLACE, REPEAT
STRLEN, STRSLICE, INDEXOF, CHARAT
```

- `STRSLICE(s, start, end)` - 子串
- `INDEXOF(s, sub)` - 子串位置
- `CHARAT(s, i)` - 取指定位置字符

#### 字典（4 个）

```aether
KEYS, VALUES, HAS, MERGE
```

#### 数学（57 个）

```aether
// 基础
ABS, FLOOR, CEIL, ROUND, ROUND_TO, SIGN, CLAMP
FACTORIAL, HYPOT

// 三角函数（弧度制）
SIN, COS, TAN, ASIN, ACOS, ATAN, ATAN2
SINH, COSH, TANH

// 指数与对数
SQRT, POW, EXP, EXP2, EXPM1, LN, LOG, LOG2, LOG1P

// 特殊函数
ERF, GAMMA

// 统计
MEAN, MEDIAN, VARIANCE, STD, QUANTILE

// 概率分布与回归（NORMAL_PDF / NORMAL_CDF 支持可选 mean、std 参数）
LINEAR_REGRESSION, NORMAL_PDF, NORMAL_CDF, POISSON_PMF

// 向量
DOT, NORM, CROSS, DISTANCE, NORMALIZE

// 矩阵
MATMUL, TRANSPOSE, DETERMINANT, INVERSE

// 常量（零参数函数）
PI(), E(), TAU(), PHI()
```

#### 文件系统（7 个）

```aether
READ_FILE, WRITE_FILE, APPEND_FILE
DELETE_FILE, FILE_EXISTS, CREATE_DIR, LIST_DIR
```

#### 网络（4 个）

```aether
HTTP_GET, HTTP_POST, HTTP_PUT, HTTP_DELETE
```

#### JSON（2 个）

```aether
JSON_PARSE, JSON_STRINGIFY
```

#### 精确计算（14 个）

```aether
TO_FRACTION, TO_FLOAT, FRAC_ADD, FRAC_SUB, FRAC_MUL, FRAC_DIV
NUMERATOR, DENOMINATOR, SIMPLIFY, GCD, LCM
ADD_WITH_PRECISION, SUB_WITH_PRECISION
MUL_WITH_PRECISION, DIV_WITH_PRECISION
```

#### 薪资计算（78 个）

涵盖基本工资、加班费、个税、年终奖、社保、工作日/节假日计算等，详见 [薪酬计算指南](docs/PAYROLL_GUIDE.md)。

#### 其他

```aether
HELP            // 查看内置函数帮助（REPL 中输入 help 同效）
CLONE, SET_PRECISION
```

### 示例：统计与预测

```aether
// 销售数据分析
Set MONTHS [1, 2, 3, 4, 5, 6]
Set SALES [120, 135, 158, 172, 195, 210]

PRINTLN("平均销售额: " + TO_STRING(MEAN(SALES)))
PRINTLN("标准差: " + TO_STRING(STD(SALES)))

// 线性回归预测：返回 [斜率, 截距, R²]
Set MODEL LINEAR_REGRESSION(MONTHS, SALES)
Set SLOPE MODEL[0]
Set INTERCEPT MODEL[1]

// 预测第 7 个月
Set MONTH7 (SLOPE * 7 + INTERCEPT)
PRINTLN("预测第7个月销量: " + TO_STRING(MONTH7))
```

### 示例：质量控制（正态分布）

```aether
// 产品重量分布分析 (μ=500g, σ=5g)
Set PASS_RATE (NORMAL_CDF(510, 500, 5) - NORMAL_CDF(490, 500, 5))
PRINTLN("合格率: " + TO_STRING(PASS_RATE * 100) + "%")
```

### 示例：解线性方程组

```aether
// 2x + y = 5, x + 3y = 7  =>  x = 1.6, y = 1.8
Set A [[2, 1], [1, 3]]
Set B [[5], [7]]

Set X MATMUL(INVERSE(A), B)
PRINTLN("解: " + TO_STRING(X))
```

### 错误处理

Aether 会在解析和运行时检测以下错误，并附带源码位置信息：

- **命名错误**: 变量/函数名不符合 UPPER_SNAKE_CASE
- **解析错误**: 语法不符合预期（含行列号与源码上下文）
- **类型错误**: 操作不支持的类型
- **除零错误**: 除法或模运算分母为零
- **未定义变量**: 使用未声明的变量
- **参数错误**: 内置函数参数数量不匹配

### 性能提示

1. **数组操作**: 优先使用内置函数（MAP、FILTER、REDUCE）而不是手写循环
2. **字符串拼接**: 大量拼接时先用数组收集再 `JOIN`
3. **矩阵运算**: 大矩阵的 `DETERMINANT`/`INVERSE` 计算较慢

---

## 🎯 开发状态

### 当前版本: v0.5.3

**已完成：**

- ✅ 完整的解释器 (Lexer, Parser, Evaluator)
- ✅ 150 个内置函数
- ✅ 增强的错误报告
- ✅ 严格的命名约定
- ✅ AST 缓存和性能优化
- ✅ 100+ 测试（持续维护）
- ✅ 无IO Trace
- ✅ 实现注入、Import和Export

**计划中：**

- 🔄 完整的尾递归优化
- 🔄 JIT 编译器
- 🔄 试算 - 在内部变量不确定的情况下，通过自动赋值为0或""来让代码跑通，用于代码初期简单测试

---

## 📄 许可证

根据 GNU General Public License Version 3 (GPLv3) 许可，附加例外条款（[LICENSE](LICENSE)）。

**注意**：所有为该项目添加代码的成员都必须签署`DCO协议`[DCO](DCO.md)

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
