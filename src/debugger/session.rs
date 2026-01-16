// src/debugger/session.rs
//! Debugger session and command processing

use crate::debugger::breakpoint::BreakpointType;
use crate::debugger::state::{DebuggerState, ExecutionMode};
use crate::evaluator::Evaluator;
use std::cell::RefCell;
use std::rc::Rc;

/// Action to take after executing a command
#[derive(Debug, Clone, PartialEq)]
pub enum CommandAction {
    /// Continue execution (e.g., after step/next/continue)
    Continue,
    /// Stay in debugger REPL
    Stay,
    /// Quit debugger
    Quit,
}

/// Debugger session
pub struct DebuggerSession {
    evaluator: Rc<RefCell<Evaluator>>,
    state: DebuggerState,
    source_code: Option<String>,
    source_file: Option<String>,
}

impl DebuggerSession {
    /// Create a new debugger session
    pub fn new(evaluator: Rc<RefCell<Evaluator>>) -> Self {
        DebuggerSession {
            evaluator,
            state: DebuggerState::new(),
            source_code: None,
            source_file: None,
        }
    }

    /// Set the source code for listing
    pub fn set_source(&mut self, source: String, file: String) {
        self.source_code = Some(source);
        self.source_file = Some(file);
    }

    /// Start the debugger session
    pub fn start(&mut self) {
        self.state.activate();
        println!("Aether Debugger v1.0");
        if let Some(file) = &self.source_file {
            println!("Debugging: {}", file);
        }
        println!("Type 'help' for available commands\n");
    }

    /// Get a mutable reference to the debugger state
    pub fn state_mut(&mut self) -> &mut DebuggerState {
        &mut self.state
    }

    /// Get a reference to the debugger state
    pub fn state(&self) -> &DebuggerState {
        &self.state
    }

    /// Handle a debugger command, returning (message, action)
    pub fn handle_command(&mut self, cmd: &str) -> (String, CommandAction) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return (String::new(), CommandAction::Stay);
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        let (msg, action) = match command.as_str() {
            "break" | "b" => self.cmd_break(args),
            "delete" | "d" => self.cmd_delete(args),
            "disable" => self.cmd_disable(args),
            "enable" => self.cmd_enable(args),
            "info" => self.cmd_info(args),
            "step" | "s" => self.cmd_step(args),
            "next" | "n" => self.cmd_next(args),
            "finish" | "f" => self.cmd_finish(args),
            "continue" | "c" => self.cmd_continue(args),
            "print" | "p" => self.cmd_print(args),
            "backtrace" | "bt" => self.cmd_backtrace(args),
            "frame" => self.cmd_frame(args),
            "list" | "l" => self.cmd_list(args),
            "help" | "h" | "?" => self.cmd_help(args),
            "quit" | "q" => self.cmd_quit(args),
            _ => (
                format!(
                    "Unknown command: {}. Type 'help' for available commands.",
                    command
                ),
                CommandAction::Stay,
            ),
        };

