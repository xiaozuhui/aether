package aether

import (
	"sync"
	"testing"
)

// TestNew 测试引擎创建
func TestNew(t *testing.T) {
	engine := New()
	if engine == nil {
		t.Fatal("New() 返回 nil")
	}
	defer engine.Close()
}

// TestVersion 测试版本获取
func TestVersion(t *testing.T) {
	version := Version()
	if version == "" {
		t.Error("Version() 返回空字符串")
	}
	t.Logf("Aether 版本: %s", version)
}

// TestSetGlobal 测试设置全局变量
func TestSetGlobal(t *testing.T) {
	engine := New()
	defer engine.Close()

	err := engine.SetGlobal("x", 42)
	if err != nil {
		t.Fatalf("SetGlobal 失败: %v", err)
	}

	val, err := engine.GetGlobal("x")
	if err != nil {
		t.Fatalf("GetGlobal 失败: %v", err)
	}

	// JSON 数字会被反序列化为 float64
	if f, ok := val.(float64); !ok || f != 42.0 {
		t.Errorf("期望 42, 得到 %v (类型: %T)", val, val)
	}
}

// TestSetGlobalComplex 测试设置复杂类型
func TestSetGlobalComplex(t *testing.T) {
	engine := New()
	defer engine.Close()

	// 测试数组
	err := engine.SetGlobal("arr", []int{1, 2, 3})
	if err != nil {
		t.Fatalf("SetGlobal 数组失败: %v", err)
	}

	val, err := engine.GetGlobal("arr")
	if err != nil {
		t.Fatalf("GetGlobal 数组失败: %v", err)
	}
	t.Logf("数组值: %v", val)

	// 测试对象
	err = engine.SetGlobal("obj", map[string]interface{}{"name": "Alice", "age": 30})
	if err != nil {
		t.Fatalf("SetGlobal 对象失败: %v", err)
	}

	val, err = engine.GetGlobal("obj")
	if err != nil {
		t.Fatalf("GetGlobal 对象失败: %v", err)
	}
	t.Logf("对象值: %v", val)
}

// TestResetEnv 测试环境重置
func TestResetEnv(t *testing.T) {
	engine := New()
	defer engine.Close()

	// 设置一个变量
	_, err := engine.Eval("Set X 42")
	if err != nil {
		t.Fatalf("Eval 失败: %v", err)
	}

	// 验证变量存在
	result, err := engine.Eval("X")
	if err != nil || result != "42" {
		t.Fatalf("变量设置不正确: %v, %s", err, result)
	}

	// 重置环境
	err = engine.ResetEnv()
	if err != nil {
		t.Fatalf("ResetEnv 失败: %v", err)
	}

	// 验证变量已清除
	_, err = engine.Eval("X")
	if err == nil {
		t.Error("ResetEnv 后期望错误,得到 nil")
	}
}

// TestTraceOperations 测试追踪操作
func TestTraceOperations(t *testing.T) {
	engine := New()
	defer engine.Close()

	// 执行带 TRACE 的代码
	_, err := engine.Eval(`TRACE("test", "Hello, World!")`)
	if err != nil {
		t.Fatalf("带 TRACE 的 Eval 失败: %v", err)
	}

	// 获取追踪
	traces, err := engine.TakeTrace()
	if err != nil {
		t.Fatalf("TakeTrace 失败: %v", err)
	}

	if len(traces) == 0 {
		t.Error("期望至少一个追踪条目")
	} else {
		t.Logf("追踪: %v", traces)
	}

	// 清除追踪
	err = engine.ClearTrace()
	if err != nil {
		t.Fatalf("ClearTrace 失败: %v", err)
	}

	// 验证已清除
	traces, err = engine.TakeTrace()
	if err != nil {
		t.Fatalf("ClearTrace 后 TakeTrace 失败: %v", err)
	}

	if len(traces) != 0 {
		t.Errorf("ClearTrace 后期望 0 个追踪,得到 %d", len(traces))
	}
}

// TestExecutionLimits 测试执行限制
func TestExecutionLimits(t *testing.T) {
	engine := New()
	defer engine.Close()

	// 设置限制
	limits := Limits{
		MaxSteps:          1000,
		MaxRecursionDepth: 50,
		MaxDurationMs:     5000,
	}

	err := engine.SetExecutionLimits(limits)
	if err != nil {
		t.Fatalf("SetExecutionLimits 失败: %v", err)
	}

	// 获取限制
	retrieved, err := engine.GetExecutionLimits()
	if err != nil {
		t.Fatalf("GetExecutionLimits 失败: %v", err)
	}

	if retrieved.MaxSteps != limits.MaxSteps ||
		retrieved.MaxRecursionDepth != limits.MaxRecursionDepth ||
		retrieved.MaxDurationMs != limits.MaxDurationMs {
		t.Errorf("限制不匹配: 得到 %+v,期望 %+v", retrieved, limits)
	}
}

