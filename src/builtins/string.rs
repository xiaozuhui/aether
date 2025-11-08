// src/builtins/string.rs
//! String manipulation built-in functions

use crate::evaluator::RuntimeError;
use crate::value::Value;

/// 分割字符串
///
/// # 功能
/// 使用指定的分隔符将字符串分割成数组。
///
/// # 参数
/// - `string`: String - 要分割的字符串
/// - `separator`: String - 分隔符
///
/// # 返回值
/// Array - 包含分割后的子字符串的数组
///
/// # 示例
/// ```aether
/// Set text "apple,banana,cherry"
/// Set fruits Split(text, ",")      # ["apple", "banana", "cherry"]
/// Set sentence "Hello World"
/// Set words Split(sentence, " ")   # ["Hello", "World"]
/// Set csv "a|b|c|d"
/// Set parts Split(csv, "|")        # ["a", "b", "c", "d"]
/// ```
pub fn split(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(sep)) => {
            let parts: Vec<Value> = s
                .split(sep.as_str())
                .map(|p| Value::String(p.to_string()))
                .collect();
            Ok(Value::Array(parts))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, String".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 转换为大写
///
/// # 功能
/// 将字符串中的所有字母转换为大写形式。
///
/// # 参数
/// - `string`: String - 要转换的字符串
///
/// # 返回值
/// String - 大写形式的字符串
///
/// # 示例
/// ```aether
/// Set text "hello world"
/// Set upper Upper(text)        # "HELLO WORLD"
/// Set mixed "Hello123"
/// Set upper Upper(mixed)       # "HELLO123"
/// ```
pub fn upper(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 转换为小写
///
/// # 功能
/// 将字符串中的所有字母转换为小写形式。
///
/// # 参数
/// - `string`: String - 要转换的字符串
///
/// # 返回值
/// String - 小写形式的字符串
///
/// # 示例
/// ```aether
/// Set text "HELLO WORLD"
/// Set lower Lower(text)        # "hello world"
/// Set mixed "Hello123"
/// Set lower Lower(mixed)       # "hello123"
/// ```
pub fn lower(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 去除首尾空白字符
///
/// # 功能
/// 移除字符串开头和结尾的空白字符（空格、制表符、换行符等）。
///
/// # 参数
/// - `string`: String - 要处理的字符串
///
/// # 返回值
/// String - 去除首尾空白后的字符串
///
/// # 示例
/// ```aether
/// Set text "  hello world  "
/// Set trimmed Trim(text)       # "hello world"
/// Set text "\t\ntest\n\t"
/// Set trimmed Trim(text)       # "test"
/// ```
pub fn trim(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 检查是否包含子字符串
///
/// # 功能
/// 检查字符串是否包含指定的子字符串。
///
/// # 参数
/// - `string`: String - 要检查的字符串
/// - `substring`: String - 要查找的子字符串
///
/// # 返回值
/// Boolean - 如果包含返回 `True`，否则返回 `False`
///
/// # 示例
/// ```aether
/// Set text "Hello World"
/// Set has Contains(text, "World")    # True
/// Set has Contains(text, "Python")   # False
/// Set email "user@example.com"
/// Set has Contains(email, "@")       # True
/// ```
pub fn contains(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        // String contains substring
        (Value::String(s), Value::String(substr)) => {
            Ok(Value::Boolean(s.contains(substr.as_str())))
        }
        // Array contains element
        (Value::Array(arr), item) => {
            for elem in arr.iter() {
                if values_equal(elem, item) {
                    return Ok(Value::Boolean(true));
                }
            }
            Ok(Value::Boolean(false))
        }
        // Dict contains key
        (Value::Dict(dict), Value::String(key)) => Ok(Value::Boolean(dict.contains_key(key))),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "(String, String) or (Array, Any) or (Dict, String)".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

// Helper function to compare values for equality
fn values_equal(a: &Value, b: &Value) -> bool {
    use Value::*;
    match (a, b) {
        (Number(a), Number(b)) => (a - b).abs() < f64::EPSILON,
        (String(a), String(b)) => a == b,
        (Boolean(a), Boolean(b)) => a == b,
        (Null, Null) => true,
        _ => false,
    }
}

/// 检查是否以指定前缀开头
///
/// # 功能
/// 检查字符串是否以指定的前缀开头。
///
/// # 参数
/// - `string`: String - 要检查的字符串
/// - `prefix`: String - 前缀字符串
///
/// # 返回值
/// Boolean - 如果以该前缀开头返回 `True`，否则返回 `False`
///
/// # 示例
/// ```aether
/// Set filename "test.txt"
/// Set starts StartsWith(filename, "test")    # True
/// Set starts StartsWith(filename, "data")    # False
/// Set url "https://example.com"
/// Set isHttps StartsWith(url, "https://")    # True
/// ```
pub fn starts_with(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(prefix)) => {
            Ok(Value::Boolean(s.starts_with(prefix.as_str())))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, String".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 检查是否以指定后缀结尾
///
/// # 功能
/// 检查字符串是否以指定的后缀结尾。
///
/// # 参数
/// - `string`: String - 要检查的字符串
/// - `suffix`: String - 后缀字符串
///
/// # 返回值
/// Boolean - 如果以该后缀结尾返回 `True`，否则返回 `False`
///
/// # 示例
/// ```aether
/// Set filename "document.pdf"
/// Set isPdf EndsWith(filename, ".pdf")       # True
/// Set isTxt EndsWith(filename, ".txt")       # False
/// Set email "user@gmail.com"
/// Set isGmail EndsWith(email, "@gmail.com")  # True
/// ```
pub fn ends_with(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(suffix)) => {
            Ok(Value::Boolean(s.ends_with(suffix.as_str())))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, String".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 替换字符串中的所有匹配项
///
/// # 功能
/// 将字符串中所有出现的子字符串替换为新的字符串。
///
/// # 参数
/// - `string`: String - 原始字符串
/// - `from`: String - 要被替换的子字符串
/// - `to`: String - 替换后的新字符串
///
/// # 返回值
/// String - 替换后的字符串
///
/// # 示例
/// ```aether
/// Set text "Hello World"
/// Set replaced Replace(text, "World", "Aether")  # "Hello Aether"
/// Set text "foo bar foo"
/// Set replaced Replace(text, "foo", "baz")       # "baz bar baz"
/// Set path "C:\\Users\\Name"
/// Set fixed Replace(path, "\\", "/")             # "C:/Users/Name"
/// ```
pub fn replace(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::String(from), Value::String(to)) => {
            Ok(Value::String(s.replace(from.as_str(), to.as_str())))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, String, String".to_string(),
            got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
        }),
    }
}

/// 重复字符串
///
/// # 功能
/// 将字符串重复指定的次数。
///
/// # 参数
/// - `string`: String - 要重复的字符串
/// - `count`: Number - 重复次数（必须是非负整数）
///
/// # 返回值
/// String - 重复后的字符串
///
/// # 错误
/// - 重复次数为负数或非整数时抛出错误
///
/// # 示例
/// ```aether
/// Set str "Ha"
/// Set laugh Repeat(str, 3)         # "HaHaHa"
/// Set dash "-"
/// Set line Repeat(dash, 10)        # "----------"
/// Set space " "
/// Set indent Repeat(space, 4)      # "    "
/// ```
pub fn repeat(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::Number(n)) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(format!(
                    "Repeat count must be a non-negative integer, got {}",
                    n
                )));
            }
            let count = *n as usize;
            Ok(Value::String(s.repeat(count)))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 字符串切片
///
/// # 功能
/// 提取字符串的子串（基于字节索引）。
///
/// # 参数
/// - `string`: String - 原始字符串
/// - `start`: Number - 起始索引（包含，从0开始）
/// - `end`: Number - 结束索引（不包含）
///
/// # 返回值
/// String - 提取的子串
///
/// # 示例
/// ```aether
/// Set text "Hello World"
/// Set sub StrSlice(text, 0, 5)     # "Hello"
/// Set sub2 StrSlice(text, 6, 11)   # "World"
/// Set sub3 StrSlice(text, 0, 1)    # "H"
/// ```
pub fn substr(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::WrongArity {
            expected: 3,
            got: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::Number(start), Value::Number(end)) => {
            if start.fract() != 0.0 || end.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(
                    "String indices must be integers".to_string(),
                ));
            }

            let start_idx = *start as i64;
            let end_idx = *end as i64;
            let len = s.len() as i64;

            // 处理负数索引
            let start_idx = if start_idx < 0 {
                (len + start_idx).max(0)
            } else {
                start_idx.min(len)
            } as usize;

            let end_idx = if end_idx < 0 {
                (len + end_idx).max(0)
            } else {
                end_idx.min(len)
            } as usize;

            if start_idx > end_idx {
                return Ok(Value::String(String::new()));
            }

            // 使用字节切片
            let result = s.get(start_idx..end_idx).unwrap_or("").to_string();

            Ok(Value::String(result))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, Number, Number".to_string(),
            got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
        }),
    }
}

