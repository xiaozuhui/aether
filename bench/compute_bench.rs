//! Aether 解释器计算基准（无第三方依赖，稳定 Rust 可直接 `cargo bench`）
//!
//! # 设计
//!
//! - **每个场景都在 DSL 侧做真实计算**，并在 Rust 侧用独立实现的镜像
//!   （LCG 复算、num-bigint 阶乘、闭式求和、逐条复写薪酬公式）精确校验
//!   结果——基准同时是语义回归网，跑分失败会以非零退出码结束；
//! - **计时口径**：单引擎复用 + 同源 AST 缓存命中，度量纯解释执行；
//!   `parse_cold` 场景例外，每次生成唯一源码，度量 lexer+parser 全链路；
//! - 深递归场景在 64MB 大栈线程内执行（树遍历解释器的 Rust 递归帧较深）；
//! - 规模可用环境变量 `AETHER_BENCH_SCALE` 缩放（默认 1.0，如
//!   `AETHER_BENCH_SCALE=0.2 cargo bench` 快速冒烟）。
//!
//! # 运行
//!
//! ```sh
//! cargo bench                 # 全部场景
//! AETHER_BENCH_ONLY=data cargo bench    # 只跑名字含 data 的场景
//! ```

use aether::{Aether, ExecutionLimits, Value};
use std::time::{Duration, Instant};

/// 按次校验结果的闭包（rep 用于唯一源码的期望值）
type Validate = Box<dyn Fn(&Value, usize) -> Result<(), String>>;

/// 单个基准场景：按次生成源码并按次校验结果
struct Scenario {
    /// 场景名（表格第一列）
    name: &'static str,
    /// 一句话说明计算内容与规模
    desc: &'static str,
    /// 计时重复次数（受 AETHER_BENCH_SCALE 缩放）
    reps: usize,
    /// 第 rep 次的 DSL 源码（多数场景忽略 rep；parse_cold 用它保证源码唯一）
    code: Box<dyn Fn(usize) -> String>,
    /// 校验该次结果
    validate: Validate,
}

fn main() {
    // 深尾递归场景的 Rust 递归帧在 debug 构建下很大；统一放大线程栈
    let worker = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(run_all)
        .expect("启动基准线程失败");
    let ok = worker.join().expect("基准线程崩溃");
    if !ok {
        std::process::exit(1);
    }
}

fn run_all() -> bool {
    let scale: f64 = std::env::var("AETHER_BENCH_SCALE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    let only = std::env::var("AETHER_BENCH_ONLY").ok();
    let scenarios: Vec<Scenario> = scenarios()
        .into_iter()
        .filter(|s| only.as_deref().is_none_or(|k| s.name.contains(k)))
        .collect();
    if scenarios.is_empty() {
        eprintln!("AETHER_BENCH_ONLY 未匹配任何场景");
        return false;
    }

    println!("Aether 计算基准（cargo bench = 优化构建；引擎复用、AST 缓存命中）");
    println!("scale = {scale}（AETHER_BENCH_SCALE 可缩放重复次数）\n");

    let mut failures: Vec<String> = Vec::new();
    println!(
        "{:<22} {:>8} {:>10} {:>12} {:>12} {:>12} {:>10}",
        "场景", "次数", "总耗时", "中位/次", "最小/次", "p95/次", "吞吐(/s)"
    );
    println!("{}", "-".repeat(92));

    for sc in &scenarios {
        let reps = ((sc.reps as f64) * scale).ceil().max(1.0) as usize;
        match run_scenario(sc, reps) {
            Ok(stats) => {
                println!(
                    "{:<22} {:>8} {:>10} {:>12} {:>12} {:>12} {:>10.1}",
                    sc.name,
                    reps,
                    fmt_dur(stats.total),
                    fmt_dur(stats.median),
                    fmt_dur(stats.min),
                    fmt_dur(stats.p95),
                    1.0 / stats.mean.as_secs_f64(),
                );
                eprintln!("  ✓ {}：{}", sc.name, sc.desc);
            }
            Err(e) => {
                println!("{:<22} {:>8} {:>10}", sc.name, reps, "FAILED");
                failures.push(format!("{}: {e}", sc.name));
            }
        }
    }

    println!("{}", "-".repeat(92));
    if failures.is_empty() {
        println!("全部 {} 个场景结果校验通过", scenarios.len());
        true
    } else {
        println!("{} 个场景失败：", failures.len());
        for f in &failures {
            eprintln!("  ✗ {f}");
        }
        false
    }
}

struct Stats {
    total: Duration,
    min: Duration,
    median: Duration,
    p95: Duration,
    mean: Duration,
}

fn run_scenario(sc: &Scenario, reps: usize) -> Result<Stats, String> {
    // 每个场景独立引擎；同一源码重复求值 → AST 缓存命中，度量纯执行
    let mut engine = Aether::new().with_limits(ExecutionLimits::unrestricted());

    // 预热 + 校验第 0 次（失败直接报错，不进入计时）
    let warm = (sc.code)(0);
    let v = engine.eval(&warm).map_err(|e| format!("预热执行失败: {e}"))?;
    (sc.validate)(&v, 0)?;

    let mut durations = Vec::with_capacity(reps);
    let total_start = Instant::now();
    for rep in 0..reps {
        let code = (sc.code)(rep);
        let start = Instant::now();
        let v = engine.eval(&code).map_err(|e| format!("第 {rep} 次执行失败: {e}"))?;
        durations.push(start.elapsed());
        (sc.validate)(&v, rep)?;
    }
    let total = total_start.elapsed();

    let mut sorted = durations.clone();
    sorted.sort();
    let n = sorted.len();
    let pick = |p: usize| sorted[p.min(n - 1)];
    let sum: Duration = durations.iter().sum();
    Ok(Stats {
        total,
        min: sorted[0],
        median: pick(n / 2),
        p95: pick(n * 95 / 100),
        mean: sum / n as u32,
    })
}

fn as_number(v: &Value) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        other => Err(format!("期望 Number，得到 {}", other.type_name())),
    }
}

