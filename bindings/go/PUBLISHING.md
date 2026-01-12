# Go Module 发布指南

本指南说明如何在现有的 Aether 仓库中直接发布 Go Module,无需创建额外仓库。

## 方案概述

直接在 `github.com/xiaozuhui/aether` 仓库中发布 Go 绑定,模块路径为:
```
github.com/xiaozuhui/aether/bindings/go
```

## 目录结构

```
aether/                          # 主仓库
├── src/                          # Rust 源码
├── bindings/
│   ├── go/                       # Go Module (在这里!)
│   │   ├── go.mod                # 定义模块
│   │   ├── aether.go             # Go 代码
│   │   ├── aether_test.go        # 测试
│   │   ├── examples/             # 示例
│   │   └── README.md             # Go 文档
│   └── aether.h                  # C 头文件
├── Cargo.toml
└── README.md
```

## 步骤 1: 配置 go.mod

确保 `bindings/go/go.mod` 配置正确:

```go
module github.com/xiaozuhui/aether/bindings/go

go 1.21

require (
    // 如果有依赖,在这里列出
)
```

**关键点:**
- 模块路径是 `github.com/xiaozuhui/aether/bindings/go`
- 不是 `github.com/xiaozuhui/aether-go`

## 步骤 2: 添加 go.mod 忽略配置

在仓库根目录创建 `.gitignore` 确保不提交不必要的文件:

```bash
# 在根目录 .gitignore 中添加
bindings/go/sumdb
bindings/go/*.sum
!bindings/go/go.sum
```

## 步骤 3: 打标签发布

### 方式 A: 主版本标签(推荐)

在主仓库打标签:

```bash
# 假设当前 Aether 版本是 v0.4.4
git tag v0.4.4
git push origin v0.4.4
```

### 方式 B: Go 版本子目录(如果需要不兼容的更改)

如果将来需要重大更新,可以遵循 Go 的版本惯例:

```bash
# 仍然在主仓库,但使用 v2 子目录
mkdir -p bindings/go/v2
# 将代码复制到 v2/ 并更新 import 路径
```

## 步骤 4: 用户如何使用

用户安装时:

```bash
# 方式 1: 获取最新版本
go get github.com/xiaozuhui/aether/bindings/go@latest

# 方式 2: 获取特定版本
go get github.com/xiaozuhui/aether/bindings/go@v0.4.4

# 方式 3: 在 go.mod 中指定
require github.com/xiaozuhui/aether/bindings/go v0.4.4
```

用户代码中:

```go
import (
    aether "github.com/xiaozuhui/aether/bindings/go"
)

func main() {
    engine := aether.New()
    defer engine.Close()

    result, err := engine.Eval("Set X 10\n(X + 20)")
}
```

## 步骤 5: CI/CD 配置(可选)

创建 `.github/workflows/go.yml`:

```yaml
name: Go Tests

on:
  push:
    paths:
      - 'bindings/go/**'
      - 'src/ffi.rs'
  pull_request:
    paths:
      - 'bindings/go/**'
      - 'src/ffi.rs'

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3
      with:
        submodules: recursive

    - name: Set up Go
      uses: actions/setup-go@v4
      with:
        go-version: '1.21'

    - name: Build Rust library
      run: |
        cargo build --release

    - name: Run Go tests
      working-directory: ./bindings/go
      run: |
        go test -v -race -coverprofile=coverage.txt
        go tool cover -func=coverage.txt

    - name: Run Go benchmarks
      working-directory: ./bindings/go
      run: go test -bench=. -benchmem
```

## 步骤 6: 文档更新

### 在主 README.md 中添加 Go 使用说明

在主仓库的 `README.md` 中添加:

```markdown
## Go 绑定

Aether 提供 Go 语言绑定,可直接在 Go 项目中使用。

### 安装

\`\`\`bash
go get github.com/xiaozuhui/aether/bindings/go@latest
\`\`\`

### 使用

\`\`\`go
import (
    aether "github.com/xiaozuhui/aether/bindings/go"
)

func main() {
    engine := aether.New()
    defer engine.Close()

    result, err := engine.Eval("Set X 10\n(X + 20)")
    if err != nil {
        panic(err)
    }
    fmt.Println(result) // 30
}
\`\`\`

详细文档请查看 [bindings/go/README.md](bindings/go/README.md)
```

