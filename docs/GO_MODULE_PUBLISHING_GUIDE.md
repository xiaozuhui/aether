# Go Module Publishing Guide

This guide explains how to publish the Aether Go bindings as a Go module.

## Prerequisites

1. Go 1.16 or later
2. GitHub account
3. Built Rust library (`cargo build --release`)

## Repository Structure

```
github.com/xiaozuhui/aether-go/
├── go.mod
├── go.sum
├── LICENSE
├── README.md
├── api/
│   └── aether.go           # Public API (enhanced)
├── internal/
│   ├── cffi/
│   │   ├── aether.h        # Generated C header
│   │   ├── aether.c        # (Optional, if needed)
│   │   └── aether.go       # CGo bindings
│   └── wasm/
│       ├── aether_wasm.go  # WASM bindings
│       └── aether_wasm.wasm # Compiled WASM
├── pkg/
│   ├── types/
│   │   └── types.go        # Public types
│   └── errors/
│       └── errors.go       # Error handling
└── examples/
    └── enhanced/
        └── main.go         # Complete example
```

## Setup

### 1. Create Go Module

```bash
mkdir -p aether-go
cd aether-go

# Initialize module
go mod init github.com/xiaozuhui/aether-go

# Create directory structure
mkdir -p internal/cffi internal/wasm pkg/types pkg/errors api examples/enhanced
```

### 2. Copy Files

From your Aether repository:

```bash
# Copy enhanced bindings
cp bindings/go/aether_enhanced.go api/aether.go

# Copy tests
cp bindings/go/aether_enhanced_test.go api/aether_test.go

# Copy C header
cp bindings/aether.h internal/cffi/aether.h

# Copy examples
cp -r bindings/go/examples/enhanced/* examples/enhanced/
```

### 3. Create go.mod

```go
module github.com/xiaozuhui/aether-go

go 1.21

require (
    github.com/tetratelabs/wazero v1.0.0 // Optional, for WASM
)
```

### 4. Create Types Package

```go
// pkg/types/types.go
package types

// Limits controls execution constraints
type Limits struct {
    MaxSteps          int
    MaxRecursionDepth int
    MaxDurationMs     int
}

// CacheStats represents cache statistics
type CacheStats struct {
    Hits   int
    Misses int
    Size   int
}

// TraceStats represents trace statistics
type TraceStats struct {
    TotalEntries int            `json:"total_entries"`
    ByLevel      map[string]int `json:"by_level"`
    ByCategory   map[string]int `json:"by_category"`
    BufferSize   int            `json:"buffer_size"`
    BufferFull   bool           `json:"buffer_full"`
}

// TraceEntry represents a single trace entry
type TraceEntry struct {
    Level     string   `json:"level"`
    Category  string   `json:"category"`
    Timestamp int64    `json:"timestamp"`
    Values    []string `json:"values"`
    Label     *string  `json:"label,omitempty"`
}
```

### 5. Create Build Tags for C-FFI vs WASM

**api/aether_cffi.go:**

```go
//go:build cffi
// +build cffi

package api

/*
#cgo LDFLAGS: -L${SRCDIR}/../../internal/cffi -laether -ldl -lm -lpthread
#cgo darwin LDFLAGS: -framework Security -framework CoreFoundation
#include "aether.h"
*/
import "C"
import "sync"

type Engine struct {
    handle *C.AetherHandle
    mu     sync.RWMutex
}

func New() *Engine {
    return &Engine{
        handle: C.aether_new(),
    }
}

// ... rest of C-FFI implementation
```

**api/aether_wasm.go:**

```go
//go:build wasm
// +build wasm

package api

import (
    "context"

    "github.com/tetratelabs/wazero"
)

type Engine struct {
    runtime wazero.Runtime
    module  wazero.CompiledModule
}

func New() (*Engine, error) {
    ctx := context.Background()
    r := wazero.NewRuntime(ctx)

    // Load WASM module
    // ...

    return &Engine{runtime: r}, nil
}

// ... rest of WASM implementation
```

## Building

### Build C-FFI Version (Default)

```bash
# Ensure Rust library is built
cd ../Aether
cargo build --release

# Build Go module
cd ../aether-go
go build ./...
```