fn fmt_dur(d: Duration) -> String {
    let us = d.as_micros() as f64;
    if us < 1_000.0 {
        format!("{us:.0}µs")
    } else if us < 1_000_000.0 {
        format!("{:.1}ms", us / 1_000.0)
    } else {
        format!("{:.2}s", us / 1_000_000.0)
    }
}

// ============================================================
// Rust 侧镜像：独立于 DSL 实现复算期望值（语义回归网）
// ============================================================

/// 与 DSL 场景完全相同的迷你 LCG（乘数 75 / 增量 74 / 模 65537，
/// 全程 32 位内整数，f64 精确），复算落在单位圆内的点数
fn monte_carlo_inside(n: u64) -> f64 {
    let mut seed: u64 = 1;
    let mut inside: u64 = 0;
    for _ in 0..n {
        seed = (seed * 75 + 74) % 65537;
        let x = (seed % 10_000) as f64 / 10_000.0;
        seed = (seed * 75 + 74) % 65537;
        let y = (seed % 10_000) as f64 / 10_000.0;
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    (inside as f64 / n as f64) * 4.0
}

/// num-bigint 阶乘（十进制字符串），校验 DSL 大整数累乘
fn factorial_string(n: u64) -> String {
    use num_bigint::BigUint;
    let mut f = BigUint::from(1u32);
    for i in 1..=n {
        f *= BigUint::from(i);
    }
    f.to_string()
}

/// 调和级数 f64 镜像（校验分数精确累加后转浮点的值）
fn harmonic_f64(n: u64) -> f64 {
    (1..=n).map(|i| 1.0 / i as f64).sum()
}

/// 个税年度 7 档累进表（与 payroll::tax::annual_tax 逐行对应）
fn annual_tax(taxable: f64) -> f64 {
    if taxable <= 0.0 {
        return 0.0;
    }
    let tax = if taxable <= 36_000.0 {
        taxable * 0.03
    } else if taxable <= 144_000.0 {
        taxable * 0.10 - 2_520.0
    } else if taxable <= 300_000.0 {
        taxable * 0.20 - 16_920.0
    } else if taxable <= 420_000.0 {
        taxable * 0.25 - 31_920.0
    } else if taxable <= 660_000.0 {
        taxable * 0.30 - 52_920.0
    } else if taxable <= 960_000.0 {
        taxable * 0.35 - 85_920.0
    } else {
        taxable * 0.45 - 181_920.0
    };
    tax.max(0.0)
}

/// 工资单总额镜像：逐条复写场景脚本调用的 6 个 CALC_* 公式
fn payroll_total(n: u64) -> f64 {
    let mut total = 0.0;
    for i in 1..=n {
        let base = 8000.0 + (i % 50) as f64 * 100.0;
        // CALC_WEEKDAY_OVERTIME：时薪 = 月薪/21.75/8，1.5 倍 ×10 小时
        let ot = base / 21.75 / 8.0 * 10.0 * 1.5;
        // CALC_MEAL_ALLOWANCE：20 元/天 × 22 天
        let meal = 20.0 * 22.0;
        // CALC_GROSS_SALARY(base, ot, bonus=0, allowance)
        let gross = base + ot + 0.0 + meal;
        // CALC_SOCIAL_INSURANCE 默认费率 8%+2%+0.5%+12%
        let social = base * (0.08 + 0.02 + 0.005 + 0.12);
        // CALC_TAXABLE_INCOME(gross, social, housing=0)：减起征点 5000
        let taxable = (gross - social - 0.0 - 5000.0).max(0.0);
        // CALC_PERSONAL_TAX（taxable≤0 时 annual_tax 恒 0，无需分支）
        let tax = annual_tax(taxable);
        // CALC_NET_SALARY(gross, social, housing=0, tax, other=0)
        let net = (gross - social - 0.0 - tax - 0.0).max(0.0);
        total += net;
    }
    total
}

/// 平方和闭式：n(n+1)(2n+1)/6（校验生成器迭代求和）
fn sum_squares(n: u64) -> f64 {
    (n * (n + 1) * (2 * n + 1) / 6) as f64
}

// ============================================================
// 场景定义
// ============================================================

fn scenarios() -> Vec<Scenario> {
    vec![
        Scenario {
            name: "monte_carlo_pi",
            desc: "蒙特卡洛 π：LCG 伪随机 40000 点落点判定（循环+取模+分支）",
            reps: 30,
            code: Box::new(|_| {
                r#"
Set SEED 1
Set INSIDE 0
Set I 0
While (I < 40000) {
    Set I I + 1
    Set SEED (SEED * 75 + 74) % 65537
    Set X (SEED % 10000) / 10000
    Set SEED (SEED * 75 + 74) % 65537
    Set Y (SEED % 10000) / 10000
    If (X * X + Y * Y <= 1) {
        Set INSIDE INSIDE + 1
    }
}
(INSIDE / 40000) * 4
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                let want = monte_carlo_inside(40_000);
                if (got - want).abs() < 1e-12 {
                    Ok(())
                } else {
                    Err(format!("π 估计 {got} ≠ 镜像 {want}"))
                }
            }),
        },
        Scenario {
            name: "big_factorial",
            desc: "1200! 大整数累乘（Fraction/BigInt 提升 + 整数快速乘路径），校验全部 3175 位",
            reps: 50,
            code: Box::new(|_| {
                r#"
Set ACC 1
Set I 1
While (I < 1200) {
    Set I I + 1
    Set ACC ACC * I
}
TO_STRING(ACC)
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = v.to_string();
                let want = factorial_string(1200);
                if got == want {
                    Ok(())
                } else {
                    Err(format!(
                        "1200! 不匹配：得到 {} 位，期望 {} 位",
                        got.len(),
                        want.len()
                    ))
                }
            }),
        },
        Scenario {
            name: "fraction_harmonic",
            desc: "调和级数 H_300 分数精确累加（FRAC_ADD/FRAC_DIV + gcd 规约）",
            reps: 50,
            code: Box::new(|_| {
                r#"
Set H TO_FRACTION(0)
Set I 0
While (I < 300) {
    Set I I + 1
    Set H FRAC_ADD(H, FRAC_DIV(TO_FRACTION(1), TO_FRACTION(I)))
}
TO_FLOAT(H)
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                let want = harmonic_f64(300);
                if (got - want).abs() < 1e-9 {
                    Ok(())
                } else {
                    Err(format!("H_300 = {got}，镜像 {want}"))
                }
            }),
        },
        Scenario {
            name: "tail_fib",
            desc: "尾递归斐波那契 FIB(70)（TCO 改写为循环的调用路径）",
            reps: 2000,
            code: Box::new(|_| {
                r#"
Func FIB(N, A, B) {
    If (N == 0) {
        Return A
    }
    Return FIB(N - 1, B, A + B)
}
FIB(70, 0, 1)
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                // fib(70) = 190392490709135 < 2^53，f64 精确
                if got == 190_392_490_709_135.0 {
                    Ok(())
                } else {
                    Err(format!("FIB(70) = {got}，期望 190392490709135"))
                }
            }),
        },
        Scenario {
            name: "deep_tail_sum",
            desc: "20000 层深尾递归 SUM（验证 TCO 循环化，无原生栈增长）",
            reps: 10,
            code: Box::new(|_| {
                r#"
Func SUM(N, ACC) {
    If (N <= 1) {
        Return ACC
    }
    Return SUM(N - 1, ACC + 1)
}
SUM(20000, 0)
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                if got == 19_999.0 {
                    Ok(())
                } else {
                    Err(format!("SUM(20000,0) = {got}，期望 19999"))
                }
            }),
        },
        Scenario {
            name: "data_structures",
            desc: "500 元素数组：PUSH 构建 + MAP/FILTER/REDUCE/SORT 管道",
            reps: 100,
            code: Box::new(|_| {
                r#"
Set DATA []
Set I 0
While (I < 500) {
    Set I I + 1
    Set DATA PUSH(DATA, (I * 37) % 500)
}
Set MAPPED MAP(DATA, Lambda X -> X * 2)
Set FILTERED FILTER(DATA, Lambda X -> X > 250)
Set SUMV REDUCE(DATA, Lambda (A, B) -> A + B, 0)
Set SORTED SORT(DATA)
SORTED[0] + SUMV + LEN(FILTERED) + LEN(MAPPED)
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                let vals: Vec<i64> = (1..=500).map(|i| (i * 37) % 500).collect();
                let want = vals.iter().min().copied().unwrap() as f64
                    + vals.iter().sum::<i64>() as f64
                    + vals.iter().filter(|&&x| x > 250).count() as f64
                    + 500.0;
                if got == want {
                    Ok(())
                } else {
                    Err(format!("min+sum+count = {got}，镜像 {want}"))
                }
            }),
        },
        Scenario {
            name: "string_build",
            desc: "字符串增长循环：400 次拼接 + STRSLICE 截断（字符语义）",
            reps: 100,
            code: Box::new(|_| {
                r#"
Set S "aether"
Set I 0
While (I < 400) {
    Set I I + 1
    Set S S + TO_STRING(I % 10)
    Set S STRSLICE(S, 0, 200)
}
LEN(S)
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                // 增长到 200 后每轮「拼 1 截 1」长度稳定
                if got == 200.0 {
                    Ok(())
                } else {
                    Err(format!("LEN(S) = {got}，期望 200"))
                }
            }),
        },
        Scenario {
            name: "payroll_batch",
            desc: "1000 人月度工资单：加班费/津贴/社保/个税/净薪 6 个 CALC_* 全链路",
            reps: 20,
            code: Box::new(|_| {
                r#"
Func PAYROLL(BASE) {
    Set OT CALC_WEEKDAY_OVERTIME(BASE, 10)
    Set MEAL CALC_MEAL_ALLOWANCE(20, 22)
    Set GROSS CALC_GROSS_SALARY(BASE, OT, 0, MEAL)
    Set SOCIAL CALC_SOCIAL_INSURANCE(BASE)
    Set TAXABLE CALC_TAXABLE_INCOME(GROSS, SOCIAL, 0)
    Set TAX CALC_PERSONAL_TAX(TAXABLE)
    Set NET CALC_NET_SALARY(GROSS, SOCIAL, 0, TAX, 0)
    Return NET
}
Set TOTAL 0
Set I 0
While (I < 1000) {
    Set I I + 1
    Set TOTAL TOTAL + PAYROLL(8000 + (I % 50) * 100)
}
TOTAL
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                let want = payroll_total(1000);
                if (got - want).abs() < 1e-6 {
                    Ok(())
                } else {
                    Err(format!("工资单总额 {got}，镜像 {want}"))
                }
            }),
        },
        Scenario {
            name: "generator_squares",
            desc: "生成器 5000 个 Yield 被 For-In 急切收集，逐元素平方累加",
            reps: 50,
            code: Box::new(|_| {
                r#"
Generator SEQ(START, END) {
    Set CUR START
    While (CUR <= END) {
        Yield CUR
        Set CUR CUR + 1
    }
}
Set TOTAL 0
For X In SEQ(1, 5000) {
    Set TOTAL TOTAL + X * X
}
TOTAL
"#
                .to_string()
            }),
            validate: Box::new(|v, _| {
                let got = as_number(v)?;
                let want = sum_squares(5000);
                if got == want {
                    Ok(())
                } else {
                    Err(format!("平方和 {got}，闭式 {want}"))
                }
            }),
        },
        Scenario {
            name: "parse_cold",
            desc: "冷解析：每次唯一源码（lexer→parser→AST 缓存插入），无缓存命中",
            reps: 20000,
            code: Box::new(|rep| {
                // 唯一源码：强制走完整解析路径并令 AST 缓存 LRU 持续淘汰
                let k = 100_000 + rep;
                format!("Set X {k}\n(X * 2) + {k}")
            }),
            validate: Box::new(|v, rep| {
                let k = (100_000 + rep) as f64;
                let got = as_number(v)?;
                if got == 3.0 * k {
                    Ok(())
                } else {
                    Err(format!("得到 {got}，期望 {}", 3.0 * k))
                }
            }),
        },
    ]
}
