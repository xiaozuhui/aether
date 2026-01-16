mod args;
mod error_context;
mod file_cmd;
mod help;
mod metrics;
mod repl;
mod runner;

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    match args::parse(&args) {
        args::CliCommand::Repl => repl::run_repl(),
        args::CliCommand::Help => help::print_cli_help(),
        args::CliCommand::Check { file } => file_cmd::check_file(&file),
        args::CliCommand::Ast { file } => file_cmd::show_ast_for_file(&file),
        args::CliCommand::Run { file, options } => runner::run_file(&file, options),
        args::CliCommand::Error { message } => {
            eprintln!("{}", message);
            eprintln!("使用 --help 查看帮助");
            std::process::exit(1);
        }
    }
}
