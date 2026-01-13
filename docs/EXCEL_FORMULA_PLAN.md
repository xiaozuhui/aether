# Excel 公式兼容 / 转写设计草案

## 目标

- 将常用 Excel 公式映射到 Aether 表达式，以便在 DSL 中重写或直接执行。
- 提供 Excel → Aether 的公式翻译（导入场景），以及 Aether → Excel 的反向生成（导出模板）。
- 保持纯函数、无 IO，不引入 Excel 文件读写能力。

## 覆盖优先级

1) 数学 / 统计：SUM, AVERAGE, MIN, MAX, COUNT, COUNTA, COUNTIF, ROUND*, ABS, POWER, SQRT。
2) 文本：CONCAT/&, TEXTJOIN(简化版), LEFT/RIGHT/MID, LEN, TRIM, UPPER/LOWER, SUBSTITUTE/REPLACE, FIND/SEARCH。
3) 日期时间：TODAY, NOW, DATE, YEAR/MONTH/DAY, HOUR/MINUTE/SECOND, DATEDIF(常见模式)。
4) 查找：IF, IFS, CHOOSE, SWITCH(若有), COALESCE-like (IFERROR/IFNA 简化)。
5) 数组/范围：以平铺列表替代真正二维范围，先聚焦无“引用滑动”的函数；暂不支持 OFFSET/INDIRECT/INDEX/MATCH 这种需要工作表上下文的函数。

## 翻译方向

- **Excel → Aether**：解析公式 AST，映射到等价内置函数或语法；不支持工作表/单元格引用，首版仅支持纯函数和字面量。
- **Aether → Excel**：对纯函数表达式做反向映射，输出可粘贴到单元格的公式字符串；遇到不支持的函数直接返回错误。

## API 形态（提议）

- `EXCEL_FORMULA_TO_AETHER(formula_str)` → 字符串（Aether 表达式）或错误。
- `AETHER_TO_EXCEL_FORMULA(expr_str)` → 字符串（Excel 公式）或错误。
- 可选：`EXCEL_NORMALIZE_FORMULA(formula_str)` 统一大小写、函数名、分隔符，用于预处理。

## 错误与兼容策略

- 未支持的函数：返回可读错误并列出最接近的支持列表。
- 参数超出范围或类型不匹配：在翻译期报错，而非运行期。
- 分隔符：统一使用逗号 `,`，不处理区域性分隔符；小数点固定为 `.`。
- 布尔/空：TRUE/FALSE → `TRUE`/`FALSE`；空值 → `NULL`。

## 分阶段落地

- **P1**：实现小型函数映射表 + 解析器（可选用简易递归下降或基于现有表达式解析模块），支持 Excel→Aether 单向转换；覆盖优先级 1-3 的子集（SUM/AVG/ROUND/文本基本函数）。
- **P2**: 增加 Aether→Excel 反向映射；扩展 IF/IFS/CHOOSE/IFERROR 等条件/容错函数；补充日期时间。
- **P3**: 考虑有限的范围操作（仅显式数组字面量），以及部分 MATCH/INDEX 简化模式；完善错误信息与文档。

## 开发备注

- 解析层可复用现有 `parser` 模块的 token/ast 基础，或在 `report` 子模块下做独立轻量解析避免耦合。
- 保持无 `std::io` / 无全局可变状态，方便在沙箱和 wasm 场景运行。
- 增量引入单元测试：每个函数映射至少一组输入/输出示例，确保与 Excel 行为一致（特别是舍入规则）。
