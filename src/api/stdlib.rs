use super::Aether;
use crate::stdlib;

impl Aether {
    /// 加载特定的标准库模块
    ///
    /// 可用模块："string_utils"、"array_utils"、"validation"、"datetime"、"testing"
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

    // ============================================================
    // 可链式调用的 stdlib 模块加载方法
    // ============================================================

    /// 加载字符串工具模块（可链式调用）
    pub fn with_stdlib_string_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("string_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载数组工具模块（可链式调用）
    pub fn with_stdlib_array_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("array_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载验证模块（可链式调用）
    pub fn with_stdlib_validation(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("validation") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载日期时间模块（可链式调用）
    pub fn with_stdlib_datetime(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("datetime") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载测试框架模块（可链式调用）
    pub fn with_stdlib_testing(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("testing") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载集合数据结构模块（可链式调用）
    pub fn with_stdlib_set(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("set") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载队列数据结构模块（可链式调用）
    pub fn with_stdlib_queue(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("queue") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载栈数据结构模块（可链式调用）
    pub fn with_stdlib_stack(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("stack") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载堆数据结构模块（可链式调用）
    pub fn with_stdlib_heap(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("heap") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载排序算法模块（可链式调用）
    pub fn with_stdlib_sorting(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("sorting") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载 JSON 处理模块（可链式调用）
    pub fn with_stdlib_json(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("json") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载 CSV 处理模块（可链式调用）
    pub fn with_stdlib_csv(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("csv") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载函数式编程工具模块（可链式调用）
    pub fn with_stdlib_functional(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("functional") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载 CLI 工具模块（可链式调用）
    pub fn with_stdlib_cli_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("cli_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载文本模板引擎模块（可链式调用）
    pub fn with_stdlib_text_template(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("text_template") {
            self.eval(code)?;
        }
        Ok(self)
    }

    /// 加载正则表达式工具模块（可链式调用）
    pub fn with_stdlib_regex_utils(mut self) -> Result<Self, String> {
        if let Some(code) = stdlib::get_module("regex_utils") {
            self.eval(code)?;
        }
        Ok(self)
    }
}
