// Package aether 为 Aether DSL 语言提供 Go 语言绑定
//
// Aether 是一个轻量级、可嵌入的领域特定语言,专为配置管理、
// 业务规则和脚本编写而设计。
//
// 基本用法:
//
//	engine := aether.New()
//	defer engine.Close()
//
//	result, err := engine.Eval(`
//	    Set X 10
//	    Set Y 20
//	    (X + Y)
//	`)
//	if err != nil {
//	    log.Fatal(err)
//	}
//	fmt.Println(result) // "30"
//
// # 线程安全
//
// Engine 完全线程安全,可以在多个 goroutine 中安全地并发使用。
// 所有公共方法都由内部 mutex 保护。
//
// # 高级特性
//
//	// 从 Go 设置变量
//	engine.SetGlobal("name", "Alice")
//
//	// 获取执行追踪
//	engine.Eval(`TRACE_DEBUG("api", "Request received")`)
//	traces, _ := engine.TakeTrace()
//
//	// 设置执行限制
//	engine.SetExecutionLimits(aether.Limits{
//	    MaxSteps: 10000,
//	    MaxRecursionDepth: 100,
//	})
//
//	// 控制缓存
//	stats := engine.CacheStats()
//	fmt.Printf("Cache hits: %d\n", stats.Hits)
package aether

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -laether -ldl -lm -lpthread
#cgo darwin LDFLAGS: -framework Security -framework CoreFoundation
#include <stdlib.h>

typedef struct AetherHandle AetherHandle;

typedef enum AetherErrorCode {
    Success = 0,
    ParseError = 1,
    RuntimeError = 2,
    NullPointer = 3,
    Panic = 4,
    InvalidJSON = 5,
    VariableNotFound = 6,
} AetherErrorCode;

typedef struct AetherLimits {
    int max_steps;
    int max_recursion_depth;
    int max_duration_ms;
} AetherLimits;

typedef struct AetherCacheStats {
    int hits;
    int misses;
    int size;
} AetherCacheStats;

AetherHandle* aether_new();
AetherHandle* aether_new_with_permissions();
int aether_eval(AetherHandle* handle, const char* code, char** result, char** error);
const char* aether_version();
void aether_free(AetherHandle* handle);
void aether_free_string(char* s);

// 增强的 API
int aether_set_global(AetherHandle* handle, const char* name, const char* value_json);
int aether_get_global(AetherHandle* handle, const char* name, char** value_json);
void aether_reset_env(AetherHandle* handle);

int aether_take_trace(AetherHandle* handle, char** trace_json);
void aether_clear_trace(AetherHandle* handle);
int aether_trace_records(AetherHandle* handle, char** trace_json);
int aether_trace_stats(AetherHandle* handle, char** stats_json);

void aether_set_limits(AetherHandle* handle, const AetherLimits* limits);
void aether_get_limits(AetherHandle* handle, AetherLimits* limits);

void aether_clear_cache(AetherHandle* handle);
void aether_cache_stats(AetherHandle* handle, AetherCacheStats* stats);

void aether_set_optimization(AetherHandle* handle, int constant_folding, int dead_code, int tail_recursion);
*/
import "C"
import (
	"encoding/json"
	"errors"
	"fmt"
	"runtime"
	"sync"
	"unsafe"
)

// Engine 表示一个线程安全的 Aether DSL 引擎
type Engine struct {
	handle *C.AetherHandle
	mu     sync.RWMutex
}

// Limits 控制执行约束
type Limits struct {
	// 最大执行步数 (-1 表示无限制)
	MaxSteps int
	// 最大递归深度 (-1 表示无限制)
	MaxRecursionDepth int
	// 最大执行时间(毫秒, -1 表示无限制)
	MaxDurationMs int
}

// CacheStats 表示缓存统计信息
type CacheStats struct {
	Hits   int
	Misses int
	Size   int
}

// TraceStats 表示追踪统计信息
type TraceStats struct {
	TotalEntries int            `json:"total_entries"`
	ByLevel      map[string]int `json:"by_level"`
	ByCategory   map[string]int `json:"by_category"`
	BufferSize   int            `json:"buffer_size"`
	BufferFull   bool           `json:"buffer_full"`
}

// TraceEntry 表示单个追踪条目
type TraceEntry struct {
	Level     string   `json:"level"`
	Category  string   `json:"category"`
	Timestamp int64    `json:"timestamp"`
	Values    []string `json:"values"`
	Label     *string  `json:"label,omitempty"`
}

