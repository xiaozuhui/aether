use aether::Aether;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

/// 基准测试：基本算术运算
fn bench_arithmetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic");

    group.bench_function("simple_addition", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("(1 + 2)")).unwrap();
        });
    });

    group.bench_function("complex_expression", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("((10 + 20) * 3 - 5) / 2")).unwrap();
        });
    });

    group.bench_function("nested_arithmetic", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box("(((1 + 2) * (3 + 4)) - ((5 + 6) / (7 - 5)))"))
                .unwrap();
        });
    });

    group.finish();
}

/// 基准测试：变量操作
fn bench_variables(c: &mut Criterion) {
    let mut group = c.benchmark_group("variables");

    group.bench_function("variable_assignment", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("Set X 100")).unwrap();
        });
    });

    group.bench_function("variable_read", |b| {
        let mut engine = Aether::new();
        engine.eval("Set X 100").unwrap();
        b.iter(|| {
            engine.eval(black_box("X")).unwrap();
        });
    });

    group.bench_function("multiple_variables", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(
                    r#"
                Set A 10
                Set B 20
                Set C 30
                (A + B + C)
            "#,
                ))
                .unwrap();
        });
    });

    group.finish();
}

/// 基准测试：函数调用
fn bench_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("functions");

    group.bench_function("builtin_function", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("PRINTLN(\"Hello\")")).unwrap();
        });
    });

    group.bench_function("user_function_call", |b| {
        let mut engine = Aether::new();
        engine
            .eval(
                r#"
            Func ADD (A, B) {
                Return (A + B)
            }
        "#,
            )
            .unwrap();
        b.iter(|| {
            engine.eval(black_box("ADD(10, 20)")).unwrap();
        });
    });

    group.bench_function("recursive_function", |b| {
        let mut engine = Aether::new();
        engine
            .eval(
                r#"
            Func FIB (N) {
                If (N <= 1) {
                    Return N
                }
                Return (FIB((N - 1)) + FIB((N - 2)))
            }
        "#,
            )
            .unwrap();
        b.iter(|| {
            engine.eval(black_box("FIB(10)")).unwrap();
        });
    });

    group.finish();
}

/// 基准测试：控制流
fn bench_control_flow(c: &mut Criterion) {
    let mut group = c.benchmark_group("control_flow");

    group.bench_function("if_statement", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(
                    r#"
                Set X 10
                If (X > 5) {
                    (X * 2)
                } Else {
                    (X / 2)
                }
            "#,
                ))
                .unwrap();
        });
    });

    group.bench_function("while_loop", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(
                    r#"
                Set I 0
                Set SUM 0
                While (I < 10) {
                    Set SUM (SUM + I)
                    Set I (I + 1)
                }
                SUM
            "#,
                ))
                .unwrap();
        });
    });

    group.bench_function("for_loop", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(
                    r#"
                Set SUM 0
                For I In RANGE(0, 10) {
                    Set SUM (SUM + I)
                }
                SUM
            "#,
                ))
                .unwrap();
        });
    });

    group.finish();
}

/// 基准测试：数组操作
fn bench_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("arrays");

    group.bench_function("array_creation", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("[1, 2, 3, 4, 5]")).unwrap();
        });
    });

    group.bench_function("array_access", |b| {
        let mut engine = Aether::new();
        engine.eval("Set ARR [10, 20, 30, 40, 50]").unwrap();
        b.iter(|| {
            engine.eval(black_box("ARR[2]")).unwrap();
        });
    });

    group.bench_function("array_iteration", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(
                    r#"
                Set ARR [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
                Set SUM 0
                For ITEM In ARR {
                    Set SUM (SUM + ITEM)
                }
                SUM
            "#,
                ))
                .unwrap();
        });
    });

    group.bench_function("array_map", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(
                    r#"
                Set ARR [1, 2, 3, 4, 5]
                Set RESULT []
                For ITEM In ARR {
                    Set RESULT (PUSH(RESULT, (ITEM * 2)))
                }
                RESULT
            "#,
                ))
                .unwrap();
        });
    });

    group.finish();
}

/// 基准测试：字典操作
fn bench_dictionaries(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionaries");

    group.bench_function("dict_creation", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(r#"{"name": "Alice", "age": 30}"#))
                .unwrap();
        });
    });

    group.bench_function("dict_access", |b| {
        let mut engine = Aether::new();
        engine
            .eval(r#"Set PERSON {"name": "Bob", "age": 25}"#)
            .unwrap();
        b.iter(|| {
            engine.eval(black_box("PERSON[\"name\"]")).unwrap();
        });
    });

    group.bench_function("dict_modification", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(
                    r#"
                Set PERSON {"name": "Charlie", "age": 35}
                Set PERSON {"name": "Charlie", "age": 36}
                PERSON["age"]
            "#,
                ))
                .unwrap();
        });
    });

    group.finish();
}

/// 基准测试：字符串操作
fn bench_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("strings");

    group.bench_function("string_concatenation", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(r#"("Hello" + " " + "World")"#))
                .unwrap();
        });
    });

    group.bench_function("string_length", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box(r#"LEN("Hello World")"#)).unwrap();
        });
    });

    group.bench_function("string_split", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine
                .eval(black_box(r#"SPLIT("a,b,c,d,e", ",")"#))
                .unwrap();
        });
    });

    group.finish();
}

