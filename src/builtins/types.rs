// src/builtins/types.rs
//! 类型操作内置函数模块
//!
//! 提供类型检查、类型转换和长度计算等功能。

use crate::evaluator::RuntimeError;
use crate::value::Value;

/// 获取值的类型名称
///
/// # 功能
/// 返回值的类型名称字符串。
///
/// # 参数
/// - `value`: 任意值
///
/// # 返回值
/// 类型名称字符串："Number", "String", "Boolean", "Null", "Array", "Dict", "Function", "Generator", "Lazy", "BuiltIn"
///
/// # 示例
/// ```aether
/// Println(TypeOf(42))          # 输出: Number
/// Println(TypeOf("hello"))     # 输出: String
/// Println(TypeOf([1, 2, 3]))   # 输出: Array
/// Println(TypeOf(True))        # 输出: Boolean
/// ```
pub fn type_of(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    let type_name = match &args[0] {
        Value::Number(_) => "Number",
        Value::Fraction(_) => "Fraction",
        Value::String(_) => "String",
        Value::Boolean(_) => "Boolean",
        Value::Null => "Null",
        Value::Array(_) => "Array",
        Value::Dict(_) => "Dict",
        Value::Function { .. } => "Function",
        Value::Generator { .. } => "Generator",
        Value::Lazy { .. } => "Lazy",
        Value::BuiltIn { .. } => "BuiltIn",
    };

    Ok(Value::String(type_name.to_string()))
}

/// 将值转换为字符串
///
/// # 功能
/// 将任意类型的值转换为其字符串表示形式。
///
/// # 参数
/// - `value`: 要转换的值（任意类型）
///
/// # 返回值
/// 字符串类型的值
///
/// # 示例
/// ```aether
/// Set NUM 42
/// Set STR ToString(NUM)         # "42"
/// Println(ToString(True))       # "true"
/// Println(ToString([1, 2, 3]))  # "[1, 2, 3]"
/// Println(ToString(Null))       # "null"
/// ```
pub fn to_string(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    Ok(Value::String(args[0].to_string()))
}

/// 将值转换为数字
///
/// # 功能
/// 将字符串、布尔值或其他类型转换为数字。
///
/// # 参数
/// - `value`: 要转换的值
///
/// # 返回值
/// 数字类型的值
///
/// # 转换规则
/// - Number → 返回原值
/// - String → 解析为浮点数（失败则报错）
/// - Boolean → true=1.0, false=0.0
/// - Null → 0.0
/// - 其他类型 → 报错
///
/// # 示例
/// ```aether
/// Set NUM ToNumber("123")       # 123.0
/// Set VAL ToNumber("3.14")      # 3.14
/// Set B1 ToNumber(True)         # 1.0
/// Set B2 ToNumber(False)        # 0.0
/// Set NULL_NUM ToNumber(Null)   # 0.0
/// ```
pub fn to_number(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => {
            s.parse::<f64>()
                .map(Value::Number)
                .map_err(|_| RuntimeError::TypeErrorDetailed {
                    expected: "parseable string".to_string(),
                    got: format!("\"{}\"", s),
                })
        }
        Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        Value::Null => Ok(Value::Number(0.0)),
        other => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number, String, Boolean or Null".to_string(),
            got: format!("{:?}", other),
        }),
    }
}

/// 获取集合的长度
///
/// # 功能
/// 返回字符串、数组或字典的元素个数。
///
/// # 参数
/// - `collection`: 字符串、数组或字典
///
/// # 返回值
/// 长度（数字）
///
/// # 示例
/// ```aether
/// Println(Len("hello"))         # 5
/// Println(Len([1, 2, 3]))       # 3
/// Println(Len({"a": 1, "b": 2}))  # 2
/// Println(Len(""))              # 0
/// Println(Len([]))              # 0
/// ```
pub fn len(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
        Value::Dict(dict) => Ok(Value::Number(dict.len() as f64)),
        other => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, Array or Dict".to_string(),
            got: format!("{:?}", other),
        }),
    }
}
