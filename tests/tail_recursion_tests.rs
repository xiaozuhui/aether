// tests/tail_recursion_tests.rs
//! 尾递归优化集成测试

use aether::{Aether, Value};

#[test]
fn test_tail_recursive_factorial() {
    let code = r#"
        Func FACTORIAL(N, ACC) {
            If (N <= 1) {
                Return ACC
            } Else {
                Return FACTORIAL(N - 1, ACC * N)
            }
        }
        
        FACTORIAL(10, 1)
    "#;

    let mut engine = Aether::new();
    let result = engine.eval(code).expect("Failed to evaluate");

    if let Value::Number(n) = result {
        assert_eq!(n, 3628800.0); // 10! = 3628800
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_tail_recursive_sum() {
    let code = r#"
        Func SUM_TO_N(N, ACC) {
            If (N <= 0) {
                Return ACC
            } Else {
                Return SUM_TO_N(N - 1, ACC + N)
            }
        }
        
        SUM_TO_N(100, 0)
    "#;

    let mut engine = Aether::new();
    let result = engine.eval(code).expect("Failed to evaluate");

    if let Value::Number(n) = result {
        assert_eq!(n, 5050.0); // sum(1..100) = 5050
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_tail_recursive_fibonacci() {
    let code = r#"
        Func FIB(N, A, B) {
            If (N == 0) {
                Return A
            } Else {
                Return FIB(N - 1, B, A + B)
            }
        }
        
        FIB(10, 0, 1)
    "#;

    let mut engine = Aether::new();
    let result = engine.eval(code).expect("Failed to evaluate");

    if let Value::Number(n) = result {
        assert_eq!(n, 55.0); // fib(10) = 55
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_large_tail_recursion() {
    // 测试大数递归，确保不会栈溢出
    let code = r#"
        Func SUM_LARGE(N, ACC) {
            If (N <= 0) {
                Return ACC
            } Else {
                Return SUM_LARGE(N - 1, ACC + N)
            }
        }
        
        SUM_LARGE(1000, 0)
    "#;

    let mut engine = Aether::new();
    let result = engine
        .eval(code)
        .expect("Failed to evaluate large recursion");

    if let Value::Number(n) = result {
        assert_eq!(n, 500500.0); // sum(1..1000) = 500500
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_very_large_tail_recursion() {
    // 测试非常大的递归深度
    let code = r#"
        Func COUNTDOWN(N, ACC) {
            If (N <= 0) {
                Return ACC
            } Else {
                Return COUNTDOWN(N - 1, ACC + 1)
            }
        }
        
        COUNTDOWN(5000, 0)
    "#;

    let mut engine = Aether::new();
    let result = engine
        .eval(code)
        .expect("Should handle 5000 iterations without stack overflow");

    if let Value::Number(n) = result {
        assert_eq!(n, 5000.0);
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_non_tail_recursion_still_works() {
    // 非尾递归函数应该仍然能正常工作（只是不会被优化）
    let code = r#"
        Func FACTORIAL(N) {
            If (N <= 1) {
                Return 1
            } Else {
                Return N * FACTORIAL(N - 1)
            }
        }
        
        FACTORIAL(5)
    "#;

    let mut engine = Aether::new();
    let result = engine.eval(code).expect("Failed to evaluate");

    if let Value::Number(n) = result {
        assert_eq!(n, 120.0); // 5! = 120
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_conditional_tail_recursion() {
    let code = r#"
        Func COLLATZ(N, STEPS) {
            If (N == 1) {
                Return STEPS
            } Else {
                If (N % 2 == 0) {
                    Return COLLATZ(N / 2, STEPS + 1)
                } Else {
                    Return COLLATZ(3 * N + 1, STEPS + 1)
                }
            }
        }
        
        COLLATZ(27, 0)
    "#;

    let mut engine = Aether::new();
    let result = engine.eval(code).expect("Failed to evaluate");

    // Collatz序列: 27需要111步到达1
    if let Value::Number(n) = result {
        assert_eq!(n, 111.0);
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_mutual_recursion_helper() {
    // 测试带辅助函数的尾递归（GCD算法）
    let code = r#"
        Func GCD(A, B) {
            If (B == 0) {
                Return A
            } Else {
                Return GCD(B, A % B)
            }
        }
        
        GCD(48, 18)
    "#;

    let mut engine = Aether::new();
    let result = engine.eval(code).expect("Failed to evaluate");

    if let Value::Number(n) = result {
        assert_eq!(n, 6.0); // gcd(48, 18) = 6
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}

#[test]
fn test_tail_recursion_with_accumulator() {
    // 测试使用数字累加器的尾递归计数
    let code = r#"
        Func COUNT_DOWN(N, ACC) {
            If (N <= 0) {
                Return ACC
            } Else {
                Return COUNT_DOWN(N - 1, ACC + 1)
            }
        }
        
        COUNT_DOWN(100, 0)
    "#;

    let mut engine = Aether::new();
    let result = engine.eval(code).expect("Failed to evaluate");

    if let Value::Number(n) = result {
        assert_eq!(n, 100.0);
    } else {
        panic!("Expected number result, got {:?}", result);
    }
}
