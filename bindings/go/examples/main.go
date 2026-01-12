package main

import (
	"encoding/json"
	"fmt"
	"log"
	"sync"
	"time"

	aether "github.com/xiaozuhui/aether/bindings/go"
)

func main() {
	fmt.Println("=== Aether DSL - Go 绑定完整演示 ===")

	// 1. 基本用法
	basicExample()

	// 2. 变量操作
	variableExample()

	// 3. 追踪与调试
	traceExample()

	// 4. 执行限制
	limitsExample()

	// 5. 缓存控制
	cacheExample()

	// 6. 线程安全
	concurrentExample()

	// 7. 复杂示例
	complexExample()
}

// basicExample 演示基本用法
func basicExample() {
	fmt.Println("1. 基本用法")
	fmt.Println("---")

	engine := aether.New()
	defer engine.Close()

	result, err := engine.Eval(`
		Set X 10
		Set Y 20
		(X + Y)
	`)
	if err != nil {
		log.Fatalf("Eval 失败: %v", err)
	}

	fmt.Printf("结果: %s\n\n", result)
}

// variableExample 演示变量操作
func variableExample() {
	fmt.Println("2. 变量操作")
	fmt.Println("---")

	engine := aether.New()
	defer engine.Close()

	// 设置简单值
	err := engine.SetGlobal("greeting", "你好")
	if err != nil {
		log.Fatalf("SetGlobal 失败: %v", err)
	}

	// 设置复杂值
	config := map[string]interface{}{
		"host":      "localhost",
		"port":      8080,
		"debug":     true,
		"endpoints": []string{"/api", "/health"},
	}
	err = engine.SetGlobal("config", config)
	if err != nil {
		log.Fatalf("SetGlobal config 失败: %v", err)
	}

	// 在 Aether 代码中使用
	result, err := engine.Eval(`
		Set MSG (greeting + " 来自 Aether!")
		MSG
	`)
	if err != nil {
		log.Fatalf("Eval 失败: %v", err)
	}
	fmt.Printf("结果: %s\n", result)

	// 获取变量
	val, err := engine.GetGlobal("greeting")
	if err != nil {
		log.Fatalf("GetGlobal 失败: %v", err)
	}
	fmt.Printf("获取的值: %v\n\n", val)
}

// traceExample 演示追踪与调试
func traceExample() {
	fmt.Println("3. 追踪与调试")
	fmt.Println("---")

	engine := aether.New()
	defer engine.Close()

	// 执行带 TRACE 的代码
	code := `
		TRACE_DEBUG("api", "请求开始")
		Set USER_ID 42
		TRACE_INFO("auth", "用户认证: ", USER_ID)
		Set RESULT (USER_ID * 2)
		TRACE_WARN("calc", "结果翻倍: ", RESULT)
	`

	_, err := engine.Eval(code)
	if err != nil {
		log.Fatalf("Eval 失败: %v", err)
	}

	// 获取简单追踪
	traces, err := engine.TakeTrace()
	if err != nil {
		log.Fatalf("TakeTrace 失败: %v", err)
	}
	fmt.Println("简单追踪:")
	for _, trace := range traces {
		fmt.Printf("  - %s\n", trace)
	}

	// 获取结构化追踪
	records, err := engine.TraceRecords()
	if err != nil {
		log.Fatalf("TraceRecords 失败: %v", err)
	}
	fmt.Println("\n结构化追踪:")
	for _, record := range records {
		fmt.Printf("  [%s] %s: %v\n", record.Level, record.Category, record.Values)
	}

	// 获取统计
	stats, err := engine.TraceStats()
	if err != nil {
		log.Fatalf("TraceStats 失败: %v", err)
	}
	fmt.Printf("\n追踪统计: %+v\n\n", stats)
}

