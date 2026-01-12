# Aether FFI Enhancement Plan

## Overview

This document outlines the plan to enhance the C-FFI layer for better Go integration, including async safety, advanced features, and WASM support.

## Current FFI API Status

### Existing Functions ([src/ffi.rs](src/ffi.rs))

```c
// ✅ Already implemented
AetherHandle* aether_new();
AetherHandle* aether_new_with_permissions();
int aether_eval(AetherHandle* handle, const char* code, char** result, char** error);
const char* aether_version();
void aether_free(AetherHandle* handle);
void aether_free_string(char* s);
```

### Missing Features

❌ **Variable Operations**
- `aether_set_global()` - Set global variable from host
- `aether_get_global()` - Get variable value
- `aether_reset_env()` - Reset environment (clear variables)

❌ **Trace/Debug**
- `aether_take_trace()` - Get trace buffer
- `aether_clear_trace()` - Clear trace buffer
- `aether_trace_records()` - Get structured trace entries

❌ **Execution Limits**
- `aether_set_limits()` - Set execution limits
- `aether_get_limits()` - Get current limits

❌ **Cache Control**
- `aether_clear_cache()` - Clear AST cache
- `aether_cache_stats()` - Get cache statistics

❌ **Optimization**
- `aether_set_optimization()` - Enable/disable optimizations

❌ **Function Registration** (Advanced)
- `aether_register_function()` - Register Go callback as DSL function

❌ **Async Safety**
- Thread-safe evaluation (for concurrent Go usage)
- Mutex protection for shared state

## Proposed New FFI API

### 1. Variable Operations

```c
/// Set a global variable from host application
/// @param handle Aether engine handle
/// @param name Variable name
/// @param value_json Variable value as JSON string
/// @return Error code (0 = success)
int aether_set_global(AetherHandle* handle, const char* name, const char* value_json);

/// Get a variable's value as JSON
/// @param handle Aether engine handle
/// @param name Variable name
/// @param value_json Output parameter (must be freed with aether_free_string)
/// @return Error code (0 = success)
int aether_get_global(AetherHandle* handle, const char* name, char** value_json);

/// Reset the runtime environment (clears all variables)
/// @param handle Aether engine handle
void aether_reset_env(AetherHandle* handle);
```

### 2. Trace/Debug Operations

```c
/// Get all trace entries as JSON array
/// @param handle Aether engine handle
/// @param trace_json Output parameter (must be freed with aether_free_string)
/// @return Error code (0 = success)
int aether_take_trace(AetherHandle* handle, char** trace_json);

/// Clear the trace buffer
/// @param handle Aether engine handle
void aether_clear_trace(AetherHandle* handle);

/// Get structured trace entries as JSON
/// @param handle Aether engine handle
/// @param trace_json Output parameter (must be freed with aether_free_string)
/// @return Error code (0 = success)
int aether_trace_records(AetherHandle* handle, char** trace_json);

/// Get trace statistics as JSON
/// @param handle Aether engine handle
/// @param stats_json Output parameter (must be freed with aether_free_string)
/// @return Error code (0 = success)
int aether_trace_stats(AetherHandle* handle, char** stats_json);
```

### 3. Execution Limits

```c
typedef struct AetherLimits {
    int max_steps;           // Maximum execution steps (-1 = unlimited)
    int max_recursion_depth; // Maximum recursion depth (-1 = unlimited)
    int max_duration_ms;     // Maximum duration in milliseconds (-1 = unlimited)
} AetherLimits;

/// Set execution limits
/// @param handle Aether engine handle
/// @param limits Limits configuration
void aether_set_limits(AetherHandle* handle, const AetherLimits* limits);

/// Get current execution limits
/// @param handle Aether engine handle
/// @param limits Output parameter
void aether_get_limits(AetherHandle* handle, AetherLimits* limits);
```

### 4. Cache Control

```c
typedef struct AetherCacheStats {
    int hits;
    int misses;
    int size;
} AetherCacheStats;

/// Clear the AST cache
/// @param handle Aether engine handle
void aether_clear_cache(AetherHandle* handle);

/// Get cache statistics
/// @param handle Aether engine handle
/// @param stats Output parameter
void aether_cache_stats(AetherHandle* handle, AetherCacheStats* stats);
```

### 5. Optimization Control

