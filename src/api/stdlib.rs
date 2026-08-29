use super::Aether;
use crate::stdlib;

impl Aether {
    /// 加载特定的标准库模块
    ///
    /// 可用模块见 `stdlib::ALL_MODULES`（string_utils、array_utils、
    /// validation、datetime、testing、set、queue、stack、heap、sorting、
    /// json、csv、functional、cli_utils、text_template、regex_utils）。
    /// 未知模块名返回 Err。
    pub fn load_stdlib_module(&mut self, module_name: &str) -> Result<(), String> {
        if let Some(code) = stdlib::get_module(module_name) {
            self.eval(code)?;
            Ok(())
        } else {
            Err(format!("Unknown stdlib module: {}", module_name))
        }
    }

    /// 加载所有标准库模块
    pub fn load_all_stdlib(&mut self) -> Result<(), String> {
        stdlib::preload_stdlib(self)
    }

    /// 加载特定标准库模块（可链式调用），未知模块名报错
    ///
    /// ```no_run
    /// # use aether::Aether;
    /// let engine = Aether::new()
    ///     .with_stdlib_module("string_utils")
    ///     .expect("模块加载失败");
    /// ```
    pub fn with_stdlib_module(mut self, module_name: &str) -> Result<Self, String> {
        self.load_stdlib_module(module_name)?;
        Ok(self)
    }
}
