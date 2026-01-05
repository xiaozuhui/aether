// src/builtins/report.rs
//! 报表生成和文档处理函数

use crate::evaluator::RuntimeError;
use crate::value::Value;
use lazy_static::lazy_static;
use rust_xlsxwriter::Workbook;
use std::collections::HashMap;
use std::sync::Mutex;

// ============================================================================
// 全局状态管理
// ============================================================================

lazy_static! {
    static ref WORKBOOK_HANDLES: Mutex<HashMap<usize, Workbook>> = Mutex::new(HashMap::new());
    static ref NEXT_WORKBOOK_ID: Mutex<usize> = Mutex::new(0);
    // 记录每个工作簿中已创建的工作表：(workbook_id, sheet_name) -> sheet_index
    static ref WORKSHEET_INDICES: Mutex<HashMap<(usize, String), usize>> = Mutex::new(HashMap::new());
}

/// 分配新的工作簿 ID
fn allocate_workbook_id() -> usize {
    let mut id = NEXT_WORKBOOK_ID.lock().unwrap();
    let current_id = *id;
    *id += 1;
    current_id
}

/// 存储工作簿并返回句柄 ID
fn store_workbook(workbook: Workbook) -> usize {
    let id = allocate_workbook_id();
    let mut handles = WORKBOOK_HANDLES.lock().unwrap();
    handles.insert(id, workbook);
    id
}

/// 移除并返回工作簿（用于保存）
fn take_workbook(id: usize) -> Result<Workbook, RuntimeError> {
    let mut handles = WORKBOOK_HANDLES
        .lock()
        .map_err(|e| RuntimeError::CustomError(format!("无法获取工作簿锁: {}", e)))?;
    handles
        .remove(&id)
        .ok_or_else(|| RuntimeError::CustomError(format!("工作簿句柄 {} 不存在", id)))
}

/// 获取工作簿的可变引用
fn get_workbook_mut(id: usize) -> Result<&'static mut Workbook, RuntimeError> {
    let mut handles = WORKBOOK_HANDLES
        .lock()
        .map_err(|e| RuntimeError::CustomError(format!("无法获取工作簿锁: {}", e)))?;

    let workbook_ptr = handles
        .get_mut(&id)
        .ok_or_else(|| RuntimeError::CustomError(format!("工作簿句柄 {} 不存在", id)))?
        as *mut Workbook;

    Ok(unsafe { &mut *workbook_ptr })
}

// ============================================================================
// Excel 函数
// ============================================================================

/// 创建新的 Excel 工作簿
pub fn excel_create(_args: &[Value]) -> Result<Value, RuntimeError> {
    let workbook = Workbook::new();
    let id = store_workbook(workbook);
    Ok(Value::Number(id as f64))
}

/// 写入单个单元格
pub fn excel_write_cell(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 5 {
        return Err(RuntimeError::WrongArity {
            expected: 5,
            got: args.len(),
        });
    }

    let id = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            });
        }
    };

    let sheet_name = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "String".to_string(),
                got: format!("{:?}", args[1]),
            });
        }
    };

    let row = match &args[2] {
        Value::Number(n) => *n as u32,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[2]),
            });
        }
    };

    let col = match &args[3] {
        Value::Number(n) => *n as u16,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[3]),
            });
        }
    };

    // 获取或创建工作表索引
    let sheet_key = (id, sheet_name.clone());
    let mut indices = WORKSHEET_INDICES
        .lock()
        .map_err(|e| RuntimeError::CustomError(format!("无法获取工作表索引锁: {}", e)))?;

    let sheet_index = if let Some(&idx) = indices.get(&sheet_key) {
        idx
    } else {
        // 第一次访问此工作表，需要创建
        let workbook = get_workbook_mut(id)?;
        let _worksheet = workbook
            .add_worksheet()
            .set_name(&sheet_name)
            .map_err(|e| RuntimeError::CustomError(format!("设置工作表名称失败: {}", e)))?;

        // 获取工作表索引（工作表按添加顺序索引）
        let idx = indices.len(); // 使用已有工作表数量作为新索引
        indices.insert(sheet_key.clone(), idx);
        idx
    };

    drop(indices); // 释放锁

    // 现在使用索引获取工作表并写入数据
    let workbook = get_workbook_mut(id)?;
    let worksheet = workbook
        .worksheets_mut()
        .get_mut(sheet_index)
        .ok_or_else(|| RuntimeError::CustomError(format!("工作表索引 {} 不存在", sheet_index)))?;

    match &args[4] {
        Value::Number(n) => {
            worksheet
                .write_number(row, col, *n)
                .map_err(|e| RuntimeError::CustomError(format!("写入数字失败: {}", e)))?;
        }
        Value::String(s) => {
            worksheet
                .write_string(row, col, s)
                .map_err(|e| RuntimeError::CustomError(format!("写入字符串失败: {}", e)))?;
        }
        Value::Boolean(b) => {
            worksheet
                .write_boolean(row, col, *b)
                .map_err(|e| RuntimeError::CustomError(format!("写入布尔值失败: {}", e)))?;
        }
        _ => {
            let s = args[4].to_string();
            worksheet
                .write_string(row, col, &s)
                .map_err(|e| RuntimeError::CustomError(format!("写入数据失败: {}", e)))?;
        }
    }

    Ok(Value::Null)
}

