# IO 权限设计说明

## 设计理念

Aether 的 IO 权限采用**场景化的默认策略**：

### 场景1：作为独立语言使用（CLI模式）

当用户通过命令行工具运行 Aether 脚本时：

```bash
aether script.aether
```

或在 REPL 交互模式中：

```bash
aether  # 进入交互模式
```

**默认行为**：✅ 所有IO权限自动启用

**设计原因**：

- 用户明确选择运行该脚本
- 这是用户的主动行为，类似运行任何其他编程语言
- 用户期望能够访问文件和网络
- 与 Python、Node.js 等语言的行为一致

### 场景2：作为DSL嵌入使用（库模式）

当 Aether 被嵌入到 Rust 应用中作为 DSL 时：

```rust
use aether::Aether;

let mut engine = Aether::new();
engine.eval(user_provided_script)?;
```

**默认行为**：❌ 所有IO权限默认禁用

**设计原因**：

- 脚本可能来自不可信的用户输入
- 需要防止恶意脚本访问文件系统或网络
- 符合"默认安全"的原则
- 应用开发者应明确选择启用哪些权限

## 使用示例

### CLI模式（自动启用IO）

```bash
# 创建一个使用IO的脚本
cat > example.aether << 'EOF'
// 文件操作
WRITE_FILE("output.txt", "Hello World")
Set DATA READ_FILE("output.txt")
PRINTLN(DATA)

// 网络请求
Set RESPONSE HTTP_GET("https://api.example.com/data")
PRINTLN(RESPONSE)
EOF

# 直接运行 - IO功能自动可用
aether example.aether
```

### 库模式（需要显式启用）

```rust
use aether::{Aether, IOPermissions};

// 场景1：完全沙箱（默认）
fn run_untrusted_script(script: &str) -> Result<Value, String> {
    let mut engine = Aether::new();  // IO禁用
    engine.eval(script)
}

// 场景2：允许文件访问
fn run_config_script(script: &str) -> Result<Value, String> {
    let mut perms = IOPermissions::default();
    perms.filesystem_enabled = true;  // 只允许文件操作
    
    let mut engine = Aether::with_permissions(perms);
    engine.eval(script)
}

// 场景3：完全信任
fn run_trusted_script(script: &str) -> Result<Value, String> {
    let mut engine = Aether::with_all_permissions();  // 启用所有IO
    engine.eval(script)
}
```

## 权限矩阵

| 使用方式 | 默认权限 | 适用场景 | 示例 |
|---------|---------|---------|------|
| CLI工具 | ✅ 全部启用 | 用户主动运行脚本 | `aether script.aether` |
| REPL | ✅ 全部启用 | 交互式开发和测试 | `aether` |
| Rust库（new） | ❌ 全部禁用 | 不可信用户脚本 | `Aether::new()` |
| Rust库（自定义） | ⚙️ 按需配置 | 特定功能需求 | `Aether::with_permissions()` |
| Rust库（信任） | ✅ 全部启用 | 管理员脚本 | `Aether::with_all_permissions()` |

## 安全考虑

### CLI模式的安全性

虽然CLI模式默认启用IO，但这不是安全问题：

1. **用户主动性**：用户明确选择运行该脚本
2. **文件系统权限**：受操作系统的文件权限保护
3. **网络防火墙**：受系统防火墙和网络策略保护
4. **明确行为**：与其他编程语言（Python、Node.js）一致

### 库模式的安全性

库模式默认禁用IO至关重要：

```rust
// ❌ 危险：如果默认启用IO
let mut engine = Aether::new();
engine.eval(user_input)?;  // 用户可能写入恶意代码

// ✅ 安全：默认禁用，需要明确启用
let mut engine = Aether::new();
// 以下代码会失败
engine.eval("DELETE_FILE('/important/data.db')")?;  // ❌ 被阻止
```

## 最佳实践

### 作为应用开发者

1. **默认使用 `Aether::new()`**：除非有明确需求
2. **最小权限原则**：只启用需要的权限
3. **权限检查**：在应用层面也进行验证

```rust
fn create_engine_for_user(user: &User) -> Aether {
    match user.role {
        Role::Admin => Aether::with_all_permissions(),
        Role::PowerUser => {
            let mut perms = IOPermissions::default();
            perms.filesystem_enabled = true;
            Aether::with_permissions(perms)
        },
        Role::Guest => Aether::new(),  // 无IO权限
    }
}
```

### 作为脚本用户

1. **CLI使用**：直接运行，无需担心权限
2. **开发测试**：使用REPL交互式测试IO功能
3. **生产环境**：了解脚本将在什么权限下运行

## API 参考

```rust
impl Aether {
    /// 创建默认引擎（IO禁用）
    /// 推荐用于嵌入场景
    pub fn new() -> Self;
    
    /// 创建自定义权限引擎
    /// 推荐用于特定需求
    pub fn with_permissions(permissions: IOPermissions) -> Self;
    
    /// 创建完整权限引擎
    /// CLI工具使用此方法
    /// 也可用于完全信任的脚本
    pub fn with_all_permissions() -> Self;
}

pub struct IOPermissions {
    pub filesystem_enabled: bool,
    pub network_enabled: bool,
}

impl IOPermissions {
    /// 默认配置（全部禁用）
    pub fn default() -> Self;
    
    /// 全部启用
    pub fn allow_all() -> Self;
    
    /// 全部禁用
    pub fn deny_all() -> Self;
}
```

## 常见问题

### Q: 为什么CLI模式不需要手动启用IO？

A: CLI模式类似于其他编程语言（Python、Node.js），用户运行脚本是主动行为，期望脚本能够完整运行。限制IO反而会让用户困惑。

### Q: 如何在库模式中临时测试IO功能？

A: 使用 `with_all_permissions()` 创建测试引擎：

```rust
#[cfg(test)]
mod tests {
    use aether::Aether;
    
    #[test]
    fn test_io_script() {
        let mut engine = Aether::with_all_permissions();
        engine.eval("WRITE_FILE('test.txt', 'data')").unwrap();
    }
}
```

### Q: 可以在CLI模式中禁用IO吗？

A: 目前不支持。如需沙箱执行，请使用Rust库模式。未来可能添加 `--no-io` 命令行参数。

### Q: 权限检查的性能开销是多少？

A: 几乎为零。权限检查只在引擎初始化时进行一次，决定是否注册IO函数。运行时没有额外开销。

## 总结

- **CLI模式**：IO默认启用，便于使用
- **库模式**：IO默认禁用，安全优先
- **灵活配置**：可根据需求自定义权限
- **性能无损**：权限检查零运行时开销
