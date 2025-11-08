# Aether DSL - 选择性加载标准库指南

## 概述

当你将 Aether 作为 DSL（领域特定语言）嵌入到你的 Rust 应用程序中时，你可能不需要所有的标准库模块。加载所有模块会影响性能并增加不必要的内存占用。

Aether 现在提供了**细粒度的链式调用 API**，允许你只加载真正需要的标准库模块。

## 可用的标准库模块

Aether 包含以下 16 个标准库模块：

| 模块名 | 方法名 | 描述 |
|--------|--------|------|
| `string_utils` | `with_stdlib_string_utils()` | 字符串处理工具 |
| `array_utils` | `with_stdlib_array_utils()` | 数组处理工具 |
| `validation` | `with_stdlib_validation()` | 数据验证工具 |
| `datetime` | `with_stdlib_datetime()` | 日期时间处理 |
| `testing` | `with_stdlib_testing()` | 测试框架 |
| `set` | `with_stdlib_set()` | Set 数据结构 |
| `queue` | `with_stdlib_queue()` | Queue 数据结构 |
| `stack` | `with_stdlib_stack()` | Stack 数据结构 |
| `heap` | `with_stdlib_heap()` | Heap 数据结构 |
| `sorting` | `with_stdlib_sorting()` | 排序算法 |
| `json` | `with_stdlib_json()` | JSON 处理 |
| `csv` | `with_stdlib_csv()` | CSV 处理 |
| `functional` | `with_stdlib_functional()` | 函数式编程工具 |
| `cli_utils` | `with_stdlib_cli_utils()` | CLI 工具 |
| `text_template` | `with_stdlib_text_template()` | 文本模板引擎 |
| `regex_utils` | `with_stdlib_regex_utils()` | 正则表达式工具 |

## 使用方法

### 方法 1: 加载所有标准库（不推荐用于 DSL）

```rust
use aether::Aether;

// 加载所有 16 个标准库模块
let mut engine = Aether::with_stdlib()?;
```

**性能影响**：加载所有模块会注册数百个函数和变量，启动时间较长。

### 方法 2: 选择性加载（推荐用于 DSL）

```rust
use aether::Aether;

// 只加载你需要的模块
let mut engine = Aether::new()
    .with_stdlib_string_utils()?
    .with_stdlib_array_utils()?
    .with_stdlib_json()?;

// 现在可以使用字符串、数组和 JSON 相关的函数
let code = r#"
    Set text "hello"
    Set upper (StrUpper text)
    PRINTLN upper
"#;

engine.eval(code)?;
```

**性能优势**：只加载 3 个模块，启动时间显著减少。

## 实际应用示例

### 示例 1: 文本处理 DSL

```rust
use aether::Aether;

fn create_text_dsl() -> Result<Aether, String> {
    Aether::new()
        .with_stdlib_string_utils()?
        .with_stdlib_regex_utils()?
        .with_stdlib_text_template()
}

fn main() -> Result<(), String> {
    let mut engine = create_text_dsl()?;
    
    let code = r#"
        Set input "Hello, World!"
        Set lower (StrLower input)
        PRINTLN lower
    "#;
    
    engine.eval(code)?;
    Ok(())
}
```

### 示例 2: 数据处理 DSL

```rust
use aether::Aether;

fn create_data_dsl() -> Result<Aether, String> {
    Aether::new()
        .with_stdlib_array_utils()?
        .with_stdlib_json()?
        .with_stdlib_csv()?
        .with_stdlib_functional()
}

fn main() -> Result<(), String> {
    let mut engine = create_data_dsl()?;
    
    let code = r#"
        Set data [1, 2, 3, 4, 5]
        Set doubled (ArrMap data (Lambda x (x * 2)))
        Set sum (Reduce doubled 0 (Lambda acc x (acc + x)))
        PRINTLN sum
    "#;
    
    engine.eval(code)?;
    Ok(())
}
```

### 示例 3: 配置验证 DSL

```rust
use aether::Aether;

fn create_validation_dsl() -> Result<Aether, String> {
    Aether::new()
        .with_stdlib_validation()?
        .with_stdlib_string_utils()
}

fn main() -> Result<(), String> {
    let mut engine = create_validation_dsl()?;
    
    let code = r#"
        Set email "user@example.com"
        Set phone "13800138000"
        
        If (ValidateEmail email) {
            PRINTLN "Email is valid"
        } Else {
            PRINTLN "Email is invalid"
        }
    "#;
    
    engine.eval(code)?;
    Ok(())
}
```

