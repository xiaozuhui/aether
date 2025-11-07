# 标准库打包说明

## 概述

Aether 标准库已经使用 Rust 的 `include_str!` 宏编译进二进制文件中。这意味着：

- ✅ 无需外部文件依赖
- ✅ 零运行时开销（编译时嵌入）
- ✅ 开箱即用
- ✅ 二进制文件完全独立

## 技术实现

### 1. 标准库模块 (`src/stdlib.rs`)

使用 `include_str!` 宏在编译时将 `.aether` 文件内容嵌入到二进制文件中：

```rust
pub const STRING_UTILS: &str = include_str!("../stdlib/string_utils.aether");
pub const ARRAY_UTILS: &str = include_str!("../stdlib/array_utils.aether");
// ...
```

### 2. 自动加载 API

提供了便捷的 API 来加载标准库：

```rust
// 创建带标准库的引擎
let mut engine = Aether::with_stdlib()?;

// 或手动加载
let mut engine = Aether::new();
engine.load_all_stdlib()?;

// 或只加载特定模块
engine.load_stdlib_module("string_utils")?;
```

### 3. 命令行支持

命令行工具自动加载标准库：

```bash
# 自动加载标准库
aether script.aether

# 不加载标准库（更快启动）
aether --no-stdlib script.aether
```

### 4. REPL 支持

REPL 模式支持按需加载：

```
aether[1]> :load stdlib        # 加载所有标准库
aether[2]> :load string_utils  # 只加载字符串工具
```

## 文件大小影响

标准库代码总计约 50-80KB（未压缩）。编译后对二进制文件的影响：

- 未优化（debug）：约 +100KB
- 优化后（release）：约 +60KB（由于编译器优化）

这对于大多数应用来说是可以接受的开销。

## 构建流程

### 编译时检查

`build.rs` 确保标准库文件改变时重新编译：

```rust
println!("cargo:rerun-if-changed=stdlib/string_utils.aether");
println!("cargo:rerun-if-changed=stdlib/array_utils.aether");
// ...
```

### 测试

运行集成测试验证标准库：

```bash
cargo test stdlib_integration
```

## 优点

1. **零配置**：用户无需手动安装或配置标准库
2. **安全**：标准库代码在编译时验证
3. **性能**：无需运行时加载文件
4. **可移植**：单个二进制文件包含一切
5. **版本一致**：标准库版本与 Aether 版本锁定

## 替代方案比较

### 方案 A：当前实现（include_str!）✅

```rust
pub const STRING_UTILS: &str = include_str!("../stdlib/string_utils.aether");
```

- ✅ 简单直接
- ✅ 零运行时开销
- ✅ 编译时验证
- ❌ 增加二进制大小

### 方案 B：压缩嵌入

```rust
pub const STRING_UTILS_COMPRESSED: &[u8] = include_bytes!("../stdlib/string_utils.aether.gz");
```

- ✅ 更小的二进制文件
- ❌ 需要解压缩（运行时开销）
- ❌ 需要额外依赖（flate2）
- ❌ 更复杂

### 方案 C：外部文件

```rust
let stdlib = fs::read_to_string("stdlib/string_utils.aether")?;
```

- ✅ 不增加二进制大小
- ✅ 可以独立更新
- ❌ 需要部署文件
- ❌ 运行时 IO 开销
- ❌ 可能出现版本不匹配

## 未来优化

可能的优化方向：

1. **懒加载**：只在首次使用时加载模块
2. **选择性编译**：通过 feature flags 选择包含哪些模块
3. **AOT 编译**：预编译标准库为 AST
4. **压缩**：在空间敏感场景使用压缩

## 示例：添加新的标准库模块

1. 创建 `.aether` 文件：

   ```bash
   touch stdlib/new_module.aether
   ```

2. 在 `src/stdlib.rs` 中添加：

   ```rust
   pub const NEW_MODULE: &str = include_str!("../stdlib/new_module.aether");
   ```

3. 更新 `ALL_MODULES` 数组：

   ```rust
   pub const ALL_MODULES: &[(&str, &str)] = &[
       // ...
       ("new_module", NEW_MODULE),
   ];
   ```

4. 更新 `get_module` 函数：

   ```rust
   pub fn get_module(name: &str) -> Option<&'static str> {
       match name {
           // ...
           "new_module" => Some(NEW_MODULE),
           _ => None,
       }
   }
   ```

5. 在 `build.rs` 中添加监听：

   ```rust
   println!("cargo:rerun-if-changed=stdlib/new_module.aether");
   ```

6. 编写测试并重新编译！

## 总结

当前的 `include_str!` 方案是最简单且最实用的选择，它提供了：

- 开箱即用的体验
- 零运行时开销
- 编译时安全保证
- 完全独立的二进制文件

对于 Aether 这样的 DSL 语言来说，这是理想的标准库打包方式。
