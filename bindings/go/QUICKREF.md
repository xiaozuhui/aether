# Go Module 快速参考

## 📦 模块信息

- **模块路径**: `github.com/xiaozuhui/aether/bindings/go`
- **仓库**: `github.com/xiaozuhui/aether` (主仓库,不需要单独的 Go 仓库)
- **版本**: 与 Rust 版本同步 (例如 v0.4.4)

## 🚀 用户如何使用

### 安装

```bash
# 最新版本
go get github.com/xiaozuhui/aether/bindings/go@latest

# 特定版本
go get github.com/xiaozuhui/aether/bindings/go@v0.4.4
```

### 代码示例

```go
import aether "github.com/xiaozuhui/aether/bindings/go"

func main() {
    engine := aether.New()
    defer engine.Close()

    result, err := engine.Eval("Set X 10\n(X + 20)")
    if err != nil {
        panic(err)
    }
    fmt.Println(result) // 30
}
```

## 📝 发布流程

### 1. 准备发布

```bash
cd bindings/go

# 运行测试
make test

# 检查 Git 状态
git status
```

### 2. 更新版本

```bash
# 在根目录编辑 Cargo.toml
vim ../../Cargo.toml
# version = "0.4.5"

# 提交
git add .
git commit -m "Bump version to v0.4.5"
git push
```

### 3. 打标签并发布

```bash
cd bindings/go
./release.sh
```

或手动:

```bash
# 打标签
git tag v0.4.5
git push origin v0.4.5

# 验证
# 等待几分钟后访问:
# https://pkg.go.dev/github.com/xiaozuhui/aether/bindings/go@v0.4.5
```

## ✅ 发布检查清单

- [ ] 版本号已更新 (Cargo.toml)
- [ ] 测试全部通过 (`make test`)
- [ ] 代码已提交 (`git push`)
- [ ] 标签已创建 (`git tag vx.x.x`)
- [ ] 标签已推送 (`git push origin vx.x.x`)
- [ ] 在 pkg.go.dev 验证可访问

## 📚 常用命令

```bash
# 构建 Rust 库
make build-lib

# 运行测试
make test

# 运行基准测试
make benchmark

# 运行示例
make example

# 清理
make clean
```

## 🔗 有用链接

- Go Module: https://pkg.go.dev/github.com/xiaozuhui/aether/bindings/go
- 仓库: https://github.com/xiaozuhui/aether
- 文档: [README.md](README.md)
- 发布指南: [PUBLISHING.md](PUBLISHING.md)
