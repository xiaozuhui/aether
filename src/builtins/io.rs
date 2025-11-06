// src/builtins/io.rs
//! I/O 内置函数模块
//!
//! 提供基础的输入输出功能，包括打印和读取用户输入。

use crate::evaluator::RuntimeError;
use crate::value::Value;
use std::io::{self, Write};

/// 打印值（不换行）
///
/// # 功能
/// 将一个或多个值输出到标准输出（stdout），不添加换行符。
/// 多个参数会用空格分隔。所有值会自动转换为字符串。
///
/// # 参数
/// - `values...`: 要打印的值（一个或多个，任意类型）
///
/// # 返回值
/// 返回 `Null`
///
/// # 示例
/// ```aether
/// Print("Hello")                    # 输出: Hello
/// Print("Result:", 42)              # 输出: Result: 42
/// Print("Sum:", 10, "+", 20, "=", 30)  # 输出: Sum: 10 + 20 = 30
/// Print([1, 2, 3])                  # 输出: [1, 2, 3]
/// ```
pub fn print(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Ok(Value::Null);
    }

    // 将所有参数转换为字符串并用空格连接
    let output = args
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{}", output);
    io::stdout().flush().unwrap();
    Ok(Value::Null)
}

/// 打印值（带换行）
///
/// # 功能
/// 将一个或多个值输出到标准输出（stdout），并在末尾添加换行符。
/// 多个参数会用空格分隔。所有值会自动转换为字符串。
///
/// # 参数
/// - `values...`: 要打印的值（一个或多个，任意类型）
///
/// # 返回值
/// 返回 `Null`
///
/// # 示例
/// ```aether
/// Println("Hello")                  # 输出: Hello\n
/// Println("Result:", 42)            # 输出: Result: 42\n
/// Println("x =", 10, "y =", 20)     # 输出: x = 10 y = 20\n
/// Println([1, 2, 3])                # 输出: [1, 2, 3]\n
/// ```
pub fn println(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        println!();
        return Ok(Value::Null);
    }

    // 将所有参数转换为字符串并用空格连接
    let output = args
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", output);
    Ok(Value::Null)
}

/// 读取用户输入
///
/// # 功能
/// 显示提示信息并读取用户从标准输入（stdin）输入的一行文本。
/// 自动去除行尾的换行符。
///
/// # 参数
/// - `prompt`: 提示信息（字符串）
///
/// # 返回值
/// 返回用户输入的字符串
///
/// # 示例
/// ```aether
/// Set NAME Input("请输入姓名: ")
/// Println("你好, " + NAME)
/// ```
pub fn input(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    // Print prompt
    print!("{}", args[0].to_string());
    io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| RuntimeError::InvalidOperation(format!("Failed to read input: {}", e)))?;

    // Remove trailing newline
    if buffer.ends_with('\n') {
        buffer.pop();
        if buffer.ends_with('\r') {
            buffer.pop();
        }
    }

    Ok(Value::String(buffer))
}
