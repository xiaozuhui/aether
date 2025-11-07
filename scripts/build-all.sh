#!/bin/bash
# Build script for all Aether language bindings

set -e  # Exit on error

echo "================================"
echo "Building Aether Language Bindings"
echo "================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo -e "${BLUE}Project root: $PROJECT_ROOT${NC}"
echo ""

# Function to print step
print_step() {
    echo -e "${GREEN}==>${NC} $1"
}

# Function to print error
print_error() {
    echo -e "${RED}Error:${NC} $1"
}

# Step 1: Build Rust core library
print_step "Building Rust core library..."
cd "$PROJECT_ROOT"
cargo build --release
if [ $? -ne 0 ]; then
    print_error "Failed to build Rust library"
    exit 1
fi
echo -e "${GREEN}✓${NC} Rust library built successfully"
echo ""

# Step 2: Generate C header file
print_step "Generating C header file..."
cargo build --release  # This triggers build.rs which generates the header
if [ -f "$PROJECT_ROOT/bindings/aether.h" ]; then
    echo -e "${GREEN}✓${NC} C header file generated: bindings/aether.h"
else
    print_error "Failed to generate C header file"
    exit 1
fi
echo ""

# Step 3: Build WASM for TypeScript/JavaScript
print_step "Building WASM module for TypeScript/JavaScript..."
if command -v wasm-pack &> /dev/null; then
    cd "$PROJECT_ROOT"
    wasm-pack build --target bundler --out-dir bindings/typescript/pkg
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} WASM module built successfully"
    else
        print_error "Failed to build WASM module"
        exit 1
    fi
else
    echo -e "${RED}Warning:${NC} wasm-pack not found. Skipping WASM build."
    echo "Install wasm-pack: https://rustwasm.github.io/wasm-pack/installer/"
fi
echo ""

# Step 4: Build TypeScript bindings
print_step "Building TypeScript bindings..."
cd "$PROJECT_ROOT/bindings/typescript"
if command -v npm &> /dev/null; then
    if [ ! -d "node_modules" ]; then
        echo "Installing npm dependencies..."
        npm install
    fi
    npm run build:ts
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} TypeScript bindings built successfully"
    else
        print_error "Failed to build TypeScript bindings"
        exit 1
    fi
else
    echo -e "${RED}Warning:${NC} npm not found. Skipping TypeScript build."
fi
echo ""

# Step 5: Test Go bindings (optional)
print_step "Testing Go bindings..."
cd "$PROJECT_ROOT/bindings/go"
if command -v go &> /dev/null; then
    go mod tidy
    # Note: Tests will only work if the Rust library is properly linked
    echo "Go bindings are ready. Run 'go test' manually to test."
    echo -e "${GREEN}✓${NC} Go bindings configured"
else
    echo -e "${RED}Warning:${NC} Go not found. Skipping Go configuration."
fi
echo ""

# Summary
echo "================================"
echo "Build Summary"
echo "================================"
echo ""
echo "✓ Rust library:       $PROJECT_ROOT/target/release/libaether.*"
echo "✓ C header:           $PROJECT_ROOT/bindings/aether.h"
echo "✓ Go bindings:        $PROJECT_ROOT/bindings/go/"
echo "✓ TypeScript bindings: $PROJECT_ROOT/bindings/typescript/"
echo ""
echo -e "${GREEN}All builds completed successfully!${NC}"
echo ""
echo "Next steps:"
echo "  - Go:         cd bindings/go && go test"
echo "  - TypeScript: cd bindings/typescript && npm test"
echo ""
