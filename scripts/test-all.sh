#!/bin/bash
# Test script for Aether language bindings

set -e

echo "================================"
echo "Testing Aether Language Bindings"
echo "================================"
echo ""

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

print_step() {
    echo -e "${GREEN}==>${NC} $1"
}

print_error() {
    echo -e "${RED}Error:${NC} $1"
}

# Test Rust core
print_step "Testing Rust core library..."
cd "$PROJECT_ROOT"
cargo test --lib
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Rust tests passed"
else
    print_error "Rust tests failed"
    exit 1
fi
echo ""

# Test Go bindings
print_step "Testing Go bindings..."
cd "$PROJECT_ROOT/bindings/go"
if command -v go &> /dev/null; then
    cargo build --release -p aether  # Ensure library is built
    go test -v
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} Go tests passed"
    else
        print_error "Go tests failed"
        exit 1
    fi
else
    echo -e "${RED}Warning:${NC} Go not found. Skipping Go tests."
fi
echo ""

# Test TypeScript bindings
print_step "Testing TypeScript bindings..."
cd "$PROJECT_ROOT/bindings/typescript"
if command -v npm &> /dev/null; then
    npm test
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} TypeScript tests passed"
    else
        print_error "TypeScript tests failed"
        exit 1
    fi
else
    echo -e "${RED}Warning:${NC} npm not found. Skipping TypeScript tests."
fi
echo ""

echo "================================"
echo -e "${GREEN}All tests passed!${NC}"
echo "================================"
