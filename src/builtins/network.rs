// src/builtins/network.rs
//! 网络IO操作函数

use crate::evaluator::RuntimeError;
use crate::value::Value;

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

/// HTTP GET 请求
///
/// # 参数
/// - URL
///
/// # 返回
/// 响应内容（字符串）
///
/// # 安全性
/// 需要启用网络权限
pub fn http_get(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let url = get_string(&args[0])?;

    // 使用 ureq 进行简单的 HTTP 请求
    match ureq::get(&url).call() {
        Ok(response) => match response.into_body().read_to_string() {
            Ok(body) => Ok(Value::String(body)),
            Err(e) => Err(RuntimeError::CustomError(format!(
                "Failed to read response body: {}",
                e
            ))),
        },
        Err(e) => Err(RuntimeError::CustomError(format!(
            "HTTP GET request failed: {}",
            e
        ))),
    }
}

/// HTTP POST 请求
///
/// # 参数
/// - URL
/// - 请求体（字符串）
/// - 可选：Content-Type（默认 "application/json"）
///
/// # 返回
/// 响应内容（字符串）
///
/// # 安全性
/// 需要启用网络权限
pub fn http_post(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let url = get_string(&args[0])?;
    let body = get_string(&args[1])?;
    let content_type = if args.len() > 2 {
        get_string(&args[2])?
    } else {
        "application/json".to_string()
    };

    match ureq::post(&url)
        .header("Content-Type", &content_type)
        .send(body.as_bytes())
    {
        Ok(response) => match response.into_body().read_to_string() {
            Ok(resp_body) => Ok(Value::String(resp_body)),
            Err(e) => Err(RuntimeError::CustomError(format!(
                "Failed to read response body: {}",
                e
            ))),
        },
        Err(e) => Err(RuntimeError::CustomError(format!(
            "HTTP POST request failed: {}",
            e
        ))),
    }
}

/// HTTP PUT 请求
///
/// # 参数
/// - URL
/// - 请求体（字符串）
/// - 可选：Content-Type（默认 "application/json"）
///
/// # 返回
/// 响应内容（字符串）
///
/// # 安全性
/// 需要启用网络权限
pub fn http_put(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let url = get_string(&args[0])?;
    let body = get_string(&args[1])?;
    let content_type = if args.len() > 2 {
        get_string(&args[2])?
    } else {
        "application/json".to_string()
    };

    match ureq::put(&url)
        .header("Content-Type", &content_type)
        .send(body.as_bytes())
    {
        Ok(response) => match response.into_body().read_to_string() {
            Ok(resp_body) => Ok(Value::String(resp_body)),
            Err(e) => Err(RuntimeError::CustomError(format!(
                "Failed to read response body: {}",
                e
            ))),
        },
        Err(e) => Err(RuntimeError::CustomError(format!(
            "HTTP PUT request failed: {}",
            e
        ))),
    }
}

/// HTTP DELETE 请求
///
/// # 参数
/// - URL
///
/// # 返回
/// 响应内容（字符串）
///
/// # 安全性
/// 需要启用网络权限
pub fn http_delete(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let url = get_string(&args[0])?;

    match ureq::delete(&url).call() {
        Ok(response) => match response.into_body().read_to_string() {
            Ok(body) => Ok(Value::String(body)),
            Err(e) => Err(RuntimeError::CustomError(format!(
                "Failed to read response body: {}",
                e
            ))),
        },
        Err(e) => Err(RuntimeError::CustomError(format!(
            "HTTP DELETE request failed: {}",
            e
        ))),
    }
}
