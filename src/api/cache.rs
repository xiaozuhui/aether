use super::Aether;
use crate::cache::CacheStats;

impl Aether {
    /// 获取缓存统计信息
    pub fn cache_stats(&self) -> CacheStats {
        self.cache.stats()
    }

    /// 清空缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 设置优化选项
    pub fn set_optimization(
        &mut self,
        constant_folding: bool,
        dead_code: bool,
        tail_recursion: bool,
    ) {
        self.optimizer.constant_folding = constant_folding;
        self.optimizer.dead_code_elimination = dead_code;
        self.optimizer.tail_recursion = tail_recursion;
    }
}
