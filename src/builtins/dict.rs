// src/builtins/dict.rs
//! Dictionary manipulation built-in functions

use crate::evaluator::RuntimeError;
use crate::value::Value;

/// 获取字典的所有键
///
/// # 功能
/// 返回字典中所有键组成的数组。
///
/// # 参数
/// - `dict`: Dict - 字典对象
///
/// # 返回值
/// Array - 包含所有键的数组（键为字符串）
///
/// # 示例
/// ```aether
/// Set person {"name": "Alice", "age": 30, "city": "Beijing"}
/// Set allKeys Keys(person)     # ["name", "age", "city"]
/// Set config {"host": "localhost", "port": 8080}
/// Set settings Keys(config)    # ["host", "port"]
/// ```
pub fn keys(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Dict(dict) => {
            let keys: Vec<Value> = dict.keys().map(|k| Value::String(k.clone())).collect();
            Ok(Value::Array(keys))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Dict".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 获取字典的所有值
///
/// # 功能
/// 返回字典中所有值组成的数组。
///
/// # 参数
/// - `dict`: Dict - 字典对象
///
/// # 返回值
/// Array - 包含所有值的数组
///
/// # 示例
/// ```aether
/// Set person {"name": "Alice", "age": 30, "city": "Beijing"}
/// Set allValues Values(person)     # ["Alice", 30, "Beijing"]
/// Set scores {"math": 95, "english": 88}
/// Set grades Values(scores)        # [95, 88]
/// ```
pub fn values(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Dict(dict) => {
            let vals: Vec<Value> = dict.values().cloned().collect();
            Ok(Value::Array(vals))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Dict".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 检查字典是否包含指定的键
///
/// # 功能
/// 检查字典中是否存在指定的键。
///
/// # 参数
/// - `dict`: Dict - 字典对象
/// - `key`: String - 要检查的键
///
/// # 返回值
/// Boolean - 如果键存在返回 `True`，否则返回 `False`
///
/// # 示例
/// ```aether
/// Set person {"name": "Alice", "age": 30}
/// Set hasName Has(person, "name")      # True
/// Set hasEmail Has(person, "email")    # False
/// Set config {"debug": True}
/// Set hasDebug Has(config, "debug")    # True
/// ```
pub fn has(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Dict(dict), Value::String(key)) => Ok(Value::Boolean(dict.contains_key(key))),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Dict, String".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 合并两个字典
///
/// # 功能
/// 合并两个字典，返回新的字典。如果两个字典有相同的键，第二个字典的值会覆盖第一个。
/// 原始字典不会被修改。
///
/// # 参数
/// - `dict1`: Dict - 第一个字典（基础字典）
/// - `dict2`: Dict - 第二个字典（覆盖字典）
///
/// # 返回值
/// Dict - 合并后的新字典
///
/// # 示例
/// ```aether
/// Set defaults {"host": "localhost", "port": 8080, "debug": False}
/// Set custom {"port": 3000, "debug": True}
/// Set config Merge(defaults, custom)
/// # {"host": "localhost", "port": 3000, "debug": True}
///
/// Set base {"a": 1, "b": 2}
/// Set extra {"c": 3, "d": 4}
/// Set combined Merge(base, extra)      # {"a": 1, "b": 2, "c": 3, "d": 4}
/// ```
pub fn merge(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Dict(dict1), Value::Dict(dict2)) => {
            let mut result = dict1.clone();
            for (k, v) in dict2 {
                result.insert(k.clone(), v.clone());
            }
            Ok(Value::Dict(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Dict, Dict".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}
