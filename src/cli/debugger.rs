// src/cli/debugger.rs
//! Debugger CLI implementation

use aether::debugger::{CommandAction, DebuggerSession};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

pub fn run_debugger(filename: &str) {
    // Read source code
    let source = match std::fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    // Create a new evaluator for the debugger
    use aether::environment::Environment;
    use aether::evaluator::Evaluator;

    let env = Rc::new(RefCell::new(Environment::new()));
    let evaluator = Rc::new(RefCell::new(Evaluator::with_env(env)));

    // Set source file in evaluator
    evaluator.borrow_mut().set_source_file(filename.to_string());

    let mut session = DebuggerSession::new(evaluator);
    session.set_source(source.clone(), filename.to_string());
    session.start();

    println!("\nDebugger ready. Type 'help' for commands.");
    println!("Note: This is a basic implementation. For full debugging support,");
    println!("the Evaluator needs to be enhanced to support pausing and resuming.");

    // Main debugger REPL loop
    loop {
        print!("(aether-debug) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!("\nExiting debugger...");
                break;
            }
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                let (msg, action) = session.handle_command(input);

                if !msg.is_empty() {
                    println!("{}", msg);
                }

                match action {
                    CommandAction::Continue => {
                        println!("Note: Full execution control is not yet implemented.");
                        println!("The program will run to completion.");

                        // For now, just try to execute the source
                        if let Err(e) = execute_program(&source) {
                            eprintln!("Execution error: {}", e);
                        }

                        show_current_context(&session, &source);
                    }
                    CommandAction::Quit => {
                        break;
                    }
                    CommandAction::Stay => {
                        // Stay in debugger REPL
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn execute_program(source: &str) -> Result<(), String> {
    use aether::Aether;

    let mut engine = Aether::new();
    engine.eval(source)?;
    Ok(())
}

fn show_current_context(session: &DebuggerSession, source: &str) {
    if let Some((file, line)) = session.state().current_location() {
        println!("\nAt {}:{}:", file, line);

        // Show context around current line
        let lines: Vec<&str> = source.lines().collect();
        let start = line.saturating_sub(3);
        let end = (line + 2).min(lines.len());

        for (idx, line_content) in lines.iter().enumerate().take(end).skip(start) {
            let marker = if idx + 1 == *line { "=>" } else { "  " };
            println!("{} {:4}: {}", marker, idx + 1, line_content);
        }
    }
}
