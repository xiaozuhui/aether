use super::Aether;
use crate::debugger::DebuggerState;
use std::cell::RefCell;
use std::rc::Rc;

impl Aether {
    /// 挂载调试器：与 `DebuggerSession` 共享同一状态实例，
    /// 命中断点/步进时回调 `hook`（在其中运行调试 REPL），返回 true 终止程序
    pub fn attach_debugger(
        &mut self,
        state: Rc<RefCell<DebuggerState>>,
        hook: Box<dyn FnMut(&mut crate::evaluator::Evaluator) -> bool>,
    ) {
        self.evaluator.attach_debugger(state, hook);
    }

    /// 卸载调试器
    pub fn detach_debugger(&mut self) {
        self.evaluator.detach_debugger();
    }

    /// 设置当前源文件（供调试器按 `file:line` 匹配断点；文件运行器应在求值前调用）
    pub fn set_source_file(&mut self, file: String) {
        self.evaluator.set_source_file(file);
    }

    /// 按作用域链查找变量（供调试器在暂停时检查变量值）
    pub fn lookup_variable(&self, name: &str) -> Option<crate::value::Value> {
        self.evaluator.lookup_variable(name)
    }

    /// 访问内部求值器（调试器命令需要读取调用栈深度、当前源文件等状态）
    pub fn evaluator_mut(&mut self) -> &mut crate::evaluator::Evaluator {
        &mut self.evaluator
    }
}
