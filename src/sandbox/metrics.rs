//! 可观测性指标收集
//!
//! 收集运行时指标，支持监控和调试。

use crate::cache::CacheStats;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// 执行指标
#[derive(Debug, Clone)]
pub struct ExecutionMetrics {
    /// 执行次数
    pub execution_count: usize,
    /// 总执行时间
    pub total_duration: Duration,
    /// 平均执行时间
    pub average_duration: Duration,
    /// 最小执行时间
    pub min_duration: Duration,
    /// 最大执行时间
    pub max_duration: Duration,
}

impl Default for ExecutionMetrics {
    fn default() -> Self {
        Self {
            execution_count: 0,
            total_duration: Duration::ZERO,
            average_duration: Duration::ZERO,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
        }
    }
}

/// 模块加载指标
#[derive(Debug, Clone)]
pub struct ModuleMetrics {
    /// 模块加载次数
    pub load_count: usize,
    /// 缓存命中次数
    pub cache_hits: usize,
    /// 缓存未命中次数
    pub cache_misses: usize,
    /// 缓存命中率
    pub hit_rate: f64,
}

impl Default for ModuleMetrics {
    fn default() -> Self {
        Self {
            load_count: 0,
            cache_hits: 0,
            cache_misses: 0,
            hit_rate: 0.0,
        }
    }
}

/// 综合指标快照
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    /// 执行指标
    pub execution: ExecutionMetrics,
    /// 模块指标
    pub modules: ModuleMetrics,
    /// TRACE 缓冲区条目数
    pub trace_entries: usize,
    /// 模块缓存大小
    pub module_cache_size: usize,
    /// AST 缓存统计
    pub ast_cache: CacheStats,
}

/// 指标收集器
pub struct MetricsCollector {
    /// 是否启用
    enabled: RwLock<bool>,
    /// 执行开始时间（当前执行）
    execution_start: RwLock<Option<Instant>>,
    /// 执行指标
    execution: RwLock<ExecutionMetrics>,
    /// 模块指标
    modules: RwLock<ModuleMetrics>,
    /// 各模块的加载次数
    module_loads: RwLock<std::collections::HashMap<String, usize>>,
}

