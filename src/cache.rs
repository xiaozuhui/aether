// src/cache.rs
//! AST缓存机制,减少重复解析

use crate::ast::Program;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// AST 缓存，用于存储已解析的程序。
///
/// - **碰撞安全**：u64 哈希仅作加速索引，命中前必须比对源码
///   完全一致，哈希碰撞不会返回错误的 AST。
/// - **LRU 淘汰**：`VecDeque` 维护真实的使用顺序，get 命中会刷新
///   新鲜度；容量满时淘汰最近最少使用的条目。
#[derive(Debug)]
pub struct ASTCache {
    /// 缓存存储: hash -> (源码, 解析后的 AST)
    cache: HashMap<u64, (String, Program)>,
    /// 按使用顺序排列的哈希键（队首 = 最近最少使用）
    order: VecDeque<u64>,
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
            cache: HashMap::with_capacity(max_size),
            order: VecDeque::with_capacity(max_size),
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

    /// 把键移到 LRU 队尾（最近使用）
    fn touch(&mut self, hash: u64) {
        if let Some(pos) = self.order.iter().position(|&k| k == hash) {
            self.order.remove(pos);
        }
        self.order.push_back(hash);
    }

    /// 从缓存中获取AST（命中要求源码完全一致）
    pub fn get(&mut self, code: &str) -> Option<Program> {
        let hash = Self::hash_code(code);
        // 先比对源码（碰撞安全），释放借用后再刷新 LRU 顺序
        let hit = self
            .cache
            .get(&hash)
            .is_some_and(|(cached_code, _)| cached_code == code);
        if hit {
            self.hits += 1;
            self.touch(hash);
            self.cache.get(&hash).map(|(_, program)| program.clone())
        } else {
            self.misses += 1;
            None
        }
    }

    /// 将AST存入缓存（容量满时淘汰最近最少使用的条目）
    pub fn insert(&mut self, code: &str, program: Program) {
        let hash = Self::hash_code(code);

        if self.cache.contains_key(&hash) {
            // 同键覆盖（源码可能不同）：保持容量不变，刷新新鲜度
            self.touch(hash);
        } else {
            while self.cache.len() >= self.max_size {
                if let Some(lru) = self.order.pop_front() {
                    self.cache.remove(&lru);
                } else {
                    break;
                }
            }
            self.order.push_back(hash);
        }

        self.cache.insert(hash, (code.to_string(), program));
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
