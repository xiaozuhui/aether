# 报表功能依赖项规划

本文档说明为实现完整的报表功能需要添加的 Rust 依赖项。

## 当前状态

✅ **已实现的功能：**

- 报表模块骨架 (`src/builtins/report.rs`)
- 数据格式化函数（FORMAT_NUMBER, FORMAT_CURRENCY, FORMAT_PERCENT）
- 单元测试

⏳ **待实现的功能：**

- Excel 读写
- Word 文档生成
- PDF 生成
- 日期格式化
- 模板引擎
- 数据透视表

## 需要添加的依赖

将以下依赖添加到 `Cargo.toml` 的 `[dependencies]` 部分：

```toml
# Excel 读取
calamine = "0.24"

# Excel 写入
rust_xlsxwriter = "0.60"

# Word 文档
docx-rs = "0.4"

# PDF 生成
printpdf = "0.7"

# 日期时间
chrono = "0.4"

# 模板引擎
handlebars = "5.1"

# 序列化（可能已有）
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 实现优先级

### 第一阶段：Excel 基础功能 (高优先级)

**依赖：** `calamine`, `rust_xlsxwriter`

**功能：**

- ✅ EXCEL_CREATE - 创建工作簿
- ✅ EXCEL_WRITE_CELL - 写入单元格
- ✅ EXCEL_WRITE_ROW - 写入行
- ✅ EXCEL_WRITE_TABLE - 写入表格
- ✅ EXCEL_SAVE - 保存文件
- ✅ EXCEL_READ_SHEET - 读取工作表
- ✅ EXCEL_READ_CELL - 读取单元格

**代码示例：**

```rust
// 使用 rust_xlsxwriter 创建 Excel
use rust_xlsxwriter::*;

fn excel_create(_args: Vec<Value>) -> Result<Value, String> {
    let workbook = Workbook::new();
    // 存储 workbook 句柄并返回 ID
    Ok(Value::Number(workbook_id as f64))
}

// 使用 calamine 读取 Excel
use calamine::{Reader, open_workbook, Xlsx};

fn excel_read_sheet(args: Vec<Value>) -> Result<Value, String> {
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("参数必须是字符串".to_string()),
    };
    
    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| format!("打开文件失败: {}", e))?;
    
    // 读取数据...
    Ok(Value::Array(data))
}
```

### 第二阶段：数据格式化 (中优先级)

**依赖：** `chrono`

**功能：**

- ✅ FORMAT_NUMBER - 已实现
- ✅ FORMAT_CURRENCY - 已实现
- ✅ FORMAT_PERCENT - 已实现
- ⏳ FORMAT_DATE - 需要 chrono

**代码示例：**

```rust
use chrono::{DateTime, Utc, NaiveDateTime};

fn format_date(args: Vec<Value>) -> Result<Value, String> {
    let timestamp = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => return Err("第一个参数必须是时间戳".to_string()),
    };
    
    let format = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => "%Y-%m-%d".to_string(),
        }
    } else {
        "%Y-%m-%d".to_string()
    };
    
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .ok_or("无效的时间戳")?;
    
    Ok(Value::String(datetime.format(&format).to_string()))
}
```

### 第三阶段：Word 文档 (中优先级)

**依赖：** `docx-rs`

**功能：**

- ⏳ WORD_CREATE
- ⏳ WORD_ADD_PARAGRAPH
- ⏳ WORD_ADD_HEADING
- ⏳ WORD_ADD_TABLE
- ⏳ WORD_SAVE

**代码示例：**

```rust
use docx_rs::*;

fn word_create(_args: Vec<Value>) -> Result<Value, String> {
    let doc = Docx::new();
    // 存储文档句柄
    Ok(Value::Number(doc_id as f64))
}

fn word_add_paragraph(args: Vec<Value>) -> Result<Value, String> {
    let doc_id = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => return Err("第一个参数必须是文档句柄".to_string()),
    };
    
    let text = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("第二个参数必须是文本".to_string()),
    };
    
    // 获取文档并添加段落
    // doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&text)));
    
    Ok(Value::Null)
}
```

### 第四阶段：模板引擎 (低优先级)

**依赖：** `handlebars`

**功能：**

- ⏳ TEMPLATE_RENDER
- ⏳ TEMPLATE_LOAD

**代码示例：**

```rust
use handlebars::Handlebars;
use serde_json::json;

