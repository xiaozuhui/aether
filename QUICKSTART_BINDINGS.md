# Aether 跨语言绑定快速开始

本文档介绍如何在不同编程语言中使用 Aether DSL。

## 🚀 快速开始

### 1. 构建 Aether 核心库

首先，构建 Rust 核心库：

```bash
# 克隆仓库
git clone https://github.com/xiaozuhui/aether
cd aether

# 构建 release 版本
cargo build --release
```

这将生成：

- 静态库：`target/release/libaether.a`
- 动态库：`target/release/libaether.dylib` (macOS) 或 `.so` (Linux) 或 `.dll` (Windows)
- C 头文件：`bindings/aether.h`

### 2. 选择你的语言

#### 🦀 Rust（原生）

最简单的方式，直接作为 Rust crate 使用：

**Cargo.toml:**

```toml
[dependencies]
aether = { path = "../path/to/aether" }
```

**main.rs:**

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
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

#### 🐹 Go

**第一步：确保 Rust 库已构建**

```bash
cd /path/to/aether
cargo build --release
```

**第二步：使用 Go 绑定**

```go
package main

import (
    "fmt"
    "log"
    
    aether "github.com/xiaozuhui/aether-go"
)

func main() {
    // 创建引擎
    engine := aether.New()
    defer engine.Close()
    
    // 执行代码
    code := `
        Set X 10
        Set Y 20
        (X + Y)
    `
    
    result, err := engine.Eval(code)
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Println("Result:", result) // 输出: Result: 30
}
```

**运行示例：**

```bash
cd bindings/go/examples
go run main.go
```

**运行测试：**

```bash
cd bindings/go
go test -v
```

#### 📘 TypeScript/JavaScript

**第一步：构建 WASM 模块**

需要安装 `wasm-pack`：

```bash
# 安装 wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 构建 WASM
cd /path/to/aether
wasm-pack build --target bundler --out-dir bindings/typescript/pkg
```

**第二步：安装依赖**

```bash
cd bindings/typescript
npm install
npm run build:ts
```

**第三步：使用**

```typescript
import { Aether } from '@xiaozuhui/aether';

async function main() {
    // 创建引擎
    const engine = await Aether.create();
    
    // 执行代码
    const code = `
        Set X 10
        Set Y 20
        (X + Y)
    `;
    
    const result = engine.eval(code);
    console.log('Result:', result); // 输出: Result: 30
}

main();
```

**Node.js (CommonJS):**

```javascript
const { Aether } = require('@xiaozuhui/aether');

async function main() {
    const engine = await Aether.create();
    const result = engine.eval('(10 + 20)');
    console.log(result); // 30
}

main();
```

## 📚 示例代码

### 基础运算

```javascript
// Aether 代码（所有语言通用）
Set X 10
Set Y 20
Set SUM (X + Y)
Set PRODUCT (X * Y)
Print "Sum:", SUM
Print "Product:", PRODUCT
PRODUCT
```

### 函数定义

```javascript
Func FACTORIAL (N) {
    If (N <= 1) {
        Return 1
    }
    Return (N * FACTORIAL(N - 1))
}

FACTORIAL(5)  // 返回 120
```

### 字符串操作

```javascript
Set GREETING "Hello"
Set NAME "World"
Set MESSAGE (GREETING + " " + NAME + "!")
Print MESSAGE  // 输出: Hello World!
```

### 数组操作

```javascript
Set NUMBERS [1, 2, 3, 4, 5]
Print "Length:", LENGTH(NUMBERS)
Print "First:", FIRST(NUMBERS)
Print "Last:", LAST(NUMBERS)
```

### 条件判断

```javascript
Func CHECK_SIGN (N) {
    If (N > 0) {
        Return "positive"
    } Else {
        If (N < 0) {
            Return "negative"
        } Else {
            Return "zero"
        }
    }
}

Print CHECK_SIGN(10)   // positive
Print CHECK_SIGN(-5)   // negative
Print CHECK_SIGN(0)    // zero
```

## 🔒 安全与权限

默认情况下，IO 操作是**禁用**的，这样可以安全地运行不受信任的脚本：

### Rust

```rust
// 默认（安全）
let engine = Aether::new();

// 启用 IO 权限
let engine = Aether::with_all_permissions();
```

### Go

```go
// 默认（安全）
engine := aether.New()

// 启用 IO 权限
engine := aether.NewWithPermissions()
```

### TypeScript

```typescript
// 默认（安全）
const engine = await Aether.create();

// 启用 IO 权限
const engine = await Aether.createWithPermissions();
```

## 🛠️ 构建所有绑定

使用提供的脚本一次构建所有语言绑定：

```bash
./scripts/build-all.sh
```

这将：

1. ✅ 构建 Rust 核心库
2. ✅ 生成 C 头文件
3. ✅ 构建 WASM 模块（如果安装了 wasm-pack）
4. ✅ 编译 TypeScript 绑定（如果安装了 npm）
5. ✅ 配置 Go 绑定（如果安装了 go）

## 🧪 运行测试

测试所有语言绑定：

```bash
./scripts/test-all.sh
```

或单独测试：

```bash
# Rust
cargo test

# Go
cd bindings/go && go test -v

# TypeScript
cd bindings/typescript && npm test
```

## 📖 更多文档

- **Go 绑定**: [bindings/go/README.md](bindings/go/README.md)
- **TypeScript 绑定**: [bindings/typescript/README.md](bindings/typescript/README.md)
- **语言绑定总览**: [bindings/README.md](bindings/README.md)
- **Aether 语言指南**: [docs/USER_GUIDE.md](docs/USER_GUIDE.md)

## 💡 性能对比

| 语言 | 性能 | 适用场景 |
|------|------|----------|
| Rust | ⭐⭐⭐⭐⭐ | 原生应用、高性能工具 |
| Go   | ⭐⭐⭐⭐ | 服务端应用、微服务 |
| TS/JS | ⭐⭐⭐ | Web 应用、Node.js 服务 |

## ❓ 常见问题

### Q: 如何在 Go 中链接 Rust 库？

A: Go 绑定使用 CGO，需要设置正确的库路径。示例代码中已包含必要的 `#cgo` 指令。

### Q: TypeScript 绑定可以在浏览器中使用吗？

A: 是的，WASM 模块可以在现代浏览器中运行。需要正确配置 webpack 或其他打包工具。

### Q: 如何处理错误？

A: 所有绑定都提供了完整的错误处理：

- Rust: `Result<Value, String>`
- Go: `(string, error)` 元组
- TypeScript: `try-catch` 或 Promise rejection

## 🤝 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 📄 许可证

Apache-2.0

## 🔗 链接

- [GitHub 仓库](https://github.com/xiaozuhui/aether)
- [问题反馈](https://github.com/xiaozuhui/aether/issues)
- [文档](docs/)
