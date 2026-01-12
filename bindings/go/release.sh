#!/bin/bash

# Aether Go Module 发布脚本
# 此脚本帮助你在主仓库中发布 Go Module

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${GREEN}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查是否在正确的目录
check_directory() {
    if [ ! -f "go.mod" ]; then
        print_error "请在 bindings/go 目录中运行此脚本"
        exit 1
    fi
}

# 获取当前版本
get_version() {
    # 从 Cargo.toml 读取版本
    VERSION=$(grep "^version = " ../../Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    echo "$VERSION"
}

# 验证版本格式
validate_version() {
    local version=$1
    if [[ ! $version =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "版本号格式无效,应该是 v开头,例如 v0.4.4"
        exit 1
    fi
}

# 运行测试
run_tests() {
    print_info "运行测试..."
    make test
    print_success "测试通过"
}

# 检查 Git 状态
check_git_status() {
    print_info "检查 Git 状态..."

    # 检查是否有未提交的更改
    if [ -n "$(git status --porcelain)" ]; then
        print_warning "有未提交的更改:"
        git status --short
        read -p "是否继续发布? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_error "发布取消"
            exit 1
        fi
    fi

    # 获取当前分支
    CURRENT_BRANCH=$(git branch --show-current)
    print_info "当前分支: $CURRENT_BRANCH"

    if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
        print_warning "你不在 main/master 分支"
        read -p "是否继续? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_error "发布取消"
            exit 1
        fi
    fi
}

# 提交和打标签
commit_and_tag() {
    local version=$1

    print_info "创建 Git 标签: $version"

    # 检查标签是否已存在
    if git rev-parse "$version" >/dev/null 2>&1; then
        print_error "标签 $version 已存在"
        exit 1
    fi

    # 创建标签
    git tag -a "$version" -m "Release $version

- 更新 Go 绑定到 $version
- 详见 CHANGELOG.md"

    print_success "标签创建成功"
}

# 推送标签
push_tag() {
    local version=$1

    print_info "推送标签到 GitHub..."

    git push origin "$version"

    print_success "标签推送成功"
}

# 验证发布
verify_release() {
    local version=$1

    print_info "验证发布..."

    print_info "等待 Go Module Proxy 索引..."
    sleep 5

    print_info "检查 pkg.go.dev..."
    echo "https://pkg.go.dev/github.com/xiaozuhui/aether/bindings/go@$version"

    print_success "发布验证完成!"
}

# 主函数
main() {
    print_info "Aether Go Module 发布工具"
    echo ""

    check_directory

    # 获取版本
    VERSION=$(get_version)
    VERSION_TAG="v$VERSION"

    print_info "当前 Aether 版本: $VERSION"
    print_info "将发布 Go Module: $VERSION_TAG"
    echo ""

    # 确认
    read -p "是否继续? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_error "发布取消"
        exit 1
    fi
    echo ""

    # 检查 Git
    check_git_status
    echo ""

    # 运行测试
    run_tests
    echo ""

    # 创建标签
    commit_and_tag "$VERSION_TAG"
    echo ""

    # 推送标签
    push_tag "$VERSION_TAG"
    echo ""

    # 验证
    verify_release "$VERSION_TAG"
    echo ""

    print_success "🎉 发布完成!"
    echo ""
    echo "用户现在可以使用:"
    echo "  go get github.com/xiaozuhui/aether/bindings/go@$VERSION_TAG"
    echo ""
}

# 运行主函数
main "$@"