fn template_render(args: Vec<Value>) -> Result<Value, String> {
    let template = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("第一个参数必须是模板字符串".to_string()),
    };
    
    let variables = match &args[1] {
        Value::Dict(d) => d.clone(),
        _ => return Err("第二个参数必须是字典".to_string()),
    };
    
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("template", template)
        .map_err(|e| format!("模板错误: {}", e))?;
    
    // 转换 variables 为 JSON
    let data = json!(variables);
    
    let result = handlebars.render("template", &data)
        .map_err(|e| format!("渲染错误: {}", e))?;
    
    Ok(Value::String(result))
}
```

### 第五阶段：PDF 生成 (低优先级)

**依赖：** `printpdf`

**功能：**

- ⏳ PDF_CREATE
- ⏳ PDF_ADD_PAGE
- ⏳ PDF_ADD_TEXT
- ⏳ PDF_SAVE

### 第六阶段：数据透视 (低优先级)

**依赖：** 无额外依赖

**功能：**

- ⏳ PIVOT_TABLE
- ⏳ GROUP_BY
- ⏳ AGGREGATE
- ⏳ CROSS_TAB

**说明：** 这些功能可以用纯 Rust 实现，不需要外部库。

## 句柄管理设计

由于 Excel 工作簿、Word 文档等对象需要在多个函数调用间保持状态，我们需要一个句柄管理系统：

```rust
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

// 全局句柄存储
lazy_static! {
    static ref WORKBOOK_HANDLES: Arc<Mutex<HashMap<usize, Workbook>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    
    static ref WORD_HANDLES: Arc<Mutex<HashMap<usize, Docx>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

// 创建工作簿时分配句柄
fn excel_create(_args: Vec<Value>) -> Result<Value, String> {
    let workbook = Workbook::new();
    let mut handles = WORKBOOK_HANDLES.lock().unwrap();
    let id = handles.len();
    handles.insert(id, workbook);
    Ok(Value::Number(id as f64))
}

// 使用句柄
fn excel_write_cell(args: Vec<Value>) -> Result<Value, String> {
    let id = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => return Err("句柄无效".to_string()),
    };
    
    let mut handles = WORKBOOK_HANDLES.lock().unwrap();
    let workbook = handles.get_mut(&id)
        .ok_or("工作簿句柄不存在")?;
    
    // 使用 workbook...
    Ok(Value::Null)
}

// 保存并释放句柄
fn excel_save(args: Vec<Value>) -> Result<Value, String> {
    let id = match &args[0] {
        Value::Number(n) => *n as usize,
        _ => return Err("句柄无效".to_string()),
    };
    
    let path = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("路径必须是字符串".to_string()),
    };
    
    let mut handles = WORKBOOK_HANDLES.lock().unwrap();
    let workbook = handles.remove(&id)
        .ok_or("工作簿句柄不存在")?;
    
    workbook.save(&path)
        .map_err(|e| format!("保存失败: {}", e))?;
    
    Ok(Value::Boolean(true))
}
```

## 添加依赖的命令

```bash
# Excel 支持
cargo add calamine
cargo add rust_xlsxwriter

# 日期时间
cargo add chrono

# 如果需要完整功能
cargo add docx-rs
cargo add printpdf
cargo add handlebars
```

## 测试策略

每个功能实现后应添加：

1. **单元测试** - 在 `src/builtins/report.rs` 中
2. **集成测试** - 在 `tests/report_tests.rs` 中
3. **示例** - 在 `examples/report_*.aether` 中

## 文档更新

实现功能后需要更新：

1. ✅ `docs/REPORT_GUIDE.md` - 已创建
2. ✅ `README.md` - 已更新
3. ✅ `examples/report_demo.aether` - 已创建
4. ⏳ 在线帮助系统 - 需要在 `src/builtins/help.rs` 中注册

## 下一步行动

1. **决定优先级**：确定最先实现哪个功能模块
2. **添加依赖**：运行 `cargo add` 命令
3. **实现核心功能**：从 Excel 基础功能开始
4. **编写测试**：确保功能正确性
5. **更新文档**：同步更新使用示例

## 兼容性考虑

- **Rust 版本**：确保依赖支持当前的 Rust 版本
- **WASM 支持**：某些库可能不支持 WASM 编译
- **平台差异**：文件路径处理在不同操作系统上可能有差异
- **性能影响**：大型 Excel 文件处理可能消耗大量内存

## 总结

报表功能的骨架已经搭建完成，格式化函数已经实现并测试通过。要完整实现 Excel、Word、PDF 功能，需要添加相应的依赖并实现句柄管理系统。建议按优先级逐步实现，先完成最常用的 Excel 功能。
