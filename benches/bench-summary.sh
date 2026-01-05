#!/bin/bash
# 从基准测试输出中提取性能摘要

if [ -d "target/criterion" ]; then
    echo "📊 Aether 基准测试性能摘要"
    echo "================================"
    echo ""
    
    # 查找所有的 benchmark 结果
    for dir in target/criterion/*/; do
        if [ -f "${dir}base/estimates.json" ] || [ -f "${dir}new/estimates.json" ]; then
            bench_name=$(basename "$dir")
            echo "📌 $bench_name"
            
            # 尝试读取 estimates.json 中的平均时间
            if [ -f "${dir}new/estimates.json" ]; then
                estimates_file="${dir}new/estimates.json"
            elif [ -f "${dir}base/estimates.json" ]; then
                estimates_file="${dir}base/estimates.json"
            fi
            
            if [ -n "$estimates_file" ] && command -v jq &> /dev/null; then
                mean=$(jq -r '.mean.point_estimate' "$estimates_file" 2>/dev/null)
                if [ -n "$mean" ] && [ "$mean" != "null" ]; then
                    # 转换为合适的单位
                    if (( $(echo "$mean < 1000" | bc -l) )); then
                        printf "   平均时间: %.2f ns\n" "$mean"
                    elif (( $(echo "$mean < 1000000" | bc -l) )); then
                        printf "   平均时间: %.2f µs\n" "$(echo "$mean / 1000" | bc -l)"
                    else
                        printf "   平均时间: %.2f ms\n" "$(echo "$mean / 1000000" | bc -l)"
                    fi
                fi
            fi
            echo ""
        fi
    done
else
    echo "❌ 未找到基准测试结果"
    echo "请先运行: cargo bench"
fi