        (msg, action)
    }

    // Command implementations

    fn cmd_break(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: break [file:]line | break function_name".to_string(),
                CommandAction::Stay,
            );
        }

        let loc = args[0];

        // Try parsing as line number first
        if let Ok(line) = loc.parse::<usize>() {
            // Line number only - use current file
            let file = self
                .state
                .current_location()
                .map(|(f, _)| f.clone())
                .or_else(|| self.source_file.clone())
                .unwrap_or_else(|| "<unknown>".to_string());

            let id = self.state.set_breakpoint(BreakpointType::Line {
                file: file.clone(),
                line,
            });
            return (
                format!("Breakpoint {} set at {}:{}", id, file, line),
                CommandAction::Stay,
            );
        }

        // Try file:line format
        if let Some(pos) = loc.find(':') {
            let file = loc[..pos].to_string();
            if let Ok(line) = loc[pos + 1..].parse::<usize>() {
                let id = self.state.set_breakpoint(BreakpointType::Line {
                    file: file.clone(),
                    line,
                });
                return (
                    format!("Breakpoint {} set at {}:{}", id, file, line),
                    CommandAction::Stay,
                );
            }
        }

        // Otherwise treat as function name
        let id = self.state.set_breakpoint(BreakpointType::Function {
            name: loc.to_string(),
        });
        (
            format!("Breakpoint {} set at function '{}'", id, loc),
            CommandAction::Stay,
        )
    }

    fn cmd_delete(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            let count = self.state.list_breakpoints().len();
            self.state.remove_all_breakpoints();
            return (
                format!("All breakpoints deleted ({})", count),
                CommandAction::Stay,
            );
        }

        if let Ok(id) = args[0].parse::<usize>() {
            if self.state.remove_breakpoint(id) {
                (format!("Breakpoint {} deleted", id), CommandAction::Stay)
            } else {
                (format!("Breakpoint {} not found", id), CommandAction::Stay)
            }
        } else {
            ("Invalid breakpoint ID".to_string(), CommandAction::Stay)
        }
    }

    fn cmd_disable(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: disable <breakpoint_id>".to_string(),
                CommandAction::Stay,
            );
        }

        if let Ok(id) = args[0].parse::<usize>() {
            if self.state.toggle_breakpoint(id, false) {
                (format!("Breakpoint {} disabled", id), CommandAction::Stay)
            } else {
                (format!("Breakpoint {} not found", id), CommandAction::Stay)
            }
        } else {
            ("Invalid breakpoint ID".to_string(), CommandAction::Stay)
        }
    }

    fn cmd_enable(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: enable <breakpoint_id>".to_string(),
                CommandAction::Stay,
            );
        }

        if let Ok(id) = args[0].parse::<usize>() {
            if self.state.toggle_breakpoint(id, true) {
                (format!("Breakpoint {} enabled", id), CommandAction::Stay)
            } else {
                (format!("Breakpoint {} not found", id), CommandAction::Stay)
            }
        } else {
            ("Invalid breakpoint ID".to_string(), CommandAction::Stay)
        }
    }

    fn cmd_info(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: info breakpoints | info locals | info args".to_string(),
                CommandAction::Stay,
            );
        }

        match args[0] {
            "breakpoints" | "break" | "bp" => {
                let breakpoints = self.state.list_breakpoints();
                if breakpoints.is_empty() {
                    return ("No breakpoints".to_string(), CommandAction::Stay);
                }

                let mut result = String::from("Breakpoints:\n");
                for bp in breakpoints {
                    let status = if bp.enabled { " enabled" } else { " disabled" };
                    result.push_str(&format!(
                        "  ID: {:3}{} | {} | hits: {} | {}\n",
                        bp.id,
                        status,
                        bp.location_string(),
                        bp.hit_count,
                        if bp.ignore_count > 0 {
                            format!("(ignore first {})", bp.ignore_count)
                        } else {
                            String::new()
                        }
                    ));
                }
                (result, CommandAction::Stay)
            }
            "locals" => {
                // TODO: Need to add API to Evaluator to get all variables
                (
                    "Local variables: Not yet implemented".to_string(),
                    CommandAction::Stay,
                )
            }
            "args" => (
                "Arguments: Not yet implemented".to_string(),
                CommandAction::Stay,
            ),
            _ => (
                format!("Unknown info command: {}", args[0]),
                CommandAction::Stay,
            ),
        }
    }

    fn cmd_step(&mut self, args: &[&str]) -> (String, CommandAction) {
        let _count = if args.is_empty() {
            1
        } else {
            args[0].parse::<usize>().unwrap_or(1)
        };

        self.state.set_execution_mode(ExecutionMode::StepInto);
        ("Stepping...".to_string(), CommandAction::Continue)
    }

    fn cmd_next(&mut self, args: &[&str]) -> (String, CommandAction) {
        let _count = if args.is_empty() {
            1
        } else {
            args[0].parse::<usize>().unwrap_or(1)
        };

        let evaluator = self.evaluator.borrow();
        let depth = evaluator.get_call_stack_depth();
        drop(evaluator);

        self.state.set_execution_mode(ExecutionMode::StepOver);
        self.state.set_step_over_depth(depth);
        ("Next...".to_string(), CommandAction::Continue)
    }

    fn cmd_finish(&mut self, _args: &[&str]) -> (String, CommandAction) {
        let evaluator = self.evaluator.borrow();
        let depth = evaluator.get_call_stack_depth();
        drop(evaluator);

        self.state.set_execution_mode(ExecutionMode::StepOut);
        self.state.set_step_over_depth(depth);
        (
            "Running until current function returns...".to_string(),
            CommandAction::Continue,
        )
    }

    fn cmd_continue(&mut self, _args: &[&str]) -> (String, CommandAction) {
        self.state.set_execution_mode(ExecutionMode::Continue);
        ("Continuing...".to_string(), CommandAction::Continue)
    }

    fn cmd_print(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: print <variable_name>".to_string(),
                CommandAction::Stay,
            );
        }

        let var_name = args[0];
        let evaluator = self.evaluator.borrow();

        match evaluator.get_global(var_name) {
            Some(value) => (format!("{} = {}", var_name, value), CommandAction::Stay),
            None => (
                format!("Variable '{}' not found", var_name),
                CommandAction::Stay,
            ),
        }
    }

    fn cmd_backtrace(&mut self, args: &[&str]) -> (String, CommandAction) {
        let evaluator = self.evaluator.borrow();
        let call_stack = evaluator.get_call_stack();

        if call_stack.is_empty() {
            drop(evaluator);
            return ("No stack.".to_string(), CommandAction::Stay);
        }

        let max_frames = if args.is_empty() {
            call_stack.len()
        } else {
            args[0].parse::<usize>().unwrap_or(call_stack.len())
        };

        let mut result = String::from("Call stack:\n");
        for (i, frame) in call_stack.iter().take(max_frames).enumerate() {
            result.push_str(&format!("#{} {}\n", i, frame.signature));
        }
        drop(evaluator);
        (result, CommandAction::Stay)
    }

    fn cmd_frame(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: frame <frame_number>".to_string(),
                CommandAction::Stay,
            );
        }

        if let Ok(_frame_num) = args[0].parse::<usize>() {
            (
                "Frame selection not yet implemented".to_string(),
                CommandAction::Stay,
            )
        } else {
            ("Invalid frame number".to_string(), CommandAction::Stay)
        }
    }

    fn cmd_list(&mut self, args: &[&str]) -> (String, CommandAction) {
        let count = if args.is_empty() {
            10
        } else {
            args[0].parse::<usize>().unwrap_or(10)
        };

        if let Some(source) = &self.source_code {
            let current_line = self
                .state
                .current_location()
                .map(|(_, line)| *line)
                .unwrap_or(1);

            let lines: Vec<&str> = source.lines().collect();
            let start = current_line.saturating_sub(count / 2);
            let end = (start + count).min(lines.len());

            let mut result = String::new();
            for (idx, line) in lines.iter().enumerate().take(end).skip(start) {
                let marker = if idx + 1 == current_line { "=>" } else { "  " };
                result.push_str(&format!("{} {:4}: {}\n", marker, idx + 1, line));
            }
            (result, CommandAction::Stay)
        } else {
            ("No source code available".to_string(), CommandAction::Stay)
        }
    }

    fn cmd_help(&mut self, _args: &[&str]) -> (String, CommandAction) {
        (HELP_TEXT.to_string(), CommandAction::Stay)
    }

    fn cmd_quit(&mut self, _args: &[&str]) -> (String, CommandAction) {
        ("Exiting debugger...".to_string(), CommandAction::Quit)
    }
}

