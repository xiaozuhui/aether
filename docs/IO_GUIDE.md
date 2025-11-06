# IO 功能使用指南

Aether 提供了文件系统和网络IO功能，但默认情况下**这些功能是禁用的**，以确保作为DSL嵌入时的安全性。

## 安全设计

### 默认行为（安全模式）

```rust
use aether::Aether;

// 默认创建的引擎禁用所有IO操作
let mut engine = Aether::new();

// 这会失败，因为IO被禁用
let code = r#"
    WRITE_FILE("file.txt", "content")
"#;

match engine.eval(code) {
    Ok(_) => println!("成功"),
    Err(e) => println!("错误: {}", e),  // 错误: Undefined variable: WRITE_FILE
}
```

### 启用IO权限

#### 方法1：启用所有IO权限

```rust
use aether::Aether;

// 创建一个启用所有IO的引擎
let mut engine = Aether::with_all_permissions();

// 现在可以使用文件系统和网络功能了
engine.eval(r#"
    WRITE_FILE("file.txt", "Hello World")
"#).unwrap();
```

#### 方法2：选择性启用

```rust
use aether::{Aether, IOPermissions};

// 只启用文件系统，不启用网络
let mut perms = IOPermissions::default();
perms.filesystem_enabled = true;
perms.network_enabled = false;

let mut engine = Aether::with_permissions(perms);

// 文件系统操作可用
engine.eval(r#"WRITE_FILE("file.txt", "content")"#).unwrap();

// 网络操作不可用
match engine.eval(r#"HTTP_GET("https://example.com")"#) {
    Err(_) => println!("网络访问被阻止"),
    _ => {}
}
```

## 文件系统函数

### READ_FILE(path)

读取文件内容。

```aether
Set CONTENT READ_FILE("data.txt")
PRINTLN(CONTENT)
```

### WRITE_FILE(path, content)

写入文件（覆盖模式）。

```aether
Set DATA "Hello from Aether!"
WRITE_FILE("output.txt", DATA)
```

### APPEND_FILE(path, content)

追加内容到文件。

```aether
APPEND_FILE("log.txt", "New log entry\n")
```

### DELETE_FILE(path)

删除文件。

```aether
DELETE_FILE("temp.txt")
```

### FILE_EXISTS(path)

检查文件是否存在。

```aether
Set EXISTS FILE_EXISTS("config.txt")
If (EXISTS) {
    PRINTLN("文件存在")
} Else {
    PRINTLN("文件不存在")
}
```

### LIST_DIR(path)

列出目录内容。

```aether
Set FILES LIST_DIR(".")
PRINTLN("文件数量: " + TO_STRING(LEN(FILES)))
```

### CREATE_DIR(path)

创建目录（递归创建）。

```aether
CREATE_DIR("data/output")
```

## 网络函数

### HTTP_GET(url)

发送HTTP GET请求。

```aether
Set RESPONSE HTTP_GET("https://api.example.com/data")
PRINTLN(RESPONSE)
```

### HTTP_POST(url, body, [content_type])

发送HTTP POST请求。

```aether
Set JSON_DATA "{\"name\":\"test\",\"value\":123}"
Set RESPONSE HTTP_POST("https://api.example.com/data", JSON_DATA)
PRINTLN(RESPONSE)

// 自定义Content-Type
Set RESPONSE2 HTTP_POST("https://api.example.com/data", JSON_DATA, "application/json")
```

### HTTP_PUT(url, body, [content_type])

发送HTTP PUT请求。

```aether
Set JSON_DATA "{\"name\":\"updated\"}"
Set RESPONSE HTTP_PUT("https://api.example.com/data/1", JSON_DATA)
```

### HTTP_DELETE(url)

发送HTTP DELETE请求。

```aether
Set RESPONSE HTTP_DELETE("https://api.example.com/data/1")
```

## 实际应用示例

### 示例1：配置文件读取

```aether
// 读取配置文件
Set CONFIG_EXISTS FILE_EXISTS("config.json")

If (CONFIG_EXISTS) {
    Set CONFIG_DATA READ_FILE("config.json")
    PRINTLN("配置已加载")
} Else {
    // 创建默认配置
    Set DEFAULT_CONFIG "{\"timeout\": 30, \"retries\": 3}"
    WRITE_FILE("config.json", DEFAULT_CONFIG)
    PRINTLN("已创建默认配置")
}
```

### 示例2：API数据处理

```aether
// 从API获取数据
Set API_RESPONSE HTTP_GET("https://api.example.com/users")

// 保存到本地文件
Set TIMESTAMP TO_STRING(1234567890)
Set FILENAME "cache_" + TIMESTAMP + ".json"
WRITE_FILE(FILENAME, API_RESPONSE)

PRINTLN("数据已缓存到: " + FILENAME)
```

### 示例3：日志记录

```aether
Func LOG_MESSAGE(message) {
    Set TIMESTAMP TO_STRING(1234567890)
    Set LOG_ENTRY "[" + TIMESTAMP + "] " + message + "\n"
    APPEND_FILE("app.log", LOG_ENTRY)
    Return True
}

LOG_MESSAGE("应用启动")
LOG_MESSAGE("处理请求")
LOG_MESSAGE("应用关闭")
```

## 安全最佳实践

1. **最小权限原则**：只启用必要的IO权限

   ```rust
   // ✅ 好的做法：只启用需要的权限
   let mut perms = IOPermissions::default();
   perms.filesystem_enabled = true;  // 只启用文件系统
   
   // ❌ 避免：无需时不要启用所有权限
   let engine = Aether::with_all_permissions();
   ```

2. **沙箱环境**：对于不可信的脚本，使用默认配置

   ```rust
   // 用户提供的脚本应该在沙箱中运行
   let mut sandbox = Aether::new();  // IO被禁用
   match sandbox.eval(user_script) {
       Ok(result) => process_result(result),
       Err(e) => handle_error(e),
   }
   ```

3. **权限检查**：在应用层面也进行权限验证

   ```rust
   fn process_with_io(script: &str, allow_io: bool) -> Result<Value, String> {
       let mut engine = if allow_io {
           Aether::with_all_permissions()
       } else {
           Aether::new()
       };
       
       engine.eval(script)
   }
   ```

## 错误处理

IO操作可能失败，应该适当处理错误：

```aether
Func SAFE_READ_FILE(path) {
    Set EXISTS FILE_EXISTS(path)
    If (EXISTS) {
        Set CONTENT READ_FILE(path)
        Return CONTENT
    } Else {
        PRINTLN("警告: 文件不存在: " + path)
        Return ""
    }
}

Set DATA SAFE_READ_FILE("data.txt")
```

## 性能考虑

1. **网络请求**：可能较慢，考虑超时和重试机制
2. **文件操作**：大文件读写可能消耗内存
3. **并发限制**：Aether 当前是单线程执行

## 平台兼容性

- 文件系统函数：支持 Windows、Linux、macOS
- 网络函数：需要网络连接，支持 HTTPS

## 限制

1. 网络函数不支持自定义请求头（除Content-Type外）
2. 文件操作是同步的，不支持异步IO
3. 没有流式读写支持
4. HTTP响应限制在合理大小（受内存限制）
