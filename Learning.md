# Aether 学习文档

> 本文面向希望**快速上手 Aether**（CLI 使用）以及希望把 Aether **嵌入到 Rust 应用**作为 DSL 的开发者。
>
> 当前仓库版本以 `Cargo.toml` 为准：`aether-azathoth v0.4.2`（crate 名为 `aether`，二进制名为 `aether`）。

---

## 目录

- [1. Aether 是什么](#1-aether-是什么)
- [2. 安装与构建](#2-安装与构建)
- [3. 命令行使用（CLI）](#3-命令行使用cli)
- [4. 交互式使用（REPL）](#4-交互式使用repl)
- [5. 语言速成：语法与约定](#5-语言速成语法与约定)
- [6. 核心语法：变量、函数、控制流](#6-核心语法变量函数控制流)
- [7. 数据结构：数组与字典](#7-数据结构数组与字典)
- [8. 内置函数（Builtins）与权限模型](#8-内置函数builtins与权限模型)
- [9. 标准库（stdlib）：模块与加载方式](#9-标准库stdlib模块与加载方式)
- [10. 精确计算与大整数](#10-精确计算与大整数)
- [11. 报表与 Excel（内置）](#11-报表与-excel内置)
- [12. Rust 嵌入：安全、性能与工程实践](#12-rust-嵌入安全性能与工程实践)
- [13. 调试、排错与最佳实践](#13-调试排错与最佳实践)
- [14. 示例与测试](#14-示例与测试)

---

## 1. Aether 是什么

Aether 是一个轻量级、可嵌入的脚本/DSL 解释器，适合：

- 在 Rust 项目中把 Aether 当成 **规则引擎 / 配置 DSL / 业务表达式语言**
- 作为独立脚本运行（CLI / REPL）
- 需要“安全优先”的嵌入式脚本：**库模式默认禁用 IO**（文件/网络）

工程上，Aether 的核心由：Lexer → Parser → Optimizer → Evaluator 组成，并提供 AST 缓存与可选优化。

---

## 2. 安装与构建

### 2.1 安装 CLI（二进制）

如果你从 crates.io 安装：

```bash
cargo install aether
```

在当前仓库本地构建：

```bash
# 克隆仓库
git clone https://github.com/xiaozuhui/aether
# 使用Rust编译
cd aether 
cargo build --release
# 运行：
./target/release/aether --help
```

### 2.2 作为 Rust 依赖引入

在你的 Rust 项目中添加依赖（示例）：

```bash
cargo add aether
```

如果你要使用异步接口，需要开启 feature：

```toml
# Cargo.toml
[dependencies]
aether = { version = "*", features = ["async"] }
```

---

## 3. 命令行使用（CLI）

### 3.1 运行脚本

```bash
aether your_script.aether
```

Aether CLI 默认会：

- 启用所有 IO 权限（允许文件/网络相关内置函数）
- 自动加载标准库（除非加 `--no-stdlib`）
- 启用文件模块加载（`Import/Export`），并以脚本所在目录作为相对导入的 base

### 3.2 常用选项

```bash
aether --help

# 只检查语法，不执行
aether --check your_script.aether

# 打印 AST（排查语法结构很有用）
aether --ast your_script.aether

# 调试模式（显示更多运行信息）
aether --debug your_script.aether

# 不自动加载 stdlib
aether --no-stdlib your_script.aether
```

---

## 4. 交互式使用（REPL）

启动 REPL：

```bash
aether
```

REPL 支持加载标准库：

```text
:load stdlib          # 加载全部 stdlib
:load string_utils    # 加载指定模块
```

REPL 内置命令：

- `help`：查看帮助
- `exit` / `quit`：退出

---

## 5. 语言速成：语法与约定

### 5.1 关键字大小写规则（很重要）

Aether 的关键字是**首字母大写**（大小写敏感）：

- `Set`, `Func`, `Lambda`, `Generator`, `Lazy`
- `If`, `Elif`, `Else`
- `For`, `In`, `While`
- `Return`, `Yield`, `Break`, `Continue`
- `Switch`, `Case`, `Default`
- `Import`, `From`, `Export`, `Throw`

布尔与空值字面量：`True`, `False`, `Null`。

### 5.2 命名约定

- **变量名/函数名**：必须为 `UPPER_SNAKE_CASE`（例如 `TOTAL_PRICE`, `CALC_TAX`）。
  - 这是语法层面强制的：`Set` 与 `Func` 会校验命名。
- **函数参数名**：允许更灵活（可用小写/下划线/数字），但建议保持一致风格。

### 5.3 注释

- 单行注释：`// comment`
- 块注释：`/* comment */`

### 5.4 语句分隔

- 换行（newline）与分号（`;`）都可作为语句分隔。

### 5.5 字符串

- 普通字符串：`"hello"`
- 多行字符串：使用三引号 `""" ... """`

---

## 6. 核心语法：变量、函数、控制流

### 6.1 变量定义（Set）

```aether
Set X 10
Set MESSAGE "HELLO"
Set IS_OK True
```

> 说明：`Set` 的变量名必须满足 `UPPER_SNAKE_CASE`。

### 6.2 函数定义（Func）

```aether
Func ADD(A, B) {
    Return (A + B)
}

PRINTLN(ADD(5, 3))
```

### 6.3 Lambda（两种写法）

块 Lambda：

```aether
Set DOUBLE Func(X) {
    Return (X * 2)
}
PRINTLN(DOUBLE(21))
```

箭头 Lambda：

```aether
Set INC Lambda X -> (X + 1)
PRINTLN(INC(41))

Set ADD2 Lambda (X, Y) -> (X + Y)
PRINTLN(ADD2(10, 20))
```

### 6.4 If 表达式（If / Elif / Else）

Aether 的 `If` 是**表达式**：

```aether
Set X 10
Set SIGN If (X > 0) {
    "POS"
} Elif (X < 0) {
    "NEG"
} Else {
    "ZERO"
}

PRINTLN(SIGN)
```

### 6.5 While 循环

```aether
Set I 0
While (I < 3) {
    PRINTLN(I)
    Set I (I + 1)
}
```

### 6.6 For 循环（两种形式）

普通形式：

```aether
For I In RANGE(0, 5) {
    PRINTLN(I)
}
```

带索引的形式：

```aether
Set ARR ["A", "B", "C"]
For IDX, VAL In ARR {
    PRINTLN(IDX, VAL)
}
```

### 6.7 Generator / Yield（惰性序列）

```aether
Generator FIB(LIMIT) {
    Set A 0
    Set B 1
    Set I 0

    While (I < LIMIT) {
        Yield A
        Set NEXT (A + B)
        Set A B
        Set B NEXT
        Set I (I + 1)
    }
}

For N In FIB(10) {
    PRINTLN(N)
}
```

### 6.8 Lazy（惰性求值）

`Lazy NAME (expr)` 会把表达式封装为惰性值（按需计算）。

```aether
Lazy EXPENSIVE(
    JSON_PARSE("{\"a\": 1, \"b\": 2}")
)

// 在真正使用时才会触发计算
PRINTLN(EXPENSIVE)
```

---

## 7. 数据结构：数组与字典

### 7.1 数组

```aether
Set ARR [1, 2, 3]
PRINTLN(ARR[0])
```

#### 索引赋值（注意空格规则）

Aether 在语法层面区分：

- `Set ARR[0] 99`：索引赋值（`ARR` 与 `[` 之间**不能有空格**）
- `Set ARR [1, 2]`：把数组字面量赋值给 `ARR`（`ARR` 与 `[` 之间**有空格**）

示例：

```aether
Set ARR [1, 2, 3]
Set ARR[0] 99
PRINTLN(ARR)
```

### 7.2 字典（Dict）

字典字面量允许 key 为**标识符或字符串**：

```aether
Set USER {name: "Alice", "age": 30}
PRINTLN(USER["age"])
```

---

## 8. 内置函数（Builtins）与权限模型

### 8.1 内置函数命名规则

内置函数在当前实现中使用**全大写命名**，例如：

- `PRINTLN`, `RANGE`, `JSON_PARSE`, `READ_FILE` …

> 注意：大小写敏感。`PRINTLN` 与 `Println` 不同。

### 8.2 IO 权限：库模式默认禁用

在 Rust 嵌入（库模式）时：

- 默认禁用文件系统与网络（更安全）
- 只有显式开启权限，相关内置函数才会被注册

受权限影响的内置能力：

- 文件系统：`READ_FILE`, `WRITE_FILE`, `LIST_DIR`, ...
- 网络：`HTTP_GET`, `HTTP_POST`, ...
- Excel：`EXCEL_*`（需要文件系统权限）

不受权限影响（始终可用）：

- `PRINT/PRINTLN/INPUT`
- 数学、数组、字符串、JSON 等计算类
- 报表格式化：`FORMAT_NUMBER/FORMAT_CURRENCY/FORMAT_PERCENT/FORMAT_DATE`

---

## 9. 标准库（stdlib）：模块与加载方式

Aether 的 stdlib 由 Aether 语言自身编写，并在编译时嵌入二进制。

> 提示：`Import/Export` 运行时模块系统已实现，但在 DSL（`Aether::new()`）默认 **禁用导入**（安全优先）。
> 在 DSL 工程落地上依然推荐把“模块/函数库”交给宿主（Rust）管理：按需把代码 `eval` 注入，或使用下文的“隔离作用域”方式执行。

### 9.1 模块列表（16 个）

- `string_utils`
- `array_utils`
- `validation`
- `datetime`
- `testing`
- `set`
- `queue`
- `stack`
- `heap`
- `sorting`
- `json`
- `csv`
- `functional`
- `cli_utils`
- `text_template`
- `regex_utils`

### 9.2 CLI/REPL 加载

- CLI：默认自动加载（可用 `--no-stdlib` 关闭）
- REPL：通过 `:load stdlib` 或 `:load <module>` 加载

### 9.3 Rust 嵌入加载

一次性加载全部 stdlib：

```rust
use aether::Aether;

let mut engine = Aether::with_stdlib()?;
engine.eval("PRINTLN(STR_TRIM(\"  hi  \"))")?;
# Ok::<(), String>(())
```

选择性加载（推荐作为 DSL）：

```rust
use aether::Aether;

let mut engine = Aether::new()
    .with_stdlib_string_utils()?
    .with_stdlib_array_utils()?
    .with_stdlib_json()?;

engine.eval("PRINTLN(STR_TO_UPPER(\"hello\"))")?;
# Ok::<(), String>(())
```

---

## 10. 精确计算与大整数

### 10.1 分数（有理数）精确计算

核心函数：

- `TO_FRACTION`, `TO_FLOAT`, `SIMPLIFY`
- `FRAC_ADD`, `FRAC_SUB`, `FRAC_MUL`, `FRAC_DIV`
- `NUMERATOR`, `DENOMINATOR`, `GCD`, `LCM`

示例：

```aether
Set FA TO_FRACTION(0.1)
Set FB TO_FRACTION(0.2)
Set FC FRAC_ADD(FA, FB)
PRINTLN(FC)            // 3/10
PRINTLN(TO_FLOAT(FC))  // 0.3
```

### 10.2 固定精度计算（金融等）

核心函数：

- `ROUND_TO`
- `ADD_WITH_PRECISION`, `SUB_WITH_PRECISION`, `MUL_WITH_PRECISION`, `DIV_WITH_PRECISION`
- `SET_PRECISION`

```aether
PRINTLN(ADD_WITH_PRECISION(0.1, 0.2, 2)) // 0.3
```

### 10.3 大整数（BigInt）

- 当整数超过 15 位，系统会自动进入大整数精确模式。
- 大整数内部与分数系统兼容（以分母为 1 的有理数表示），避免浮点精度损失。

```aether
Set A 999999999999999999999999999999
Set B 1
PRINTLN(A + B)
```

---

## 11. 报表与 Excel（内置）

### 11.1 格式化函数（始终可用）

```aether
PRINTLN(FORMAT_NUMBER(1234567.89, 2))
PRINTLN(FORMAT_CURRENCY(1234.56, "¥", 2))
PRINTLN(FORMAT_PERCENT(0.1234, 2))
```

提示：当前版本 `FORMAT_DATE` 已注册但尚未实现，调用会返回运行时错误。

### 11.2 Excel 函数（需要文件系统权限）

当启用了文件系统权限（CLI/REPL 默认启用；或 Rust 里打开 `filesystem_enabled`）后，会注册一组 `EXCEL_*` 函数。

其中当前版本已实现：

- `EXCEL_CREATE`：创建工作簿句柄
- `EXCEL_WRITE_CELL`：写入单元格（会按需创建工作表）
- `EXCEL_SAVE`：保存并释放工作簿句柄

其余接口（如 `EXCEL_WRITE_ROW/EXCEL_READ_SHEET/...`）目前为占位符：已注册但会返回“尚未实现”的运行时错误。

示例：生成一个最小 Excel：

```aether
Set WB EXCEL_CREATE()
EXCEL_WRITE_CELL(WB, "Sheet1", 0, 0, "NAME")
EXCEL_WRITE_CELL(WB, "Sheet1", 0, 1, "SCORE")
EXCEL_WRITE_CELL(WB, "Sheet1", 1, 0, "Alice")
EXCEL_WRITE_CELL(WB, "Sheet1", 1, 1, 95)
EXCEL_SAVE(WB, "report.xlsx")
```

---

## 12. Rust 嵌入：安全、性能与工程实践

### 12.1 最小嵌入（默认安全，无 IO）

```rust
use aether::Aether;

fn main() -> Result<(), String> {
    let mut engine = Aether::new(); // 默认禁用 IO
    let result = engine.eval("Set X 10\n(X + 20)")?;
    println!("{}", result);
    Ok(())
}
```

### 12.2 开启 IO 权限（按需开启）

```rust
use aether::{Aether, IOPermissions};

let mut perms = IOPermissions::default();
perms.filesystem_enabled = true; // 只开文件系统
let mut engine = Aether::with_permissions(perms);

engine.eval("WRITE_FILE(\"out.txt\", \"hi\")")?;
# Ok::<(), String>(())
```

完全开启：

```rust
use aether::Aether;

let mut engine = Aether::with_all_permissions();
```

### 12.3 高性能执行：三种引擎模式

适合“高频、重复执行 DSL”的场景：

1. `GlobalEngine`：线程局部单例，缓存效果最好（单线程高频）

```rust
use aether::engine::GlobalEngine;

let r = GlobalEngine::eval_isolated("Set X 10\n(X + 20)")?;
# Ok::<(), String>(())
```

1. `EnginePool`：引擎池，适合多线程/并发

```rust
use aether::engine::EnginePool;

let pool = EnginePool::new(8);
let mut engine = pool.acquire();
engine.eval("Set X 1\n(X + 1)")?;
# Ok::<(), String>(())
```

1. `ScopedEngine`：闭包风格，适合偶尔调用

```rust
use aether::engine::ScopedEngine;

let r = ScopedEngine::eval("Set X 10\n(X + 20)")?;
# Ok::<(), String>(())
```

### 12.4 AST 缓存与优化选项

查看缓存命中率：

```rust
let stats = engine.cache_stats();
println!("hits={} misses={}", stats.hits, stats.misses);
```

控制优化（常量折叠 / 死代码消除 / 尾递归优化开关）：

```rust
engine.set_optimization(true, true, false);
```

### 12.5 异步调用（可选 feature）

```rust
use aether::Aether;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut engine = Aether::new();
    let v = engine.eval_async("Set X 10\n(X + 20)").await?;
    println!("{}", v);
    Ok(())
}
```

### 12.6 Python → Aether（DSL 前置转译，可选）

如果你的输入侧已经是 Python（或类 Python 的表达式），你可以在 Rust 中先把 Python 转译为 Aether，再交给 Aether 引擎执行。

安全建议：

- 转译阶段会做“安全默认”的拒绝（例如 `numpy`、文件/网络 IO、`print/input` 等）
- 执行阶段建议使用 `Aether::new()`（库模式默认禁用 IO），避免把 IO 权限带进 DSL

```rust
use aether::{Aether, Value};
use aether::pytranspile::{python_to_aether, TranspileOptions};

fn main() -> Result<(), String> {
    let py = r#"
x = [1, 2, 3]
y = {"a": 1, "b": 2}
z = y["a"] + 12.34
z
"#;

    let res = python_to_aether(py, &TranspileOptions::default());
    if res.diagnostics.has_errors() {
        // 这里的 diagnostics 会告诉你为什么被拒绝/哪里不支持
        return Err(format!("{}", res.diagnostics));
    }

    let aether_code = res.aether.unwrap();

    let mut engine = Aether::new(); // 默认安全：无 IO
    let v = engine.eval(&aether_code)?;
    if v != Value::Null {
        println!("{}", v);
    }
    Ok(())
}
```

### 12.7 无 IO 调试：TRACE（推荐用于 DSL）

在 DSL 场景中你通常会选择 `Aether::new()`（默认禁用 IO），因此脚本里不能使用 `PRINT/PRINTLN/INPUT` 作为 debug。

推荐做法是使用 `TRACE(...)`：

- `TRACE(...)` 不产生任何 IO 副作用（不写 stdout/文件/网络）
- 只把信息写入引擎的**内存 trace 缓冲区**
- Rust 宿主在执行后通过 `engine.take_trace()` 把日志取走并自行处理

补充：

- 每条 trace 会自动带递增序号前缀：`#1 ...`, `#2 ...`
- 可选标签：`TRACE("label", x, y)` 会记录为 `#N [label] x y`
- 缓冲区有上限（默认 1024 条）；超出会丢弃最旧条目

脚本侧：

```aether
Set X [1, 2, 3]
Set Y {"a": 12}
Set Z (Y["a"] + 3)

TRACE("X=" + TO_STRING(X))
TRACE({"y": Y, "z": Z})

Z
```

宿主侧（Rust）：

```rust
use aether::Aether;

fn main() -> Result<(), String> {
    let mut engine = Aether::new();

    let _ = engine.eval(r#"
        Set X 10
        TRACE("x=" + TO_STRING(X))
        Set Y (X + 1)
        TRACE({"y": Y})
        Y
    "#)?;

    let trace = engine.take_trace();
    // 这里由宿主决定如何输出/落库
    println!("trace={:?}", trace);
    Ok(())
}

### 12.8 宿主注入与隔离作用域（类似 PyO3 globals）

你的典型场景可能是：

- **参数数据来自 Rust**（而不是先 `eval("Set ...")`）
- **函数定义来自数据库**（一条条 Aether `Func ...` 字符串）
- 需要多次执行：每次注入不同的数据/函数，**不能互相污染**

为此引擎提供了三个对宿主友好的能力：

- `engine.set_global(name, Value)`：直接注入 Rust 侧 `Value` 到全局环境（无需 `eval`）
- `engine.with_isolated_scope(|engine| ...)`：创建一个子作用域，闭包结束后自动恢复环境（本次注入/定义不会泄漏）
- `engine.reset_env()`：强制清空整个环境（会清掉通过 `eval` 加载的 stdlib/函数；慎用）

最小示例（Rust 数据 + DB 函数 + 脚本，一次执行后自动清理）：

```rust
use aether::{Aether, Value};
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let mut engine = Aether::new();

    let db_funcs: Vec<String> = vec![
        r#"Func ADD_TAX (amount, rate) { Return (amount * (1 + rate)) }"#.to_string(),
        r#"Func APPLY_DISCOUNT (subtotal, coupon) { Return (subtotal - coupon) }"#.to_string(),
    ];

    let script = r#"
Set net APPLY_DISCOUNT(INPUT[\"subtotal\"], INPUT[\"coupon\"])
ADD_TAX(net, RATE)
"#;

    let out = engine.with_isolated_scope(|engine| {
        engine.set_global("RATE", Value::Number(0.08));

        let mut input = HashMap::new();
        input.insert("subtotal".to_string(), Value::Number(1000.0));
        input.insert("coupon".to_string(), Value::Number(50.0));
        engine.set_global("INPUT", Value::Dict(input));

        for f in &db_funcs {
            engine.eval(f)?;
        }

        engine.eval(script)
    })?;

    println!("out={}", out);

    // 作用域已恢复：INPUT/函数不会泄漏到下一次执行
    assert!(engine.eval("INPUT").is_err());
    assert!(engine.eval("ADD_TAX(1, 0.1)").is_err());

    Ok(())
}
```

---

## 13. 调试、排错与最佳实践

### 13.1 先用 `--check`，再用 `--ast`

- `--check`：快速确认语法是否正确
- `--ast`：确认你的代码被解析成了什么结构

```bash
aether --check examples/stats_demo.aether
aether --ast examples/stats_demo.aether
```

### 13.2 常见报错

- **变量名/函数名不符合大写下划线**：`Set myVar 10` 会在解析阶段报错。
- **括号/大括号不匹配**：`If (X > 0) { ... }` 的括号与花括号必须配对。
- **索引赋值空格问题**：
  - `Set ARR[0] 1` ✅
  - `Set ARR [0] 1` ❌（会把 `[0] 1` 当成值表达式导致语法/运行时问题）

### 13.3 内置函数大小写

内置函数使用全大写：`PRINTLN`, `READ_FILE`, `HTTP_GET` 等。

---

## 14. 示例与测试

### 14.1 示例脚本

- `examples/`：语言特性与场景示例（统计、精度、报表、引擎模式等）
- `stdlib/examples/`：标准库模块示例

运行示例：

```bash
cargo run --release examples/report_demo.aether
```

### 14.2 运行测试

```bash
cargo test
```

也可以运行脚本辅助测试：

```bash
./scripts/test-all.sh
```