const HELP_TEXT: &str = r#"
Aether Debugger Commands

Execution Control:
  step [N]        Step N times (default 1), stepping into function calls
  next [N]        Step N times (default 1), stepping over function calls
  finish          Execute until the current function returns
  continue        Continue execution until next breakpoint

Breakpoints:
  break [file:]line  Set breakpoint at line
  break function     Set breakpoint at function entry
  delete [N]         Delete breakpoint N (or all if N not specified)
  disable [N]        Disable breakpoint N
  enable [N]         Enable breakpoint N
  info breakpoints   List all breakpoints

Stack & Variables:
  backtrace [N]      Print backtrace of N frames (all if N not specified)
  frame N            Select and print stack frame N
  print expr         Print value of expression/variable
  info locals        Print local variables

Source:
  list [N]           List N lines of source (default 10)

Miscellaneous:
  help               Show this help message
  quit               Exit debugger

Examples:
  (aether-debug) break 15           # Set breakpoint at line 15
  (aether-debug) break calc.aether:20  # Set at file:line
  (aether-debug) break processData  # Set at function entry
  (aether-debug) next               # Step over
  (aether-debug) step               # Step into
  (aether-debug) print X            # Show variable X
  (aether-debug) backtrace          # Show call stack
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use std::cell::RefCell;

    fn create_test_session() -> DebuggerSession {
        let env = Rc::new(RefCell::new(Environment::new()));
        let evaluator = Rc::new(RefCell::new(Evaluator::with_env(env)));
        let mut session = DebuggerSession::new(evaluator);
        session.set_source(
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n".to_string(),
            "test.aether".to_string(),
        );
        session
    }

    #[test]
    fn test_break_command() {
        let mut session = create_test_session();

        let (result, _) = session.handle_command("break 10");
        assert!(result.contains("Breakpoint"));
    }

    #[test]
    fn test_step_command() {
        let mut session = create_test_session();

        let (_, action) = session.handle_command("step");
        assert_eq!(action, CommandAction::Continue);
        assert_eq!(session.state().execution_mode(), &ExecutionMode::StepInto);
    }

    #[test]
    fn test_next_command() {
        let mut session = create_test_session();

        let (_, action) = session.handle_command("next");
        assert_eq!(action, CommandAction::Continue);
        assert_eq!(session.state().execution_mode(), &ExecutionMode::StepOver);
    }

    #[test]
    fn test_continue_command() {
        let mut session = create_test_session();

        let (_, action) = session.handle_command("continue");
        assert_eq!(action, CommandAction::Continue);
        assert_eq!(session.state().execution_mode(), &ExecutionMode::Continue);
    }
}
