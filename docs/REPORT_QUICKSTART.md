# 报表功能快速开始

## 已完成功能 ✅

### 1. 数据格式化函数

以下格式化函数已实现并通过测试：

```aether
# 格式化数字（带千分位分隔符）
formatted = FORMAT_NUMBER(1234567.89, 2)  
# 结果: "1,234,567.89"

# 格式化货币
amount = FORMAT_CURRENCY(1234.56, "¥", 2)
# 结果: "¥1,234.56"

# 美元格式
usd = FORMAT_CURRENCY(1234.56, "$", 2)
# 结果: "$1,234.56"

# 格式化百分比
percent = FORMAT_PERCENT(0.1234, 2)
# 结果: "12.34%"
```

### 2. 报表模块骨架

完整的报表函数接口已定义在 `src/builtins/report.rs`：

**Excel 函数（45个）：**

- 读取：EXCEL_READ_SHEET, EXCEL_READ_CELL, EXCEL_READ_RANGE
- 写入：EXCEL_CREATE, EXCEL_WRITE_CELL, EXCEL_WRITE_ROW, EXCEL_SAVE
- 格式：EXCEL_SET_CELL_FORMAT, EXCEL_MERGE_CELLS
- 图表：EXCEL_ADD_BAR_CHART, EXCEL_ADD_LINE_CHART, EXCEL_ADD_PIE_CHART

**Word 函数（9个）：**

- 基础：WORD_CREATE, WORD_ADD_PARAGRAPH, WORD_ADD_HEADING, WORD_SAVE
- 模板：WORD_LOAD_TEMPLATE, WORD_FILL_TEMPLATE

**PDF 函数（5个）：**

- PDF_CREATE, PDF_ADD_PAGE, PDF_ADD_TEXT, PDF_SAVE

**数据处理函数（8个）：**

- PIVOT_TABLE, GROUP_BY, AGGREGATE, CROSS_TAB
- FORMAT_NUMBER, FORMAT_CURRENCY, FORMAT_PERCENT, FORMAT_DATE

## 使用示例

### 示例 1：基础格式化

```aether
# examples/report_demo.aether
amount = 1234567.89
PRINT("原始:", amount)
PRINT("格式化:", FORMAT_NUMBER(amount, 2))
PRINT("货币:", FORMAT_CURRENCY(amount, "¥", 2))
PRINT("百分比:", FORMAT_PERCENT(0.15, 1))
```

运行：

```bash
cargo run --release examples/report_demo.aether
```

### 示例 2：Excel 报表（待实现）

```aether
# 创建工作簿
workbook = EXCEL_CREATE()

# 写入数据
headers = ["姓名", "部门", "销售额"]
EXCEL_WRITE_ROW(workbook, "Sheet1", 0, headers)

employees = [
    ["张三", "销售部", 120000],
    ["李四", "市场部", 98000]
]

row = 1
FOR emp IN employees {
    EXCEL_WRITE_ROW(workbook, "Sheet1", row, emp)
    row = row + 1
}

# 保存
EXCEL_SAVE(workbook, "report.xlsx")
```

## 测试结果

```bash
$ cargo test report::tests --lib
running 3 tests
test builtins::report::tests::test_format_number ... ok
test builtins::report::tests::test_format_currency ... ok
test builtins::report::tests::test_format_percent ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```

## 下一步实现

### 阶段 1：Excel 基础功能（高优先级）

1. **添加依赖：**

   ```bash
   cargo add calamine        # Excel 读取
   cargo add rust_xlsxwriter # Excel 写入
   ```

2. **实现函数：**
   - EXCEL_CREATE - 创建工作簿
   - EXCEL_WRITE_CELL - 写入单元格
   - EXCEL_WRITE_ROW - 写入行
   - EXCEL_SAVE - 保存文件
   - EXCEL_READ_SHEET - 读取工作表

3. **句柄管理：**

   ```rust
   use lazy_static::lazy_static;
   use std::sync::{Arc, Mutex};
   
   lazy_static! {
       static ref WORKBOOK_HANDLES: Arc<Mutex<HashMap<usize, Workbook>>> = 
           Arc::new(Mutex::new(HashMap::new()));
   }
   ```

