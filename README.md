# Aether

<div align="center">

## A lightweight, embeddable domain-specific language (DSL)

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
- 🔍 **Enhanced Error Reporting**: Detailed error messages with line and column numbers
- ✅ **Strict Naming Conventions**: Enforced UPPER_SNAKE_CASE for consistency
- 🔒 **Security-First Design**: IO disabled by default in library mode, enabled in CLI mode

## 🌟 Features

- **Basic Types**: Numbers, Strings, Booleans, Arrays, Dictionaries
- **Functions**: First-class functions with closures
- **Control Flow**: If/Else, While, For loops
- **Generators**: Lazy sequences with `Generator` keyword
- **Lazy Evaluation**: Deferred computation with `Lazy` keyword
- **Naming Convention**: Enforced UPPER_SNAKE_CASE for variables, functions, and parameters
- **Error Reporting**: Detailed error messages with line/column numbers and helpful suggestions
- **Rich Standard Library**: 190+ built-in functions including:
  - I/O operations (PRINT, PRINTLN, INPUT)
  - **File system operations**: READ_FILE, WRITE_FILE, LIST_DIR, etc.
  - **Network operations**: HTTP_GET, HTTP_POST, HTTP_PUT, HTTP_DELETE
  - Type conversions and introspection
  - Array and string manipulation
  - Dictionary operations
  - **Advanced mathematics**: Linear algebra, statistics, probability distributions
  - **Precise arithmetic**: Fraction-based calculations for exact results
  - **Precision arithmetic**: Fixed decimal place calculations for financial computations
  - **Scientific computing**: Linear regression, normal/Poisson distributions, matrix inversion
  - **Payroll calculations**: Comprehensive payroll and compensation module (78 functions)
    - Basic salary calculations (hourly, daily, monthly, annual)
    - Overtime pay (weekday 1.5x, weekend 2x, holiday 3x)
    - Personal income tax (7-bracket progressive tax, annual bonus tax)
    - Social insurance and housing fund
    - Attendance and leave management
    - Bonuses and allowances
    - Salary conversion and proration (21.75 legal pay days standard)
    - Date/time calculations for payroll
    - Statistical analysis for compensation data
- **Flexible Security Model**:
  - **CLI mode**: IO enabled by default (convenient for direct usage)
  - **Library mode**: IO disabled by default (secure for DSL embedding)
  - Granular permission control for filesystem and network operations

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

### Command-Line Usage (IO Enabled by Default)

When you run Aether scripts from the command line, all IO capabilities are **automatically enabled** for your convenience:

**Run a script file:**

```bash
# Run an Aether script - IO is enabled automatically
aether my_script.aether

# Example: File operations work out of the box
aether examples/test_cli_io.aether
```

**Interactive REPL:**

```bash
# Start interactive mode - IO is enabled automatically
aether

# You can use file and network operations directly:
aether[1]> WriteFile("test.txt", "Hello, World!")
true
aether[2]> ReadFile("test.txt")
"Hello, World!"
aether[3]> HttpGet("https://api.github.com")
"{...}"
aether[4]> exit
```

### Library Usage (Rust) - Secure by Default

When embedding Aether as a DSL, **IO is disabled by default** for security:

```rust
use aether::{Aether, IOPermissions};

fn main() {
    // Option 1: Default (no IO) - secure for untrusted scripts
    let mut engine = Aether::new();
    
    // Option 2: Custom permissions - granular control
    let permissions = IOPermissions {
        filesystem_enabled: true,   // Allow file operations
        network_enabled: false,      // Block network operations
    };
    let mut engine = Aether::with_permissions(permissions);
    
    // Option 3: Full permissions - trust all operations
    let mut engine = Aether::with_all_permissions();
    
    // Basic arithmetic (always works, no IO needed)
    let code = r#"
        Set X 10
        Set Y 20
        (X + Y)
    "#;
    
    match engine.eval(code) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // File operations require permissions
    let code = r#"
        WriteFile("output.txt", "Result: 30")
        ReadFile("output.txt")
    "#;
    
    // This will fail with default Aether::new() (secure)
    // This will work with Aether::with_all_permissions()
    match engine.eval(code) {
        Ok(result) => println!("File content: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
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

### Precise and Precision Arithmetic

```javascript
// Avoid floating-point precision issues with fractions
Set A 0.1
Set B 0.2
Println(A + B)  // May show: 0.30000000000000004

// Use fraction arithmetic for exact results
Set FA TO_FRACTION(0.1)
Set FB TO_FRACTION(0.2)
Set FC FRAC_ADD(FA, FB)
Println(FC)           // Shows: 3/10
Println(TO_FLOAT(FC)) // Shows: 0.3

// Financial calculations with fixed precision
Set PRICE1 19.99
Set PRICE2 29.99
Set TOTAL ADD_WITH_PRECISION(PRICE1, PRICE2, 2)
Println(TOTAL)  // Shows: 49.98

// Calculate tax with 2 decimal places
Set TAX MUL_WITH_PRECISION(TOTAL, 0.08, 2)
Println(TAX)    // Shows: 4.00
```

### File System and Network Operations

**CLI Mode (IO Enabled Automatically):**

When running Aether from the command line, all IO operations work out of the box:

```javascript
// File operations - works in CLI mode without configuration
WriteFile("data.txt", "Hello, World!")
Set CONTENT ReadFile("data.txt")
Println(CONTENT)  // Prints: Hello, World!

// Check if file exists
If FileExists("data.txt") {
    Println("File exists!")
    DeleteFile("data.txt")
}