## 版本管理建议

### 与 Rust 版本同步

由于 Go 绑定依赖 Rust 库,建议:

1. **版本号保持一致**: Rust v0.4.4 → Go v0.4.4
2. **同时发布**: 打同一个 tag `v0.4.4`
3. **更新 go.mod**: 确保 go.mod 中的版本信息正确

### 发布流程

```bash
# 1. 更新版本号
# 编辑 Cargo.toml: version = "0.4.5"

# 2. 更新 CHANGELOG
echo "## v0.4.5" >> CHANGELOG.md

# 3. 提交更改
git add .
git commit -m "Bump version to v0.4.5"

# 4. 打标签
git tag v0.4.5
git push origin main
git push origin v0.4.5

# 5. 发布到 crates.io (Rust)
cargo publish

# Go Module 自动通过 git tag 可用
```

## 常见问题

### Q: Go Module Proxy 会自动索引吗?

A: 是的!一旦你推送 tag 到 GitHub,Go Module Proxy 会在几秒到几分钟内自动索引。

验证: https://pkg.go.dev/github.com/xiaozuhui/aether/bindings/go

### Q: 用户需要先编译 Rust 库吗?

A: 看情况:

**方式 1: 预编译库(推荐)**
- 在 GitHub Releases 中上传编译好的 `libaether.a`
- 提供 Makefile 自动下载

**方式 2: 用户自己编译**
- 文档中说明需要先运行 `cargo build --release`
- 大多数 Go 开发者都有 Rust 工具链

### Q: 如何处理预编译库?

创建 `bindings/go/Makefile`:

```makefile
.PHONY: build-lib

# 下载预编译库
build-lib:
	@echo "下载预编译库..."
	@curl -L -o libaether.a https://github.com/xiaozuhui/aether/releases/download/$(VERSION)/libaether.a

# 或从源码构建
build-from-source:
	@echo "从源码构建..."
	@cd ../.. && cargo build --release
```

### Q: import 路径太长怎么办?

A: 可以使用模块别名:

```go
import aether "github.com/xiaozuhui/aether/bindings/go"
```

或者推荐用户在自己的 go.mod 中添加:

```go
// 在用户的 go.mod 中
module myproject

require github.com/xiaozuhui/aether/bindings/go v0.4.4

// 在代码中
import aether "github.com/xiaozuhui/aether/bindings/go"
```

## 完整示例

### 检查清单

发布前检查:

- [ ] `bindings/go/go.mod` 正确配置
- [ ] 所有测试通过: `go test ./...`
- [ ] 更新主 README.md 添加 Go 使用说明
- [ ] 更新 CHANGELOG
- [ ] 代码已提交: `git push`
- [ ] 标签已推送: `git push origin v0.4.4`
- [ ] 验证 pkg.go.dev 能访问

### 验证发布

```bash
# 创建临时目录测试
mkdir /tmp/test-aether-go
cd /tmp/test-aether-go
go mod init test

# 尝试获取
go get github.com/xiaozuhui/aether/bindings/go@v0.4.4

# 检查 go.mod
cat go.mod

# 创建 main.go 测试
cat > main.go << 'EOF'
package main

import (
   "fmt"
    aether "github.com/xiaozuhui/aether/bindings/go"
)

func main() {
    engine := aether.New()
    defer engine.Close()

    result, err := engine.Eval("Set X 10\n(X + 20)")
    if err != nil {
        panic(err)
    }
    fmt.Println("Result:", result)
}
EOF

# 运行测试
go run main.go
```

## 总结

✅ **无需创建新仓库**
✅ **模块路径**: `github.com/xiaozuhui/aether/bindings/go`
✅ **版本同步**: 与 Rust 主版本保持一致
✅ **用户使用**: `go get github.com/xiaozuhui/aether/bindings/go@latest`
✅ **自动索引**: Go Module Proxy 自动识别 git tags

这种方式最简单,维护成本最低,推荐使用!