impl MetricsCollector {
    /// 创建新的指标收集器
    pub fn new() -> Self {
        Self {
            enabled: RwLock::new(false),
            execution_start: RwLock::new(None),
            execution: RwLock::new(ExecutionMetrics::default()),
            modules: RwLock::new(ModuleMetrics::default()),
            module_loads: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// 启用指标收集
    pub fn enable(&self) {
        *self.enabled.write().unwrap() = true;
    }

    /// 禁用指标收集
    pub fn disable(&self) {
        *self.enabled.write().unwrap() = false;
    }

    /// 是否启用
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read().unwrap()
    }

    /// 记录执行开始
    pub fn record_execution_start(&self) {
        if !self.is_enabled() {
            return;
        }
        *self.execution_start.write().unwrap() = Some(Instant::now());
    }

    /// 记录执行结束
    pub fn record_execution_end(&self) {
        if !self.is_enabled() {
            return;
        }

        let start = self.execution_start.write().unwrap().take();
        if let Some(start_time) = start {
            let duration = start_time.elapsed();

            let mut exec = self.execution.write().unwrap();
            exec.execution_count += 1;
            exec.total_duration += duration;
            exec.average_duration = exec.total_duration / exec.execution_count as u32;
            exec.min_duration = exec.min_duration.min(duration);
            exec.max_duration = exec.max_duration.max(duration);
        }
    }

    /// 记录模块加载
    pub fn record_module_load(&self, module_id: &str, cached: bool) {
        if !self.is_enabled() {
            return;
        }

        let mut modules = self.modules.write().unwrap();
        modules.load_count += 1;
        if cached {
            modules.cache_hits += 1;
        } else {
            modules.cache_misses += 1;
        }
        if modules.load_count > 0 {
            modules.hit_rate = modules.cache_hits as f64 / modules.load_count as f64;
        }

        let mut loads = self.module_loads.write().unwrap();
        *loads.entry(module_id.to_string()).or_insert(0) += 1;
    }

    /// 获取当前指标快照
    pub fn snapshot(
        &self,
        trace_entries: usize,
        module_cache_size: usize,
        ast_cache: &CacheStats,
    ) -> MetricsSnapshot {
        MetricsSnapshot {
            execution: self.execution.read().unwrap().clone(),
            modules: self.modules.read().unwrap().clone(),
            trace_entries,
            module_cache_size,
            ast_cache: ast_cache.clone(),
        }
    }

    /// 重置所有指标
    pub fn reset(&self) {
        *self.execution.write().unwrap() = ExecutionMetrics::default();
        *self.modules.write().unwrap() = ModuleMetrics::default();
        self.module_loads.write().unwrap().clear();
    }

    /// 获取模块加载次数（用于调试）
    pub fn module_load_count(&self, module_id: &str) -> usize {
        self.module_loads
            .read()
            .unwrap()
            .get(module_id)
            .copied()
            .unwrap_or(0)
    }

    /// 获取所有模块加载次数
    pub fn all_module_loads(&self) -> std::collections::HashMap<String, usize> {
        self.module_loads.read().unwrap().clone()
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector_enable_disable() {
        let collector = MetricsCollector::new();
        assert!(!collector.is_enabled());

        collector.enable();
        assert!(collector.is_enabled());

        collector.disable();
        assert!(!collector.is_enabled());
    }

    #[test]
    fn test_execution_metrics() {
        let collector = MetricsCollector::new();
        collector.enable();

        // 记录执行
        collector.record_execution_start();
        std::thread::sleep(Duration::from_millis(10));
        collector.record_execution_end();

        let snapshot = collector.snapshot(
            0,
            0,
            &CacheStats {
                size: 0,
                max_size: 0,
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
            },
        );

        assert_eq!(snapshot.execution.execution_count, 1);
        assert!(snapshot.execution.total_duration.as_millis() >= 10);
        assert_eq!(
            snapshot.execution.min_duration,
            snapshot.execution.max_duration
        );
    }

    #[test]
    fn test_module_metrics() {
        let collector = MetricsCollector::new();
        collector.enable();

        // 记录模块加载
        collector.record_module_load("test_module", false); // 未命中
        collector.record_module_load("test_module", true); // 命中
        collector.record_module_load("other_module", true); // 命中

        let snapshot = collector.snapshot(
            0,
            0,
            &CacheStats {
                size: 0,
                max_size: 0,
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
            },
        );

        assert_eq!(snapshot.modules.load_count, 3);
        assert_eq!(snapshot.modules.cache_hits, 2);
        assert_eq!(snapshot.modules.cache_misses, 1);
        assert!((snapshot.modules.hit_rate - 0.666).abs() < 0.01); // 约 66.6%
    }

    #[test]
    fn test_metrics_disabled_no_collect() {
        let collector = MetricsCollector::new();
        // 不启用

        collector.record_execution_start();
        collector.record_execution_end();
        collector.record_module_load("test", true);

        let snapshot = collector.snapshot(
            0,
            0,
            &CacheStats {
                size: 0,
                max_size: 0,
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
            },
        );

        // 所有指标都应该是 0
        assert_eq!(snapshot.execution.execution_count, 0);
        assert_eq!(snapshot.modules.load_count, 0);
    }

    #[test]
    fn test_metrics_reset() {
        let collector = MetricsCollector::new();
        collector.enable();

        collector.record_execution_start();
        collector.record_execution_end();
        collector.record_module_load("test", true);

        collector.reset();

        let snapshot = collector.snapshot(
            0,
            0,
            &CacheStats {
                size: 0,
                max_size: 0,
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
            },
        );

        assert_eq!(snapshot.execution.execution_count, 0);
        assert_eq!(snapshot.modules.load_count, 0);
        assert_eq!(collector.all_module_loads().len(), 0);
    }

    #[test]
    fn test_module_load_counts() {
        let collector = MetricsCollector::new();
        collector.enable();

        collector.record_module_load("module_a", false);
        collector.record_module_load("module_a", false);
        collector.record_module_load("module_b", true);

        assert_eq!(collector.module_load_count("module_a"), 2);
        assert_eq!(collector.module_load_count("module_b"), 1);
        assert_eq!(collector.module_load_count("module_c"), 0);

        let loads = collector.all_module_loads();
        assert_eq!(loads.len(), 2);
    }
}
