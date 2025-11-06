#!/bin/bash
# 综合测试脚本

echo "=== Aether IO功能测试 ==="
echo ""

echo "1. 测试权限控制..."
cargo run --example test_permissions --quiet
echo ""

echo "2. 测试文件系统操作..."
cargo run --example demo_io --quiet
echo ""

echo "3. 清理测试文件..."
rm -f test.txt demo_output.txt test_dir 2>/dev/null || true
echo "✓ 清理完成"

echo ""
echo "=== 所有测试完成 ==="
