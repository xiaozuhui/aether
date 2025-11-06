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
        (Value::String(s), Value::String(substr)) => {
            Ok(Value::Boolean(s.contains(substr.as_str())))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, String".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
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
