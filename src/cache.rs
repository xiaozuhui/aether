// src/cache.rs
//! AST缓存机制,减少重复解析

use crate::ast::Program;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// AST缓存,用于存储已解析的程序
#[derive(Debug)]
pub struct ASTCache {
    /// 缓存存储: hash -> 解析后的AST
    cache: HashMap<u64, Program>,
    /// 缓存大小限制
    max_size: usize,
    /// 缓存命中统计
    hits: usize,
    /// 缓存未命中统计
    misses: usize,
}

impl ASTCache {
    /// 创建新的AST缓存
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// 创建指定容量的AST缓存
    pub fn with_capacity(max_size: usize) -> Self {
        ASTCache {
            cache: HashMap::with_capacity(max_size.min(100)),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    /// 计算代码的哈希值
    fn hash_code(code: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        hasher.finish()
    }

    /// 从缓存中获取AST
    pub fn get(&mut self, code: &str) -> Option<Program> {
        let hash = Self::hash_code(code);
        if let Some(program) = self.cache.get(&hash) {
            self.hits += 1;
            Some(program.clone())
        } else {
            self.misses += 1;
            None
        }
    }

    /// 将AST存入缓存
    pub fn insert(&mut self, code: &str, program: Program) {
        let hash = Self::hash_code(code);

        // 如果缓存已满,使用简单的FIFO策略清理
        if self.cache.len() >= self.max_size {
            // 清理最早的10%条目
            let to_remove = (self.max_size / 10).max(1);
            let keys_to_remove: Vec<u64> = self.cache.keys().take(to_remove).copied().collect();
            for key in keys_to_remove {
                self.cache.remove(&key);
            }
        }

        self.cache.insert(hash, program);
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// 获取缓存统计信息
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses > 0 {
                self.hits as f64 / (self.hits + self.misses) as f64
            } else {
                0.0
            },
        }
    }
}

impl Default for ASTCache {
    fn default() -> Self {
        Self::new()
    }
}

/// 缓存统计信息
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// 当前缓存大小
    pub size: usize,
    /// 最大缓存大小
    pub max_size: usize,
    /// 缓存命中次数
    pub hits: usize,
    /// 缓存未命中次数
    pub misses: usize,
    /// 缓存命中率
    pub hit_rate: f64,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cache Stats: size={}/{}, hits={}, misses={}, hit_rate={:.2}%",
            self.size,
            self.max_size,
            self.hits,
            self.misses,
            self.hit_rate * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = ASTCache::new();
        let code = "Set X 10";

        // 第一次获取应该失败
        assert!(cache.get(code).is_none());
        assert_eq!(cache.stats().misses, 1);

        // 插入后应该能获取
        let program: Program = vec![];
        cache.insert(code, program.clone());
        assert!(cache.get(code).is_some());
        assert_eq!(cache.stats().hits, 1);
    }

    #[test]
    fn test_cache_capacity() {
        let mut cache = ASTCache::with_capacity(5);
        let program: Program = vec![];

        // 插入超过容量的条目
        for i in 0..10 {
            let code = format!("Set X {}", i);
            cache.insert(&code, program.clone());
        }

        // 缓存大小应该被限制
        assert!(cache.stats().size <= 5);
    }
}