```c
/// Set optimization options
/// @param handle Aether engine handle
/// @param constant_folding Enable constant folding
/// @param dead_code_elimination Enable dead code elimination
/// @param tail_recursion Enable tail recursion optimization
void aether_set_optimization(
    AetherHandle* handle,
    int constant_folding,
    int dead_code_elimination,
    int tail_recursion
);
```

### 6. Thread-Safe Evaluation (Async Safety)

```c
/// Evaluate code with internal mutex lock (thread-safe)
/// @param handle Aether engine handle
/// @param code Aether code to evaluate
/// @param result Output parameter (must be freed with aether_free_string)
/// @param error Output parameter (must be freed with aether_free_string)
/// @return Error code (0 = success)
int aether_eval_safe(
    AetherHandle* handle,
    const char* code,
    char** result,
    char** error
);

/// Lock the engine for manual thread management
void aether_lock(AetherHandle* handle);

/// Unlock the engine
void aether_unlock(AetherHandle* handle);
```

### 7. Function Registration (Advanced - Optional)

```c
/// Callback function type for host-registered functions
typedef const char* (*AetherHostFunction)(int argc, const char** argv, char** error);

/// Register a host function
/// @param handle Aether engine handle
/// @param name Function name in DSL
/// @param func Function pointer
/// @param user_data User data to pass to function
/// @return Error code (0 = success)
int aether_register_function(
    AetherHandle* handle,
    const char* name,
    AetherHostFunction func,
    void* user_data
);

/// Unregister a host function
/// @param handle Aether engine handle
/// @param name Function name
void aether_unregister_function(AetherHandle* handle, const char* name);
```

## Implementation Priority

### Phase 1: Core Enhancements (High Priority)
✅ **Variable Operations**
- `aether_set_global()`
- `aether_get_global()`
- `aether_reset_env()`

✅ **Trace Operations**
- `aether_take_trace()`
- `aether_clear_trace()`
- `aether_trace_records()`

✅ **Thread Safety**
- `aether_eval_safe()`
- `aether_lock()` / `aether_unlock()`

### Phase 2: Performance & Control (Medium Priority)
✅ **Execution Limits**
- `aether_set_limits()`
- `aether_get_limits()`

✅ **Cache Control**
- `aether_clear_cache()`
- `aether_cache_stats()`

✅ **Optimization**
- `aether_set_optimization()`

### Phase 3: Advanced Features (Low Priority)
⚠️ **Function Registration**
- `aether_register_function()`
- `aether_unregister_function()`

## Go Binding Design

### Enhanced Go API

```go
package aether

import (
    "encoding/json"
    "sync"
)

// Engine represents a thread-safe Aether DSL engine
type Engine struct {
    handle *C.AetherHandle
    mu     sync.RWMutex
}

// New creates a new thread-safe Aether engine
func New() *Engine {
    return &Engine{
        handle: C.aether_new(),
    }
}

// SetGlobal sets a global variable (thread-safe)
func (e *Engine) SetGlobal(name string, value interface{}) error {
    e.mu.Lock()
    defer e.mu.Unlock()

    jsonData, err := json.Marshal(value)
    if err != nil {
        return err
    }

    cName := C.CString(name)
    cValue := C.CString(string(jsonData))
    defer C.free(unsafe.Pointer(cName))
    defer C.free(unsafe.Pointer(cValue))

    status := C.aether_set_global(e.handle, cName, cValue)
    if status != C.Success {
        return fmt.Errorf("failed to set global variable")
    }

    return nil
}

// GetGlobal gets a variable value (thread-safe)
func (e *Engine) GetGlobal(name string) (interface{}, error) {
    e.mu.RLock()
    defer e.mu.RUnlock()

    cName := C.CString(name)
    defer C.free(unsafe.Pointer(cName))

    var valueJSON *C.char
    status := C.aether_get_global(e.handle, cName, &valueJSON)
    if status != C.Success {
        return nil, fmt.Errorf("variable not found: %s", name)
    }
    defer C.aether_free_string(valueJSON)

    var result interface{}
    err := json.Unmarshal([]byte(C.GoString(valueJSON)), &result)
    return result, err
}

// TakeTrace returns all trace entries
func (e *Engine) TakeTrace() ([]TraceEntry, error) {
    e.mu.RLock()
    defer e.mu.RUnlock()

    var traceJSON *C.char
    status := C.aether_take_trace(e.handle, &traceJSON)
    if status != C.Success {
        return nil, fmt.Errorf("failed to get trace")
    }
    defer C.aether_free_string(traceJSON)

    var entries []TraceEntry
    err := json.Unmarshal([]byte(C.GoString(traceJSON)), &entries)
    return entries, err
}

// SetExecutionLimits sets execution limits
func (e *Engine) SetExecutionLimits(limits Limits) error {
    e.mu.Lock()
    defer e.mu.Unlock()

    cLimits := C.AetherLimits{
        max_steps: C.int(limits.MaxSteps),
        max_recursion_depth: C.int(limits.MaxRecursionDepth),
        max_duration_ms: C.int(limits.MaxDurationMs),
    }

    C.aether_set_limits(e.handle, &cLimits)
    return nil
}

// ClearCache clears the AST cache
func (e *Engine) ClearCache() error {
    e.mu.Lock()
    defer e.mu.Unlock()

    C.aether_clear_cache(e.handle)
    return nil
}

// CacheStats returns cache statistics
func (e *Engine) CacheStats() CacheStats {
    e.mu.RLock()
    defer e.mu.RUnlock()

    var stats C.AetherCacheStats
    C.aether_cache_stats(e.handle, &stats)

    return CacheStats{
        Hits:   int(stats.hits),
        Misses: int(stats.misses),
        Size:   int(stats.size),
    }
}

// EvalSafe evaluates code with automatic locking (thread-safe)
func (e *Engine) EvalSafe(code string) (string, error) {
    e.mu.Lock()
    defer e.mu.Unlock()

    // ... implementation ...
}
```

