# Aether

<div align="center">

**A lightweight, embeddable domain-specific language (DSL)**

[![Crates.io](https://img.shields.io/crates/v/aether.svg)](https://crates.io/crates/aether)
[![Documentation](https://docs.rs/aether/badge.svg)](https://docs.rs/aether)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)

[Documentation](https://github.com/yourusername/aether/blob/main/DESIGN.md) | [Development Guide](https://github.com/yourusername/aether/blob/main/DEVELOPMENT.md)

</div>

## 🎯 Overview

Aether is a modern, lightweight scripting language designed to be embedded in Rust, Go, and TypeScript applications. It provides:

- 🚀 **High Performance**: Rust-based interpreter with zero-cost abstractions
- 🔌 **Easy Integration**: Simple API for Rust, Go, and TypeScript
- 🌍 **Cross-Platform**: Supports x86_64, ARM64, and WebAssembly
- ✨ **Modern Features**: Generators, lazy evaluation, and functional programming
- 📝 **Simple Syntax**: Easy to learn and read

## 🌟 Features

- **Basic Types**: Numbers, Strings, Booleans, Arrays, Dictionaries
- **Functions**: First-class functions with closures
- **Control Flow**: If/Else, While, For loops
- **Generators**: Lazy sequences with `Generator` keyword
- **Lazy Evaluation**: Deferred computation with `Lazy` keyword
- **Rich Standard Library**: 95 built-in functions including:
  - I/O operations (Print, Println, Input)
  - Type conversions and introspection
  - Array and string manipulation
  - Dictionary operations
  - **Advanced mathematics**: Linear algebra, statistics, probability distributions
  - **Scientific computing**: Linear regression, normal/Poisson distributions, matrix inversion

## 📦 Installation

### As a Library (Rust)

```toml
[dependencies]
aether = "0.1"
```

### As a Command-Line Tool

Build from source:

```bash
git clone https://github.com/yourusername/aether.git
cd aether
cargo build --release
# The executable will be at target/release/aether
```

Or install with cargo:

```bash
cargo install aether
```

### Go

```bash
go get github.com/yourusername/aether-go
```

### TypeScript/JavaScript

```bash
npm install @yourusername/aether
```

## 🚀 Quick Start

### Command-Line Usage

**Run a script file:**

```bash
# Run an Aether script
aether my_script.aether

# Example: stats_demo.aether
aether examples/stats_demo.aether
```

**Interactive REPL:**

```bash
# Start interactive mode (no arguments)
aether

# Then type Aether code:
aether[1]> Set X 10
aether[2]> Set Y 20
aether[3]> (X + Y)
30
aether[4]> help      # Show help
aether[5]> exit      # Exit REPL
```

### Library Usage (Rust)

```rust
use aether::Aether;

fn main() {
    let mut engine = Aether::new();
    
    // Basic arithmetic
    let code = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;
    
    match engine.eval(code) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Using built-in functions
    let code = r#"
        Set NUMBERS Range(1, 6)
        Set TOTAL Sum(NUMBERS)
        Println(Join(NUMBERS, ", "))
        Println(TOTAL)
    "#;
    
    engine.eval(code).unwrap();
}
```

### Library Usage (Go)

```go
package main

import (
    "fmt"
    "github.com/yourusername/aether-go"
)

func main() {
    engine := aether.New()
    defer engine.Close()
    
    code := `
        Set X 10
        Set Y 20
        Return (X + Y)
    `
    
    result, err := engine.Eval(code)
    if err != nil {
        fmt.Println("Error:", err)
        return
    }
    fmt.Println("Result:", result)
}
```

### Library Usage (TypeScript)

```typescript
import { Aether } from '@yourusername/aether';

async function main() {
    const engine = new Aether();
    await engine.init();
    
    const code = `
        Set X 10
        Set Y 20
        Return (X + Y)
    `;
    
    const result = engine.eval(code);
    console.log('Result:', result);
}

main();
```

## 📚 Language Examples

### Variables and Functions

```javascript
// Variables
Set COUNT 10
Set MESSAGE "Hello, Aether"

// Functions
Func ADD (A, B) {
    Return (A + B)
}

Set RESULT ADD(5, 3)
Print "5 + 3 =", RESULT
```

### Control Flow

```javascript
// If-Else
Func ABS (X) {
    If (X < 0) {
        Return (0 - X)
    } Else {
        Return X
    }
}

// For Loop
For I In RANGE(0, 5) {
    Print "Number:", I
}
```

### Generators

```javascript
Generator FIBONACCI (LIMIT) {
    Set A 0
    Set B 1
    Set COUNT 0
    
    While (COUNT < LIMIT) {
        Yield A
        Set NEXT (A + B)
        Set A B
        Set B NEXT
        Set COUNT (COUNT + 1)
    }
}

For NUM In FIBONACCI(10) {
    Print NUM
}
```

### Lazy Evaluation

```javascript
Lazy EXPENSIVE_DATA (
    Print "Loading large dataset..."
    Return LOAD_FILE("big_data.json")
)

// Data is only loaded when accessed
If (NEEDS_ANALYSIS) {
    Set DATA EXPENSIVE_DATA
    PROCESS(DATA)
}
```

## 🛠️ Development Status

Aether is currently in **version 0.1.0**. Current features:

- ✅ Complete interpreter (Lexer, Parser, Evaluator)
- ✅ **95 built-in functions** across all categories
- ✅ **114 tests passing** (100% pass rate)
- ✅ Advanced math library (linear regression, probability distributions, matrix operations)
- 🔄 Go bindings (planned)
- 🔄 TypeScript/WASM bindings (planned)

See [docs/USER_GUIDE.md](docs/USER_GUIDE.md) for complete function reference and [CHANGELOG.md](CHANGELOG.md) for version history.

## 📖 Documentation

- **[User Guide](docs/USER_GUIDE.md)** - Complete reference for all 95 built-in functions
- **[Changelog](CHANGELOG.md)** - Version history and roadmap
- **[Development Guide](DEVELOPMENT.md)** - Implementation and contribution guide
- **[Architecture](cross-language-architecture.md)** - Cross-language design

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/aether.git
cd aether

# Build and test
cargo build
cargo test

# Run benchmarks
cargo bench
```

## 📄 License

Licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>).

## 🙏 Acknowledgments

Aether is inspired by:

- [Lua](https://www.lua.org/) - Embeddable scripting language
- [Rhai](https://rhai.rs/) - Embedded scripting for Rust
- [Crafting Interpreters](https://craftinginterpreters.com/) - Excellent book on interpreter design

## 📬 Contact

- GitHub Issues: [github.com/yourusername/aether/issues](https://github.com/yourusername/aether/issues)
- Discussions: [github.com/yourusername/aether/discussions](https://github.com/yourusername/aether/discussions)

---

Made with ❤️ by the Aether contributors
