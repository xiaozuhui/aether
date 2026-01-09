//! 模块缓存生命周期管理
//!
//! 提供显式的模块缓存管理 API，支持 TTL 和容量限制。

use crate::value::Value;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// 模块缓存条目
#[derive(Debug, Clone)]
struct ModuleCacheEntry {
    /// 导出的符号
    exports: HashMap<String, Value>,
    /// 加载时间
    loaded_at: Instant,
    /// 访问次数
    #[allow(dead_code)]
    access_count: usize,
}

/// 模块缓存统计信息
#[derive(Debug, Clone)]
pub struct ModuleCacheStats {
    /// 当前缓存的模块数量
    pub module_count: usize,
    /// 总加载次数
    pub total_loads: usize,
    /// 缓存命中次数
    pub cache_hits: usize,
    /// 缓存未命中次数
    pub cache_misses: usize,
    /// 命中率
    pub hit_rate: f64,
}

/// 模块缓存管理器
pub struct ModuleCacheManager {
    /// 缓存存储
    cache: RwLock<HashMap<String, ModuleCacheEntry>>,
    /// 最大缓存数量
    max_size: usize,
    /// TTL（秒）
    ttl_secs: u64,
    /// 统计信息
    stats: RwLock<ModuleCacheStats>,
}

impl ModuleCacheManager {
    /// 创建新的缓存管理器
    pub fn new(max_size: usize, ttl_secs: u64) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            max_size,
            ttl_secs,
            stats: RwLock::new(ModuleCacheStats {
                module_count: 0,
                total_loads: 0,
                cache_hits: 0,
                cache_misses: 0,
                hit_rate: 0.0,
            }),
        }
    }

    /// 获取模块导出（如果存在且未过期）
    pub fn get(&self, module_id: &str) -> Option<HashMap<String, Value>> {
        let cache = self.cache.read().ok()?;
        let entry = cache.get(module_id)?;

        // 检查 TTL
        if self.ttl_secs > 0 {
            let elapsed = entry.loaded_at.elapsed().as_secs();
            if elapsed > self.ttl_secs {
                return None; // 已过期
            }
        }

        // 更新统计
        if let Ok(mut stats) = self.stats.write() {
            stats.cache_hits += 1;
            stats.total_loads += 1;
            if stats.total_loads > 0 {
                stats.hit_rate = stats.cache_hits as f64 / stats.total_loads as f64;
            }
        }

        Some(entry.exports.clone())
    }

    /// 插入模块导出
    pub fn insert(&self, module_id: String, exports: HashMap<String, Value>) {
        // 检查容量限制
        if self.max_size > 0 {
            let mut cache = self.cache.write().unwrap();
            if cache.len() >= self.max_size {
                // 清理最旧的 10% 条目
                let to_remove = (self.max_size / 10).max(1);
                self.evict_oldest(&mut cache, to_remove);
            }

            cache.insert(
                module_id.clone(),
                ModuleCacheEntry {
                    exports,
                    loaded_at: Instant::now(),
                    access_count: 0,
                },
            );

            // 更新统计
            if let Ok(mut stats) = self.stats.write() {
                stats.cache_misses += 1;
                stats.total_loads += 1;
                stats.module_count = cache.len();
                if stats.total_loads > 0 {
                    stats.hit_rate = stats.cache_hits as f64 / stats.total_loads as f64;
                }
            }
        }
    }

    /// 清理过期条目
    pub fn cleanup_expired(&self) {
        if self.ttl_secs == 0 {
            return;
        }

        let ttl = Duration::from_secs(self.ttl_secs);
        let mut cache = self.cache.write().unwrap();
        let now = Instant::now();

        cache.retain(|_, entry| now.duration_since(entry.loaded_at) < ttl);

        if let Ok(mut stats) = self.stats.write() {
            stats.module_count = cache.len();
        }
    }

    /// 清空所有缓存
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();

        if let Ok(mut stats) = self.stats.write() {
            stats.module_count = 0;
            // 保留命中率统计，不清零
        }
    }

    /// 移除特定模块缓存
    pub fn remove(&self, module_id: &str) -> bool {
        let mut cache = self.cache.write().unwrap();
        let removed = cache.remove(module_id).is_some();

        if removed {
            if let Ok(mut stats) = self.stats.write() {
                stats.module_count = cache.len();
            }
        }

        removed
    }

    /// 获取统计信息
    pub fn stats(&self) -> ModuleCacheStats {
        self.stats.read().unwrap().clone()
    }

    /// 获取缓存的模块 ID 列表
    pub fn cached_modules(&self) -> Vec<String> {
        self.cache.read().unwrap().keys().cloned().collect()
    }

    /// 清理最旧的条目（内部方法）
    fn evict_oldest(&self, cache: &mut HashMap<String, ModuleCacheEntry>, count: usize) {
        // 收集需要移除的模块 ID
        let mut entries: Vec<_> = cache.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.loaded_at);

        let to_remove: Vec<String> = entries
            .iter()
            .take(count)
            .map(|(module_id, _)| (*module_id).clone())
            .collect();

        // 现在可以安全地移除
        for module_id in to_remove {
            cache.remove(&module_id);
        }
    }
}

