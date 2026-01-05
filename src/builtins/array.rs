// src/builtins/array.rs
//! Array manipulation built-in functions

use crate::evaluator::RuntimeError;
use crate::value::Value;

/// 生成数字范围数组
///
/// # 功能
/// 生成一个数字序列数组，支持三种调用方式：
/// - `Range(end)`: 生成从 0 到 end（不含）的数组
/// - `Range(start, end)`: 生成从 start 到 end（不含）的数组
/// - `Range(start, end, step)`: 生成从 start 到 end（不含），步长为 step 的数组
///
/// # 参数
/// - `end`: Number - 结束值（不包含），当只有一个参数时
/// - `start`: Number - 起始值（包含），当有两个或三个参数时
/// - `end`: Number - 结束值（不包含）
/// - `step`: Number - 步长（可选，默认为 1），可以为负数
///
/// # 返回值
/// Array - 包含生成的数字序列的数组
///
/// # 错误
/// - 步长为 0 时抛出错误
/// - 参数类型不是 Number 时抛出类型错误
///
/// # 示例
/// ```aether
/// Set nums Range(5)           # [0, 1, 2, 3, 4]
/// Set nums Range(2, 8)        # [2, 3, 4, 5, 6, 7]
/// Set nums Range(0, 10, 2)    # [0, 2, 4, 6, 8]
/// Set nums Range(10, 0, -2)   # [10, 8, 6, 4, 2]
/// ```
pub fn range(args: &[Value]) -> Result<Value, RuntimeError> {
    let (start, end, step) = match args.len() {
        1 => {
            // range(end) -> 0..end
            match &args[0] {
                Value::Number(n) => (0.0, *n, 1.0),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "Number".to_string(),
                        got: format!("{:?}", args[0]),
                    });
                }
            }
        }
        2 => {
            // range(start, end) -> start..end
            match (&args[0], &args[1]) {
                (Value::Number(s), Value::Number(e)) => (*s, *e, 1.0),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "Number, Number".to_string(),
                        got: format!("{:?}, {:?}", args[0], args[1]),
                    });
                }
            }
        }
        3 => {
            // range(start, end, step) -> start..end by step
            match (&args[0], &args[1], &args[2]) {
                (Value::Number(s), Value::Number(e), Value::Number(st)) => (*s, *e, *st),
                _ => {
                    return Err(RuntimeError::TypeErrorDetailed {
                        expected: "Number, Number, Number".to_string(),
                        got: format!("{:?}, {:?}, {:?}", args[0], args[1], args[2]),
                    });
                }
            }
        }
        n => {
            return Err(RuntimeError::WrongArity {
                expected: 1,
                got: n,
            });
        }
    };

    if step == 0.0 {
        return Err(RuntimeError::InvalidOperation(
            "Range step cannot be zero".to_string(),
        ));
    }

    let mut result = Vec::new();
    let mut current = start;

    if step > 0.0 {
        while current < end {
            result.push(Value::Number(current));
            current += step;
        }
    } else {
        while current > end {
            result.push(Value::Number(current));
            current += step;
        }
    }

    Ok(Value::Array(result))
}

