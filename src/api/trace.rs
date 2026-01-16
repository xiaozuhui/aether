use super::Aether;

impl Aether {
    /// 清空内存中的 TRACE 缓冲区。
    ///
    /// 这是为 DSL 安全调试设计的：脚本调用 `TRACE(...)` 来记录
    /// 值，宿主应用程序通过此方法带外读取它们。
    pub fn take_trace(&mut self) -> Vec<String> {
        self.evaluator.take_trace()
    }

    /// 清除 TRACE 缓冲区而不返回它。
    pub fn clear_trace(&mut self) {
        self.evaluator.clear_trace();
    }

    /// 获取所有结构化的跟踪条目
    ///
    /// 返回带有级别、类别、时间戳等的结构化跟踪条目向量。
    pub fn trace_records(&self) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_records()
    }

    /// 按级别过滤跟踪条目
    pub fn trace_by_level(
        &self,
        level: crate::runtime::TraceLevel,
    ) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_level(level)
    }

    /// 按类别过滤跟踪条目
    pub fn trace_by_category(&self, category: &str) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_category(category)
    }

    /// 按标签过滤跟踪条目
    pub fn trace_by_label(&self, label: &str) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_by_label(label)
    }

    /// 对跟踪条目应用复杂过滤器
    pub fn trace_filter(
        &self,
        filter: &crate::runtime::TraceFilter,
    ) -> Vec<crate::runtime::TraceEntry> {
        self.evaluator.trace_filter(filter)
    }

    /// 获取跟踪统计信息
    ///
    /// 返回关于跟踪条目的统计信息，包括按级别和类别的计数。
    pub fn trace_stats(&self) -> crate::runtime::TraceStats {
        self.evaluator.trace_stats()
    }

    /// 设置 TRACE 缓冲区大小
    ///
    /// 这将设置 TRACE 缓冲区可以存储的最大条目数。
    /// 如果新大小小于当前条目数，多余的条目将被从缓冲区前端移除。
    pub fn set_trace_buffer_size(&mut self, size: usize) {
        self.evaluator.set_trace_buffer_size(size);
    }

    /// 获取当前顶级执行的 step 计数。
    ///
    /// 该计数在每次调用 `eval(...)` / `eval_report(...)`（以及它们的文件包装器）开始时被重置。
    ///
    /// 说明：step 目前按“语句级”计数（每求值一条语句 +1）。
    pub fn step_count(&self) -> usize {
        self.evaluator.step_count()
    }
}
