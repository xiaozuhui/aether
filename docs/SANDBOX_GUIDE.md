# Aether 沙箱安全指南

## 概述

Aether 的沙箱系统为 DSL 嵌入模式提供了强大的安全隔离能力，防止恶意脚本访问未授权的资源。

## 核心特性

### 1. 路径安全验证

自动防止路径遍历攻击（如 `../`）和越权访问。

### 2. 沙箱配置预设

提供三种预设配置，覆盖 90% 的使用场景：
- `dsl_safe()` - DSL 默认安全（禁用所有 IO）
- `cli_full_access()` - CLI 完全访问
- `sandboxed(root_dir)` - 受限沙箱（仅 root_dir 内访问）

### 3. 向后兼容

所有现有 API 继续工作，无需修改代码。

## 快速开始

### 基本使用

```rust
use aether::{Aether, IOPermissions, ScopedValidator, PathValidator};
use std::path::PathBuf;

// 1. 创建引擎并启用文件系统权限
let mut perms = IOPermissions::default();
perms.filesystem_enabled = true;
let mut engine = Aether::with_permissions(perms);

// 2. 设置路径验证器（限制在 /safe/dir 内）
let validator = PathValidator::with_root_dir(PathBuf::from("/safe/dir"));
let _scope = ScopedValidator::set(validator);

// 3. 执行代码（路径验证自动生效）
let code = r#"
    Set CONTENT READ_FILE("test.txt")
    CONTENT
"#;
let result = engine.eval(code)?;
```

### 使用 SandboxConfig

```rust
use aether::{Aether, SandboxConfig};
use std::path::PathBuf;

// DSL 安全模式（禁用所有 IO）
let engine = Aether::with_sandbox(SandboxConfig::dsl_safe());

// CLI 完全访问模式
let engine = Aether::with_sandbox(SandboxConfig::cli_full_access());

// 受限沙箱模式（推荐）
let engine = Aether::with_sandbox(
    SandboxConfig::sandboxed(PathBuf::from("./safe"))
);
```

## 安全特性

### 1. 阻止路径遍历

```rust
// ❌ 被阻止
READ_FILE("../../etc/passwd")
READ_FILE("../secret.txt")

// ✅ 允许（在 root_dir 内）
READ_FILE("data.txt")
READ_FILE("subdir/config.json")
```

### 2. 阻止绝对路径

```rust
// ❌ 被阻止
READ_FILE("/etc/passwd")
READ_FILE("C:\\Windows\\System32\\config")

// ✅ 允许（相对路径）
READ_FILE("config.txt")
```

### 3. 文件扩展名白名单

```rust
use aether::{PathValidator, PathRestriction};
use std::collections::HashSet;

let mut allowed = HashSet::new();
allowed.insert("txt".to_string());
allowed.insert("json".to_string());

let restriction = PathRestriction {
    root_dir: PathBuf::from("./data"),
    allow_absolute: false,
    allow_parent_traversal: false,
    allowed_extensions: Some(allowed),
};

let validator = PathValidator::new(restriction);
// 只有 .txt 和 .json 文件可以被访问
```

## API 参考

### PathValidator

路径验证器，用于验证文件路径是否在允许的范围内。

```rust
let validator = PathValidator::with_root_dir(PathBuf::from("/safe"));

// 验证路径
match validator.validate_and_normalize(Path::new("test.txt")) {
    Ok(validated) => println!("Valid path: {}", validated.display()),
    Err(e) => eprintln!("Path validation failed: {}", e),
}
```

### ScopedValidator

RAII 模式的验证器作用域管理，自动清理。

```rust
{
    let _scope = ScopedValidator::set(validator);
    // 在此作用域内，验证器生效
    engine.eval("READ_FILE('test.txt')")?;
} // 作用域结束，验证器自动清除
```

### SandboxConfig

统一的沙箱配置。

```rust
pub struct SandboxConfig {
    /// IO 权限
    pub io_permissions: IOPermissions,

    /// 文件系统策略（Disabled/ReadOnly/FullAccess）
    pub filesystem_policy: SandboxPolicy,

    /// 文件系统路径限制
    pub filesystem_restriction: Option<PathRestriction>,

    /// 模块系统策略
    pub module_policy: SandboxPolicy,

    /// 模块路径限制
    pub module_restriction: Option<PathRestriction>,

    /// 是否启用指标收集
    pub enable_metrics: bool,

    /// 最大模块缓存数量
    pub max_module_cache_size: usize,

    /// 模块缓存 TTL（秒）
    pub module_cache_ttl_secs: u64,
}
```

## 最佳实践

### 1. 默认使用安全配置

```rust
// ✅ 推荐：默认安全
let engine = Aether::new(); // DSL 模式，禁用所有 IO

// ❌ 不推荐：不必要地启用所有权限
let engine = Aether::with_all_permissions();
```

### 2. 使用受限沙箱

```rust
// ✅ 推荐：明确指定允许的目录
let engine = Aether::with_sandbox(
    SandboxConfig::sandboxed(PathBuf::from("/app/data"))
);

// ❌ 不推荐：允许访问整个文件系统
let mut perms = IOPermissions::allow_all();
let engine = Aether::with_permissions(perms);
```

### 3. 组合使用权限和沙箱

```rust
let mut config = SandboxConfig::sandboxed(PathBuf::from("/safe"));

// 禁用网络访问
config.io_permissions.network_enabled = false;

// 启用指标收集
config.enable_metrics = true;

let engine = Aether::with_sandbox(config);
```

### 4. 显式清理验证器

```rust
// 使用 ScopedValidator 自动清理
{
    let _scope = ScopedValidator::set(validator);
    engine.eval(code)?;
} // 自动清理

// 或手动清理
set_filesystem_validator(Some(validator));
engine.eval(code)?;
set_filesystem_validator(None); // 手动清理
```

## 安全检查清单

- [ ] 使用 `Aether::new()` 或 `SandboxConfig::dsl_safe()` 作为默认配置
- [ ] 仅在必要时启用 IO 权限
- [ ] 使用 `PathValidator` 限制文件系统访问范围
- [ ] 禁用不需要的文件扩展名
- [ ] 定期审查和更新沙箱配置
- [ ] 记录所有安全相关的配置决策

## 故障排查

### Q: 为什么路径验证失败了？

A: 检查以下几点：
1. 路径是否包含 `..`（父目录遍历）？
2. 路径是否是绝对路径（如果禁止）？
3. 路径是否在 `root_dir` 范围内？
4. 文件扩展名是否在白名单中？

### Q: 如何调试路径验证？

A: 使用 `validate_and_normalize()` 方法并查看错误：

```rust
match validator.validate_and_normalize(path) {
    Ok(validated) => println!("Valid: {}", validated.display()),
    Err(e) => eprintln!("Invalid: {}", e),
}
```

### Q: 沙箱是否影响性能？

A: 路径验证的开销很小（< 10%），且可以：
1. 缓存验证结果
2. 仅在必要时启用验证
3. 使用性能基准测试验证

## 后续优化

阶段2的后续优化（阶段3+）包括：
1. 内存 resolver（从内存加载模块）
2. DB resolver（从数据库加载模块）
3. 执行限制（步数、递归深度、内存）
4. 结构化 TRACE 事件
5. 规则调试器（可视化执行流程）

## 参考资源

- [ROADMAP.md](./ROADMAP.md) - 项目路线图
- [src/sandbox/](../src/sandbox/) - 沙箱模块源代码
- [tests/sandbox_tests.rs](../tests/sandbox_tests.rs) - 集成测试示例