// Directory operations
CreateDir("output")
Set FILES ListDir(".")
For FILE In FILES {
    Println(FILE)
}

// Network operations - works in CLI mode without configuration
Set RESPONSE HttpGet("https://api.github.com")
Println(RESPONSE)

// POST request with custom content type
Set DATA '{"name": "test"}'
Set RESULT HttpPost("https://api.example.com/data", DATA, "application/json")
Println(RESULT)
```

**Library Mode (Requires Explicit Permissions):**

When embedding Aether as a DSL, you must explicitly enable IO for security:

```rust
use aether::{Aether, IOPermissions};

// Option 1: Enable all IO (if you trust the scripts)
let mut engine = Aether::with_all_permissions();

// Option 2: Enable only specific operations (recommended)
let permissions = IOPermissions {
    filesystem_enabled: true,   // Allow file operations
    network_enabled: false,      // Block network operations
};
let mut engine = Aether::with_permissions(permissions);

// Option 3: No IO at all (most secure, default)
let mut engine = Aether::new();  // IO disabled by default

let code = r#"
    WriteFile("output.txt", "Result: 42")
    ReadFile("output.txt")
"#;

match engine.eval(code) {
    Ok(result) => println!("{}", result),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Security Model:**

- **CLI mode**: IO enabled by default (you explicitly run the script)
- **Library mode**: IO disabled by default (scripts may be untrusted)

See [docs/IO_QUICKSTART.md](docs/IO_QUICKSTART.md) and [docs/IO_PERMISSIONS.md](docs/IO_PERMISSIONS.md) for details.

### Enhanced Error Reporting

```javascript
// Invalid variable name (not UPPER_SNAKE_CASE)
Set myVar 10
// Error: Parse error at line 1, column 4: Invalid identifier 'myVar' - 
// 变量名和函数名必须使用全大写字母和下划线（例如：MY_VAR, CALCULATE_SUM）

// Correct variable name
Set MY_VAR 10  // ✅ Correct

// Syntax error with location
Set RESULT (X + Y
// Error: Parse error at line 1, column 18: Expected RightParen, found Newline
```

## 🛠️ Development Status

Aether is currently in **version 0.1.0**. Current features:

- ✅ Complete interpreter (Lexer, Parser, Evaluator)
- ✅ **112 built-in functions** across all categories (including 17 new precision/fraction functions)
- ✅ **Enhanced error reporting** with line/column numbers and detailed messages
- ✅ **Strict naming conventions** (UPPER_SNAKE_CASE enforcement)
- ✅ **114 tests passing** (100% pass rate)
- ✅ Advanced math library (linear regression, probability distributions, matrix operations)
- ✅ Precise arithmetic (fraction-based calculations)
- ✅ Precision arithmetic (fixed decimal place calculations)
- ✅ Go bindings via C-FFI
- ✅ TypeScript/JavaScript bindings via WASM
- 🔄 Python bindings (planned)

See [docs/USER_GUIDE.md](docs/USER_GUIDE.md), [docs/PRECISION_GUIDE.md](docs/PRECISION_GUIDE.md), [docs/ERROR_REPORTING.md](docs/ERROR_REPORTING.md), and [QUICKSTART_BINDINGS.md](QUICKSTART_BINDINGS.md) for complete documentation. Check [CHANGELOG.md](CHANGELOG.md) for version history.

## 📖 Documentation

- **[User Guide](docs/USER_GUIDE.md)** - Complete reference for all built-in functions
- **[Precision Guide](docs/PRECISION_GUIDE.md)** - Guide to precise and precision arithmetic
- **[Error Reporting Guide](docs/ERROR_REPORTING.md)** - Error messages and naming conventions
- **[IO Quick Start](docs/IO_QUICKSTART.md)** - File system and network operations with security
- **[IO Permissions Guide](docs/IO_PERMISSIONS.md)** - Understanding CLI vs library security models
- **[Language Bindings Quick Start](QUICKSTART_BINDINGS.md)** - Using Aether from Go, TypeScript/JavaScript
- **[Language Bindings Overview](bindings/README.md)** - Detailed guide for all language bindings
- **[Changelog](CHANGELOG.md)** - Version history and roadmap
- **[Development Guide](DEVELOPMENT.md)** - Implementation and contribution guide

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

# Quick benchmark (faster, reduced sampling)
./scripts/bench.sh quick

# Run specific benchmark category
./scripts/bench.sh arithmetic
```

### Performance Benchmarks

Aether includes comprehensive performance benchmarks using Criterion.rs:

```bash
# Run all benchmarks
cargo bench

# Run quick benchmarks (10 samples, ~2-3 minutes)
cargo bench -- --sample-size 10

# View results in browser
open target/criterion/report/index.html
```

**Benchmark Coverage:**

- Arithmetic operations (simple, complex, nested)
- Variable operations (assignment, reading, multiple variables)
- Function calls (built-in, user-defined, recursive)
- Control flow (if/else, while, for loops)
- Data structures (arrays, dictionaries, strings)
- Parsing performance (lexer, parser, full execution)
- Different program sizes (small, medium, large)

See [`benches/README.md`](benches/README.md) and [`benches/QUICKSTART.md`](benches/QUICKSTART.md) for detailed documentation.

**Performance Comparison Workflow:**

```bash
# Save baseline before optimization
cargo bench -- --save-baseline before

# Make your changes...

# Compare with baseline
cargo bench -- --baseline before
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
