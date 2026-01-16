#!/bin/bash
# 发布 Aether 到 crates.io 的自动化脚本

set -e  # 遇到错误立即退出

echo "🚀 Aether 发布脚本"
echo "=================="

# 检查是否有未提交的更改
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ 错误：有未提交的更改，请先提交所有更改"
    git status --short
    exit 1
fi

# 获取当前版本
CURRENT_VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
echo "📦 当前版本: v$CURRENT_VERSION"

# 询问是否继续
read -p "是否继续发布版本 v$CURRENT_VERSION? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ 取消发布"
    exit 1
fi

echo ""
echo "🔍 步骤 1/7: 运行测试..."
cargo test --all || {
    echo "❌ 测试失败"
    exit 1
}

echo ""
echo "🔍 步骤 2/7: 检查代码格式..."
cargo fmt --check || {
    echo "⚠️  代码格式不符合规范，正在自动格式化..."
    cargo fmt
    exit 1
}

echo ""
echo "🔍 步骤 3/7: 运行 clippy..."
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "❌ Clippy 检查失败" 
    exit 1
}

echo ""
echo "🔍 步骤 4/7: 构建发布版本..."
cargo build --release || {
    echo "❌ 构建失败"
    exit 1
}

echo ""
echo "🔍 步骤 5/7: 生成文档..."
cargo doc --no-deps || {
    echo "❌ 文档生成失败"
    exit 1
}

echo ""
echo "🔍 步骤 6/7: 打包测试..."
cargo package --allow-dirty || {
    echo "❌ 打包失败"
    exit 1
}

echo ""
echo "🔍 步骤 7/7: 创建 Git 标签..."
git tag -a "v$CURRENT_VERSION" -m "Release version $CURRENT_VERSION" || {
    echo "⚠️  标签可能已存在，跳过..."
}

echo ""
echo "📤 准备发布到 crates.io..."
read -p "确认发布? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ 取消发布"
    exit 1
fi

echo ""
echo "🚀 发布到 crates.io..."
cargo publish || {
    echo "❌ 发布失败"
    exit 1
}

echo ""
echo "📤 推送标签到远程仓库..."
git push origin "v$CURRENT_VERSION" || {
    echo "⚠️  标签推送失败，请手动推送: git push origin v$CURRENT_VERSION"
}

echo ""
echo "✅ 发布成功！"
echo ""
echo "📊 查看统计: https://crates.io/crates/aether-azathoth"
echo "📖 查看文档: https://docs.rs/aether-azathoth/latest/aether/"
echo "🔗 仓库地址: https://github.com/xiaozuhui/aether"
echo ""
echo "🎉 版本 v$CURRENT_VERSION 已成功发布到 crates.io!"