### 阶段 2：日期格式化（中优先级）

1. **添加依赖：**

   ```bash
   cargo add chrono
   ```

2. **实现函数：**
   - FORMAT_DATE - 格式化日期时间

### 阶段 3：Word 文档（中优先级）

1. **添加依赖：**

   ```bash
   cargo add docx-rs
   ```

2. **实现函数：**
   - WORD_CREATE, WORD_ADD_PARAGRAPH
   - WORD_ADD_HEADING, WORD_ADD_TABLE
   - WORD_SAVE

### 阶段 4：模板引擎（低优先级）

1. **添加依赖：**

   ```bash
   cargo add handlebars
   ```

2. **实现函数：**
   - TEMPLATE_RENDER
   - TEMPLATE_LOAD

### 阶段 5：数据透视（低优先级）

纯 Rust 实现，不需要额外依赖：

- PIVOT_TABLE
- GROUP_BY
- AGGREGATE
- CROSS_TAB

## 文档

- 📘 [报表指南](docs/REPORT_GUIDE.md) - 完整使用指南
- 📋 [实现计划](docs/REPORT_IMPLEMENTATION_PLAN.md) - 详细实现步骤
- 💡 [示例代码](examples/report_demo.aether) - 实际使用示例

## 关键设计决策

### 1. 句柄管理

Excel 工作簿、Word 文档等对象需要在多个函数调用间保持状态，使用全局句柄存储：

```rust
// 创建时分配句柄
fn excel_create() -> Result<Value, String> {
    let workbook = Workbook::new();
    let id = store_workbook(workbook);
    Ok(Value::Number(id as f64))
}

// 使用句柄
fn excel_write_cell(args: Vec<Value>) -> Result<Value, String> {
    let id = get_handle_id(&args[0])?;
    let workbook = get_workbook(id)?;
    // 使用 workbook...
}

// 保存并释放句柄
fn excel_save(args: Vec<Value>) -> Result<Value, String> {
    let id = get_handle_id(&args[0])?;
    let workbook = remove_workbook(id)?;
    workbook.save(path)?;
    Ok(Value::Boolean(true))
}
```

### 2. 错误处理

所有 IO 操作都返回 `Result<Value, String>`，便于错误传播：

```rust
fn excel_read_sheet(args: Vec<Value>) -> Result<Value, String> {
    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("参数必须是字符串".to_string()),
    };
    
    let workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| format!("打开失败: {}", e))?;
    
    // ...
}
```

### 3. 参数验证

所有函数都进行严格的参数类型和数量检查：

```rust
fn format_currency(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("FORMAT_CURRENCY 需要至少 1 个参数".to_string());
    }
    
    let amount = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("第一个参数必须是数字".to_string()),
    };
    
    // ...
}
```

## 性能考虑

- **大文件处理**：Excel 文件可能很大，考虑流式读取
- **内存管理**：及时释放不再使用的句柄
- **批量操作**：EXCEL_WRITE_TABLE 比多次 EXCEL_WRITE_CELL 更高效
- **缓存优化**：频繁访问的工作簿考虑缓存

## 兼容性

- ✅ Rust 1.70+
- ✅ macOS, Linux, Windows
- ⚠️ WASM 支持受限（某些文件 IO 库不支持）
- ✅ 多线程安全（使用 Mutex）

## 贡献

如果你想贡献报表功能的实现：

1. 选择一个功能模块（Excel/Word/PDF）
2. 添加相应的依赖
3. 实现函数并添加测试
4. 更新文档和示例
5. 提交 PR

## 总结

✅ **已完成：**

- 报表模块基础架构
- 数据格式化函数（3个）
- 完整的函数接口定义（70+ 个）
- 使用文档和示例
- 单元测试

⏳ **待完成：**

- Excel 读写实现
- Word 文档生成
- PDF 生成
- 日期格式化
- 模板引擎
- 数据透视表

🚀 **优先级：**

1. Excel 基础功能（最高）
2. 日期格式化（高）
3. Word 文档（中）
4. 模板引擎（低）
5. 数据透视表（低）

建议从 Excel 基础功能开始实现，这是最常用的报表功能。
