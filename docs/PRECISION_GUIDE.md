# Aether 精确计算和精度计算功能

本文档介绍 Aether 语言中的实数精确计算和精度计算功能。

## 1. 精确计算 (分数运算)

精确计算使用有理数 (分数) 来避免浮点数的精度问题。

### 转换函数

#### `TO_FRACTION(number)`

将浮点数转换为分数。

```aether
Set HALF TO_FRACTION(0.5)
PRINTLN(HALF)  // 输出: 1/2

Set THIRD TO_FRACTION(0.333333)
PRINTLN(THIRD)  // 输出: 333333/1000000
```

#### `TO_FLOAT(fraction)`

将分数转换回浮点数。

```aether
Set F TO_FRACTION(0.75)
Set NUM TO_FLOAT(F)
PRINTLN(NUM)  // 输出: 0.75
```

#### `SIMPLIFY(fraction)`

简化分数到最简形式。

```aether
Set F TO_FRACTION(0.5)
Set SIMPLE SIMPLIFY(F)
PRINTLN(SIMPLE)  // 输出: 1/2
```

### 分数运算

#### `FRAC_ADD(frac1, frac2)`

分数加法。

```aether
Set A TO_FRACTION(0.5)    // 1/2
Set B TO_FRACTION(0.25)   // 1/4
Set SUM FRAC_ADD(A, B)
PRINTLN(SUM)              // 输出: 3/4
```

#### `FRAC_SUB(frac1, frac2)`

分数减法。

```aether
Set A TO_FRACTION(0.75)   // 3/4
Set B TO_FRACTION(0.25)   // 1/4
Set DIFF FRAC_SUB(A, B)
PRINTLN(DIFF)             // 输出: 1/2
```

#### `FRAC_MUL(frac1, frac2)`

分数乘法。

```aether
Set A TO_FRACTION(0.5)    // 1/2
Set B TO_FRACTION(0.5)    // 1/2
Set PROD FRAC_MUL(A, B)
PRINTLN(PROD)             // 输出: 1/4
```

#### `FRAC_DIV(frac1, frac2)`

分数除法。

```aether
Set A TO_FRACTION(0.5)    // 1/2
Set B TO_FRACTION(0.25)   // 1/4
Set QUOT FRAC_DIV(A, B)
PRINTLN(QUOT)             // 输出: 2/1
```

### 分数属性

#### `NUMERATOR(fraction)`

获取分子。

```aether
Set F TO_FRACTION(0.75)   // 3/4
Set N NUMERATOR(F)
PRINTLN(N)                // 输出: 3
```

#### `DENOMINATOR(fraction)`

获取分母。

```aether
Set F TO_FRACTION(0.75)   // 3/4
Set D DENOMINATOR(F)
PRINTLN(D)                // 输出: 4
```

### 数论函数

#### `GCD(a, b)`

计算最大公约数。

```aether
Set G GCD(12, 18)
PRINTLN(G)  // 输出: 6
```

#### `LCM(a, b)`

计算最小公倍数。

```aether
Set L LCM(12, 18)
PRINTLN(L)  // 输出: 36
```

## 2. 精度计算

精度计算在指定的小数位数下进行四舍五入运算。

### 四舍五入

#### `ROUND_TO(number, precision)`

将数字四舍五入到指定小数位数。

```aether
Set PI 3.14159265
Set PI2 ROUND_TO(PI, 2)
PRINTLN(PI2)  // 输出: 3.14

Set PI4 ROUND_TO(PI, 4)
PRINTLN(PI4)  // 输出: 3.1416
```

### 带精度的四则运算

这些函数在运算前先对操作数进行精度处理，然后再对结果进行四舍五入。

#### `ADD_WITH_PRECISION(a, b, precision)`

带精度的加法。

```aether
Set SUM ADD_WITH_PRECISION(0.1, 0.2, 2)
PRINTLN(SUM)  // 输出: 0.3 (而不是 0.30000000000000004)
```

#### `SUB_WITH_PRECISION(a, b, precision)`

带精度的减法。

```aether
Set DIFF SUB_WITH_PRECISION(5.0, 3.333, 2)
PRINTLN(DIFF)  // 输出: 1.67
```

#### `MUL_WITH_PRECISION(a, b, precision)`

带精度的乘法。

```aether
Set PROD MUL_WITH_PRECISION(3.456, 2.5, 2)
PRINTLN(PROD)  // 输出: 8.64
```

#### `DIV_WITH_PRECISION(a, b, precision)`

带精度的除法。

```aether
Set QUOT DIV_WITH_PRECISION(10.0, 3.0, 2)
PRINTLN(QUOT)  // 输出: 3.33
```

#### `SET_PRECISION(number, precision)`

将数字设置为指定精度 (等价于 ROUND_TO)。

```aether
Set NUM SET_PRECISION(3.14159, 3)
PRINTLN(NUM)  // 输出: 3.142
```

## 3. 实际应用示例

### 避免浮点数精度问题

```aether
// 浮点数运算的问题
Set A 0.1
Set B 0.2
Set C A + B
PRINTLN(C)  // 输出: 0.30000000000000004

// 使用分数的精确计算
Set FA TO_FRACTION(0.1)
Set FB TO_FRACTION(0.2)
Set FC FRAC_ADD(FA, FB)
PRINTLN(FC)             // 输出: 3/10
PRINTLN(TO_FLOAT(FC))   // 输出: 0.3
```

### 金融计算

```aether
// 计算购物总价和税金
Set PRICE1 19.99
Set PRICE2 29.99
Set PRICE3 9.99

Set SUBTOTAL ADD_WITH_PRECISION(PRICE1, PRICE2, 2)
Set SUBTOTAL ADD_WITH_PRECISION(SUBTOTAL, PRICE3, 2)
PRINTLN(SUBTOTAL)  // 输出: 59.97

Set TAX_RATE 0.08
Set TAX MUL_WITH_PRECISION(SUBTOTAL, TAX_RATE, 2)
PRINTLN(TAX)  // 输出: 4.80

Set TOTAL ADD_WITH_PRECISION(SUBTOTAL, TAX, 2)
PRINTLN(TOTAL)  // 输出: 64.77
```

### 科学计算

```aether
// 化简分数
Set F1 TO_FRACTION(0.66666)
Set F2 TO_FRACTION(0.33333)
Set F3 FRAC_ADD(F1, F2)
PRINTLN(F3)  // 近似 1/1

// 数论计算
Set G GCD(48, 18)
PRINTLN(G)  // 输出: 6

Set L LCM(12, 8)
PRINTLN(L)  // 输出: 24
```

## 4. 最佳实践

1. **选择合适的方法**：
   - 需要完全精确的计算 (如金融) → 使用分数运算
   - 需要固定小数位数的结果 → 使用精度计算
   - 常规科学计算 → 使用标准浮点数运算

2. **分数运算的限制**：
   - 分数可能变得很大 (如 `999999/2000000`)
   - 适合有限次运算，不适合循环中大量使用

3. **精度选择**：
   - 金融计算通常使用 2 位小数
   - 科学计算根据需求选择 4-6 位小数
   - 避免过高精度导致的性能问题

4. **类型转换**：
   - 分数和浮点数可以相互转换
   - 分数运算后可以用 `TO_FLOAT` 转换为易读的形式