/// 保存 Excel 文件
pub fn excel_save(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }

    let id = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            });
        }
    };

    let file_path = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "String".to_string(),
                got: format!("{:?}", args[1]),
            });
        }
    };

    let mut workbook = take_workbook(id)?;
    workbook
        .save(&file_path)
        .map_err(|e| RuntimeError::CustomError(format!("保存Excel文件失败: {}", e)))?;

    Ok(Value::Boolean(true))
}

// ============================================================================
// 格式化函数
// ============================================================================

/// 格式化数字
pub fn format_number(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let number = match &args[0] {
        Value::Number(n) => *n,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            });
        }
    };

    let decimals = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => *n as usize,
            _ => 2,
        }
    } else {
        2
    };

    let use_separator = if args.len() > 2 {
        match &args[2] {
            Value::Boolean(b) => *b,
            _ => true,
        }
    } else {
        true
    };

    let formatted = format!("{:.prec$}", number, prec = decimals);

    if use_separator {
        let parts: Vec<&str> = formatted.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

        let (sign, abs_integer) = if let Some(stripped) = integer_part.strip_prefix('-') {
            ("-", stripped)
        } else {
            ("", integer_part)
        };

        let chars: Vec<char> = abs_integer.chars().collect();
        let mut result = String::new();
        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i).is_multiple_of(3) {
                result.push(',');
            }
            result.push(*c);
        }

        let final_result = if decimal_part.is_empty() {
            format!("{}{}", sign, result)
        } else {
            format!("{}{}.{}", sign, result, decimal_part)
        };

        Ok(Value::String(final_result))
    } else {
        Ok(Value::String(formatted))
    }
}

/// 格式化货币
pub fn format_currency(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let amount = match &args[0] {
        Value::Number(n) => *n,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            });
        }
    };

    let symbol = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => "¥".to_string(),
        }
    } else {
        "¥".to_string()
    };

    let decimals = if args.len() > 2 {
        match &args[2] {
            Value::Number(n) => *n as usize,
            _ => 2,
        }
    } else {
        2
    };

    let formatted_amount = format_number(&[
        Value::Number(amount),
        Value::Number(decimals as f64),
        Value::Boolean(true),
    ])?;

    let result = match formatted_amount {
        Value::String(s) => format!("{}{}", symbol, s),
        _ => return Err(RuntimeError::CustomError("格式化失败".to_string())),
    };

    Ok(Value::String(result))
}

/// 格式化百分比
pub fn format_percent(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
        });
    }

    let number = match &args[0] {
        Value::Number(n) => *n * 100.0,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            });
        }
    };

    let decimals = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => *n as usize,
            _ => 2,
        }
    } else {
        2
    };

    Ok(Value::String(format!(
        "{:.prec$}%",
        number,
        prec = decimals
    )))
}

/// 格式化日期（占位符）
pub fn format_date(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "FORMAT_DATE 功能尚未实现".to_string(),
    ))
}

// ============================================================================
// 占位符函数
// ============================================================================

pub fn excel_read_sheet(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "EXCEL_READ_SHEET 功能尚未实现".to_string(),
    ))
}

pub fn excel_read_cell(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "EXCEL_READ_CELL 功能尚未实现".to_string(),
    ))
}

pub fn excel_read_range(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "EXCEL_READ_RANGE 功能尚未实现".to_string(),
    ))
}

pub fn excel_get_sheets(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "EXCEL_GET_SHEETS 功能尚未实现".to_string(),
    ))
}

pub fn excel_write_row(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "EXCEL_WRITE_ROW 功能尚未实现".to_string(),
    ))
}

pub fn excel_write_column(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "EXCEL_WRITE_COLUMN 功能尚未实现".to_string(),
    ))
}

pub fn excel_write_table(_args: &[Value]) -> Result<Value, RuntimeError> {
    Err(RuntimeError::CustomError(
        "EXCEL_WRITE_TABLE 功能尚未实现".to_string(),
    ))
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        let result = format_number(&[Value::Number(1234.5678)]).unwrap();
        assert_eq!(result, Value::String("1,234.57".to_string()));

        let result = format_number(&[Value::Number(1234.5678), Value::Number(3.0)]).unwrap();
        assert_eq!(result, Value::String("1,234.568".to_string()));

        let result = format_number(&[
            Value::Number(1234.5678),
            Value::Number(2.0),
            Value::Boolean(false),
        ])
        .unwrap();
        assert_eq!(result, Value::String("1234.57".to_string()));
    }

    #[test]
    fn test_format_currency() {
        let result = format_currency(&[Value::Number(1234.56)]).unwrap();
        assert_eq!(result, Value::String("¥1,234.56".to_string()));

        let result =
            format_currency(&[Value::Number(1234.56), Value::String("$".to_string())]).unwrap();
        assert_eq!(result, Value::String("$1,234.56".to_string()));
    }

    #[test]
    fn test_format_percent() {
        let result = format_percent(&[Value::Number(0.1234)]).unwrap();
        assert_eq!(result, Value::String("12.34%".to_string()));

        let result = format_percent(&[Value::Number(0.1234), Value::Number(1.0)]).unwrap();
        assert_eq!(result, Value::String("12.3%".to_string()));
    }

    #[test]
    fn test_excel_create() {
        let result = excel_create(&[]).unwrap();
        match result {
            Value::Number(n) => assert!(n >= 0.0),
            _ => panic!("Expected Number"),
        }
    }
}
