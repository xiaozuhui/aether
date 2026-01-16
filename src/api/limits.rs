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
}
