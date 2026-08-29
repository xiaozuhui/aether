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
/// Set TEXT "apple,banana,cherry"
/// Set FRUITS SPLIT(TEXT, ",")      // ["apple", "banana", "cherry"]
/// Set SENTENCE "Hello World"
/// Set WORDS SPLIT(SENTENCE, " ")   // ["Hello", "World"]
/// Set CSV "a|b|c|d"
/// Set PARTS SPLIT(CSV, "|")        // ["a", "b", "c", "d"]
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
/// Set TEXT "hello world"
/// Set UPPER UPPER(TEXT)        // "HELLO WORLD"
/// Set MIXED "Hello123"
/// Set UPPER UPPER(MIXED)       // "HELLO123"
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
/// Set TEXT "HELLO WORLD"
/// Set LOWER LOWER(TEXT)        // "hello world"
/// Set MIXED "Hello123"
/// Set LOWER LOWER(MIXED)       // "hello123"
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
/// Set TEXT "  hello world  "
/// Set TRIMMED TRIM(TEXT)       // "hello world"
/// Set TEXT "\t\ntest\n\t"
/// Set TRIMMED TRIM(TEXT)       // "test"
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
/// Set TEXT "Hello World"
/// Set HAS CONTAINS(TEXT, "World")    // True
/// Set HAS CONTAINS(TEXT, "Python")   // False
/// Set EMAIL "user@example.com"
/// Set HAS CONTAINS(EMAIL, "@")       // True
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
    a.equals(b)
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
/// Set FILENAME "test.txt"
/// Set STARTS STARTS_WITH(FILENAME, "test")    // True
/// Set STARTS STARTS_WITH(FILENAME, "data")    // False
/// Set URL "https://example.com"
/// Set ISHTTPS STARTS_WITH(URL, "https://")    // True
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
/// Set FILENAME "document.pdf"
/// Set ISPDF ENDS_WITH(FILENAME, ".pdf")       // True
/// Set ISTXT ENDS_WITH(FILENAME, ".txt")       // False
/// Set EMAIL "user@gmail.com"
/// Set ISGMAIL ENDS_WITH(EMAIL, "@gmail.com")  // True
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
/// Set TEXT "Hello World"
/// Set REPLACED REPLACE(TEXT, "World", "Aether")  // "Hello Aether"
/// Set TEXT "foo bar foo"
/// Set REPLACED REPLACE(TEXT, "foo", "baz")       // "baz bar baz"
/// Set PATH "C:\\Users\\Name"
/// Set FIXED REPLACE(PATH, "\\", "/")             // "C:/Users/Name"
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
/// Set STR "Ha"
/// Set LAUGH REPEAT(STR, 3)         // "HaHaHa"
/// Set DASH "-"
/// Set LINE REPEAT(DASH, 10)        // "----------"
/// Set SPACE " "
/// Set INDENT REPEAT(SPACE, 4)      // "    "
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
/// 提取字符串的子串（基于 **Unicode 字符** 索引）。
///
/// # 参数
/// - `string`: String - 原始字符串
/// - `start`: Number - 起始索引（包含，从 0 开始；负数从尾部数）
/// - `end`: Number - 结束索引（不包含；负数从尾部数）
///
/// # 返回值
/// String - 提取的子串；越界端点钳制到范围内，start >= end 返回空串
///
/// # 示例
/// ```aether
/// Set TEXT "Hello World"
/// Set A STRSLICE(TEXT, 0, 5)      // "Hello"
/// Set B STRSLICE(TEXT, 6, 11)     // "World"
/// Set C STRSLICE("你好世界", 0, 2) // "你好"（字符语义，非字节）
/// Set D STRSLICE("你好世界", -2, 4) // "世界"
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

            let len = s.chars().count() as i64;
            // 负索引从尾部数（按字符），越界钳制到 [0, len]
            let clamp = |idx: i64| -> i64 {
                let i = if idx < 0 { len + idx } else { idx };
                i.clamp(0, len)
            };
            let start_idx = clamp(*start as i64);
            let end_idx = clamp(*end as i64);

            if start_idx >= end_idx {
                return Ok(Value::String(String::new()));
            }

            let result: String = s
                .chars()
                .skip(start_idx as usize)
                .take((end_idx - start_idx) as usize)
                .collect();
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
/// 返回字符串的**字符数**（多字节字符按 1 个字符计）。
///
/// # 参数
/// - `string`: String - 要测量的字符串
///
/// # 返回值
/// Number - 字符串长度
///
/// # 示例
/// ```aether
/// Set TEXT "Hello"
/// Set A STRLEN(TEXT)      // 5
/// Set B STRLEN("你好")    // 2（字符语义，非 6 字节）
/// ```
pub fn strlen(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 查找子串位置
///
/// # 功能
/// 查找子串在字符串中首次出现的**字符**位置，未找到返回 -1。
///
/// # 参数
/// - `string`: String - 原始字符串
/// - `substring`: String - 要查找的子串
///
/// # 返回值
/// Number - 子串起始字符位置（从 0 开始），未找到返回 -1
///
/// # 示例
/// ```aether
/// Set TEXT "Hello World"
/// Set A INDEXOF(TEXT, "World")  // 6
/// Set B INDEXOF(TEXT, "xyz")    // -1
/// Set C INDEXOF(TEXT, "l")      // 2（第一个 l）
/// Set D INDEXOF("héllo", "l")   // 2（字符位置）
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
            // find 返回字节偏移，换算为字符位置
            Some(byte_pos) => Ok(Value::Number(s[..byte_pos].chars().count() as f64)),
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
/// 获取字符串中指定**字符**位置的字符。
/// 负索引从尾部数（-1 是最后一个字符）；越界报错（与 `S[I]` 对齐）。
///
/// # 参数
/// - `string`: String - 原始字符串
/// - `index`: Number - 字符位置（从 0 开始）
///
/// # 返回值
/// String - 该位置的单个字符
///
/// # 错误
/// - 索引越界（含负索引超出尾部范围）
///
/// # 示例
/// ```aether
/// Set A CHARAT("Hello", 0)    // "H"
/// Set B CHARAT("Hello", -1)   // "o"（从尾部数）
/// Set C CHARAT("你好", -1)    // "好"
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

            let len = s.chars().count() as i64;
            let index = *idx as i64;
            // 负索引从尾部数（按字符）
            let index = if index < 0 { len + index } else { index };
            if index < 0 || index >= len {
                return Err(RuntimeError::InvalidOperation(format!(
                    "CharAt index {idx} out of range for string of {len} chars"
                )));
            }

            let ch = s.chars().nth(index as usize).expect("已校验范围内");
            Ok(Value::String(ch.to_string()))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String, Number".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}
