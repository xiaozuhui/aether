# Aether Language Bindings

This directory contains language bindings for Aether, allowing it to be used from multiple programming languages.

## Available Bindings

### 🦀 Rust (Native)

The core Aether implementation is in Rust. Use it directly:

```rust
use aether::Aether;

fn main() {
    let mut engine = Aether::new();
    let result = engine.eval("Set X 10\n(X + 20)").unwrap();
    println!("Result: {}", result);
}
```

**Documentation**: See main [README.md](../README.md)

---

### 🐹 Go

Go bindings using CGO and C-FFI.

**Location**: `bindings/go/`

**Installation**:

```bash
# First, build the Aether library
cargo build --release

# Then use in your Go project
go get github.com/xiaozuhui/aether-go
```

**Usage**:

```go
import "github.com/xiaozuhui/aether-go"

engine := aether.New()
defer engine.Close()

result, err := engine.Eval(`
    Set X 10
    Set Y 20
    (X + Y)
`)
```

**Documentation**: [Go Bindings README](go/README.md)

---

### 📘 TypeScript/JavaScript

WebAssembly bindings for Node.js and browsers.

**Location**: `bindings/typescript/`

**Installation**:

```bash
npm install @xiaozuhui/aether
```

**Usage**:

```typescript
import { Aether } from '@xiaozuhui/aether';

const engine = await Aether.create();
const result = engine.eval(`
    Set X 10
    Set Y 20
    (X + Y)
`);
console.log(result); // 30
```

**Documentation**: [TypeScript Bindings README](typescript/README.md)

---

## Building All Bindings

Use the provided build script to build all language bindings:

```bash
./scripts/build-all.sh
```

This will:

1. Build the Rust core library
2. Generate C header files
3. Build the WASM module for TypeScript/JavaScript
4. Compile TypeScript bindings

## Testing All Bindings

Run tests for all language bindings:

```bash
./scripts/test-all.sh
```

## Architecture

```
┌─────────────────────────────────────────────┐
│         Aether Core (Rust)                  │
│  - Lexer, Parser, Evaluator                 │
│  - Built-in functions                       │
│  - Value system                             │
└─────────────────────────────────────────────┘
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
   ┌────────┐  ┌────────┐  ┌────────┐
   │  Rust  │  │ C-FFI  │  │  WASM  │
   │ Native │  │        │  │        │
   └────────┘  └────────┘  └────────┘
                    │           │
                    ▼           ▼
               ┌────────┐  ┌────────┐
               │   Go   │  │   TS   │
               └────────┘  └────────┘
```

## Language Binding Details

### C-FFI Layer

The C-FFI layer (`src/ffi.rs`) provides:

- `aether_new()`: Create engine
- `aether_eval()`: Evaluate code
- `aether_free()`: Free engine
- `aether_free_string()`: Free strings
- `aether_version()`: Get version

### Go Bindings

- Uses CGO to call C-FFI functions
- Provides idiomatic Go API
- Automatic memory management with finalizers
- Error handling using Go's `error` type

### TypeScript/JavaScript Bindings

- Compiled to WebAssembly using `wasm-pack`
- Automatic type conversion between JS and Aether values
- Promise-based async initialization
- Works in both Node.js and browsers

## Performance Considerations

### Rust Native

- **Fastest**: Direct function calls, no overhead
- Use for: High-performance applications, native tools

### Go (C-FFI)

- **Fast**: Small FFI overhead for each call
- Use for: Server applications, CLI tools, microservices

### TypeScript/JavaScript (WASM)

- **Good**: WASM near-native performance
- Initial load time for WASM module
- Use for: Web applications, Node.js services

## Security Considerations

All bindings support two modes:

1. **Default (Restricted)**: IO operations disabled
   - Safe for untrusted user scripts
   - Suitable for embedded DSL use cases

2. **With Permissions**: IO operations enabled
   - Only use with trusted scripts
   - Required for file/network operations

```rust
// Rust - restricted
let engine = Aether::new();

// Rust - with permissions
let engine = Aether::with_all_permissions();
```

```go
// Go - restricted
engine := aether.New()

// Go - with permissions
engine := aether.NewWithPermissions()
```

```typescript
// TypeScript - restricted
const engine = await Aether.create();

// TypeScript - with permissions
const engine = await Aether.createWithPermissions();
```

## Examples

Each binding directory contains an `examples/` folder with usage examples:

- **Go**: `bindings/go/examples/main.go`
- **TypeScript**: `bindings/typescript/examples/basic.ts`

## Contributing

When adding new language bindings:

1. Add a new directory under `bindings/`
2. Implement the binding using appropriate FFI method
3. Provide comprehensive tests
4. Write documentation and examples
5. Update this README
6. Update build scripts

## Supported Platforms

| Platform       | Rust | Go  | TypeScript |
|----------------|------|-----|------------|
| Linux x86_64   | ✅   | ✅  | ✅         |
| macOS x86_64   | ✅   | ✅  | ✅         |
| macOS ARM64    | ✅   | ✅  | ✅         |
| Windows x86_64 | ✅   | ✅  | ✅         |
| WASM32         | ✅   | ❌  | ✅         |

## Future Bindings

Planned language bindings:

- [ ] Python (using PyO3)
- [ ] Java/Kotlin (JNI)
- [ ] C# (.NET)
- [ ] Ruby (FFI)

## License

Apache-2.0

## Links

- [Main Repository](https://github.com/xiaozuhui/aether)
- [Issue Tracker](https://github.com/xiaozuhui/aether/issues)
- [Documentation](../docs/)