/// 获取字符串长度
///
/// # 功能
/// 返回字符串的字符数（注意：多字节字符按字符数计算）。
///
/// # 参数
/// - `string`: String - 要测量的字符串
///
/// # 返回值
/// Number - 字符串长度
///
/// # 示例
/// ```aether
/// Set text "Hello"
/// Set length StrLen(text)          # 5
/// Set chinese "你好"
/// Set length2 StrLen(chinese)      # 2
/// ```
pub fn strlen(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 查找子串位置
///
/// # 功能
/// 查找子串在字符串中首次出现的位置，未找到返回 -1。
///
/// # 参数
/// - `string`: String - 原始字符串
/// - `substring`: String - 要查找的子串
///
/// # 返回值
/// Number - 子串的起始位置（从0开始），未找到返回 -1
///
/// # 示例
/// ```aether
/// Set text "Hello World"
/// Set pos IndexOf(text, "World")   # 6
/// Set pos2 IndexOf(text, "xyz")    # -1
/// Set pos3 IndexOf(text, "l")      # 2 (第一个l)
/// ```
pub fn index_of(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(substr)) => match s.find(substr.as_str()) {
            Some(pos) => Ok(Value::Number(pos as f64)),
            None => Ok(Value::Number(-1.0)),
        },
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, String".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 获取指定位置的字符
///
/// # 功能
/// 获取字符串中指定索引位置的字符。
///
/// # 参数
/// - `string`: String - 原始字符串
/// - `index`: Number - 字符位置（从0开始）
///
/// # 返回值
/// String - 该位置的字符，索引越界返回空字符串
///
/// # 示例
/// ```aether
/// Set text "Hello"
/// Set ch CharAt(text, 0)           # "H"
/// Set ch2 CharAt(text, 4)          # "o"
/// Set ch3 CharAt(text, 10)         # ""
/// ```
pub fn char_at(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::Number(idx)) => {
            if idx.fract() != 0.0 {
                return Err(RuntimeError::InvalidOperation(
                    "Index must be an integer".to_string(),
                ));
            }

            let index = *idx as i64;
            let len = s.len() as i64;

            // 处理负数索引
            let index = if index < 0 {
                (len + index).max(0)
            } else {
                index.min(len)
            } as usize;

            let ch = s
                .chars()
                .nth(index)
                .map(|c| c.to_string())
                .unwrap_or_default();
            Ok(Value::String(ch))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}
