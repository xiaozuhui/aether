use crate::cli::{args::RunOptions, error_context, metrics};
use aether::{Aether, FileSystemModuleResolver};
use serde_json::json;
use std::fs;

pub fn run_file(filename: &str, options: RunOptions) {
    // Check if debugger mode is enabled
    if options.debugger_mode {
        crate::cli::debugger::run_debugger(filename);
        return;
    }

    let mut engine = if options.load_stdlib {
        match Aether::with_stdlib() {
            Ok(engine) => engine,
            Err(e) => {
                eprintln!("警告: 标准库加载失败: {}", e);
                eprintln!("继续运行但不加载标准库...");
                Aether::with_all_permissions()
            }
        }
    } else {
        Aether::with_all_permissions()
    };

    if options.debug_mode {
        println!("=== 调试模式 ===");
        println!("文件: {}", filename);
        println!(
            "标准库: {}",
            if options.load_stdlib {
                "已加载"
            } else {
                "未加载"
            }
        );
        println!();
    }

    engine.set_module_resolver(Box::new(FileSystemModuleResolver::default()));

    if let Some(size) = options.trace_buffer_size {
        engine.set_trace_buffer_size(size);
        if options.debug_mode {
            println!("TRACE 缓冲区大小: {}", size);
            println!();
        }
    }

    if options.json_error {
        let start = std::time::Instant::now();
        let cache_before = engine.cache_stats();
        match engine.eval_file_report(filename) {
            Ok(result) => {
                if options.metrics_json_mode {
                    metrics::print_metrics_json(
                        &engine,
                        start.elapsed(),
                        cache_before,
                        result,
                        options.metrics_json_pretty_mode,
                    );
                    return;
                }

                if options.debug_mode {
                    println!("=== 执行结果 ===");
                }
                if result != aether::Value::Null {
                    println!("{}", result);
                }

                if options.metrics_mode {
                    let elapsed = start.elapsed();
                    let cache_after = engine.cache_stats();
                    let trace_stats = engine.trace_stats();
                    let step_count = engine.step_count();
                    metrics::print_metrics(
                        elapsed,
                        &cache_before,
                        &cache_after,
                        &trace_stats,
                        step_count,
                    );
                }

                if options.debug_mode {
                    println!("\n=== 执行完成 ===");
                }
            }
            Err(report) => {
                if options.metrics_json_mode {
                    let payload = json!({
                        "ok": false,
                        "error": report.to_json_value(),
                    });
                    metrics::print_json(payload, options.metrics_json_pretty_mode);
                    std::process::exit(1);
                }

                eprintln!("{}", report.to_json_pretty());
                std::process::exit(1);
            }
        }
        return;
    }

    let start = std::time::Instant::now();
    let cache_before = engine.cache_stats();
    match engine.eval_file(filename) {
        Ok(result) => {
            if options.metrics_json_mode {
                metrics::print_metrics_json(
                    &engine,
                    start.elapsed(),
                    cache_before,
                    result,
                    options.metrics_json_pretty_mode,
                );
                return;
            }

            if options.debug_mode {
                println!("=== 执行结果 ===");
            }
            if result != aether::Value::Null {
                println!("{}", result);
            }

            if options.metrics_mode {
                let elapsed = start.elapsed();
                let cache_after = engine.cache_stats();
                let trace_stats = engine.trace_stats();
                let step_count = engine.step_count();
                metrics::print_metrics(
                    elapsed,
                    &cache_before,
                    &cache_after,
                    &trace_stats,
                    step_count,
                );
            }

            if options.debug_mode {
                println!("\n=== 执行完成 ===");
            }

            if options.show_trace {
                let trace = engine.take_trace();
                println!("=== TRACE ===");
                if trace.is_empty() {
                    println!("(empty)");
                } else {
                    for line in trace {
                        println!("{}", line);
                    }
                }
                println!();
            }

            if options.show_trace_stats {
                let stats = engine.trace_stats();
                println!("=== TRACE STATS ===");
                println!("buffer_size: {}", stats.buffer_size);
                println!("total_entries: {}", stats.total_entries);
                println!("buffer_full: {}", stats.buffer_full);
                println!("by_level: {:?}", stats.by_level);
                println!("by_category: {:?}", stats.by_category);
                println!();
            }
        }
        Err(e) => {
            if options.metrics_json_mode {
                let payload = json!({
                    "ok": false,
                    "error": e,
                });
                metrics::print_json(payload, options.metrics_json_pretty_mode);
                std::process::exit(1);
            }

            eprintln!("✗ 运行时错误:");

            if let Ok(code) = fs::read_to_string(filename) {
                error_context::print_detailed_error(&code, &e);
            } else {
                eprintln!("{}", e);
            }

            std::process::exit(1);
        }
    }
}