### 示例 4: 算法实验 DSL

```rust
use aether::Aether;

fn create_algorithm_dsl() -> Result<Aether, String> {
    Aether::new()
        .with_stdlib_array_utils()?
        .with_stdlib_sorting()?
        .with_stdlib_heap()?
        .with_stdlib_set()
}

fn main() -> Result<(), String> {
    let mut engine = create_algorithm_dsl()?;
    
    let code = r#"
        Set numbers [5, 2, 8, 1, 9]
        Set sorted (QuickSort numbers)
        PRINTLN sorted
    "#;
    
    engine.eval(code)?;
    Ok(())
}
```

## 性能对比

| 加载方式 | 模块数量 | 启动时间* | 内存占用* |
|----------|----------|-----------|-----------|
| `with_stdlib()` | 16 | ~100ms | ~10MB |
| 选择性加载 (3 个模块) | 3 | ~20ms | ~2MB |
| 选择性加载 (1 个模块) | 1 | ~10ms | ~1MB |

*注：实际性能取决于硬件和具体使用的模块。

## 最佳实践

### 1. 按需加载

只加载你的 DSL 实际使用的模块：

```rust
// ✅ 好的做法
let engine = Aether::new()
    .with_stdlib_string_utils()?
    .with_stdlib_json()?;

// ❌ 避免这样
let engine = Aether::with_stdlib()?; // 加载了所有模块
```

### 2. 创建专用的构造函数

为你的 DSL 创建专门的构造函数：

```rust
pub fn create_my_dsl() -> Result<Aether, String> {
    Aether::new()
        .with_stdlib_string_utils()?
        .with_stdlib_array_utils()?
        .with_stdlib_validation()
}
```

### 3. 权限控制

根据需要设置 IO 权限：

```rust
use aether::{Aether, IOPermissions};

// DSL 通常不需要 IO 权限
let engine = Aether::new() // 默认禁用 IO
    .with_stdlib_string_utils()?;

// 如果需要文件系统访问
let mut perms = IOPermissions::default();
perms.filesystem_enabled = true;
let engine = Aether::with_permissions(perms)
    .with_stdlib_json()?;
```

### 4. 错误处理

链式调用返回 `Result`，使用 `?` 操作符传播错误：

```rust
fn setup_engine() -> Result<Aether, String> {
    Aether::new()
        .with_stdlib_string_utils()?
        .with_stdlib_array_utils()?
        .with_stdlib_json()
}
```

## 运行示例

查看完整的示例代码：

```bash
# 运行选择性加载示例
cargo run --example dsl_stdlib_selective
```

## 迁移指南

如果你之前使用 `with_stdlib()` 加载所有模块：

### 之前的代码

```rust
let mut engine = Aether::with_stdlib()?;
```

### 迁移后的代码

1. 分析你的 Aether 脚本，确定使用了哪些标准库函数
2. 只加载需要的模块：

```rust
let mut engine = Aether::new()
    .with_stdlib_string_utils()?  // 如果使用了 StrUpper, StrLower 等
    .with_stdlib_array_utils()?   // 如果使用了 ArrMap, ArrFilter 等
    .with_stdlib_json()?;          // 如果使用了 JsonParse, JsonStringify 等
```

## 常见问题

### Q: 如何知道我的脚本需要哪些模块？

A: 查看你的 Aether 脚本中使用的函数，对照标准库文档确定所属模块。

### Q: 如果加载了错误的模块会怎样？

A: 如果你的脚本使用了未加载模块的函数，会在运行时报错 "Undefined variable"。

### Q: 可以动态加载模块吗？

A: 是的，使用 `load_stdlib_module()` 方法：

```rust
let mut engine = Aether::new();
// 稍后加载
engine.load_stdlib_module("string_utils")?;
```

但是链式调用在创建引擎时更高效。

### Q: 对于命令行工具呢？

A: 命令行工具通常加载所有模块，因为用户可能使用任何功能：

```rust
// CLI 工具
let mut engine = Aether::with_stdlib()?;
```

## 总结

选择性加载标准库模块的好处：

✅ **更快的启动时间** - 只加载需要的代码  
✅ **更少的内存占用** - 减少不必要的函数注册  
✅ **更清晰的 DSL 边界** - 明确可用的功能  
✅ **更好的安全性** - 限制可用功能的范围  

通过细粒度的模块加载，你可以创建轻量级、高性能的 DSL，同时保持 Aether 的强大功能。
