# Aether Go Bindings

Go language bindings for the Aether DSL.

## Installation

First, build the Aether library:

```bash
cd ../..
cargo build --release
```

Then, use the Go bindings:

```bash
cd bindings/go
go mod tidy
```

## Usage

### Basic Example

```go
package main

import (
    "fmt"
    "log"
    
    "github.com/xiaozuhui/aether-go"
)

func main() {
    // Create a new Aether engine
    engine := aether.New()
    defer engine.Close()
    
    // Evaluate some code
    code := `
        Set X 10
        Set Y 20
        (X + Y)
    `
    
    result, err := engine.Eval(code)
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Println("Result:", result) // Output: Result: 30
}
```

### With IO Permissions

```go
// Create engine with IO permissions enabled
engine := aether.NewWithPermissions()
defer engine.Close()

code := `
    Set DATA (READ_FILE("data.txt"))
    Print "Data:", DATA
`

_, err := engine.Eval(code)
if err != nil {
    log.Fatal(err)
}
```

### Functions and Control Flow

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
fmt.Println(result) // Output: 120
```

## API Reference

### `New() *Aether`

Creates a new Aether engine with default (restricted) IO permissions.

### `NewWithPermissions() *Aether`

Creates a new Aether engine with all IO permissions enabled.

### `(*Aether) Eval(code string) (string, error)`

Evaluates Aether code and returns the result.

### `(*Aether) Close()`

Frees resources. Should be called when done with the engine.

### `Version() string`

Returns the Aether engine version.

## Building

The Go bindings require the Aether library to be built first:

```bash
# Build the Aether library
cd ../..
cargo build --release

# Run Go tests
cd bindings/go
go test -v
```

## License

Apache-2.0
