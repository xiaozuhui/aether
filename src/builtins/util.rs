// src/builtins/util.rs
//! 内置函数共用的小工具（取参样板统一）

use crate::evaluator::RuntimeError;
use crate::value::Value;

/// 取 Number 参数（类型不符时报详细类型错误）
pub fn get_number(val: &Value) -> Result<f64, RuntimeError> {
    match val {
        Value::Number(n) => Ok(*n),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number".to_string(),
            got: format!("{:?}", val),
        }),
    }
}

/// 取 String 参数
pub fn get_string(val: &Value) -> Result<String, RuntimeError> {
    match val {
        Value::String(s) => Ok(s.clone()),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String".to_string(),
            got: format!("{:?}", val),
        }),
    }
}
