# 发布 Aether 到 crates.io 指南

本文档介绍如何将 Aether 发布到 crates.io，以及如何在其他项目中使用。

## 📋 前置准备

### 1. 注册 crates.io 账号

访问 <https://crates.io/> 并使用 GitHub 账号登录。

### 2. 获取 API Token

1. 登录后，访问 <https://crates.io/settings/tokens>
2. 点击 "New Token"
3. 输入 token 名称（如 "aether-publishing"）
4. 复制生成的 token

### 3. 配置本地认证

```bash
# 登录 crates.io
cargo login <your-api-token>
```

这会将 token 保存到 `~/.cargo/credentials.toml`

## 🔍 发布前检查

### 1. 确保代码质量

```bash
# 运行所有测试
cargo test --all

# 运行文档测试
cargo test --doc

# 检查代码格式
cargo fmt --check

# 运行 clippy
cargo clippy -- -D warnings
```

### 2. 检查 Cargo.toml

确保以下字段已正确填写：

```toml
[package]
name = "aether"                    # crate 名称
version = "0.2.0"                  # 版本号（遵循语义化版本）
edition = "2021"                   # Rust 版本
authors = ["your-name <email>"]    # 作者信息
description = "..."                # 简短描述（< 200 字符）
license = "Apache-2.0"             # 许可证
repository = "https://github.com/xiaozuhui/aether"  # 仓库地址
documentation = "https://docs.rs/aether"            # 文档地址
readme = "README.md"               # README 文件
keywords = ["dsl", "interpreter"]  # 关键词（最多 5 个）
categories = ["parser-implementations"]  # 分类
```

### 3. 更新 README.md

确保 README.md 包含：

- 项目简介
- 快速开始示例
- 安装说明
- 基本使用方法
- 文档链接
- 许可证信息

### 4. 检查文档

```bash
# 生成并查看文档
cargo doc --open --no-deps
```

确保所有公共 API 都有文档注释。

### 5. 本地打包测试

```bash
# 模拟打包（不实际发布）
cargo package --allow-dirty

# 查看打包内容
cargo package --list

# 测试打包后的 crate 是否能正常构建
cargo package && cargo publish --dry-run
```

## 🚀 发布流程

### 1. 提交所有更改

```bash
git add .
git commit -m "Prepare for v0.2.0 release"
git push origin master
```

### 2. 打标签

```bash
# 创建版本标签
git tag -a v0.2.0 -m "Release version 0.2.0"

# 推送标签到远程
git push origin v0.2.0
```

### 3. 发布到 crates.io

```bash
# 发布
cargo publish
```

如果是首次发布，可能需要等待几分钟进行审核。

### 4. 验证发布

访问 <https://crates.io/crates/aether> 确认发布成功。

## 📥 在其他项目中使用

### 方法 1: 基本使用

在其他项目的 `Cargo.toml` 中添加：

```toml
[dependencies]
aether = "0.2"
```

然后在代码中使用：

```rust
use aether::Aether;

fn main() -> Result<(), String> {
    let mut engine = Aether::new();
    
    let code = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;
    
    let result = engine.eval(code)?;
    println!("Result: {}", result);
    
    Ok(())
}
```

### 方法 2: 选择性加载标准库

```rust
use aether::Aether;

fn main() -> Result<(), String> {
    // 只加载需要的标准库模块
    let mut engine = Aether::new()
        .with_stdlib_string_utils()?
        .with_stdlib_array_utils()?
        .with_stdlib_json()?;
    
    // 使用引擎
    engine.eval("...")?;
    
    Ok(())
}
```

### 方法 3: 启用特定功能

如果有 features，可以这样指定：

```toml
[dependencies]
aether = { version = "0.2", features = ["full-stdlib"] }
```

### 方法 4: 从 Git 直接使用（开发版本）

```toml
[dependencies]
aether = { git = "https://github.com/xiaozuhui/aether", branch = "master" }
```

### 方法 5: 本地路径依赖（开发时）

```toml
[dependencies]
aether = { path = "../aether" }
```

## 🔄 版本更新流程

### 1. 更新版本号

修改 `Cargo.toml` 中的版本号：

```toml
version = "0.2.1"  # 遵循语义化版本
```

版本号规则：

- **主版本号** (0.x.x): 不兼容的 API 变更
- **次版本号** (x.1.x): 向后兼容的功能新增
- **修订号** (x.x.1): 向后兼容的问题修正

### 2. 更新 CHANGELOG.md

记录版本变更：

```markdown
## [0.2.1] - 2025-11-08

### Added
- 新增选择性加载标准库功能

### Changed
- 优化性能

### Fixed
- 修复 bug
```

### 3. 重复发布流程

```bash
git commit -am "Bump version to 0.2.1"
git tag -a v0.2.1 -m "Release version 0.2.1"
git push origin master
git push origin v0.2.1
cargo publish
```

## 📊 发布后的维护

### 查看下载统计

访问 <https://crates.io/crates/aether/stats>

### 更新文档

文档会自动发布到 <https://docs.rs/aether>

### 监控 Issues 和 PR

及时响应 GitHub 上的 issues 和 pull requests。

## ⚠️ 常见问题

### 1. 发布失败："crate name is already taken"

crate 名称已被占用，需要更换名称。

### 2. 发布失败："failed to verify package"

本地构建失败，检查依赖和代码。

```bash
cargo build --release
cargo test
```

### 3. 撤销已发布的版本

**注意：** crates.io 不允许删除已发布的版本，只能 yank（标记为不推荐）：

```bash
cargo yank --vers 0.2.0
```

取消 yank：

```bash
cargo yank --vers 0.2.0 --undo
```

### 4. 更新依赖版本

定期更新依赖：

```bash
cargo update
cargo outdated  # 需要安装 cargo-outdated
```

## 🔒 安全建议

1. **不要提交 API token** 到 git 仓库
2. **定期轮换 API token**
3. **使用 CI/CD 自动化发布**（如 GitHub Actions）
4. **启用 2FA** 保护 crates.io 账号

## 📚 参考资源

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Policies](https://crates.io/policies)
- [Semantic Versioning](https://semver.org/)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 🎯 最佳实践

1. **保持向后兼容** - 尽量不破坏现有 API
2. **详细的文档** - 每个公共 API 都应有文档
3. **完善的测试** - 覆盖主要功能
4. **及时更新** - 修复 bug 并发布新版本
5. **良好的 CHANGELOG** - 清晰记录每次变更
6. **响应社区** - 及时处理 issues 和 PRs