impl Default for ModuleCacheManager {
    fn default() -> Self {
        Self::new(100, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_cache_basic() {
        let manager = ModuleCacheManager::new(10, 0);

        // 插入模块
        let mut exports = HashMap::new();
        exports.insert("foo".to_string(), Value::Number(42.0));
        manager.insert("test_module".to_string(), exports.clone());

        // 获取模块
        let retrieved = manager.get("test_module");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get("foo").unwrap(), &Value::Number(42.0));
    }

    #[test]
    fn test_module_cache_miss() {
        let manager = ModuleCacheManager::new(10, 0);

        // 获取不存在的模块
        let retrieved = manager.get("nonexistent");
        assert!(retrieved.is_none());

        // 检查统计
        let stats = manager.stats();
        assert_eq!(stats.cache_misses, 0); // 还没有插入
        assert_eq!(stats.cache_hits, 0);
    }

    #[test]
    fn test_module_cache_max_size() {
        let manager = ModuleCacheManager::new(3, 0); // 最多 3 个
        let exports = HashMap::new();

        // 插入 5 个模块
        for i in 0..5 {
            manager.insert(format!("module{}", i), exports.clone());
        }

        // 应该只保留 3 个
        assert_eq!(manager.cached_modules().len(), 3);
        assert_eq!(manager.stats().module_count, 3);
    }

    #[test]
    fn test_module_cache_clear() {
        let manager = ModuleCacheManager::new(10, 0);
        let mut exports = HashMap::new();
        exports.insert("foo".to_string(), Value::Number(42.0));

        manager.insert("test".to_string(), exports);
        assert_eq!(manager.cached_modules().len(), 1);

        manager.clear();
        assert_eq!(manager.cached_modules().len(), 0);
    }

    #[test]
    fn test_module_cache_remove() {
        let manager = ModuleCacheManager::new(10, 0);
        let mut exports = HashMap::new();
        exports.insert("foo".to_string(), Value::Number(42.0));

        manager.insert("test".to_string(), exports);
        assert!(manager.remove("test"));
        assert!(!manager.remove("nonexistent"));
        assert_eq!(manager.cached_modules().len(), 0);
    }

    #[test]
    fn test_module_cache_ttl() {
        let manager = ModuleCacheManager::new(10, 1); // 1秒 TTL
        let mut exports = HashMap::new();
        exports.insert("foo".to_string(), Value::Number(42.0));

        manager.insert("test".to_string(), exports.clone());

        // 立即获取应该成功
        assert!(manager.get("test").is_some());

        // 等待 2 秒后应该过期
        std::thread::sleep(Duration::from_secs(2));
        assert!(manager.get("test").is_none());
    }

    #[test]
    fn test_module_cache_stats() {
        let manager = ModuleCacheManager::new(10, 0);
        let mut exports = HashMap::new();
        exports.insert("foo".to_string(), Value::Number(42.0));

        // 插入模块
        manager.insert("test".to_string(), exports.clone());

        // 命中
        manager.get("test");
        manager.get("test");

        // 未命中
        manager.get("nonexistent");

        let stats = manager.stats();
        assert_eq!(stats.total_loads, 3);
        assert_eq!(stats.cache_hits, 2);
        assert_eq!(stats.cache_misses, 1);
        assert!((stats.hit_rate - 0.666).abs() < 0.01); // 约 66.6%
    }
}
