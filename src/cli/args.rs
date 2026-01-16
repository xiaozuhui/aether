#[derive(Debug, Clone)]
pub struct RunOptions {
    pub load_stdlib: bool,
    pub debug_mode: bool,
    pub json_error: bool,
    pub metrics_mode: bool,
    pub metrics_json_mode: bool,
    pub metrics_json_pretty_mode: bool,
    pub show_trace: bool,
    pub show_trace_stats: bool,
    pub trace_buffer_size: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum CliCommand {
    Repl,
    Help,
    Check { file: String },
    Ast { file: String },
    Run { file: String, options: RunOptions },
    Error { message: String },
}

pub fn parse(args: &[String]) -> CliCommand {
    if args.len() <= 1 {
        return CliCommand::Repl;
    }

    // Flags
    let load_stdlib = !args.contains(&"--no-stdlib".to_string());
    let show_ast = args.contains(&"--ast".to_string());
    let check_only = args.contains(&"--check".to_string());
    let debug_mode = args.contains(&"--debug".to_string());

    let metrics_mode = args.contains(&"--metrics".to_string());
    let metrics_json_mode = args.contains(&"--metrics-json".to_string());
    let metrics_json_pretty_mode = args.contains(&"--metrics-json-pretty".to_string());
    let metrics_json_output = metrics_json_mode || metrics_json_pretty_mode;

    let show_trace = args.contains(&"--trace".to_string());
    let show_trace_stats = args.contains(&"--trace-stats".to_string());
    let trace_buffer_size = get_usize_flag_value(args, "--trace-buffer-size");

    let json_error = args.contains(&"--json-error".to_string());
    let show_help = args.contains(&"--help".to_string()) || args.contains(&"-h".to_string());

    if show_help {
        return CliCommand::Help;
    }

    let script_file = find_script_file(args);
    let Some(file) = script_file else {
        return CliCommand::Error {
            message: "错误: 未指定脚本文件".to_string(),
        };
    };

    if check_only {
        return CliCommand::Check {
            file: file.to_string(),
        };
    }

    if show_ast {
        return CliCommand::Ast {
            file: file.to_string(),
        };
    }

    CliCommand::Run {
        file: file.to_string(),
        options: RunOptions {
            load_stdlib,
            debug_mode,
            json_error,
            metrics_mode,
            metrics_json_mode: metrics_json_output,
            metrics_json_pretty_mode,
            show_trace,
            show_trace_stats,
            trace_buffer_size,
        },
    }
}

fn get_usize_flag_value(args: &[String], flag: &str) -> Option<usize> {
    args.iter().position(|a| a == flag).and_then(|idx| {
        args.get(idx + 1)
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|v| *v > 0)
    })
}

fn find_script_file(args: &[String]) -> Option<&str> {
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];

        // Flags with a following value
        if arg == "--trace-buffer-size" {
            i += 2;
            continue;
        }

        if arg.starts_with("--") {
            i += 1;
            continue;
        }

        if arg.starts_with('-') {
            i += 1;
            continue;
        }

        return Some(arg.as_str());
    }

    None
}
