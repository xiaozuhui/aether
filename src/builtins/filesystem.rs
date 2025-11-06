// src/builtins/filesystem.rs
//! 文件系统IO操作函数

use crate::evaluator::RuntimeError;
use crate::value::Value;
use std::fs;
use std::path::Path;

/// 辅助函数：安全地获取字符串参数
fn get_string(val: &Value) -> Result<String, RuntimeError> {
    match val {
        Value::String(s) => Ok(s.clone()),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "String".to_string(),
            got: format!("{:?}", val),
        }),
    }
}

/// 读取文件内容
///
/// # 参数
/// - 文件路径
///
/// # 返回
/// 文件内容（字符串）
///
/// # 安全性
/// 需要启用文件系统权限
pub fn read_file(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let path = get_string(&args[0])?;

    match fs::read_to_string(&path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(RuntimeError::CustomError(format!(
            "Failed to read file '{}': {}",
            path, e
        ))),
    }
}

/// 写入文件内容（覆盖）
///
/// # 参数
/// - 文件路径
/// - 文件内容
///
/// # 返回
/// 成功返回 true
///
/// # 安全性
/// 需要启用文件系统权限
pub fn write_file(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let path = get_string(&args[0])?;
    let content = get_string(&args[1])?;

    match fs::write(&path, content) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(RuntimeError::CustomError(format!(
            "Failed to write file '{}': {}",
            path, e
        ))),
    }
}

/// 追加文件内容
///
/// # 参数
/// - 文件路径
/// - 追加内容
///
/// # 返回
/// 成功返回 true
///
/// # 安全性
/// 需要启用文件系统权限
pub fn append_file(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let path = get_string(&args[0])?;
    let content = get_string(&args[1])?;

    match fs::OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut file) => {
            use std::io::Write;
            match file.write_all(content.as_bytes()) {
                Ok(_) => Ok(Value::Boolean(true)),
                Err(e) => Err(RuntimeError::CustomError(format!(
                    "Failed to append to file '{}': {}",
                    path, e
                ))),
            }
        }
        Err(e) => Err(RuntimeError::CustomError(format!(
            "Failed to open file '{}': {}",
            path, e
        ))),
    }
}

/// 删除文件
///
/// # 参数
/// - 文件路径
///
/// # 返回
/// 成功返回 true
///
/// # 安全性
/// 需要启用文件系统权限
pub fn delete_file(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let path = get_string(&args[0])?;

    match fs::remove_file(&path) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(RuntimeError::CustomError(format!(
            "Failed to delete file '{}': {}",
            path, e
        ))),
    }
}

/// 检查文件是否存在
///
/// # 参数
/// - 文件路径
///
/// # 返回
/// 存在返回 true，不存在返回 false
pub fn file_exists(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let path = get_string(&args[0])?;
    Ok(Value::Boolean(Path::new(&path).exists()))
}

/// 列出目录内容
///
/// # 参数
/// - 目录路径
///
/// # 返回
/// 文件和目录名称的数组
///
/// # 安全性
/// 需要启用文件系统权限
pub fn list_dir(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let path = get_string(&args[0])?;

    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut items = Vec::new();
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if let Some(name) = e.file_name().to_str() {
                            items.push(Value::String(name.to_string()));
                        }
                    }
                    Err(e) => {
                        return Err(RuntimeError::CustomError(format!(
                            "Failed to read directory entry: {}",
                            e
                        )))
                    }
                }
            }
            Ok(Value::Array(items))
        }
        Err(e) => Err(RuntimeError::CustomError(format!(
            "Failed to list directory '{}': {}",
            path, e
        ))),
    }
}

/// 创建目录
///
/// # 参数
/// - 目录路径
///
/// # 返回
/// 成功返回 true
///
/// # 安全性
/// 需要启用文件系统权限
pub fn create_dir(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let path = get_string(&args[0])?;

    match fs::create_dir_all(&path) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(RuntimeError::CustomError(format!(
            "Failed to create directory '{}': {}",
            path, e
        ))),
    }
}