## WASM Support Plan

### Option 1: wazero (Pure Go, Recommended)

```go
package aether

import (
    "github.com/tetratelabs/wazero"
    "github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

// WASMEngine uses WebAssembly instead of C-FFI
type WASMEngine struct {
    runtime wazero.Runtime
    module  wazero.CompiledModule
    memory  []byte
}

// NewWASM creates a new WASM-based Aether engine
func NewWASM() (*WASMEngine, error) {
    ctx := context.Background()

    // Create wazero runtime
    r := wazero.NewRuntime(ctx)

    // Instantiate WASI
    _, err := wasi_snapshot_preview1.Instantiate(ctx, r)
    if err != nil {
        return nil, err
    }

    // Load Aether WASM module
    wasmBytes, err := os.ReadFile("aether_wasm_bg.wasm")
    if err != nil {
        return nil, err
    }

    module, err := r.CompileModule(ctx, wasmBytes)
    if err != nil {
        return nil, err
    }

    return &WASMEngine{
        runtime: r,
        module:  module,
    }, nil
}

// Eval evaluates Aether code using WASM
func (e *WASMEngine) Eval(code string) (string, error) {
    ctx := context.Background()

    // Instantiate module
    inst, err := e.runtime.InstantiateModule(ctx, e.module, wazero.NewModuleConfig())
    if err != nil {
        return "", err
    }

    // Call eval function
    evalFunc := inst.ExportedFunction("aether_eval")
    results, err := evalFunc.Call(ctx, uint64(len(code)))
    if err != nil {
        return "", err
    }

    // Read result from memory
    resultPtr := results[0]
    // ... read from memory ...

    return result, nil
}
```

### Option 2: wasmer-go

```go
import "github.com/wasmerio/wasmer-go/wasmer"

type WasmerEngine struct {
    engine *wasmer.Engine
    store  *wasmer.Store
    module *wasmer.Module
    instance *wasmer.Instance
}

func NewWasmer() (*WasmerEngine, error) {
    engine := wasmer.NewEngine()
    store := wasmer.NewStore(engine)

    wasmBytes, _ := os.ReadFile("aether_wasm_bg.wasm")
    module, _ := wasmer.NewModule(store, wasmBytes)

    // ... instantiate and configure ...

    return &WasmerEngine{
        engine: engine,
        store:  store,
        module: module,
    }, nil
}
```

## Go Module Publishing Plan

### 1. Repository Structure

```
github.com/xiaozuhui/aether-go/
├── go.mod
├── go.sum
├── README.md
├── LICENSE
├── api/
│   └── aether.go        # Public API
├── internal/
│   ├── cffi/            # C-FFI bindings
│   │   ├── aether.h
│   │   ├── aether.c
│   │   └── aether.go
│   └── wasm/            # WASM bindings
│       ├── aether_wasm.go
│       └── aether_wasm.wasm
├── pkg/
│   ├── types/           # Public types
│   └── errors/          # Error handling
└── examples/
    └── basic/
        └── main.go
```