// TestCacheStats 测试缓存统计
func TestCacheStats(t *testing.T) {
	engine := New()
	defer engine.Close()

	// 执行代码两次(第二次应该命中缓存)
	code := "Set X 10\n(X + 20)"
	_, err := engine.Eval(code)
	if err != nil {
		t.Fatalf("第一次 Eval 失败: %v", err)
	}

	_, err = engine.Eval(code)
	if err != nil {
		t.Fatalf("第二次 Eval 失败: %v", err)
	}

	// 获取统计
	stats, err := engine.CacheStats()
	if err != nil {
		t.Fatalf("CacheStats 失败: %v", err)
	}

	t.Logf("缓存统计: 命中=%d, 未命中=%d, 大小=%d",
		stats.Hits, stats.Misses, stats.Size)

	if stats.Hits < 1 {
		t.Error("期望至少 1 次缓存命中")
	}

	// 清除缓存
	err = engine.ClearCache()
	if err != nil {
		t.Fatalf("ClearCache 失败: %v", err)
	}

	// 验证已清除
	stats, err = engine.CacheStats()
	if err != nil {
		t.Fatalf("ClearCache 后 CacheStats 失败: %v", err)
	}

	if stats.Size != 0 {
		t.Errorf("ClearCache 后期望缓存大小 0,得到 %d", stats.Size)
	}
}

// TestSetOptimization 测试优化设置
func TestSetOptimization(t *testing.T) {
	engine := New()
	defer engine.Close()

	err := engine.SetOptimization(true, true, false)
	if err != nil {
		t.Fatalf("SetOptimization 失败: %v", err)
	}

	// 仅验证不崩溃
	_, err = engine.Eval("Set X 10\n(X + 20)")
	if err != nil {
		t.Fatalf("SetOptimization 后 Eval 失败: %v", err)
	}
}

// TestThreadSafety 测试并发执行
func TestThreadSafety(t *testing.T) {
	engine := New()
	defer engine.Close()

	const numGoroutines = 10
	const iterations = 100

	var wg sync.WaitGroup
	for i := 0; i < numGoroutines; i++ {
		wg.Add(1)
		go func(n int) {
			defer wg.Done()
			for j := 0; j < iterations; j++ {
				code := "Set X 10\n(X + 20)"
				result, err := engine.Eval(code)
				if err != nil {
					t.Errorf("并发执行错误: %v", err)
					return
				}
				if result != "30" {
					t.Errorf("期望 30,得到 %s", result)
					return
				}
			}
		}(i)
	}
	wg.Wait()
}

// TestEvalAfterClose 测试 Close 后的行为
func TestEvalAfterClose(t *testing.T) {
	engine := New()
	engine.Close()

	_, err := engine.Eval("Set X 10")
	if err == nil {
		t.Error("Close 后 Eval 期望错误,得到 nil")
	}
}

// TestCloseMultipleTimes 测试 Close 是幂等的
func TestCloseMultipleTimes(t *testing.T) {
	engine := New()
	engine.Close()
	engine.Close() // 不应 panic
}

// BenchmarkBasicEval 基准测试:基本执行
func BenchmarkBasicEval(b *testing.B) {
	engine := New()
	defer engine.Close()

	code := `
		Func ADD (A, B) {
			Return (A + B)
		}
		ADD(15, 15)
	`

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := engine.Eval(code)
		if err != nil {
			b.Fatal(err)
		}
	}
}

// BenchmarkConcurrentEval 基准测试:并发执行
func BenchmarkConcurrentEval(b *testing.B) {
	engine := New()
	defer engine.Close()

	code := "Set X 10\n(X + 20)"

	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			_, err := engine.Eval(code)
			if err != nil {
				b.Fatal(err)
			}
		}
	})
}

// BenchmarkSetGetGlobal 基准测试:变量操作
func BenchmarkSetGetGlobal(b *testing.B) {
	engine := New()
	defer engine.Close()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		err := engine.SetGlobal("x", 42)
		if err != nil {
			b.Fatal(err)
		}
		_, err = engine.GetGlobal("x")
		if err != nil {
			b.Fatal(err)
		}
	}
}