/// 基准测试：精确数学运算
fn bench_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("precision");

    group.bench_function("fraction_conversion", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("TO_FRACTION(0.333333)")).unwrap();
        });
    });

    group.bench_function("large_number_arithmetic", |b| {
        let mut engine = Aether::new();
        b.iter(|| {
            engine.eval(black_box("(123456789 * 987654321)")).unwrap();
        });
    });

    group.finish();
}

/// 基准测试：不同规模的程序
fn bench_program_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("program_sizes");

    // 小型程序
    group.bench_function("small_program", |b| {
        let mut engine = Aether::new();
        let code = r#"
            Set X 10
            Set Y 20
            (X + Y)
        "#;
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    // 中型程序
    group.bench_function("medium_program", |b| {
        let mut engine = Aether::new();
        let code = r#"
            Func FACTORIAL (N) {
                If (N <= 1) {
                    Return 1
                }
                Return (N * FACTORIAL((N - 1)))
            }
            
            Set RESULT 1
            For I In RANGE(1, 11) {
                Set RESULT (RESULT + FACTORIAL(I))
            }
            RESULT
        "#;
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    // 大型程序
    group.bench_function("large_program", |b| {
        let mut engine = Aether::new();
        let code = r#"
            Set DATA []
            For I In RANGE(0, 100) {
                Set DATA (PUSH(DATA, {"id": I, "value": (I * 2)}))
            }

            Func PROCESS_DATA (ITEMS) {
                Set RESULT []
                For ITEM In ITEMS {
                    If (ITEM["value"] > 50) {
                        Set RESULT (PUSH(RESULT, ITEM))
                    }
                }
                Return RESULT
            }

            Set FILTERED PROCESS_DATA(DATA)
            LEN(FILTERED)
        "#;
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    group.finish();
}

/// 基准测试：解析性能（独立测试词法分析和语法分析）
fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");

    group.bench_function("lexer_only", |b| {
        use aether::{Lexer, Token};
        let code = r#"
            Set X 10
            Set Y 20
            Func ADD (A, B) {
                Return (A + B)
            }
            ADD(X, Y)
        "#;
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(code));
            loop {
                let token = lexer.next_token();
                if token == Token::EOF {
                    break;
                }
            }
        });
    });

    group.bench_function("parser_only", |b| {
        use aether::Parser;
        let code = r#"
            Set X 10
            Set Y 20
            Func ADD (A, B) {
                Return (A + B)
            }
            ADD(X, Y)
        "#;
        b.iter(|| {
            let mut parser = Parser::new(black_box(code));
            parser.parse_program().unwrap();
        });
    });

    group.bench_function("parse_and_eval", |b| {
        let mut engine = Aether::new();
        let code = r#"
            Set X 10
            Set Y 20
            Func ADD (A, B) {
                Return (A + B)
            }
            ADD(X, Y)
        "#;
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    group.finish();
}

/// 基准测试：不同大小的斐波那契数列（测试递归性能）
fn bench_fibonacci_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    for n in [5, 10, 15].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            let mut engine = Aether::new();
            engine
                .eval(
                    r#"
                Func FIB (N) {
                    If (N <= 1) {
                        Return N
                    }
                    Return (FIB((N - 1)) + FIB((N - 2)))
                }
            "#,
                )
                .unwrap();

            b.iter(|| {
                engine.eval(black_box(&format!("FIB({})", n))).unwrap();
            });
        });
    }

    group.finish();
}

/// 基准测试：工资计算模块（如果启用）
fn bench_payroll(c: &mut Criterion) {
    let mut group = c.benchmark_group("payroll");

    group.bench_function("basic_payroll_calculation", |b| {
        let mut engine = Aether::new();
        let code = r#"
            Set BASIC_SALARY 5000
            Set BONUS 1000
            Set DEDUCTION 500
            (BASIC_SALARY + BONUS - DEDUCTION)
        "#;
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    group.finish();
}

/// 基准测试：尾递归优化
fn bench_tail_recursion(c: &mut Criterion) {
    let mut group = c.benchmark_group("tail_recursion");

    group.bench_function("tail_recursive_factorial_10", |b| {
        let mut engine = Aether::new();
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
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    group.bench_function("tail_recursive_sum_100", |b| {
        let mut engine = Aether::new();
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
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    group.bench_function("tail_recursive_fibonacci_20", |b| {
        let mut engine = Aether::new();
        let code = r#"
            Func FIB(N, A, B) {
                If (N == 0) {
                    Return A
                } Else {
                    Return FIB(N - 1, B, A + B)
                }
            }
            FIB(20, 0, 1)
        "#;
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    group.bench_function("tail_recursive_deep_1000", |b| {
        let mut engine = Aether::new();
        let code = r#"
            Func COUNTDOWN(N, ACC) {
                If (N <= 0) {
                    Return ACC
                } Else {
                    Return COUNTDOWN(N - 1, ACC + 1)
                }
            }
            COUNTDOWN(1000, 0)
        "#;
        b.iter(|| {
            engine.eval(black_box(code)).unwrap();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_arithmetic,
    bench_variables,
    bench_functions,
    bench_control_flow,
    bench_arrays,
    bench_dictionaries,
    bench_strings,
    bench_precision,
    bench_program_sizes,
    bench_parsing,
    bench_fibonacci_sizes,
    bench_payroll,
    bench_tail_recursion
);

criterion_main!(benches);
