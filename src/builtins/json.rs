// src/builtins/json.rs
//! JSON 处理内置函数模块
//!
//! 提供 JSON 解析和序列化功能。

use crate::evaluator::RuntimeError;
use crate::value::Value;
use num_traits::ToPrimitive;
use std::collections::HashMap;

/// 将 JSON 字符串解析为 Aether 值
///
/// # 功能
/// 解析 JSON 格式的字符串，转换为对应的 Aether 数据结构。
///
/// # 参数
/// - `json_str`: JSON 格式的字符串
///
/// # 返回值
/// 解析后的 Aether 值（Dict、Array、String、Number、Boolean 或 Null）
///
/// # 映射规则
/// - JSON object → Dict
/// - JSON array → Array
/// - JSON string → String
/// - JSON number → Number
/// - JSON boolean → Boolean
/// - JSON null → Null
///
/// # 示例
/// ```aether
/// Set JSON_STR "{\"name\":\"Alice\",\"age\":30}"
/// Set OBJ JSON_PARSE(JSON_STR)
/// Println(OBJ["name"])  # 输出: Alice
/// ```
pub fn json_parse(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    let json_str = match &args[0] {
        Value::String(s) => s,
        other => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "String".to_string(),
                got: format!("{:?}", other),
            });
        }
    };

    // 使用 serde_json 解析
    let json_value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| RuntimeError::CustomError(format!("JSON parse error: {}", e)))?;

    // 转换为 Aether Value
    json_to_value(&json_value)
}

/// 将 Aether 值序列化为 JSON 字符串
///
/// # 功能
/// 将 Aether 数据结构转换为 JSON 格式的字符串。
///
/// # 参数
/// - `value`: 要序列化的值
/// - `indent`: （可选）缩进空格数，默认为 0（紧凑格式）
///
/// # 返回值
/// JSON 格式的字符串
///
/// # 示例
/// ```aether
/// Set OBJ {"name": "Alice", "age": 30}
/// Set JSON_STR JSON_STRINGIFY(OBJ)
/// Println(JSON_STR)  # 输出: {"name":"Alice","age":30}
///
/// # 格式化输出（2空格缩进）
/// Set PRETTY JSON_STRINGIFY(OBJ, 2)
/// ```
pub fn json_stringify(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    let value = &args[0];
    let indent = if args.len() == 2 {
        match &args[1] {
            Value::Number(n) => *n as usize,
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Number".to_string(),
                    got: format!("{:?}", other),
                });
            }
        }
    } else {
        0
    };

    // 转换为 serde_json::Value
    let json_value = value_to_json(value)?;

    // 序列化
    let json_str = if indent > 0 {
        serde_json::to_string_pretty(&json_value)
    } else {
        serde_json::to_string(&json_value)
    }
    .map_err(|e| RuntimeError::CustomError(format!("JSON stringify error: {}", e)))?;

    Ok(Value::String(json_str))
}

/// 将 serde_json::Value 转换为 Aether Value
fn json_to_value(json: &serde_json::Value) -> Result<Value, RuntimeError> {
    match json {
        serde_json::Value::Null => Ok(Value::Null),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Number(i as f64))
            } else if let Some(u) = n.as_u64() {
                Ok(Value::Number(u as f64))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err(RuntimeError::CustomError("Invalid JSON number".to_string()))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let mut aether_arr = Vec::new();
            for item in arr {
                aether_arr.push(json_to_value(item)?);
            }
            Ok(Value::Array(aether_arr))
        }
        serde_json::Value::Object(obj) => {
            let mut aether_dict = HashMap::new();
            for (key, val) in obj {
                aether_dict.insert(key.clone(), json_to_value(val)?);
            }
            Ok(Value::Dict(aether_dict))
        }
    }
}

/// 将 Aether Value 转换为 serde_json::Value
fn value_to_json(value: &Value) -> Result<serde_json::Value, RuntimeError> {
    match value {
        Value::Null => Ok(serde_json::Value::Null),
        Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
        Value::Number(n) => Ok(serde_json::json!(n)),
        Value::String(s) => Ok(serde_json::Value::String(s.clone())),
        Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(value_to_json(item)?);
            }
            Ok(serde_json::Value::Array(json_arr))
        }
        Value::Dict(dict) => {
            let mut json_obj = serde_json::Map::new();
            for (key, val) in dict {
                json_obj.insert(key.clone(), value_to_json(val)?);
            }
            Ok(serde_json::Value::Object(json_obj))
        }
        Value::Fraction(f) => {
            // 将分数转换为浮点数
            let float_val = f.numer().to_f64().unwrap_or(0.0) / f.denom().to_f64().unwrap_or(1.0);
            Ok(serde_json::json!(float_val))
        }
        other => Err(RuntimeError::CustomError(format!(
            "Cannot convert {:?} to JSON",
            other
        ))),
    }
}