### 2. Build Tags

```go
//go:build cffi
// +build cffi

package aether

// C-FFI implementation
type Engine struct {
    handle *C.AetherHandle
}
```

```go
//go:build wasm
// +build wasm

package aether

// WASM implementation
type Engine struct {
    runtime wazero.Runtime
    module  wazero.CompiledModule
}
```

```go
//go:build !cffi && !wasm
// +build !cffi,!wasm

package aether

// Fallback implementation (error)
type Engine struct {
    // ...
}
```

### 3. Usage

```bash
# Install with C-FFI (default)
go get github.com/xiaozuhui/aether-go

# Install with WASM support
go get -tags wasm github.com/xiaozuhui/aether-go

# Install both (runtime selection)
go get -tags cffi,wasm github.com/xiaozuhui/aether-go
```

## Implementation Timeline

### Week 1: Rust FFI Extensions
- [ ] Implement variable operations
- [ ] Implement trace operations
- [ ] Implement execution limits
- [ ] Implement cache control
- [ ] Implement optimization control
- [ ] Add thread safety (mutex)
- [ ] Write Rust tests

### Week 2: Go Bindings Enhancement
- [ ] Implement enhanced Go API
- [ ] Add thread safety with sync.RWMutex
- [ ] Implement error handling
- [ ] Write Go tests
- [ ] Create examples

### Week 3: WASM Support
- [ ] Compile Rust to WASM
- [ ] Implement wazero-based Go binding
- [ ] Test WASM functionality
- [ ] Benchmark vs C-FFI

### Week 4: Go Module Publishing
- [ ] Set up repository structure
- [ ] Configure build tags
- [ ] Write documentation
- [ ] Create release workflow
- [ ] Publish to GitHub

## Testing Strategy

### Unit Tests
```go
func TestSetGlobal(t *testing.T) {
    engine := New()
    defer engine.Close()

    err := engine.SetGlobal("x", 42)
    assert.NoError(t, err)

    val, err := engine.GetGlobal("x")
    assert.NoError(t, err)
    assert.Equal(t, 42, val)
}

func TestThreadSafety(t *testing.T) {
    engine := New()
    defer engine.Close()

    var wg sync.WaitGroup
    for i := 0; i < 100; i++ {
        wg.Add(1)
        go func(n int) {
            defer wg.Done()
            engine.EvalSafe(fmt.Sprintf("Set X %d", n))
        }(i)
    }
    wg.Wait()
}
```

### Benchmarks
```go
func BenchmarkEvalSafe(b *testing.B) {
    engine := New()
    defer engine.Close()

    code := "Set X 10\n(X + 20)"

    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        engine.EvalSafe(code)
    }
}

func BenchmarkWASMEval(b *testing.B) {
    engine, _ := NewWASM()
    code := "Set X 10\n(X + 20)"

    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        engine.Eval(code)
    }
}
```

## Migration Guide

### From Old API to New API

```go
// Old API (still supported)
engine := aether.New()
result, err := engine.Eval("Set X 10\n(X + 20)")

// New API (thread-safe)
engine := aether.New()
result, err := engine.EvalSafe("Set X 10\n(X + 20)")

// New API (with manual locking)
engine := aether.New()
engine.Lock()
result, err := engine.Eval("Set X 10\n(X + 20)")
engine.Unlock()

// New API (with variables)
engine := aether.New()
engine.SetGlobal("X", 10)
result, err := engine.EvalSafe("(X + 20)")

// New API (with tracing)
engine := aether.New()
engine.EvalSafe(`TRACE("debug", "Hello")`)
traces, _ := engine.TakeTrace()
for _, trace := range traces {
    fmt.Println(trace)
}
```

## Conclusion

This enhancement plan provides:
- ✅ Thread-safe concurrent usage
- ✅ Advanced variable manipulation
- ✅ Comprehensive tracing/debugging
- ✅ Execution control and limits
- ✅ Performance optimization
- ✅ WASM alternative for cross-platform
- ✅ Clean Go Module API
- ✅ Backward compatibility

Next steps: Implement Phase 1 features in Rust FFI layer.
