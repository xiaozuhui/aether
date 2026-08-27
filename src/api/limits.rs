use super::Aether;
use crate::runtime::ExecutionLimits;

impl Aether {
    // ============================================================
    // 执行限制
    // ============================================================

    /// 使用执行限制创建新的 Aether 引擎
    pub fn with_limits(mut self, limits: ExecutionLimits) -> Self {
        self.evaluator.set_limits(limits);
        self
    }

    /// 设置执行限制
    pub fn set_limits(&mut self, limits: ExecutionLimits) {
        self.evaluator.set_limits(limits);
    }

    /// 获取当前执行限制
    pub fn limits(&self) -> &ExecutionLimits {
        self.evaluator.limits()
    }

    // ============================================================
    // 大整数阈值
    // ============================================================

    /// 设置大整数切换阈值。
    ///
    /// 整数字面量位数超过该阈值时，解析期会将其构造为任意精度 BigInteger。
    /// 默认值 15（接近 f64 精度极限）。
    ///
    /// 注意：AST 缓存按源码文本命中，修改阈值后已在缓存中的程序不会重新解析；
    /// 如需立即生效可配合 `reset_env()` 或对新代码使用。
    pub fn set_bigint_threshold(&mut self, threshold: usize) {
        self.bigint_threshold = threshold;
        // Import 路径的解析也走 Evaluator，保持一致
        self.evaluator.set_bigint_threshold(threshold);
    }

    /// 获取当前大整数切换阈值
    pub fn bigint_threshold(&self) -> usize {
        self.bigint_threshold
    }
}
