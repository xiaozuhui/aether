# Aether 错误报告和命名规范

本文档介绍 Aether 语言的错误报告机制和命名规范。

## 1. 增强的错误报告

Aether 现在提供详细的错误信息，包括：

- **行号**：错误发生的具体行
- **列号**：错误发生的具体列
- **错误类型**：明确的错误分类
- **详细说明**：清晰的错误原因和建议

### 错误示例

#### 命名规范错误

```aether
Set myVar 10
```

**错误输出：**

```
错误: Parse error at line 1, column 0: Invalid identifier 'myVar' - 
变量名和函数名必须使用全大写字母和下划线（例如：MY_VAR, CALCULATE_SUM）
```

#### 语法错误

```aether
Set RESULT (X + Y
```

**错误输出：**

```
错误: Parse error at line 1, column 18: Expected RightParen, found Newline
```

#### 类型错误

```aether
Func ADD(a, b) {
    Return a + b
}
```

**错误输出：**

```
错误: Parse error at line 1, column 9: Invalid identifier 'a' - 
变量名和函数名必须使用全大写字母和下划线（例如：MY_VAR, CALCULATE_SUM）
```

## 2. 命名规范

Aether 强制执行严格的命名规范，以提高代码可读性和一致性。

### 规则

1. **变量名** 必须使用 **全大写字母 + 下划线** (UPPER_SNAKE_CASE)
2. **函数名** 必须使用 **全大写字母 + 下划线** (UPPER_SNAKE_CASE)
3. **参数名** 必须使用 **全大写字母 + 下划线** (UPPER_SNAKE_CASE)
4. **关键字** 使用 **首字母大写** (PascalCase)：Set, Func, If, While, For 等

### 有效的标识符

```aether
// ✅ 正确的变量名
Set MY_VAR 10
Set USER_NAME "Alice"
Set TOTAL_COUNT 100
Set VAR_123 "valid"
Set X 5
Set Y_COORDINATE 10

// ✅ 正确的函数名
Func CALCULATE_SUM(A, B) {
    Return A + B
}

Func GET_USER_DATA(USER_ID) {
    Return "data"
}

Func PROCESS_ARRAY(INPUT_ARRAY, FILTER_FN) {
    Return FILTER(INPUT_ARRAY, FILTER_FN)
}
```

### 无效的标识符

```aether
// ❌ 错误：小写字母
Set myVar 10              // 错误
Set userName "Alice"       // 错误

// ❌ 错误：驼峰命名
Set myVarName 10           // 错误
Set totalCount 100         // 错误

// ❌ 错误：混合大小写
Set MyVar 10               // 错误
Set my_VAR 20              // 错误

// ❌ 错误：小写函数名
Func calculateSum(a, b) {  // 错误
    Return a + b
}

// ❌ 错误：以数字开头
Set 123VAR 10              // 错误
```

## 3. 错误类型

### ParseError::UnexpectedToken

**描述**：解析器遇到了意外的标记。

**示例：**

```aether
Set X )
```

**错误输出：**

```
Parse error at line 1, column 6: Expected identifier, found RightParen
```

### ParseError::InvalidIdentifier

**描述**：标识符不符合命名规范。

**示例：**

```aether
Set myVariable 10
```

**错误输出：**

```
Parse error at line 1, column 4: Invalid identifier 'myVariable' - 
变量名和函数名必须使用全大写字母和下划线（例如：MY_VAR, CALCULATE_SUM）
```

### ParseError::InvalidExpression

**描述**：表达式语法无效。

**示例：**

```aether
Set X + * 10
```

**错误输出：**

```
Parse error at line 1, column 8: Invalid expression - Unexpected token in expression
```

### ParseError::UnexpectedEOF

**描述**：文件意外结束。

**示例：**

```aether
Func MY_FUNC(X, Y) {
    Set RESULT X + Y
```

**错误输出：**

```
Parse error at line 3, column 0: Unexpected end of file
```

## 4. 最佳实践

### 1. 使用描述性的名称

```aether
// ✅ 好的命名
Set USER_COUNT 100
Set TOTAL_REVENUE 50000.0
Func CALCULATE_TAX_AMOUNT(INCOME, TAX_RATE) { ... }

// ❌ 不好的命名
Set X 100
Set Y 50000.0
Func CALC(A, B) { ... }
```

### 2. 使用下划线分隔单词

```aether
// ✅ 清晰易读
Set FIRST_NAME "Alice"
Set LAST_NAME "Smith"
Set FULL_ADDRESS "123 Main St"

// ❌ 难以阅读
Set FIRSTNAME "Alice"
Set LASTNAME "Smith"
Set FULLADDRESS "123 Main St"
```

### 3. 避免过长的名称

```aether
// ✅ 适当长度
Set USER_DATA {}
Set CALCULATION_RESULT 0

// ❌ 过长
Set THIS_IS_THE_USER_DATA_THAT_WE_NEED_TO_PROCESS {}
Set THE_FINAL_CALCULATION_RESULT_AFTER_ALL_OPERATIONS 0
```

### 4. 常量使用全大写

```aether
// ✅ 常量命名
Set PI 3.14159
Set MAX_RETRIES 3
Set DEFAULT_TIMEOUT 5000
```

### 5. 布尔值使用明确的前缀

```aether
// ✅ 布尔值命名
Set IS_ACTIVE True
Set HAS_PERMISSION False
Set CAN_EDIT True
```

## 5. 错误调试技巧

### 1. 查看完整的错误信息

错误信息包含行号和列号，帮助你快速定位问题。

```
Parse error at line 15, column 23: Invalid identifier 'userName'
                   ^^        ^^
                   |         |
                   行号      列号
```

### 2. 逐步调试

如果有多个错误，Aether 会在第一个错误处停止。修复一个错误后重新运行。

### 3. 使用注释临时禁用代码

```aether
// Set RESULT SOME_PROBLEMATIC_CODE()
Set RESULT 0  // 临时使用默认值
```

### 4. 检查括号匹配

确保所有的括号、花括号、方括号都正确配对。

```aether
// ✅ 正确
Set RESULT (A + B) * (C + D)

// ❌ 错误：缺少右括号
Set RESULT (A + B * (C + D)
```

## 6. 从其他语言迁移

### 从 JavaScript/TypeScript 迁移

```javascript
// JavaScript
let myVar = 10;
function calculateSum(a, b) {
    return a + b;
}
```

```aether
// Aether
Set MY_VAR 10
Func CALCULATE_SUM(A, B) {
    Return A + B
}
```

### 从 Python 迁移

```python
# Python
my_var = 10
def calculate_sum(a, b):
    return a + b
```

```aether
// Aether
Set MY_VAR 10
Func CALCULATE_SUM(A, B) {
    Return A + B
}
```

### 从 Go 迁移

```go
// Go
myVar := 10
func calculateSum(a int, b int) int {
    return a + b
}
```

```aether
// Aether
Set MY_VAR 10
Func CALCULATE_SUM(A, B) {
    Return A + B
}
```

## 7. 总结

Aether 的命名规范和错误报告机制旨在：

- 提高代码可读性和一致性
- 快速定位和修复错误
- 降低学习曲线
- 促进团队协作

遵循这些规范将帮助你编写更清晰、更易维护的 Aether 代码。
