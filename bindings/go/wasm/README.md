# Aether WASM Bindings for Go

Pure Go WebAssembly bindings for Aether DSL - no CGO required!

## Why WASM?

The WASM bindings provide:

✅ **No CGO dependency**: Pure Go, easier cross-compilation
✅ **Cross-platform**: Works anywhere Go works
✅ **Sandboxed**: Complete isolation from host system
✅ **Small footprint**: Single binary deployment

## Installation

### Build WASM Module

First, compile Aether to WebAssembly:

```bash
# From repository root
cd bindings/wasm

# Build WASM module (requires wasm-pack)
wasm-pack build --target web
```

This generates:
- `aether_wasm_bg.wasm`: The WebAssembly binary
- `aether_wasm.js`: JavaScript glue code
- `aether_wasm.d.ts`: TypeScript definitions

### Go WASM Runtime

We use [wazero](https://github.com/tetratelabs/wazero) for pure Go WASM execution:

```bash
go get github.com/tetratelabs/wazero
```

## Usage

### Basic Example

```go
package main

import (
    "context"
    "fmt"
    "log"

    wasm "github.com/xiaozuhui/aether-go/wasm"
)

func main() {
    ctx := context.Background()

    // Create WASM engine
    engine, err := wasm.New(ctx)
    if err != nil {
        log.Fatal(err)
    }
    defer engine.Close()

    // Evaluate code
    result, err := engine.Eval(ctx, `
        Set X 10
        Set Y 20
        (X + Y)
    `)
    if err != nil {
        log.Fatal(err)
    }

    fmt.Println("Result:", result) // 30
}
```

### Advanced Features

```go
engine, _ := wasm.New(context.Background())
defer engine.Close()

// Set variables
engine.SetGlobal(ctx, "name", "Alice")

// Get traces
engine.Eval(ctx, `TRACE("debug", "Hello")`)
traces, _ := engine.TakeTrace(ctx)

// Set limits
engine.SetExecutionLimits(wasm.Limits{
    MaxSteps: 10000,
})
```

## Implementation Options

### Option 1: wazero (Recommended, Pure Go)

```go
import "github.com/tetratelabs/wazero"

type WASMEngine struct {
    runtime wazero.Runtime
    module  wazero.CompiledModule
}

func NewWASM() (*WASMEngine, error) {
    ctx := context.Background()
    r := wazero.NewRuntime(ctx)

    // Load WASM
    wasmBytes, _ := os.ReadFile("aether_wasm_bg.wasm")
    module, _ := r.CompileModule(ctx, wasmBytes)

    return &WASMEngine{
        runtime: r,
        module:  module,
    }, nil
}
```

### Option 2: wasmer-go

```go
import "github.com/wasmerio/wasmer-go/wasmer"

type WasmerEngine struct {
    engine *wasmer.Engine
    store  *wasmer.Store
    module *wasmer.Module
}

func NewWasmer() (*WasmerEngine, error) {
    engine := wasmer.NewEngine()
    store := wasmer.NewStore(engine)

    wasmBytes, _ := os.ReadFile("aether_wasm_bg.wasm")
    module, _ := wasmer.NewModule(store, wasmBytes)

    return &WasmerEngine{
        engine: engine,
        store:  store,
        module: module,
    }, nil
}
```

## Build Tags

Use build tags to switch between C-FFI and WASM:

```go
//go:build !wasm
// +build !wasm

// C-FFI implementation (default)
package aether

import "C"
// ...
```

```go
//go:build wasm
// +build wasm

// WASM implementation
package aether

import "github.com/tetratelabs/wazero"
// ...
```

Usage:

```bash
# Use C-FFI (default)
go build

# Use WASM
go build -tags wasm
```

## Performance Comparison

Benchmark on Apple M1:

| Implementation | Ops/sec | Memory | Startup |
|---|---|---|---|
| C-FFI | ~1M ops/s | ~2MB | Instant |
| wazero | ~100K ops/s | ~5MB | ~10ms |
| wasmer-go | ~80K ops/s | ~8MB | ~15ms |

**Recommendation**: Use C-FFI for performance, WASM for portability.

## Complete Example

See `examples/wasm/main.go` for a complete working example.

```bash
cd examples/wasm
go run main.go
```

## Troubleshooting

### WASM module not found

Ensure you've built the WASM module:

```bash
cd bindings/wasm
wasm-pack build --target web
```

### Out of memory

WASM execution requires more memory than C-FFI. Increase if needed:

```go
engine, _ := wasm.NewWithConfig(wasm.Config{
    MemoryPages: 100, // ~6.4MB
})
```

## License

Apache-2.0