/// 添加元素到数组末尾
///
/// # 功能
/// 在数组末尾添加一个新元素，返回新的数组。原数组不会被修改。
///
/// # 参数
/// - `array`: Array - 原始数组
/// - `value`: Any - 要添加的元素（任意类型）
///
/// # 返回值
/// Array - 包含新元素的新数组
///
/// # 示例
/// ```aether
/// Set arr [1, 2, 3]
/// Set newArr Push(arr, 4)     # [1, 2, 3, 4]
/// Println(arr)                # [1, 2, 3] (原数组不变)
/// ```
pub fn push(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Ok(Value::Array(new_arr))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 移除并返回数组的最后一个元素
///
/// # 功能
/// 移除数组的最后一个元素，返回一个包含两个元素的数组：
/// - 第一个元素：移除最后元素后的新数组
/// - 第二个元素：被移除的元素
///
/// # 参数
/// - `array`: Array - 要操作的数组
///
/// # 返回值
/// Array - `[新数组, 被移除的元素]`
///
/// # 错误
/// - 空数组时抛出错误
///
/// # 示例
/// ```aether
/// Set arr [1, 2, 3, 4]
/// Set result Pop(arr)          # [[1, 2, 3], 4]
/// Set newArr result[0]         # [1, 2, 3]
/// Set popped result[1]         # 4
/// ```
pub fn pop(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(RuntimeError::InvalidOperation(
                    "Cannot pop from empty array".to_string(),
                ));
            }
            let mut new_arr = arr.clone();
            let popped = new_arr.pop().unwrap();
            Ok(Value::Array(vec![Value::Array(new_arr), popped]))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 将数组元素连接成字符串
///
/// # 功能
/// 将数组中的所有元素转换为字符串，并用指定的分隔符连接。
///
/// # 参数
/// - `array`: Array - 要连接的数组
/// - `separator`: String - 分隔符
///
/// # 返回值
/// String - 连接后的字符串
///
/// # 示例
/// ```aether
/// Set arr [1, 2, 3, 4]
/// Set str Join(arr, ", ")      # "1, 2, 3, 4"
/// Set str Join(arr, "-")       # "1-2-3-4"
/// Set words ["Hello", "World"]
/// Set str Join(words, " ")     # "Hello World"
/// ```
pub fn join(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::String(sep)) => {
            let strings: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
            Ok(Value::String(strings.join(sep)))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array, String".to_string(),
            got: format!("{:?}, {:?}", args[0], args[1]),
        }),
    }
}

/// 反转数组
///
/// # 功能
/// 返回一个新数组，元素顺序与原数组相反。原数组不会被修改。
///
/// # 参数
/// - `array`: Array - 要反转的数组
///
/// # 返回值
/// Array - 反转后的新数组
///
/// # 示例
/// ```aether
/// Set arr [1, 2, 3, 4, 5]
/// Set reversed Reverse(arr)    # [5, 4, 3, 2, 1]
/// Println(arr)                 # [1, 2, 3, 4, 5] (原数组不变)
/// ```
pub fn reverse(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.reverse();
            Ok(Value::Array(new_arr))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 对数字数组进行排序
///
/// # 功能
/// 对数字数组进行升序排序，返回新的已排序数组。原数组不会被修改。
///
/// # 参数
/// - `array`: Array - 要排序的数字数组
///
/// # 返回值
/// Array - 升序排列的新数组
///
/// # 错误
/// - 数组包含非数字元素时抛出类型错误
///
/// # 示例
/// ```aether
/// Set nums [3, 1, 4, 1, 5, 9, 2, 6]
/// Set sorted Sort(nums)        # [1, 1, 2, 3, 4, 5, 6, 9]
/// Println(nums)                # [3, 1, 4, 1, 5, 9, 2, 6] (原数组不变)
/// ```
pub fn sort(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let mut numbers: Vec<f64> = Vec::new();
            for val in arr {
                match val {
                    Value::Number(n) => numbers.push(*n),
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        });
                    }
                }
            }
            numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Ok(Value::Array(
                numbers.into_iter().map(Value::Number).collect(),
            ))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 计算数字数组的总和
///
/// # 功能
/// 计算数字数组中所有元素的和。
///
/// # 参数
/// - `array`: Array - 数字数组
///
/// # 返回值
/// Number - 数组元素的总和
///
/// # 错误
/// - 数组包含非数字元素时抛出类型错误
///
/// # 示例
/// ```aether
/// Set nums [1, 2, 3, 4, 5]
/// Set total Sum(nums)          # 15
/// Set prices [10.5, 20.0, 5.5]
/// Set total Sum(prices)        # 36.0
/// ```
pub fn sum(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let mut total = 0.0;
            for val in arr {
                match val {
                    Value::Number(n) => total += n,
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        });
                    }
                }
            }
            Ok(Value::Number(total))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 获取数组中的最大值
///
/// # 功能
/// 返回数字数组中的最大值。
///
/// # 参数
/// - `array`: Array - 数字数组
///
/// # 返回值
/// Number - 数组中的最大值
///
/// # 错误
/// - 空数组时抛出错误
/// - 数组包含非数字元素时抛出类型错误
///
/// # 示例
/// ```aether
/// Set nums [3, 7, 2, 9, 1]
/// Set maximum Max(nums)        # 9
/// Set scores [85.5, 92.0, 78.5, 95.5]
/// Set highest Max(scores)      # 95.5
/// ```
pub fn max(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(RuntimeError::InvalidOperation(
                    "Cannot get max of empty array".to_string(),
                ));
            }

            let mut max_val = f64::NEG_INFINITY;
            for val in arr {
                match val {
                    Value::Number(n) => {
                        if *n > max_val {
                            max_val = *n;
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        });
                    }
                }
            }
            Ok(Value::Number(max_val))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// 获取数组中的最小值
