# Aether Go 绑定

Aether DSL 语言的 Go 语言绑定。

## 特性

✅ **线程安全**: 完全的并发安全,使用 `sync.RWMutex` 保护
✅ **变量操作**: 从 Go 设置/获取变量
✅ **追踪与调试**: 结构化日志和执行追踪
✅ **执行限制**: 控制资源使用
✅ **缓存控制**: AST 缓存与统计
✅ **优化**: 可配置的优化选项
✅ **易于集成**: 简洁、符合 Go 习惯的 API

## 安装

首先,构建 Aether 库:

```bash
cd ../..
cargo build --release
```

然后,使用 Go 绑定:

```bash
cd bindings/go
go mod tidy
```

## 快速开始

### 基本示例

```go
package main

import (
    "fmt"
    "log"

    "github.com/xiaozuhui/aether-go"
)

func main() {
    // 创建一个新的 Aether 引擎
    engine := aether.New()
    defer engine.Close()

    // 执行一些代码
    code := `
        Set X 10
        Set Y 20
        (X + Y)
    `

    result, err := engine.Eval(code)
    if err != nil {
        log.Fatal(err)
    }

    fmt.Println("结果:", result) // 输出: 结果: 30
}
```

### 启用 IO 权限

```go
// 创建启用 IO 权限的引擎
engine := aether.NewWithPermissions()
defer engine.Close()

code := `
    Set DATA (READ_FILE("data.txt"))
    Print "数据:", DATA
`

_, err := engine.Eval(code)
if err != nil {
    log.Fatal(err)
}
```

### 函数和控制流

```go
code := `
    Func FACTORIAL (N) {
        If (N <= 1) {
            Return 1
        }
        Return (N * FACTORIAL(N - 1))
    }

    FACTORIAL(5)
`

result, _ := engine.Eval(code)
fmt.Println(result) // 输出: 120
```

## 高级用法

### 变量操作

```go
engine := aether.New()
defer engine.Close()

// 从 Go 设置变量
engine.SetGlobal("name", "Alice")

// 在 Aether 中使用
result, _ := engine.Eval(`"Hello, " + name`)
fmt.Println(result) // Hello, Alice

// 获取变量值
value, _ := engine.GetGlobal("name")
fmt.Println(value) // Alice

// 设置复杂类型
engine.SetGlobal("config", map[string]interface{}{
    "host": "localhost",
    "port": 8080,
})
```

### 追踪与调试

```go
engine := aether.New()
defer engine.Close()

// 执行带 TRACE 的代码
engine.Eval(`
    TRACE_DEBUG("api", "请求开始")
    Set X 42
    TRACE_INFO("calc", "X = ", X)
`)

// 获取追踪
traces, _ := engine.TakeTrace()
for _, trace := range traces {
    fmt.Println(trace)
}

// 获取结构化记录
records, _ := engine.TraceRecords()
for _, record := range records {
    fmt.Printf("[%s] %s: %v\n",
        record.Level, record.Category, record.Values)
}

// 获取统计
stats, _ := engine.TraceStats()
fmt.Printf("总追踪数: %d\n", stats.TotalEntries)
```

### 执行限制

```go
engine := aether.New()
defer engine.Close()

// 设置限制
engine.SetExecutionLimits(aether.Limits{
    MaxSteps:          10000,
    MaxRecursionDepth: 100,
    MaxDurationMs:     5000,
})

// 获取限制
limits, _ := engine.GetExecutionLimits()
fmt.Printf("最大步数: %d\n", limits.MaxSteps)
```

### 缓存控制

```go
engine := aether.New()
defer engine.Close()

// 多次执行代码(第二次使用缓存)
code := "Set X 10\n(X + 20)"
for i := 0; i < 10; i++ {
    engine.Eval(code)
}

// 获取缓存统计
stats, _ := engine.CacheStats()
fmt.Printf("缓存命中: %d, 未命中: %d\n",
    stats.Hits, stats.Misses)

// 清除缓存
engine.ClearCache()
```

