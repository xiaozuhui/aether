# IO 功能快速开始

## 概述

Aether 的 IO 权限采用**场景化的默认策略**：

- **CLI模式**（命令行工具）：IO功能**默认启用** ✅
- **库模式**（嵌入DSL）：IO功能**默认禁用** ❌

这样设计既保证了作为独立语言使用时的便利性，又确保了作为DSL嵌入时的安全性。

## 快速上手

### CLI模式：直接使用（IO自动启用）

创建一个使用IO的脚本：

```aether
// example.aether
PRINTLN("=== 文件操作示例 ===")

// 写入文件
WRITE_FILE("hello.txt", "Hello from Aether!")

// 读取文件
Set CONTENT READ_FILE("hello.txt")
PRINTLN("文件内容: " + CONTENT)

// 网络请求
Set RESPONSE HTTP_GET("https://httpbin.org/json")
PRINTLN("API响应: " + RESPONSE)
```

运行脚本：

```bash
aether example.aether  # IO功能自动可用！
```

在REPL中交互：

```bash
$ aether
aether[1]> WRITE_FILE("test.txt", "测试")
true
aether[2]> READ_FILE("test.txt")
测试
aether[3]> exit
```

### 库模式：需要显式启用（安全优先）

当Aether被嵌入到你的Rust应用中时，IO**默认禁用**以确保安全性。

## 为什么CLI和库模式不同？

## 为什么CLI和库模式不同？

### CLI模式（IO默认启用）

用户通过 `aether script.aether` 运行脚本时：

- 这是**用户的主动选择**，类似运行Python或Node.js脚本
- 用户**期望脚本能够完整运行**，包括文件和网络操作
- 受操作系统的文件权限和防火墙保护
- 与其他编程语言的行为一致

### 库模式（IO默认禁用）

当你的应用允许用户编写和运行 Aether 脚本时（例如：配置脚本、数据处理脚本、业务规则脚本），你需要防止：

- 读取或修改敏感文件
- 发送网络请求泄露数据
- 删除重要文件
- 访问不该访问的资源

因此，Aether 采用"默认拒绝"的安全策略。

## 使用场景

### 场景1：完全沙箱环境（推荐用于用户脚本）

```rust
use aether::Aether;

// 用户提供的脚本在沙箱中运行，无IO权限
let user_script = r#"
    Set X 100
    Set Y 200
    PRINTLN("计算结果: " + TO_STRING(X + Y))
"#;

let mut engine = Aether::new();  // 默认配置：IO禁用
match engine.eval(user_script) {
    Ok(_) => println!("执行成功"),
    Err(e) => println!("执行失败: {}", e),
}

// 如果用户尝试访问IO，会失败：
let malicious_script = r#"
    WRITE_FILE("/etc/passwd", "hacked")
"#;

match engine.eval(malicious_script) {
    Ok(_) => println!("安全问题！"),
    Err(_) => println!("已阻止恶意操作"),  // ✅ 这里会执行
}
```

### 场景2：受控的文件访问（推荐用于配置管理）

```rust
use aether::{Aether, IOPermissions};

// 只允许访问文件系统，不允许网络请求
let mut perms = IOPermissions::default();
perms.filesystem_enabled = true;
perms.network_enabled = false;

let mut engine = Aether::with_permissions(perms);

let config_script = r#"
    // 可以读写配置文件
    Set CONFIG READ_FILE("config.json")
    PRINTLN("配置已加载")
    
    // 但不能发送网络请求
    // HTTP_GET("https://evil.com")  // ❌ 会失败
"#;

engine.eval(config_script).unwrap();
```

### 场景3：完全信任的脚本（推荐用于管理员脚本）

```rust
use aether::Aether;

// 完全信任的脚本，启用所有IO
let admin_script = r#"
    // 可以读取文件
    Set DATA READ_FILE("data.txt")
    
    // 可以发送网络请求
    Set RESPONSE HTTP_GET("https://api.example.com/data")
    
    // 可以写入文件
    WRITE_FILE("output.txt", RESPONSE)
"#;

let mut engine = Aether::with_all_permissions();
engine.eval(admin_script).unwrap();
```

## API 参考

### 创建引擎

```rust
// 1. 默认配置（无IO）
let engine = Aether::new();

// 2. 自定义权限
let mut perms = IOPermissions::default();
perms.filesystem_enabled = true;
perms.network_enabled = false;
let engine = Aether::with_permissions(perms);

// 3. 启用所有权限
let engine = Aether::with_all_permissions();

// 4. 禁用所有权限（等同于 Aether::new()）
let perms = IOPermissions::deny_all();
let engine = Aether::with_permissions(perms);
```

### 权限配置

