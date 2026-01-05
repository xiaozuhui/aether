#!/bin/bash
# Aether 基准测试运行脚本

set -e

echo "🚀 开始运行 Aether 基准测试..."
echo ""

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查是否传入参数
if [ $# -eq 0 ]; then
    echo -e "${BLUE}运行所有基准测试...${NC}"
    cargo bench
else
    case "$1" in
        "arithmetic")
            echo -e "${BLUE}运行算术运算基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- arithmetic
            ;;
        "variables")
            echo -e "${BLUE}运行变量操作基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- variables
            ;;
        "functions")
            echo -e "${BLUE}运行函数调用基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- functions
            ;;
        "control")
            echo -e "${BLUE}运行控制流基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- control_flow
            ;;
        "arrays")
            echo -e "${BLUE}运行数组操作基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- arrays
            ;;
        "dicts")
            echo -e "${BLUE}运行字典操作基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- dictionaries
            ;;
        "strings")
            echo -e "${BLUE}运行字符串操作基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- strings
            ;;
        "precision")
            echo -e "${BLUE}运行精确数学基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- precision
            ;;
        "parsing")
            echo -e "${BLUE}运行解析性能基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- parsing
            ;;
        "sizes")
            echo -e "${BLUE}运行程序规模基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- program_sizes
            ;;
        "fib")
            echo -e "${BLUE}运行斐波那契递归基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- fibonacci
            ;;
        "payroll")
            echo -e "${BLUE}运行工资计算基准测试...${NC}"
            cargo bench --bench aether_benchmarks -- payroll
            ;;
        "quick")
            echo -e "${YELLOW}快速模式：减少样本数量...${NC}"
            cargo bench -- --sample-size 10
            ;;
        "save")
            if [ -z "$2" ]; then
                echo -e "${YELLOW}请提供基线名称: ./scripts/bench.sh save <baseline_name>${NC}"
                exit 1
            fi
            echo -e "${BLUE}保存基准测试结果到基线: $2${NC}"
            cargo bench -- --save-baseline "$2"
            ;;
        "compare")
            if [ -z "$2" ]; then
                echo -e "${YELLOW}请提供基线名称: ./scripts/bench.sh compare <baseline_name>${NC}"
                exit 1
            fi
            echo -e "${BLUE}与基线比较: $2${NC}"
            cargo bench -- --baseline "$2"
            ;;
        "report")
            echo -e "${BLUE}打开基准测试报告...${NC}"
            if [ -f "target/criterion/report/index.html" ]; then
                open "target/criterion/report/index.html" || xdg-open "target/criterion/report/index.html" 2>/dev/null || echo "请手动打开: target/criterion/report/index.html"
            else
                echo -e "${YELLOW}报告不存在，请先运行基准测试${NC}"
            fi
            ;;
        "clean")
            echo -e "${BLUE}清理基准测试结果...${NC}"
            rm -rf target/criterion
            echo -e "${GREEN}✅ 清理完成${NC}"
            ;;
        "help"|"-h"|"--help")
            echo "用法: $0 [选项]"
            echo ""
            echo "选项:"
            echo "  (无参数)        运行所有基准测试"
            echo "  arithmetic      算术运算测试"
            echo "  variables       变量操作测试"
            echo "  functions       函数调用测试"
            echo "  control         控制流测试"
            echo "  arrays          数组操作测试"
            echo "  dicts           字典操作测试"
            echo "  strings         字符串操作测试"
            echo "  precision       精确数学测试"
            echo "  parsing         解析性能测试"
            echo "  sizes           程序规模测试"
            echo "  fib             斐波那契递归测试"
            echo "  payroll         工资计算测试"
            echo "  quick           快速模式（减少样本）"
            echo "  save <name>     保存基准测试到基线"
            echo "  compare <name>  与指定基线比较"
            echo "  report          打开HTML报告"
            echo "  clean           清理测试结果"
            echo "  help            显示此帮助信息"
            echo ""
            echo "示例:"
            echo "  $0                    # 运行所有测试"
            echo "  $0 arithmetic         # 只运行算术测试"
            echo "  $0 save before        # 保存基线"
            echo "  $0 compare before     # 与基线比较"
            echo "  $0 report             # 查看报告"
            exit 0
            ;;
        *)
            echo -e "${YELLOW}未知选项: $1${NC}"
            echo "使用 '$0 help' 查看帮助信息"
            exit 1
            ;;
    esac
fi

echo ""
echo -e "${GREEN}✅ 基准测试完成！${NC}"
echo -e "${BLUE}📊 查看详细报告: target/criterion/report/index.html${NC}"
