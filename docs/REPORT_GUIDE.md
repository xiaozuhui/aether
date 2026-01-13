# 报表函数指南

本指南介绍 Aether 中的报表生成和文档处理功能，包括 Excel、Word、PDF 文档的操作。

## 目录

- [Excel 操作](#excel-操作)
- [Word 文档](#word-文档)
- [PDF 生成](#pdf-生成)
- [数据处理](#数据处理)
- [模板引擎](#模板引擎)

# 报表函数指南（暂缓）

当前版本已移除 Excel/Word/PDF 的读写与格式化内置函数，避免在 DSL 内承载重 IO 功能。

后续方向：聚焦 **Excel 公式兼容 / 转写**，包括

- 解析 Excel 公式并映射到等价的 Aether 表达式
- 将 Aether 表达式导出为 Excel 公式（生成模板时复用）
- 优先覆盖数学/统计/文本函数，其次日期时间与查找引用

设计草案请参见 [EXCEL_FORMULA_PLAN.md](EXCEL_FORMULA_PLAN.md)。
    {"region": "华南", "product": "A", "amount": 90},
    {"region": "华南", "product": "B", "amount": 110}
]

# 创建数据透视表

pivot = PIVOT_TABLE(
    sales_data,
    ["region"],           # 行字段
    ["product"],          # 列字段
    ["amount"],          # 值字段
    "sum"                # 聚合函数
)
PRINT(pivot)

```

### 分组聚合

```aether
# 按区域分组
grouped = GROUP_BY(sales_data, ["region"])

# 聚合计算
result = AGGREGATE(grouped, {
    "total_amount": {"field": "amount", "func": "sum"},
    "avg_amount": {"field": "amount", "func": "avg"},
    "count": {"field": "amount", "func": "count"}
})
PRINT(result)
```

### 交叉表

```aether
# 创建交叉表
cross = CROSS_TAB(
    sales_data,
    "region",    # 行字段
    "product",   # 列字段
    "amount"     # 值字段
)
PRINT(cross)
```

## 格式化函数

### 数字格式化

```aether
# 格式化数字（千分位分隔符）
formatted = FORMAT_NUMBER(1234567.89, 2)  # "1,234,567.89"
PRINT(formatted)

# 不使用千分位分隔符
no_sep = FORMAT_NUMBER(1234567.89, 2, FALSE)  # "1234567.89"
PRINT(no_sep)

# 格式化货币
currency = FORMAT_CURRENCY(1234.56, "¥", 2)  # "¥1,234.56"
PRINT(currency)

# 美元格式
usd = FORMAT_CURRENCY(1234.56, "$", 2)  # "$1,234.56"
PRINT(usd)

# 格式化百分比
percent = FORMAT_PERCENT(0.1234, 2)  # "12.34%"
PRINT(percent)

# 格式化日期
date_str = FORMAT_DATE(1704067200, "%Y-%m-%d")  # "2024-01-01"
PRINT(date_str)
```

## 模板引擎

```aether
# 使用模板字符串
template = "你好，{{name}}！你的订单金额是 {{amount}}。"
variables = {
    "name": "张三",
    "amount": "¥1,234.56"
}
result = TEMPLATE_RENDER(template, variables)
PRINT(result)  # "你好，张三！你的订单金额是 ¥1,234.56。"

# 加载模板文件
template = TEMPLATE_LOAD("email_template.html")
html = TEMPLATE_RENDER(template, variables)
```

## 完整示例

### 示例 1：销售报表生成

```aether
# 1. 读取销售数据
sales_data = EXCEL_READ_SHEET("sales_2024_q1.xlsx", "Sales")

# 2. 数据分析
by_region = GROUP_BY(sales_data, ["region"])
summary = AGGREGATE(by_region, {
    "total": {"field": "amount", "func": "sum"},
    "average": {"field": "amount", "func": "avg"},
    "count": {"field": "amount", "func": "count"}
})

# 3. 创建 Excel 报表
workbook = EXCEL_CREATE()

# 写入汇总数据
EXCEL_WRITE_CELL(workbook, "Summary", 0, 0, "区域")
EXCEL_WRITE_CELL(workbook, "Summary", 0, 1, "总销售额")
EXCEL_WRITE_CELL(workbook, "Summary", 0, 2, "平均销售额")
EXCEL_WRITE_CELL(workbook, "Summary", 0, 3, "订单数")

row = 1
FOR region IN summary {
    EXCEL_WRITE_CELL(workbook, "Summary", row, 0, region["region"])
    EXCEL_WRITE_CELL(workbook, "Summary", row, 1, region["total"])
    EXCEL_WRITE_CELL(workbook, "Summary", row, 2, region["average"])
    EXCEL_WRITE_CELL(workbook, "Summary", row, 3, region["count"])
    row = row + 1
}

# 添加图表
regions = MAP(summary, LAMBDA(x) { x["region"] })
totals = MAP(summary, LAMBDA(x) { x["total"] })
EXCEL_ADD_BAR_CHART(workbook, "Summary", regions, totals, "区域销售额对比")

# 保存报表
EXCEL_SAVE(workbook, "sales_report_2024_q1.xlsx")

PRINT("报表生成完成！")
```

### 示例 2：合同生成

```aether
# 员工信息
employees = [
    {"name": "张三", "position": "高级工程师", "salary": 15000},
    {"name": "李四", "position": "产品经理", "salary": 18000},
    {"name": "王五", "position": "UI设计师", "salary": 12000}
]

# 批量生成合同
FOR employee IN employees {
    # 加载模板
    doc = WORD_LOAD_TEMPLATE("contract_template.docx")
    
    # 填充变量
    variables = {
        "employee_name": employee["name"],
        "position": employee["position"],
        "salary": FORMAT_CURRENCY(employee["salary"], "¥", 2),
        "date": FORMAT_DATE(TIME(), "%Y年%m月%d日"),
        "company": "ABC科技有限公司"
    }
    doc = WORD_FILL_TEMPLATE(doc, variables)
    
    # 保存合同
    filename = "contract_" + employee["name"] + ".docx"
    WORD_SAVE(doc, filename)
    PRINT("已生成合同:", filename)
}
```

### 示例 3：数据透视报表

```aether
# 读取原始数据
data = EXCEL_READ_SHEET("sales_detail.xlsx", "Data")

# 创建数据透视表
pivot = PIVOT_TABLE(
    data,
    ["region", "product_category"],  # 行：区域和产品类别
    ["quarter"],                     # 列：季度
    ["amount"],                      # 值：销售额
    "sum"                           # 聚合：求和
)

# 创建新工作簿
workbook = EXCEL_CREATE()

# 写入透视表结果
EXCEL_WRITE_TABLE(workbook, "Pivot", 0, 0, pivot)

# 格式化表头
header_format = {
    "bold": TRUE,
    "bg_color": "#4472C4",
    "font_color": "#FFFFFF",
    "align": "center"
}
FOR col IN RANGE(0, LEN(pivot[0])) {
    EXCEL_SET_CELL_FORMAT(workbook, "Pivot", 0, col, header_format)
}

# 设置列宽
FOR col IN RANGE(0, LEN(pivot[0])) {
    EXCEL_SET_COLUMN_WIDTH(workbook, "Pivot", col, 15)
}

# 添加总计行
last_row = LEN(pivot)
EXCEL_WRITE_CELL(workbook, "Pivot", last_row, 0, "总计")
FOR col IN RANGE(1, LEN(pivot[0])) {
    formula = "=SUM(" + COLUMN_LETTER(col) + "2:" + COLUMN_LETTER(col) + STR(last_row) + ")"
    EXCEL_ADD_FORMULA(workbook, "Pivot", last_row, col, formula)
}

# 保存报表
EXCEL_SAVE(workbook, "pivot_report.xlsx")
PRINT("数据透视表已生成！")
```

## 函数参考

### Excel 读取函数

- `EXCEL_READ_SHEET(file_path, sheet_name)` - 读取整个工作表
- `EXCEL_READ_CELL(file_path, sheet, row, col)` - 读取单个单元格
- `EXCEL_READ_RANGE(file, sheet, r1, c1, r2, c2)` - 读取范围
- `EXCEL_GET_SHEETS(file_path)` - 获取所有工作表名

### Excel 写入函数

- `EXCEL_CREATE()` - 创建工作簿
- `EXCEL_WRITE_CELL(wb, sheet, row, col, value)` - 写入单元格
- `EXCEL_WRITE_ROW(wb, sheet, row, values)` - 写入行
- `EXCEL_WRITE_COLUMN(wb, sheet, col, values)` - 写入列
- `EXCEL_WRITE_TABLE(wb, sheet, row, col, data)` - 写入表格
- `EXCEL_SAVE(wb, path)` - 保存文件

### Excel 格式化函数

- `EXCEL_SET_CELL_FORMAT(wb, sheet, row, col, format)` - 设置格式
- `EXCEL_SET_COLUMN_WIDTH(wb, sheet, col, width)` - 设置列宽
- `EXCEL_SET_ROW_HEIGHT(wb, sheet, row, height)` - 设置行高
- `EXCEL_MERGE_CELLS(wb, sheet, r1, c1, r2, c2)` - 合并单元格
- `EXCEL_ADD_FORMULA(wb, sheet, row, col, formula)` - 添加公式

### Excel 图表函数

- `EXCEL_ADD_CHART(wb, sheet, type, range, options)` - 添加图表
- `EXCEL_ADD_BAR_CHART(wb, sheet, categories, values, title)` - 柱状图
- `EXCEL_ADD_LINE_CHART(wb, sheet, categories, values, title)` - 折线图
- `EXCEL_ADD_PIE_CHART(wb, sheet, labels, values, title)` - 饼图

### Word 文档函数

- `WORD_CREATE()` - 创建文档
- `WORD_ADD_PARAGRAPH(doc, text, style)` - 添加段落
- `WORD_ADD_HEADING(doc, text, level)` - 添加标题
- `WORD_ADD_TABLE(doc, data, has_header)` - 添加表格
- `WORD_ADD_IMAGE(doc, path, width, height)` - 添加图片
- `WORD_SAVE(doc, path)` - 保存文档

### Word 模板函数

- `WORD_LOAD_TEMPLATE(path)` - 加载模板
- `WORD_FILL_TEMPLATE(doc, variables)` - 填充模板
- `WORD_REPLACE_TEXT(doc, old, new)` - 替换文本

### PDF 函数

- `PDF_CREATE()` - 创建PDF
- `PDF_ADD_PAGE(pdf, width, height)` - 添加页面
- `PDF_ADD_TEXT(pdf, page, text, x, y, size)` - 添加文本
- `PDF_ADD_TABLE(pdf, page, data, x, y)` - 添加表格
- `PDF_SAVE(pdf, path)` - 保存PDF

### 数据处理函数

- `PIVOT_TABLE(data, rows, cols, values, func)` - 数据透视表
- `GROUP_BY(data, fields)` - 分组
- `AGGREGATE(grouped, functions)` - 聚合
- `CROSS_TAB(data, row_field, col_field, value_field)` - 交叉表

### 格式化函数

- `FORMAT_NUMBER(num, decimals, use_sep)` - 格式化数字
- `FORMAT_CURRENCY(amount, symbol, decimals)` - 格式化货币
- `FORMAT_PERCENT(num, decimals)` - 格式化百分比
- `FORMAT_DATE(timestamp, format)` - 格式化日期

### 模板函数

- `TEMPLATE_RENDER(template, variables)` - 渲染模板
- `TEMPLATE_LOAD(path)` - 加载模板文件

## 注意事项

1. **文件路径**：支持相对路径和绝对路径
2. **索引从0开始**：行和列索引都从0开始
3. **内存管理**：处理大文件时注意内存使用
4. **错误处理**：建议使用 TRY-CATCH 包装文件操作
5. **性能优化**：批量操作优于多次单个操作

## 下一步

- 查看 [IO 指南](IO_GUIDE.md) 了解文件操作
- 查看 [用户指南](USER_GUIDE.md) 了解语言基础
- 查看示例文件 `examples/report_demo.aether`
