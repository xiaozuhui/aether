# Aether Standard Library (stdlib)

Aether 编程语言的标准库，提供丰富的工具函数，帮助开发者更高效地使用 Aether。

## 📚 目录

- [简介](#简介)
- [安装使用](#安装使用)
- [库列表](#库列表)
  - [String Utils (字符串工具)](#string-utils-字符串工具)
  - [Array Utils (数组工具)](#array-utils-数组工具)
  - [Validation (数据验证)](#validation-数据验证)
  - [DateTime (日期时间)](#datetime-日期时间)
  - [Testing (测试框架)](#testing-测试框架)
  - [Set (集合)](#set-集合)
  - [Queue (队列)](#queue-队列)
  - [Stack (栈)](#stack-栈)
  - [Heap (堆)](#heap-堆)
  - [Sorting (排序算法)](#sorting-排序算法)
- [示例](#示例)
- [贡献](#贡献)

## 简介

Aether 标准库是完全使用 Aether 语言编写的工具集合，展示了 Aether 的自举能力。所有库都遵循 Aether 的命名规范：

- **函数名和变量名**：全部大写 + 下划线（如 `STR_TRIM`, `MY_VARIABLE`）
- **关键字**：首字母大写（如 `Set`, `If`, `While`, `Return`）

## 安装使用

### 🎉 标准库已内置（推荐）

从 v0.1.0 开始，标准库已经编译进 Aether 二进制文件中！你可以直接使用：

#### 命令行使用

```bash
# 运行脚本时自动加载标准库
aether your_script.aether

# 如果不需要标准库（更快启动）
aether --no-stdlib your_script.aether
```

#### REPL 中使用

```bash
aether

# 在 REPL 中加载标准库
aether[1]> :load stdlib
✓ 标准库加载成功

# 或者只加载特定模块
aether[2]> :load string_utils
✓ 模块 'string_utils' 加载成功
```

#### 在代码中使用

```rust
use aether::Aether;

// 方式 1: 创建带标准库的引擎（推荐）
let mut engine = Aether::with_stdlib().unwrap();

// 方式 2: 手动加载
let mut engine = Aether::new();
engine.load_all_stdlib().unwrap();

// 方式 3: 只加载特定模块
let mut engine = Aether::new();
engine.load_stdlib_module("string_utils").unwrap();
engine.load_stdlib_module("array_utils").unwrap();
```

### 传统方式（可选）

如果你想从文件加载标准库：

```bash
# 方式 1: 在脚本中显式加载（不推荐，因为已内置）
# 这种方式主要用于开发和测试

# 方式 2: 手动运行库文件
aether stdlib/string_utils.aether your_script.aether
```

### 快速测试

运行内置测试验证标准库：

```bash
# 测试所有标准库功能
aether stdlib/examples/stdlib_test.aether

# 测试特定模块
aether stdlib/examples/string_demo.aether
aether stdlib/examples/array_demo.aether
```

## 库列表

### String Utils (字符串工具)

**文件**: `string_utils.aether`

提供丰富的字符串操作函数。

#### 核心函数

| 函数 | 说明 | 示例 |
|------|------|------|
| `STR_TRIM(str)` | 移除两端空白 | `STR_TRIM("  hello  ")` → `"hello"` |
| `STR_TRIM_LEFT(str)` | 移除左侧空白 | `STR_TRIM_LEFT("  hello")` → `"hello"` |
| `STR_TRIM_RIGHT(str)` | 移除右侧空白 | `STR_TRIM_RIGHT("hello  ")` → `"hello"` |
| `STR_PAD_LEFT(str, len, char)` | 左侧填充 | `STR_PAD_LEFT("5", 3, "0")` → `"005"` |
| `STR_PAD_RIGHT(str, len, char)` | 右侧填充 | `STR_PAD_RIGHT("hi", 5, " ")` → `"hi   "` |
| `STR_PAD_CENTER(str, len, char)` | 居中填充 | `STR_PAD_CENTER("hi", 6, "-")` → `"--hi--"` |
| `STR_REPEAT(str, count)` | 重复字符串 | `STR_REPEAT("*", 5)` → `"*****"` |
| `STR_REVERSE(str)` | 反转字符串 | `STR_REVERSE("abc")` → `"cba"` |
| `STR_CONTAINS(str, substr)` | 检查包含 | `STR_CONTAINS("hello", "ell")` → `True` |
| `STR_STARTS_WITH(str, prefix)` | 检查前缀 | `STR_STARTS_WITH("hello", "he")` → `True` |
| `STR_ENDS_WITH(str, suffix)` | 检查后缀 | `STR_ENDS_WITH("hello", "lo")` → `True` |
| `STR_INDEX_OF(str, substr)` | 查找位置 | `STR_INDEX_OF("hello", "ll")` → `2` |
| `STR_REPLACE_ALL(str, old, new)` | 替换所有 | `STR_REPLACE_ALL("aa", "a", "b")` → `"bb"` |
| `STR_SPLIT(str, delim)` | 分割字符串 | `STR_SPLIT("a,b,c", ",")` → `["a","b","c"]` |
| `STR_JOIN(arr, delim)` | 连接数组 | `STR_JOIN(["a","b"], ",")` → `"a,b"` |
| `STR_TO_UPPER(str)` | 转大写 | `STR_TO_UPPER("hello")` → `"HELLO"` |
| `STR_TO_LOWER(str)` | 转小写 | `STR_TO_LOWER("HELLO")` → `"hello"` |
| `STR_IS_EMPTY(str)` | 检查为空 | `STR_IS_EMPTY("")` → `True` |
| `STR_IS_NUMERIC(str)` | 检查数字 | `STR_IS_NUMERIC("123")` → `True` |
| `STR_IS_ALPHA(str)` | 检查字母 | `STR_IS_ALPHA("abc")` → `True` |

### Array Utils (数组工具)

**文件**: `array_utils.aether`

提供高级数组操作函数。

#### 核心函数

| 函数 | 说明 | 示例 |
|------|------|------|
| `ARR_UNIQUE(arr)` | 去重 | `ARR_UNIQUE([1,2,2,3])` → `[1,2,3]` |
| `ARR_FLATTEN(arr)` | 扁平化一层 | `ARR_FLATTEN([[1,2],[3]])` → `[1,2,3]` |
| `ARR_FLATTEN_DEEP(arr)` | 深度扁平化 | `ARR_FLATTEN_DEEP([[[1]]])` → `[1]` |
| `ARR_CHUNK(arr, size)` | 分块 | `ARR_CHUNK([1,2,3,4], 2)` → `[[1,2],[3,4]]` |
| `ARR_ZIP(arr1, arr2)` | 压缩 | `ARR_ZIP([1,2], [3,4])` → `[[1,3],[2,4]]` |
| `ARR_PARTITION(arr, op, val)` | 分区 | `ARR_PARTITION([1,5,3], ">", 3)` → `[[5],[1,3]]` |
| `ARR_INDEX_OF(arr, val)` | 查找索引 | `ARR_INDEX_OF([1,2,3], 2)` → `1` |
| `ARR_CONTAINS(arr, val)` | 检查包含 | `ARR_CONTAINS([1,2,3], 2)` → `True` |
| `ARR_REVERSE(arr)` | 反转 | `ARR_REVERSE([1,2,3])` → `[3,2,1]` |
| `ARR_SLICE(arr, start, end)` | 切片 | `ARR_SLICE([1,2,3,4], 1, 3)` → `[2,3]` |
| `ARR_TAKE(arr, n)` | 取前n个 | `ARR_TAKE([1,2,3], 2)` → `[1,2]` |
| `ARR_SKIP(arr, n)` | 跳过前n个 | `ARR_SKIP([1,2,3], 1)` → `[2,3]` |
| `ARR_SUM(arr)` | 求和 | `ARR_SUM([1,2,3])` → `6` |
| `ARR_AVERAGE(arr)` | 平均值 | `ARR_AVERAGE([1,2,3])` → `2` |
| `ARR_MAX(arr)` | 最大值 | `ARR_MAX([1,5,3])` → `5` |
| `ARR_MIN(arr)` | 最小值 | `ARR_MIN([1,5,3])` → `1` |
| `ARR_INTERSECTION(a, b)` | 交集 | `ARR_INTERSECTION([1,2],[2,3])` → `[2]` |
| `ARR_UNION(a, b)` | 并集 | `ARR_UNION([1,2],[2,3])` → `[1,2,3]` |
| `ARR_DIFFERENCE(a, b)` | 差集 | `ARR_DIFFERENCE([1,2],[2,3])` → `[1]` |
| `ARR_RANGE(start, end, step)` | 范围 | `ARR_RANGE(1, 10, 2)` → `[1,3,5,7,9]` |
| `ARR_FILL(val, count)` | 填充 | `ARR_FILL("x", 3)` → `["x","x","x"]` |
| `ARR_COUNT(arr, val)` | 计数 | `ARR_COUNT([1,2,2,3], 2)` → `2` |
| `ARR_FREQUENCY(arr)` | 频率统计 | `ARR_FREQUENCY([1,1,2])` → `{"1":2,"2":1}` |

### Validation (数据验证)

**文件**: `validation.aether`

提供常用的数据验证函数。

#### 核心函数

| 函数 | 说明 | 示例 |
|------|------|------|
| `VALIDATE_EMAIL(email)` | 验证邮箱 | `VALIDATE_EMAIL("a@b.com")` → `True` |
| `VALIDATE_PHONE_CN(phone)` | 验证中国手机 | `VALIDATE_PHONE_CN("13812345678")` → `True` |
| `VALIDATE_PHONE_GENERAL(phone)` | 验证通用电话 | `VALIDATE_PHONE_GENERAL("123-456-7890")` → `True` |
| `VALIDATE_ID_CARD_CN(id)` | 验证身份证 | `VALIDATE_ID_CARD_CN("110101199001011234")` → `True` |
| `VALIDATE_URL(url)` | 验证URL | `VALIDATE_URL("https://example.com")` → `True` |
| `VALIDATE_DOMAIN(domain)` | 验证域名 | `VALIDATE_DOMAIN("example.com")` → `True` |
| `VALIDATE_RANGE(val, min, max)` | 验证范围 | `VALIDATE_RANGE(5, 1, 10)` → `True` |
| `VALIDATE_POSITIVE(val)` | 验证正数 | `VALIDATE_POSITIVE(5)` → `True` |
| `VALIDATE_INTEGER(val)` | 验证整数 | `VALIDATE_INTEGER("123")` → `True` |
| `VALIDATE_LENGTH(str, min, max)` | 验证长度 | `VALIDATE_LENGTH("hi", 1, 5)` → `True` |
| `VALIDATE_ALPHA(str)` | 验证字母 | `VALIDATE_ALPHA("abc")` → `True` |
| `VALIDATE_NUMERIC(str)` | 验证数字 | `VALIDATE_NUMERIC("123")` → `True` |
| `VALIDATE_ALPHANUMERIC(str)` | 验证字母数字 | `VALIDATE_ALPHANUMERIC("abc123")` → `True` |
| `VALIDATE_PASSWORD_STRONG(pwd)` | 强密码 | `VALIDATE_PASSWORD_STRONG("Abc12345")` → `True` |
| `VALIDATE_PASSWORD_MEDIUM(pwd)` | 中等密码 | `VALIDATE_PASSWORD_MEDIUM("abc123")` → `True` |
| `VALIDATE_IPV4(ip)` | 验证IPv4 | `VALIDATE_IPV4("192.168.1.1")` → `True` |
| `VALIDATE_DATE_FORMAT(date)` | 验证日期格式 | `VALIDATE_DATE_FORMAT("2024-12-25")` → `True` |
| `VALIDATE_TIME_FORMAT(time)` | 验证时间格式 | `VALIDATE_TIME_FORMAT("14:30:45")` → `True` |
| `VALIDATE_USERNAME(user)` | 验证用户名 | `VALIDATE_USERNAME("user_123")` → `True` |

### DateTime (日期时间)

**文件**: `datetime.aether`

提供日期时间处理功能。

#### 核心函数

| 函数 | 说明 | 示例 |
|------|------|------|
| `DT_IS_LEAP_YEAR(year)` | 判断闰年 | `DT_IS_LEAP_YEAR(2024)` → `True` |
| `DT_DAYS_IN_MONTH(year, month)` | 月份天数 | `DT_DAYS_IN_MONTH(2024, 2)` → `29` |
| `DT_DAY_OF_YEAR(y, m, d)` | 年内第几天 | `DT_DAY_OF_YEAR(2024, 3, 1)` → `61` |
| `DT_IS_VALID_DATE(y, m, d)` | 验证日期 | `DT_IS_VALID_DATE(2024, 2, 29)` → `True` |
| `DT_IS_VALID_TIME(h, m, s)` | 验证时间 | `DT_IS_VALID_TIME(14, 30, 45)` → `True` |
| `DT_COMPARE_DATE(y1,m1,d1,y2,m2,d2)` | 比较日期 | `DT_COMPARE_DATE(2024,1,1,2024,12,31)` → `-1` |
| `DT_DAYS_BETWEEN(y1,m1,d1,y2,m2,d2)` | 天数差 | `DT_DAYS_BETWEEN(2024,1,1,2024,1,31)` → `30` |
| `DT_FORMAT_DATE(y, m, d)` | 格式化日期 | `DT_FORMAT_DATE(2024, 1, 5)` → `"2024-01-05"` |
| `DT_FORMAT_TIME(h, m, s)` | 格式化时间 | `DT_FORMAT_TIME(9, 5, 3)` → `"09:05:03"` |
| `DT_FORMAT_DATETIME(y,m,d,h,mi,s)` | 格式化日期时间 | `DT_FORMAT_DATETIME(...)` → `"2024-01-05 09:05:03"` |
| `DT_MONTH_NAME(month)` | 月份英文名 | `DT_MONTH_NAME(1)` → `"January"` |
| `DT_MONTH_NAME_CN(month)` | 月份中文名 | `DT_MONTH_NAME_CN(1)` → `"一月"` |
| `DT_DAY_OF_WEEK(y, m, d)` | 星期几(0-6) | `DT_DAY_OF_WEEK(2024, 1, 1)` → `1` |
| `DT_WEEKDAY_NAME(y, m, d)` | 星期英文名 | `DT_WEEKDAY_NAME(2024,1,1)` → `"Monday"` |
| `DT_WEEKDAY_NAME_CN(y, m, d)` | 星期中文名 | `DT_WEEKDAY_NAME_CN(2024,1,1)` → `"星期一"` |
| `DT_ADD_DAYS(y, m, d, days)` | 加天数 | `DT_ADD_DAYS(2024,1,28,5)` → `[2024,2,2]` |
| `DT_ADD_MONTHS(y, m, d, months)` | 加月数 | `DT_ADD_MONTHS(2024,1,31,1)` → `[2024,2,29]` |
| `DT_ADD_YEARS(y, m, d, years)` | 加年数 | `DT_ADD_YEARS(2024,2,29,1)` → `[2025,2,28]` |
| `DT_CALCULATE_AGE(by,bm,bd,cy,cm,cd)` | 计算年龄 | `DT_CALCULATE_AGE(1990,6,15,2024,12,1)` → `34` |
| `DT_GET_QUARTER(month)` | 获取季度 | `DT_GET_QUARTER(7)` → `3` |
| `DT_IS_WEEKEND(y, m, d)` | 是否周末 | `DT_IS_WEEKEND(2024,1,6)` → `True` |
| `DT_IS_WEEKDAY(y, m, d)` | 是否工作日 | `DT_IS_WEEKDAY(2024,1,8)` → `True` |

### Testing (测试框架)

**文件**: `testing.aether`

提供单元测试框架。

#### 核心函数

**断言函数**：

| 函数 | 说明 |
|------|------|
| `ASSERT_TRUE(val, msg)` | 断言为真 |
| `ASSERT_FALSE(val, msg)` | 断言为假 |
| `ASSERT_EQUAL(actual, expected, msg)` | 断言相等 |
| `ASSERT_NOT_EQUAL(actual, expected, msg)` | 断言不相等 |
| `ASSERT_NULL(val, msg)` | 断言为 null |
| `ASSERT_NOT_NULL(val, msg)` | 断言不为 null |
| `ASSERT_GREATER(actual, expected, msg)` | 断言大于 |
| `ASSERT_LESS(actual, expected, msg)` | 断言小于 |
| `ASSERT_CONTAINS(arr, val, msg)` | 断言包含 |
| `ASSERT_LENGTH(arr, len, msg)` | 断言长度 |
| `ASSERT_TYPE(val, type, msg)` | 断言类型 |
| `ASSERT_IN_RANGE(val, min, max, msg)` | 断言范围 |

**测试组织**：

| 函数 | 说明 |
|------|------|
| `TEST_SUITE(name)` | 开始测试套件 |
| `TEST_CASE(name)` | 开始测试用例 |
| `TEST_SUMMARY()` | 打印测试摘要 |
| `TEST_RESET()` | 重置计数器 |

**Mock 工具**：

| 函数 | 说明 |
|------|------|
| `MOCK_CREATE()` | 创建 mock 对象 |
| `MOCK_CALL(mock, args)` | 记录调用 |
| `MOCK_WAS_CALLED(mock)` | 检查是否被调用 |
| `MOCK_CALL_COUNT(mock)` | 获取调用次数 |

## 示例

### 完整的用户注册验证示例

```aether
// 加载所需的库
// aether stdlib/string_utils.aether stdlib/validation.aether user_registration.aether

Func VALIDATE_USER_REGISTRATION(USERNAME, EMAIL, PASSWORD, AGE) {
    Println("Validating user registration...")
    
    // 验证用户名
    Set USERNAME_TRIMMED STR_TRIM(USERNAME)
    If (!VALIDATE_USERNAME(USERNAME_TRIMMED)) {
        Println("❌ Invalid username")
        Return False
    }
    
    // 验证邮箱
    If (!VALIDATE_EMAIL(EMAIL)) {
        Println("❌ Invalid email")
        Return False
    }
    
    // 验证密码强度
    If (!VALIDATE_PASSWORD_STRONG(PASSWORD)) {
        Println("❌ Password not strong enough")
        Return False
    }
    
    // 验证年龄范围
    If (!VALIDATE_RANGE(AGE, 18, 120)) {
        Println("❌ Age must be between 18 and 120")
        Return False
    }
    
    Println("✅ All validations passed!")
    Return True
}

// 测试
Set RESULT VALIDATE_USER_REGISTRATION("alice_123", "alice@example.com", "SecurePass123", 25)
Println("Registration result: " + ToString(RESULT))
```

### 数据处理示例

```aether
// 数据清洗和统计
Func PROCESS_SALES_DATA(RAW_DATA) {
    // 去重
    Set UNIQUE_DATA ARR_UNIQUE(RAW_DATA)
    
    // 过滤掉无效数据（小于0）
    Set PARTITIONED ARR_PARTITION(UNIQUE_DATA, ">=", 0)
    Set VALID_DATA ArrGet(PARTITIONED, 0)
    
    // 计算统计信息
    Set TOTAL ARR_SUM(VALID_DATA)
    Set AVG ARR_AVERAGE(VALID_DATA)
    Set MAX_VAL ARR_MAX(VALID_DATA)
    Set MIN_VAL ARR_MIN(VALID_DATA)
    
    // 返回结果
    Set RESULT {}
    Set RESULT DictSet(RESULT, "total", TOTAL)
    Set RESULT DictSet(RESULT, "average", AVG)
    Set RESULT DictSet(RESULT, "max", MAX_VAL)
    Set RESULT DictSet(RESULT, "min", MIN_VAL)
    Set RESULT DictSet(RESULT, "count", ArrLen(VALID_DATA))
    
    Return RESULT
}
```

## 贡献

欢迎贡献新的库或改进现有库！请遵循以下规范：

1. **命名规范**：
   - 函数名全部大写，使用下划线分隔（如 `MY_FUNCTION`）
   - 变量名全部大写，使用下划线分隔（如 `MY_VAR`）
   - 关键字首字母大写（如 `Set`, `If`, `While`）

2. **文档规范**：
   - 为每个函数添加注释说明
   - 使用分类注释（如 `// ==================== 分类名 ====================`）
   - 提供使用示例

3. **测试**：
   - 为新功能编写测试用例
   - 使用 `testing.aether` 框架

4. **示例**：
   - 在 `examples/` 目录下提供使用示例
   - 示例要清晰易懂

---

### Set (集合)

**文件**: `stdlib/set.aether`

集合数据结构，保证元素唯一性，提供集合运算。

#### 主要函数

- `SET_NEW()` - 创建空集合
- `SET_FROM_ARRAY(ARR)` - 从数组创建集合（自动去重）
- `SET_ADD(SET, ITEM)` - 添加元素
- `SET_REMOVE(SET, ITEM)` - 移除元素
- `SET_CONTAINS(SET, ITEM)` - 检查是否包含元素
- `SET_SIZE(SET)` - 获取集合大小
- `SET_UNION(SET1, SET2)` - 并集
- `SET_INTERSECTION(SET1, SET2)` - 交集
- `SET_DIFFERENCE(SET1, SET2)` - 差集
- `SET_IS_SUBSET(SET1, SET2)` - 检查子集关系

**示例**: `stdlib/examples/set_demo.aether`

---

### Queue (队列)

**文件**: `stdlib/queue.aether`

先进先出（FIFO）队列数据结构。

#### 主要函数

- `QUEUE_NEW()` - 创建空队列
- `QUEUE_ENQUEUE(QUEUE, ITEM)` - 入队
- `QUEUE_DEQUEUE(QUEUE)` - 出队，返回 `{"queue": 新队列, "value": 值}`
- `QUEUE_PEEK(QUEUE)` - 查看队首元素
- `QUEUE_SIZE(QUEUE)` - 获取队列大小
- `QUEUE_IS_EMPTY(QUEUE)` - 检查是否为空
- `QUEUE_ENQUEUE_ALL(QUEUE, ARR)` - 批量入队
- `QUEUE_DEQUEUE_N(QUEUE, N)` - 批量出队

**示例**: `stdlib/examples/queue_demo.aether`

---

### Stack (栈)

**文件**: `stdlib/stack.aether`

后进先出（LIFO）栈数据结构。

#### 主要函数

- `STACK_NEW()` - 创建空栈
- `STACK_PUSH(STACK, ITEM)` - 压栈
- `STACK_POP(STACK)` - 出栈，返回 `{"stack": 新栈, "value": 值}`
- `STACK_PEEK(STACK)` - 查看栈顶元素
- `STACK_SIZE(STACK)` - 获取栈大小
- `STACK_IS_EMPTY(STACK)` - 检查是否为空
- `STACK_SWAP_TOP(STACK)` - 交换栈顶两元素
- `STACK_ROTATE_UP(STACK)` - 栈底移到栈顶
- `STACK_ROTATE_DOWN(STACK)` - 栈顶移到栈底

**示例**: `stdlib/examples/stack_demo.aether`

---

### Heap (堆)

**文件**: `stdlib/heap.aether`

最小堆和最大堆数据结构，适用于优先级队列和堆排序。

#### 主要函数

**最小堆**:

- `MIN_HEAP_NEW()` - 创建空最小堆
- `MIN_HEAP_FROM_ARRAY(ARR)` - 从数组创建最小堆
- `MIN_HEAP_INSERT(HEAP, VALUE)` - 插入元素
- `MIN_HEAP_EXTRACT(HEAP)` - 提取最小值
- `MIN_HEAP_PEEK(HEAP)` - 查看最小值
- `MIN_HEAP_GET_K_MIN(HEAP, K)` - 获取前K个最小元素

**最大堆**:

- `MAX_HEAP_NEW()` - 创建空最大堆
- `MAX_HEAP_FROM_ARRAY(ARR)` - 从数组创建最大堆
- `MAX_HEAP_INSERT(HEAP, VALUE)` - 插入元素
- `MAX_HEAP_EXTRACT(HEAP)` - 提取最大值
- `MAX_HEAP_PEEK(HEAP)` - 查看最大值
- `MAX_HEAP_GET_K_MAX(HEAP, K)` - 获取前K个最大元素

**堆排序**:

- `HEAP_SORT_ASC(ARR)` - 升序堆排序
- `HEAP_SORT_DESC(ARR)` - 降序堆排序

**示例**: `stdlib/examples/heap_demo.aether`

---

### Sorting (排序算法)

**文件**: `stdlib/sorting.aether`

多种排序算法实现，适用于数组排序。

#### 排序算法

**简单排序**（适合小数据集）:

- `BUBBLE_SORT(ARR)` / `BUBBLE_SORT_DESC(ARR)` - 冒泡排序 O(n²)
- `SELECTION_SORT(ARR)` / `SELECTION_SORT_DESC(ARR)` - 选择排序 O(n²)
- `INSERTION_SORT(ARR)` / `INSERTION_SORT_DESC(ARR)` - 插入排序 O(n²)

**高效排序**（适合大数据集）:

- `MERGE_SORT(ARR)` / `MERGE_SORT_DESC(ARR)` - 归并排序 O(n log n)
- `QUICK_SORT(ARR)` / `QUICK_SORT_DESC(ARR)` - 快速排序 O(n log n) 平均
- `HEAP_SORT(ARR)` / `HEAP_SORT_DESC(ARR)` - 堆排序 O(n log n)

**特殊排序**:

- `COUNTING_SORT(ARR)` - 计数排序 O(n+k)，仅适用于非负整数

**通用接口**:

- `SORT(ARR)` - 通用排序（默认快速排序，升序）
- `SORT_DESC(ARR)` - 通用排序（降序）

**辅助函数**:

- `IS_SORTED_ASC(ARR)` - 检查是否升序
- `IS_SORTED_DESC(ARR)` - 检查是否降序

**示例**: `stdlib/examples/sorting_demo.aether`

---

## 许可证

与 Aether 主项目使用相同的许可证。

## 联系方式

如有问题或建议，请在 Aether 项目仓库提交 issue。

---

**Made with ❤️ using Aether**
