use super::Aether;
use crate::evaluator::ErrorReport;
use crate::parser::Parser;
use crate::value::Value;

impl Aether {
    /// 求值 Aether 代码并返回结果
    pub fn eval(&mut self, code: &str) -> Result<Value, String> {
        // 在开始新的顶级求值之前清除任何之前的调用栈帧。
        self.evaluator.clear_call_stack();
        self.evaluator.reset_step_counter();

        // 尝试从缓存获取AST
        let program = if let Some(cached_program) = self.cache.get(code) {
            cached_program
        } else {
            // 解析代码
            let mut parser = Parser::new(code);
            let program = parser
                .parse_program()
                .map_err(|e| format!("Parse error: {}", e))?;

            // 优化AST
            let optimized = self.optimizer.optimize_program(&program);

            // 将优化后的结果存入缓存
            self.cache.insert(code, optimized.clone());
            optimized
        };

        // 求值程序
        self.evaluator
            .eval_program(&program)
            .map_err(|e| format!("Runtime error: {}", e))
    }

    /// 求值 Aether 代码并在失败时返回结构化的错误报告。
    ///
    /// 这适用于需要机器可读诊断的集成。
    pub fn eval_report(&mut self, code: &str) -> Result<Value, ErrorReport> {
        // 在开始新的顶级求值之前清除任何之前的调用栈帧。
        self.evaluator.clear_call_stack();
        self.evaluator.reset_step_counter();

        // 首先尝试 AST 缓存
        let program = if let Some(cached_program) = self.cache.get(code) {
            cached_program
        } else {
            let mut parser = Parser::new(code);
            let program = parser
                .parse_program()
                .map_err(|e| ErrorReport::parse_error(e.to_string()))?;

            let optimized = self.optimizer.optimize_program(&program);
            self.cache.insert(code, optimized.clone());
            optimized
        };

        self.evaluator
            .eval_program(&program)
            .map_err(|e| e.to_error_report())
    }

    /// 配置用于 `Import/Export` 的模块解析器。
    ///
    /// 默认情况下（DSL 嵌入），解析器出于安全考虑被禁用。
    pub fn set_module_resolver(&mut self, resolver: Box<dyn crate::module_system::ModuleResolver>) {
        self.evaluator.set_module_resolver(resolver);
    }

    /// 推送用于解析相对导入的基础目录上下文。
    ///
    /// 这通常由基于文件的运行器（CLI）在调用 `eval()` 之前使用。
    pub fn push_import_base(&mut self, module_id: String, base_dir: Option<std::path::PathBuf>) {
        self.evaluator.push_import_base(module_id, base_dir);
    }

    /// 弹出最近的基础目录上下文。
    pub fn pop_import_base(&mut self) {
        self.evaluator.pop_import_base();
    }

    /// 从文件路径求值 Aether 脚本。
    ///
    /// 这是一个便利包装器，它：
    /// - 读取文件
    /// - 推送导入基础上下文（module_id = 规范路径；base_dir = 父目录）
    /// - 求值代码
    /// - 弹出导入基础上下文
    ///
    /// 注意：这**不会**启用任何模块解析器。为了 DSL 安全性，除非您明确调用 `set_module_resolver(...)`，否则模块加载保持禁用状态。
    pub fn eval_file(&mut self, path: impl AsRef<std::path::Path>) -> Result<Value, String> {
        let path = path.as_ref();

        let code = std::fs::read_to_string(path).map_err(|e| format!("IO error: {}", e))?;

        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let base_dir = canon.parent().map(|p| p.to_path_buf());

        self.push_import_base(canon.display().to_string(), base_dir);
        let res = self.eval(&code);
        self.pop_import_base();
        res
    }

    /// 从文件路径求值 Aether 脚本，在失败时返回结构化的错误报告。
    pub fn eval_file_report(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Value, ErrorReport> {
        let path = path.as_ref();

        let code = std::fs::read_to_string(path)
            .map_err(|e| ErrorReport::io_error(format!("IO error: {e}")))?;

        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let base_dir = canon.parent().map(|p| p.to_path_buf());

        self.push_import_base(canon.display().to_string(), base_dir);
        let res = self.eval_report(&code);
        self.pop_import_base();
        res
    }

    /// 从宿主应用程序设置全局变量，而不使用 `eval()`。
    ///
    /// 当您已经有 Rust 端数据并希望将其作为 `Value` 注入脚本环境时，这很有用。
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.evaluator.set_global(name.to_string(), value);
    }

    /// 重置运行时环境（变量/函数），同时保持内置函数注册。
    ///
    /// 注意：这会清除通过 `eval()` 引入的任何内容（包括 stdlib 代码）。
    pub fn reset_env(&mut self) {
        self.evaluator.reset_env();
    }

    /// 在隔离的子作用域内运行闭包。
    ///
    /// 在闭包内注入或定义的所有变量/函数将在返回时被丢弃，而外部环境被保留。
    ///
    /// 这是为 "DSL 宿主" 场景设计的：注入 Rust 数据 + 加载每请求的
    /// Aether 函数（例如从 DB）+ 运行脚本，而不跨请求污染。
    pub fn with_isolated_scope<R>(
        &mut self,
        f: impl FnOnce(&mut Aether) -> Result<R, String>,
    ) -> Result<R, String> {
        let prev_env = self.evaluator.enter_child_scope();
        let result = f(self);
        self.evaluator.restore_env(prev_env);
        result
    }

    /// 异步求值 Aether 代码（需要 "async" 特性）
    ///
    /// 这是围绕 `eval()` 的便利包装器，在后台任务中运行。
    /// 用于将 Aether 集成到异步 Rust 应用程序中。
    #[cfg(feature = "async")]
    pub async fn eval_async(&mut self, code: &str) -> Result<Value, String> {
        // 由于 Aether 内部使用 Rc (非 Send)，我们在当前线程执行
        // 但通过 tokio::task::yield_now() 让出执行权，避免阻塞事件循环
        tokio::task::yield_now().await;
        self.eval(code)
    }
}