```rust
pub struct IOPermissions {
    pub filesystem_enabled: bool,  // 文件系统访问
    pub network_enabled: bool,     // 网络访问
}

impl IOPermissions {
    // 创建默认配置（所有IO禁用）
    pub fn default() -> Self;
    
    // 启用所有权限
    pub fn allow_all() -> Self;
    
    // 禁用所有权限
    pub fn deny_all() -> Self;
}
```

## 可用函数

### 文件系统函数（需要 filesystem_enabled = true）

| 函数 | 说明 | 示例 |
|------|------|------|
| READ_FILE | 读取文件 | `READ_FILE("data.txt")` |
| WRITE_FILE | 写入文件 | `WRITE_FILE("out.txt", "content")` |
| APPEND_FILE | 追加文件 | `APPEND_FILE("log.txt", "entry\n")` |
| DELETE_FILE | 删除文件 | `DELETE_FILE("temp.txt")` |
| FILE_EXISTS | 检查文件存在 | `FILE_EXISTS("config.txt")` |
| LIST_DIR | 列出目录 | `LIST_DIR(".")` |
| CREATE_DIR | 创建目录 | `CREATE_DIR("output")` |

### 网络函数（需要 network_enabled = true）

| 函数 | 说明 | 示例 |
|------|------|------|
| HTTP_GET | GET请求 | `HTTP_GET("https://api.example.com")` |
| HTTP_POST | POST请求 | `HTTP_POST(url, body, "application/json")` |
| HTTP_PUT | PUT请求 | `HTTP_PUT(url, body)` |
| HTTP_DELETE | DELETE请求 | `HTTP_DELETE(url)` |

## 安全最佳实践

### ✅ 推荐做法

1. **默认禁用**：除非必要，使用 `Aether::new()`
2. **最小权限**：只启用需要的权限
3. **权限检查**：在应用层面也进行权限验证
4. **路径限制**：在应用层面限制文件访问路径
5. **超时控制**：为脚本执行设置超时

```rust
// 好的做法：根据用户角色分配权限
fn create_engine(user_role: UserRole) -> Aether {
    match user_role {
        UserRole::Admin => Aether::with_all_permissions(),
        UserRole::User => {
            let mut perms = IOPermissions::default();
            perms.filesystem_enabled = true;  // 只允许文件访问
            Aether::with_permissions(perms)
        },
        UserRole::Guest => Aether::new(),  // 无IO权限
    }
}
```

### ❌ 避免的做法

```rust
// 不好的做法：默认启用所有权限
let engine = Aether::with_all_permissions();  // ❌

// 不好的做法：未检查用户角色就启用IO
let mut engine = Aether::with_all_permissions();
engine.eval(untrusted_script).unwrap();  // ❌ 危险！
```

## 错误处理

IO操作可能失败，建议在脚本中处理错误：

```aether
// 检查文件是否存在
Set FILE_EXISTS FILE_EXISTS("data.txt")
If (FILE_EXISTS) {
    Set DATA READ_FILE("data.txt")
    PRINTLN("数据已加载")
} Else {
    PRINTLN("文件不存在")
}
```

## 完整示例

```rust
use aether::{Aether, IOPermissions};

fn main() {
    // 场景1：处理配置文件（只需文件访问）
    let config_script = r#"
        Set CONFIG_FILE "app.config"
        Set EXISTS FILE_EXISTS(CONFIG_FILE)
        
        If (EXISTS) {
            Set CONFIG READ_FILE(CONFIG_FILE)
            PRINTLN("配置: " + CONFIG)
        } Else {
            Set DEFAULT_CONFIG "timeout=30\nretries=3"
            WRITE_FILE(CONFIG_FILE, DEFAULT_CONFIG)
            PRINTLN("已创建默认配置")
        }
    "#;

    let mut perms = IOPermissions::default();
    perms.filesystem_enabled = true;
    
    let mut engine = Aether::with_permissions(perms);
    match engine.eval(config_script) {
        Ok(_) => println!("配置处理成功"),
        Err(e) => eprintln!("错误: {}", e),
    }

    // 场景2：数据同步（需要文件和网络）
    let sync_script = r#"
        // 从API获取数据
        Set API_DATA HTTP_GET("https://api.example.com/data")
        
        // 保存到本地
        WRITE_FILE("cache.json", API_DATA)
        PRINTLN("数据已同步")
    "#;

    let mut sync_engine = Aether::with_all_permissions();
    match sync_engine.eval(sync_script) {
        Ok(_) => println!("同步成功"),
        Err(e) => eprintln!("同步失败: {}", e),
    }
}
```

## 更多信息

- [完整文档](docs/IO_GUIDE.md)
- [API 参考](docs/API.md)
- [示例代码](examples/)