### 环境重置

```go
engine := aether.New()
defer engine.Close()

// 执行设置变量的代码
engine.Eval("Set X 42\nSet Y 100")

// 重置环境(清除所有变量)
engine.ResetEnv()

// 变量现在已清除
_, err := engine.Eval("X")
// 错误: 未定义的变量: X
```

## 线程安全

引擎完全线程安全,可以并发使用:

```go
engine := aether.New()
defer engine.Close()

var wg sync.WaitGroup

// 并发运行 100 个 goroutine
for i := 0; i < 100; i++ {
    wg.Add(1)
    go func(n int) {
        defer wg.Done()

        code := fmt.Sprintf("Set X %d\n(X * 2)", n)
        result, err := engine.Eval(code)
        if err != nil {
            log.Printf("错误: %v", err)
            return
        }
        log.Printf("结果: %s", result)
    }(i)
}

wg.Wait()
```

## API 参考

### 类型

- `Engine`: 线程安全的 Aether DSL 引擎
- `Limits`: 执行限制配置
- `CacheStats`: 缓存统计
- `TraceStats`: 追踪统计
- `TraceEntry`: 结构化追踪条目

### 函数

#### 引擎创建

- `New() *Engine`: 创建禁用 IO 的引擎
- `NewWithPermissions() *Engine`: 创建启用所有 IO 权限的引擎
- `Version() string`: 获取 Aether 版本

#### 执行

- `Eval(code string) (string, error)`: 执行 Aether 代码

#### 变量

- `SetGlobal(name string, value interface{}) error`: 设置全局变量
- `GetGlobal(name string) (interface{}, error)`: 获取全局变量
- `ResetEnv() error`: 重置环境(清除所有变量)

#### 追踪与调试

- `TakeTrace() ([]string, error)`: 获取所有追踪条目
- `TraceRecords() ([]TraceEntry, error)`: 获取结构化追踪
- `TraceStats() (*TraceStats, error)`: 获取追踪统计
- `ClearTrace() error`: 清除追踪缓冲区

#### 执行控制

- `SetExecutionLimits(Limits) error`: 设置执行限制
- `GetExecutionLimits() (*Limits, error)`: 获取当前限制

#### 缓存

- `CacheStats() (*CacheStats, error)`: 获取缓存统计
- `ClearCache() error`: 清除 AST 缓存

#### 优化

- `SetOptimization(constantFolding, deadCode, tailRecursion bool) error`: 设置优化选项

#### 生命周期

- `Close()`: 释放引擎资源(幂等)

## 构建

Go 绑定需要先构建 Aether 库:

```bash
# 构建 Aether 库
cd ../..
cargo build --release

# 运行 Go 测试
cd bindings/go
go test -v
```

## 示例

参见 `examples/` 目录中的更多示例:

- `examples/enhanced/main.go`: 完整的功能演示

## 性能

在 Apple M1 上的基准测试结果:

```
BenchmarkBasicEval-8             1000000    1023 ns/op
BenchmarkConcurrentEval-8         5000000     423 ns/op
BenchmarkSetGetGlobal-8           3000000     567 ns/op
```

## 错误处理

所有函数都返回错误。始终检查错误:

```go
result, err := engine.Eval(code)
if err != nil {
    // 处理错误
    if strings.Contains(err.Error(), "Parse error") {
        // 语法错误
    } else if strings.Contains(err.Error(), "Runtime error") {
        // 运行时错误
    }
}
```

## 安全性

### 默认模式(受限)

IO 操作默认**禁用**以保证安全:

```go
engine := aether.New()
// 文件操作将失败
_, err := engine.Eval(`READ_FILE("/etc/passwd")`)
// 错误: Runtime error: File IO is disabled
```

### 启用权限

仅对可信代码启用 IO:

```go
engine := aether.NewWithPermissions()
// 文件操作现在可用
content, err := engine.Eval(`READ_FILE("/tmp/data.txt")`)
```

## 许可证

Apache-2.0