### Build WASM Version

```bash
# Build with WASM tag
go build -tags wasm ./...
```

### Build Both

```bash
# Build all variants
go build ./...
go build -tags wasm ./...
```

## Testing

```bash
# Test all
go test ./...

# Test specific package
go test ./api

# Test with race detector
go test -race ./...

# Test with coverage
go test -cover ./...

# Benchmarks
go test -bench=. -benchmem ./...
```

## Versioning

Follow semantic versioning:

1. Update `go.mod`:

```bash
# Bump version
go mod edit -module=github.com/xiaozuhui/aether-go/v2
```

2. Update version in code:

```go
const Version = "2.0.0"
```

3. Tag release:

```bash
git tag v2.0.0
git push origin v2.0.0
```

## Publishing

### 1. Create GitHub Repository

```bash
# Create repository on GitHub
gh repo create aether-go --public --source=. --remote=origin
```

### 2. Push to GitHub

```bash
git add .
git commit -m "Initial release: v2.0.0"
git push origin main
git push origin v2.0.0
```

### 3. Go Module Proxy

The Go module proxy will automatically pick up your module:

```bash
# Verify it's available
go list -m -versions github.com/xiaozuhui/aether-go
```

## Usage

Users can now install your module:

```bash
# Install
go get github.com/xiaozuhui/aether-go@latest

# Or specific version
go get github.com/xiaozuhui/aether-go@v2.0.0

# Use in code
import "github.com/xiaozuhui/aether-go/api"
```

## CI/CD

### GitHub Actions

Create `.github/workflows/test.yml`:

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        go-version: [1.21, 1.22]
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}

    steps:
    - uses: actions/checkout@v3

    - name: Set up Go
      uses: actions/setup-go@v4
      with:
        go-version: ${{ matrix.go-version }}

    - name: Build Rust library
      run: |
        git clone https://github.com/xiaozuhui/aether.git
        cd aether
        cargo build --release

    - name: Test
      run: go test -v -race ./...
```

### Release Workflow

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Set up Go
      uses: actions/setup-go@v4
      with:
        go-version: '1.22'

    - name: Run GoReleaser
      uses: goreleaser/goreleaser-action@v4
      with:
        distribution: goreleaser
        version: latest
        args: release --clean
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Documentation

### pkg.go.dev Documentation

Your module will automatically appear on:
https://pkg.go.dev/github.com/xiaozuhui/aether-go

To update:

```bash
# Request indexing
curl https://pkg.go.dev/fetch/github.com/xiaozuhui/aether-go@latest
```

## Troubleshooting

### CGo Errors

```
# error: 'aether.h' file not found
```

Solution: Ensure Rust library is built:

```bash
cd ../Aether
cargo build --release
```

### WASM Not Found

```
# error: aether_wasm.wasm not found
```

Solution: Build WASM module:

```bash
cd ../Aether/bindings/wasm
wasm-pack build --target web
```

### Import Path Errors

```
# cannot find package "github.com/xiaozuhui/aether-go/..."
```

Solution: Ensure module is pushed to GitHub:

```bash
git push origin main
```

Then update:

```bash
go mod tidy
go get -u
```

## Maintenance

### Regular Updates

1. Update dependencies:
```bash
go get -u ./...
go mod tidy
```

2. Run tests:
```bash
go test ./...
```

3. Update documentation

4. Tag new release:
```bash
git tag v2.0.1
git push origin v2.0.1
```

### Supporting New Aether Versions

When Aether releases a new version:

1. Update submodule or copy new files
2. Update bindings if API changed
3. Test thoroughly
4. Release new minor version

## Checklist Before Release

- [ ] All tests pass
- [ ] Benchmarks run successfully
- [ ] Documentation updated
- [ ] README updated
- [ ] CHANGELOG updated
- [ ] Version number updated
- [ ] Tagged in Git
- [ ] Pushed to GitHub
- [ ] Verified on pkg.go.dev

## References

- [Go Modules Reference](https://golang.org/ref/mod)
- [pkg.go.dev](https://pkg.go.dev)
- [SemVer](https://semver.org/)