// New 创建一个新的 Aether 引擎实例,默认禁用 IO 权限
//
// 出于安全考虑,作为嵌入式 DSL 使用时,IO 操作默认是禁用的。
// 如需启用 IO 操作,请使用 NewWithPermissions()。
func New() *Engine {
	e := &Engine{
		handle: C.aether_new(),
	}
	runtime.SetFinalizer(e, (*Engine).Close)
	return e
}

// NewWithPermissions 创建一个启用所有 IO 权限的 Aether 引擎
//
// 警告: 仅当你信任要执行的脚本时才使用,因为它允许文件系统和网络操作。
func NewWithPermissions() *Engine {
	e := &Engine{
		handle: C.aether_new_with_permissions(),
	}
	runtime.SetFinalizer(e, (*Engine).Close)
	return e
}

// Eval 执行给定的 Aether 代码并返回结果字符串
//
// 此方法是线程安全的,可以从多个 goroutine 并发调用
//
// 如果代码解析失败或遇到运行时错误,则返回错误
func (e *Engine) Eval(code string) (string, error) {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle == nil {
		return "", errors.New("aether: 引擎已关闭")
	}

	cCode := C.CString(code)
	defer C.free(unsafe.Pointer(cCode))

	var result *C.char
	var errorMsg *C.char

	status := C.aether_eval(e.handle, cCode, &result, &errorMsg)

	if status != C.Success {
		if errorMsg != nil {
			defer C.aether_free_string(errorMsg)
			errStr := C.GoString(errorMsg)
			return "", fmt.Errorf("aether: %s", errStr)
		}
		return "", errors.New("aether: 未知错误")
	}

	if result != nil {
		defer C.aether_free_string(result)
		return C.GoString(result), nil
	}

	return "", nil
}

// SetGlobal 从 Go 代码设置全局变量
//
// 值会序列化为 JSON 后传递给 Aether 引擎
// 此方法是线程安全的
func (e *Engine) SetGlobal(name string, value interface{}) error {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle == nil {
		return errors.New("aether: 引擎已关闭")
	}

	jsonData, err := json.Marshal(value)
	if err != nil {
		return fmt.Errorf("无法将值序列化为 JSON: %w", err)
	}

	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))

	cValue := C.CString(string(jsonData))
	defer C.free(unsafe.Pointer(cValue))

	status := C.aether_set_global(e.handle, cName, cValue)
	if status != C.Success {
		return fmt.Errorf("设置全局变量 '%s' 失败 (错误代码: %d)", name, status)
	}

	return nil
}

// GetGlobal 获取变量的值
//
// 值从 JSON 反序列化。使用类型断言获取底层类型
// 此方法是线程安全的
func (e *Engine) GetGlobal(name string) (interface{}, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	if e.handle == nil {
		return nil, errors.New("aether: 引擎已关闭")
	}

	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))

	var valueJSON *C.char
	status := C.aether_get_global(e.handle, cName, &valueJSON)
	if status != C.Success {
		return nil, fmt.Errorf("变量未找到: %s (错误代码: %d)", name, status)
	}
	defer C.aether_free_string(valueJSON)

	var result interface{}
	err := json.Unmarshal([]byte(C.GoString(valueJSON)), &result)
	if err != nil {
		return nil, fmt.Errorf("解析变量值失败: %w", err)
	}

	return result, nil
}

// ResetEnv 重置运行时环境(清除所有变量)
//
// 此方法是线程安全的
func (e *Engine) ResetEnv() error {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle == nil {
		return errors.New("aether: 引擎已关闭")
	}

	C.aether_reset_env(e.handle)
	return nil
}

// TakeTrace 返回所有追踪条目
//
// 此方法是线程安全的
func (e *Engine) TakeTrace() ([]string, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	if e.handle == nil {
		return nil, errors.New("aether: 引擎已关闭")
	}

	var traceJSON *C.char
	status := C.aether_take_trace(e.handle, &traceJSON)
	if status != C.Success {
		return nil, fmt.Errorf("获取追踪失败 (错误代码: %d)", status)
	}
	defer C.aether_free_string(traceJSON)

	var traces []string
	err := json.Unmarshal([]byte(C.GoString(traceJSON)), &traces)
	if err != nil {
		return nil, fmt.Errorf("解析追踪 JSON 失败: %w", err)
	}

	return traces, nil
}

