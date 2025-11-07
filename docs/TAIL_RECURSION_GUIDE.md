# Aether 尾递归优化指南

## 概述

尾递归优化（Tail Call Optimization, TCO）是一种编译器优化技术，它将尾递归函数转换为迭代循环，从而避免栈溢出并提高性能。

Aether语言内置了完整的尾递归优化功能，可以自动识别并优化尾递归函数。

## 什么是尾递归？

**尾递归**是指函数的最后一个操作是调用自身，没有任何后续操作。

### 尾递归示例

```aether
# 尾递归阶乘函数
Func factorial(n, acc) {
    If (n <= 1) {
        Return acc
    } Else {
        Return factorial(n - 1, acc * n)  # 尾调用
    }
}
```

在这个例子中，`factorial(n - 1, acc * n)` 是函数的最后一个操作，因此这是一个尾递归函数。

### 非尾递归示例

```aether
# 非尾递归阶乘函数
Func factorial(n) {
    If (n <= 1) {
        Return 1
    } Else {
        Return n * factorial(n - 1)  # 递归调用后还有乘法操作
    }
}
```

在这个例子中，递归调用 `factorial(n - 1)` 之后还需要执行乘法操作 `n *`，因此这不是尾递归。

## 尾递归优化原理

尾递归优化将递归函数转换为等价的循环形式。以下是转换过程：

### 转换前（递归）

```aether
Func sum(n, acc) {
    If (n <= 0) {
        Return acc
    } Else {
        Return sum(n - 1, acc + n)
    }
}
```

### 转换后（循环）

```aether
Func sum(n, acc) {
    Set _loop_n = n
    Set _loop_acc = acc
    Set _loop_continue = True
    
    While (_loop_continue) {
        If (n <= 0) {
            Set _loop_continue = False
            Return acc
        } Else {
            # 更新参数
            Set _loop_n = n - 1
            Set _loop_acc = acc + n
            Set n = _loop_n
            Set acc = _loop_acc
            # 继续循环
        }
    }
}
```

优化后的函数使用循环而不是递归调用，避免了栈溢出问题。

## 优化器使用

### 启用优化

Aether的优化器默认启用尾递归优化。你可以通过以下方式控制：

```rust
use aether::optimizer::Optimizer;

// 创建优化器（默认启用所有优化）
let optimizer = Optimizer::new();

// 或者自定义优化选项
let mut optimizer = Optimizer {
    tail_recursion: true,        // 启用尾递归优化
    constant_folding: true,       // 启用常量折叠
    dead_code_elimination: true,  // 启用死代码消除
};

// 优化程序
let optimized_program = optimizer.optimize_program(&program);
```

### 检测尾递归

优化器会自动检测以下类型的尾递归：

1. **简单尾递归**：函数直接返回递归调用结果
2. **条件尾递归**：在If表达式的所有分支中返回递归调用
3. **多分支尾递归**：在多个elif分支中都有尾递归调用

## 实际应用示例

### 示例1: 累加器模式

将普通递归转换为尾递归的常见模式是使用累加器（accumulator）：

```aether
# 普通递归（可能栈溢出）
Func sum_to_n(n) {
    If (n <= 0) {
        Return 0
    } Else {
        Return n + sum_to_n(n - 1)
    }
}

# 尾递归版本（优化后变为循环）
Func sum_to_n_tail(n, acc) {
    If (n <= 0) {
        Return acc
    } Else {
        Return sum_to_n_tail(n - 1, acc + n)
    }
}

# 调用方式
Set result = sum_to_n_tail(10000, 0)  # 可以处理大数
```

### 示例2: 斐波那契数列

```aether
# 尾递归斐波那契
Func fib(n, a, b) {
    If (n == 0) {
        Return a
    } Else {
        Return fib(n - 1, b, a + b)
    }
}

# 计算第1000个斐波那契数
Set result = fib(1000, 0, 1)
```

### 示例3: 列表操作

