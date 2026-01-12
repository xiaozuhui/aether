.PHONY: help build-lib test clean install-example

# 默认目标
help:
	@echo "可用的命令:"
	@echo "  make build-lib    - 从源码构建 Rust 库"
	@echo "  make test         - 运行 Go 测试"
	@echo "  make benchmark    - 运行基准测试"
	@echo "  make example      - 运行示例"
	@echo "  make clean        - 清理构建文件"

# 从源码构建 Rust 库
build-lib:
	@echo "正在构建 Rust 库..."
	@cd ../../ && cargo build --release
	@echo "✅ Rust 库构建完成: ../../target/release/libaether.a"

# 运行测试
test: build-lib
	@echo "正在运行 Go 测试..."
	go test -v -race -coverprofile=coverage.txt
	@echo "✅ 测试完成"
	@go tool cover -func=coverage.txt | tail -1

# 运行基准测试
benchmark: build-lib
	@echo "正在运行基准测试..."
	go test -bench=. -benchmem

# 运行示例
example: build-lib
	@echo "正在运行示例..."
	go run examples/enhanced/main.go

# 清理
clean:
	@echo "清理构建文件..."
	@go clean
	@rm -f coverage.txt
	@echo "✅ 清理完成"