// TraceRecords 返回结构化的追踪条目
//
// 此方法是线程安全的
func (e *Engine) TraceRecords() ([]TraceEntry, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	if e.handle == nil {
		return nil, errors.New("aether: 引擎已关闭")
	}

	var traceJSON *C.char
	status := C.aether_trace_records(e.handle, &traceJSON)
	if status != C.Success {
		return nil, fmt.Errorf("获取追踪记录失败 (错误代码: %d)", status)
	}
	defer C.aether_free_string(traceJSON)

	var entries []TraceEntry
	err := json.Unmarshal([]byte(C.GoString(traceJSON)), &entries)
	if err != nil {
		return nil, fmt.Errorf("解析追踪记录 JSON 失败: %w", err)
	}

	return entries, nil
}

// TraceStats 返回追踪统计信息
//
// 此方法是线程安全的
func (e *Engine) TraceStats() (*TraceStats, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	if e.handle == nil {
		return nil, errors.New("aether: 引擎已关闭")
	}

	var statsJSON *C.char
	status := C.aether_trace_stats(e.handle, &statsJSON)
	if status != C.Success {
		return nil, fmt.Errorf("获取追踪统计失败 (错误代码: %d)", status)
	}
	defer C.aether_free_string(statsJSON)

	var stats TraceStats
	err := json.Unmarshal([]byte(C.GoString(statsJSON)), &stats)
	if err != nil {
		return nil, fmt.Errorf("解析追踪统计 JSON 失败: %w", err)
	}

	return &stats, nil
}

// ClearTrace 清除追踪缓冲区
//
// 此方法是线程安全的
func (e *Engine) ClearTrace() error {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle == nil {
		return errors.New("aether: 引擎已关闭")
	}

	C.aether_clear_trace(e.handle)
	return nil
}

// SetExecutionLimits 设置执行限制
//
// 此方法是线程安全的
func (e *Engine) SetExecutionLimits(limits Limits) error {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle == nil {
		return errors.New("aether: 引擎已关闭")
	}

	cLimits := C.AetherLimits{
		max_steps:           C.int(limits.MaxSteps),
		max_recursion_depth: C.int(limits.MaxRecursionDepth),
		max_duration_ms:      C.int(limits.MaxDurationMs),
	}

	C.aether_set_limits(e.handle, &cLimits)
	return nil
}

// GetExecutionLimits 获取当前执行限制
//
// 此方法是线程安全的
func (e *Engine) GetExecutionLimits() (*Limits, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	if e.handle == nil {
		return nil, errors.New("aether: 引擎已关闭")
	}

	var cLimits C.AetherLimits
	C.aether_get_limits(e.handle, &cLimits)

	return &Limits{
		MaxSteps:          int(cLimits.max_steps),
		MaxRecursionDepth: int(cLimits.max_recursion_depth),
		MaxDurationMs:     int(cLimits.max_duration_ms),
	}, nil
}

// ClearCache 清除 AST 缓存
//
// 此方法是线程安全的
func (e *Engine) ClearCache() error {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle == nil {
		return errors.New("aether: 引擎已关闭")
	}

	C.aether_clear_cache(e.handle)
	return nil
}

// CacheStats 返回缓存统计信息
//
// 此方法是线程安全的
func (e *Engine) CacheStats() (*CacheStats, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	if e.handle == nil {
		return nil, errors.New("aether: 引擎已关闭")
	}

	var cStats C.AetherCacheStats
	C.aether_cache_stats(e.handle, &cStats)

	return &CacheStats{
		Hits:   int(cStats.hits),
		Misses: int(cStats.misses),
		Size:   int(cStats.size),
	}, nil
}

// SetOptimization 设置优化选项
//
// 此方法是线程安全的
func (e *Engine) SetOptimization(constantFolding, deadCodeElimination, tailRecursion bool) error {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle == nil {
		return errors.New("aether: 引擎已关闭")
	}

	cf := 0
	if constantFolding {
		cf = 1
	}
	dc := 0
	if deadCodeElimination {
		dc = 1
	}
	tr := 0
	if tailRecursion {
		tr = 1
	}

	C.aether_set_optimization(e.handle, C.int(cf), C.int(dc), C.int(tr))
	return nil
}

// Version 返回 Aether 引擎的版本字符串
func Version() string {
	return C.GoString(C.aether_version())
}

// Close 释放与 Aether 引擎关联的资源
//
// 调用 Close() 后,引擎将无法再使用
// 可以安全地多次调用 Close()
func (e *Engine) Close() {
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.handle != nil {
		C.aether_free(e.handle)
		e.handle = nil
	}
}

// 为了向后兼容,保留 Aether 类型别名
type Aether = Engine

// NewLegacy 是旧版本的创建函数,保留用于向后兼容
// Deprecated: 使用 New() 代替
func NewLegacy() *Aether {
	return New()
}
