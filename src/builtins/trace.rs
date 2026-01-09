// src/builtins/trace.rs
//
// 结构化 TRACE 内置函数。
//
// 注意：这些函数在 evaluator 中有特殊处理，以便直接写入引擎的内存 TRACE 缓冲区。

use crate::evaluator::RuntimeError;
use crate::value::Value;

/// TRACE - 基础 TRACE 函数（向后兼容）
///
/// 用法: TRACE(value1, value2, ...) 或 TRACE("label", value1, value2, ...)
pub fn trace(_args: &[Value]) -> Result<Value, RuntimeError> {
    // 在 evaluator 中有特殊处理
    Ok(Value::Null)
}

/// TRACE_DEBUG - 调试级别 TRACE
///
/// 用法: TRACE_DEBUG("category", value1, value2, ...)
pub fn trace_debug(_args: &[Value]) -> Result<Value, RuntimeError> {
    // 在 evaluator 中有特殊处理
    Ok(Value::Null)
}

/// TRACE_INFO - 信息级别 TRACE（默认）
///
/// 用法: TRACE_INFO("category", value1, value2, ...)
pub fn trace_info(_args: &[Value]) -> Result<Value, RuntimeError> {
    // 在 evaluator 中有特殊处理
    Ok(Value::Null)
}

/// TRACE_WARN - 警告级别 TRACE
///
/// 用法: TRACE_WARN("category", value1, value2, ...)
pub fn trace_warn(_args: &[Value]) -> Result<Value, RuntimeError> {
    // 在 evaluator 中有特殊处理
    Ok(Value::Null)
}

/// TRACE_ERROR - 错误级别 TRACE
///
/// 用法: TRACE_ERROR("category", value1, value2, ...)
pub fn trace_error(_args: &[Value]) -> Result<Value, RuntimeError> {
    // 在 evaluator 中有特殊处理
    Ok(Value::Null)
}