///
/// # 功能
/// 返回数字数组中的最小值。
///
/// # 参数
/// - `array`: Array - 数字数组
///
/// # 返回值
/// Number - 数组中的最小值
///
/// # 错误
/// - 空数组时抛出错误
/// - 数组包含非数字元素时抛出类型错误
///
/// # 示例
/// ```aether
/// Set nums [3, 7, 2, 9, 1]
/// Set minimum Min(nums)        # 1
/// Set temps [18.5, 22.0, 15.5, 20.5]
/// Set lowest Min(temps)        # 15.5
/// ```
pub fn min(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(RuntimeError::InvalidOperation(
                    "Cannot get min of empty array".to_string(),
                ));
            }

            let mut min_val = f64::INFINITY;
            for val in arr {
                match val {
                    Value::Number(n) => {
                        if *n < min_val {
                            min_val = *n;
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeErrorDetailed {
                            expected: "Array of Numbers".to_string(),
                            got: format!("Array containing {:?}", val),
                        });
                    }
                }
            }
            Ok(Value::Number(min_val))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// Map 函数
///
/// # 功能
/// 将函数应用到数组的每个元素，返回新数组。
///
/// # 参数
/// - `array`: Array - 输入数组
/// - `func`: Function - 转换函数
///
/// # 返回值
/// Array - 转换后的新数组
///
/// # 注意
/// 此函数期望在求值器上下文中调用，但由于实现限制，
/// 建议在 Aether 标准库中使用循环来实现 map 功能。
///
/// # 示例
/// ```aether
/// Set doubled Map([1, 2, 3], Fun(x) { Return x * 2 })  # [2, 4, 6]
/// ```
pub fn map(_args: &[Value]) -> Result<Value, RuntimeError> {
    // 注意：真正的 map 实现应该在求值器层面，因为需要调用用户定义的函数
    // 这里提供一个占位符实现，建议在 stdlib 中实现
    Err(RuntimeError::InvalidOperation(
        "MAP requires function evaluation context. Use stdlib implementation or manual loops instead.".to_string(),
    ))
}

/// Filter 函数
///
/// # 功能
/// 筛选数组中满足条件的元素，返回新数组。
///
/// # 参数
/// - `array`: Array - 输入数组
/// - `predicate`: Function - 判断函数，返回布尔值
///
/// # 返回值
/// Array - 筛选后的新数组
///
/// # 注意
/// 此函数期望在求值器上下文中调用，但由于实现限制，
/// 建议在 Aether 标准库中使用循环来实现 filter 功能。
///
/// # 示例
/// ```aether
/// Set evens Filter([1, 2, 3, 4], Fun(x) { Return x % 2 == 0 })  # [2, 4]
/// ```
pub fn filter(_args: &[Value]) -> Result<Value, RuntimeError> {
    // 注意：真正的 filter 实现应该在求值器层面，因为需要调用用户定义的函数
    // 这里提供一个占位符实现，建议在 stdlib 中实现
    Err(RuntimeError::InvalidOperation(
        "FILTER requires function evaluation context. Use stdlib implementation or manual loops instead.".to_string(),
    ))
}

/// Reduce 函数（占位符）
///
/// # 功能
/// 此函数为占位符，实际的 Reduce 功能由求值器（evaluator）实现。
/// Reduce 用于将数组归约为单个值。
///
/// # 注意
/// 不应直接调用此函数，应使用语言层面的 Reduce 语法。
///
/// # 示例
/// ```aether
/// # 实际使用（由求值器处理）:
/// Set sum Reduce([1, 2, 3, 4], 0, Fun(acc, x) { Return acc + x })  # 10
/// ```
pub fn reduce(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::InvalidOperation(
        "Reduce requires function evaluation context - use evaluator's reduce implementation"
            .to_string(),
    ))
}