```aether
# 尾递归列表反转
Func reverse_list(lst, acc) {
    If (Length(lst) == 0) {
        Return acc
    } Else {
        Set head = First(lst)
        Set tail = Rest(lst)
        Return reverse_list(tail, Prepend(head, acc))
    }
}

Set original = [1, 2, 3, 4, 5]
Set reversed = reverse_list(original, [])
```

### 示例4: 条件分支中的尾递归

```aether
# Collatz序列（3n+1问题）
Func collatz(n, steps) {
    If (n == 1) {
        Return steps
    } Else {
        If (n % 2 == 0) {
            Return collatz(n / 2, steps + 1)
        } Else {
            Return collatz(3 * n + 1, steps + 1)
        }
    }
}

Set result = collatz(27, 0)
```

## 性能优势

尾递归优化提供以下性能优势：

1. **避免栈溢出**：递归深度不再受栈大小限制
2. **减少内存使用**：不需要保存多个栈帧
3. **提高执行速度**：循环通常比函数调用更快
4. **支持大数处理**：可以处理数千甚至数万次迭代

### 性能对比

```aether
# 未优化的递归：可能在n=1000时栈溢出
Func sum_recursive(n) {
    If (n <= 0) { Return 0 }
    Else { Return n + sum_recursive(n - 1) }
}

# 尾递归优化版本：可以处理n=10000或更大
Func sum_tail(n, acc) {
    If (n <= 0) { Return acc }
    Else { Return sum_tail(n - 1, acc + n) }
}
```

## 优化限制

并非所有递归函数都能被优化为尾递归：

### 不能优化的情况

1. **递归调用后有操作**

```aether
Func factorial(n) {
    If (n <= 1) { Return 1 }
    Else { Return n * factorial(n - 1) }  # 递归后还有乘法
}
```

2. **多个递归调用**

```aether
Func fib(n) {
    If (n <= 1) { Return n }
    Else { Return fib(n-1) + fib(n-2) }  # 两个递归调用
}
```

3. **相互递归**（目前版本暂不支持）

```aether
Func even(n) {
    If (n == 0) { Return True }
    Else { Return odd(n - 1) }
}

Func odd(n) {
    If (n == 0) { Return False }
    Else { Return even(n - 1) }
}
```

### 如何改写为尾递归

大多数递归函数可以通过引入累加器参数改写为尾递归：

```aether
# 原始递归
Func factorial(n) {
    If (n <= 1) { Return 1 }
    Else { Return n * factorial(n - 1) }
}

# 尾递归版本（添加累加器）
Func factorial_tail(n, acc) {
    If (n <= 1) { Return acc }
    Else { Return factorial_tail(n - 1, n * acc) }
}

# 包装函数
Func factorial(n) {
    Return factorial_tail(n, 1)
}
```

## 调试优化

如果你想查看优化器是否识别了尾递归，可以启用调试输出：

```rust
use aether::optimizer::Optimizer;
use aether::parser::Parser;

let code = "Func fact(n,a){If(n<=1){Return a}Else{Return fact(n-1,n*a)}}";
let program = Parser::new(code).parse().unwrap();

let optimizer = Optimizer::new();
let optimized = optimizer.optimize_program(&program);

// 检查是否包含While循环（优化标志）
// 优化后的函数体会包含While语句
```

## 最佳实践

1. **优先使用尾递归**：在编写递归函数时，尽量使用尾递归模式
2. **使用累加器**：通过累加器参数传递中间结果
3. **提供包装函数**：为尾递归函数提供友好的包装接口
4. **测试大数据**：确保函数能处理大规模数据而不会栈溢出
5. **性能测试**：使用benchmarks比较优化前后的性能

## 参考资料

- [examples/test_tail_recursion.aether](../examples/test_tail_recursion.aether) - 完整测试用例
- [src/optimizer.rs](../src/optimizer.rs) - 优化器实现源码
- [DEVELOPMENT.md](../DEVELOPMENT.md) - 开发者文档

## 总结

Aether的尾递归优化使得递归编程更加实用和高效。通过将递归转换为循环，你可以编写优雅的递归代码，同时享受迭代的性能优势。

记住：只要函数的最后一个操作是调用自身，优化器就会自动将其转换为高效的循环形式！