// limitsExample 演示执行限制
func limitsExample() {
	fmt.Println("4. 执行限制")
	fmt.Println("---")

	engine := aether.New()
	defer engine.Close()

	// 设置限制
	limits := aether.Limits{
		MaxSteps:          1000,
		MaxRecursionDepth: 50,
		MaxDurationMs:     1000,
	}

	err := engine.SetExecutionLimits(limits)
	if err != nil {
		log.Fatalf("SetExecutionLimits 失败: %v", err)
	}

	// 获取限制
	retrieved, err := engine.GetExecutionLimits()
	if err != nil {
		log.Fatalf("GetExecutionLimits 失败: %v", err)
	}

	fmt.Printf("限制设置: MaxSteps=%d, MaxDepth=%d, MaxDuration=%dms\n\n",
		retrieved.MaxSteps, retrieved.MaxRecursionDepth, retrieved.MaxDurationMs)
}

// cacheExample 演示缓存控制
func cacheExample() {
	fmt.Println("5. 缓存控制")
	fmt.Println("---")

	engine := aether.New()
	defer engine.Close()

	code := "Set X 10\nSet Y 20\n(X + Y)"

	// 多次执行
	for i := 0; i < 5; i++ {
		_, err := engine.Eval(code)
		if err != nil {
			log.Fatalf("Eval 失败: %v", err)
		}
	}

	// 检查缓存统计
	stats, err := engine.CacheStats()
	if err != nil {
		log.Fatalf("CacheStats 失败: %v", err)
	}
	fmt.Printf("清除前: 命中=%d, 未命中=%d, 大小=%d\n",
		stats.Hits, stats.Misses, stats.Size)

	// 清除缓存
	engine.ClearCache()

	// 再次检查
	stats, err = engine.CacheStats()
	if err != nil {
		log.Fatalf("清除后 CacheStats 失败: %v", err)
	}
	fmt.Printf("清除后: 命中=%d, 未命中=%d, 大小=%d\n\n",
		stats.Hits, stats.Misses, stats.Size)
}

// concurrentExample 演示线程安全
func concurrentExample() {
	fmt.Println("6. 线程安全")
	fmt.Println("---")

	engine := aether.New()
	defer engine.Close()

	const numGoroutines = 10
	const iterations = 100

	var wg sync.WaitGroup
	start := time.Now()

	for i := 0; i < numGoroutines; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < iterations; j++ {
				code := fmt.Sprintf("Set X %d\n(X * 2)", id)
				result, err := engine.Eval(code)
				if err != nil {
					log.Printf("Goroutine %d 错误: %v", id, err)
					return
				}
				_ = result // 使用结果
			}
		}(i)
	}

	wg.Wait()
	elapsed := time.Since(start)

	fmt.Printf("完成 %d 次操作,耗时 %v\n",
		numGoroutines*iterations, elapsed)
	fmt.Printf("平均: %v 每次操作\n\n",
		elapsed/time.Duration(numGoroutines*iterations))
}

// complexExample 演示复杂示例
func complexExample() {
	fmt.Println("7. 复杂示例")
	fmt.Println("---")

	engine := aether.New()
	defer engine.Close()

	// 设置配置
	config := map[string]interface{}{
		"users": []map[string]interface{}{
			{"id": 1, "name": "张三", "score": 100},
			{"id": 2, "name": "李四", "score": 85},
			{"id": 3, "name": "王五", "score": 95},
		},
		"threshold": 90,
	}
	engine.SetGlobal("config", config)

	// 执行复杂逻辑
	code := `
		Func FILTER_HIGH_SCORE (USERS, THRESHOLD) {
			Set RESULT []
			For USER In USERS {
				If ((USER["score"] > THRESHOLD)) {
					Set RESULT [RESULT, USER]
				}
			}
			Return RESULT
		}

		Set HIGH_SCORERS (FILTER_HIGH_SCORE(config["users"], config["threshold"]))
		LENGTH(HIGH_SCORERS)
	`

	result, err := engine.Eval(code)
	if err != nil {
		log.Fatalf("复杂 Eval 失败: %v", err)
	}

	fmt.Printf("高分人数: %s\n", result)

	// 获取追踪
	traces, _ := engine.TakeTrace()
	if len(traces) > 0 {
		fmt.Println("\n执行追踪:")
		for _, trace := range traces {
			fmt.Printf("  %s\n", trace)
		}
	}

	// 获取缓存统计
	stats, _ := engine.CacheStats()
	statsJSON, _ := json.MarshalIndent(stats, "", "  ")
	fmt.Printf("\n缓存统计:\n%s\n", string(statsJSON))
}
