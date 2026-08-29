// src/debugger/session.rs
//! Debugger session and command processing

use crate::debugger::breakpoint::BreakpointType;
use crate::debugger::state::{DebuggerState, ExecutionMode};
use crate::evaluator::Evaluator;
use std::cell::RefCell;
use std::collections::HashSet;
use std::io::{self, Write};
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
///
/// 与 Evaluator 共享同一个 `DebuggerState`（`Rc<RefCell<>>`），
/// 因此命令对断点/执行模式的修改立即对求值器生效。
/// 求值器状态（变量、调用栈）在暂停回调时通过 `&mut Evaluator` 传入。
pub struct DebuggerSession {
    state: Rc<RefCell<DebuggerState>>,
    source_code: Option<String>,
    source_file: Option<String>,
    /// 可命中的语句起始行集合（由 CLI 从 AST 收集注入，用于断点行校验）
    breakable_lines: Option<HashSet<usize>>,
    /// 用户在暂停交互中发出 quit 后置位（宿主据此区分 DebugPause 与真实错误）
    terminated: bool,
}

impl DebuggerSession {
    /// Create a new debugger session around a shared debugger state
    pub fn new(state: Rc<RefCell<DebuggerState>>) -> Self {
        DebuggerSession {
            state,
            source_code: None,
            source_file: None,
            breakable_lines: None,
            terminated: false,
        }
    }

    /// Set the source code for listing
    pub fn set_source(&mut self, source: String, file: String) {
        self.source_code = Some(source);
        self.source_file = Some(file);
    }

    /// 注入可命中的语句起始行集合（供 `break N` 校验）
    pub fn set_breakable_lines(&mut self, lines: HashSet<usize>) {
        self.breakable_lines = Some(lines);
    }

    /// 共享的调试器状态（供宿主调用 `Evaluator::attach_debugger`）
    pub fn shared_state(&self) -> Rc<RefCell<DebuggerState>> {
        Rc::clone(&self.state)
    }

    /// 用户是否已通过 quit 终止程序（宿主据此把 DebugPause 排除出错误处理）
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }

    /// Start the debugger session
    pub fn start(&mut self) {
        self.state.borrow_mut().activate();
        println!("Aether Debugger v1.0");
        if let Some(file) = &self.source_file {
            println!("Debugging: {}", file);
        }
        println!("Type 'help' for available commands\n");
    }

    /// 暂停时的交互循环：打印位置与源码上下文，逐条读取命令直到继续/退出。
    ///
    /// 返回 true 表示终止程序（求值器将以 DebugPause 传播）。
    /// stdin 读到 EOF 时停用调试器并放行到程序结束，避免无限 EOF 循环。
    pub fn run_pause_loop(&mut self, ev: &mut Evaluator) -> bool {
        let (file, line) = self
            .state
            .borrow()
            .current_location()
            .cloned()
            .unwrap_or_else(|| ("<unknown>".to_string(), 0));
        println!("\nPaused at {}:{}\n", file, line);
        print!("{}", self.listing(9));

        loop {
            print!("(aether-debug) ");
            let _ = io::stdout().flush();

            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => {
                    self.state.borrow_mut().deactivate();
                    println!("\n(Input closed; running to program end)");
                    return false;
                }
                Ok(_) => {}
                Err(_) => return false,
            }

            let (msg, action) = self.handle_command(ev, line.trim());
            if !msg.is_empty() {
                println!("{}", msg);
            }
            match action {
                CommandAction::Continue => return false,
                CommandAction::Stay => continue,
                CommandAction::Quit => {
                    self.terminated = true;
                    return true;
                }
            }
        }
    }

    /// Handle a debugger command, returning (message, action)
    pub fn handle_command(&mut self, ev: &mut Evaluator, cmd: &str) -> (String, CommandAction) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return (String::new(), CommandAction::Stay);
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        let (msg, action) = match command.as_str() {
            "break" | "b" => self.cmd_break(ev, args),
            "delete" | "d" => self.cmd_delete(args),
            "disable" => self.cmd_disable(args),
            "enable" => self.cmd_enable(args),
            "info" => self.cmd_info(args),
            "step" | "s" => self.cmd_step(args),
            "next" | "n" => self.cmd_next(ev, args),
            "finish" | "f" => self.cmd_finish(ev),
            "continue" | "c" => self.cmd_continue(),
            "print" | "p" => self.cmd_print(ev, args),
            "backtrace" | "bt" => self.cmd_backtrace(ev, args),
            "frame" => self.cmd_frame(args),
            "list" | "l" => self.cmd_list(args),
            "help" | "h" | "?" => self.cmd_help(),
            "quit" | "q" => self.cmd_quit(),
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

    fn cmd_break(&mut self, ev: &mut Evaluator, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: break [file:]line [if condition] | break function_name".to_string(),
                CommandAction::Stay,
            );
        }

        let loc = args[0];
        // `break <line> if <cond>`：位置命中后用当前环境求值条件，
        // 为真才暂停（条件以空格重组，含空格的表达式可直接书写）
        let condition: Option<String> = if args.len() >= 3 && args[1] == "if" {
            Some(args[2..].join(" "))
        } else {
            None
        };

        // Try parsing as line number first
        if let Ok(line) = loc.parse::<usize>() {
            // Line number only - use current file
            let file = ev
                .get_source_file()
                .map(str::to_string)
                .or_else(|| {
                    self.state
                        .borrow()
                        .current_location()
                        .map(|(f, _)| f.clone())
                })
                .or_else(|| self.source_file.clone())
                .unwrap_or_else(|| "<unknown>".to_string());

            let id = self.state.borrow_mut().set_breakpoint(match &condition {
                Some(cond) => BreakpointType::Conditional {
                    file: file.clone(),
                    line,
                    condition: cond.clone(),
                },
                None => BreakpointType::Line {
                    file: file.clone(),
                    line,
                },
            });

            // 校验该行是否存在语句起点（允许设置，仅提示可能永不命中）
            let unbreakable = self
                .breakable_lines
                .as_ref()
                .is_some_and(|lines| !lines.contains(&line));
            let mut msg = format!("Breakpoint {} set at {}:{}", id, file, line);
            if let Some(cond) = &condition {
                msg.push_str(&format!(" if {cond}"));
            }
            if unbreakable {
                msg.push_str(&format!(
                    "\nNote: line {} has no statement start; breakpoint may never trigger",
                    line
                ));
            }
            return (msg, CommandAction::Stay);
        }

        // Try file:line format
        if let Some(pos) = loc.find(':') {
            let file = loc[..pos].to_string();
            if let Ok(line) = loc[pos + 1..].parse::<usize>() {
                let id = self.state.borrow_mut().set_breakpoint(match &condition {
                    Some(cond) => BreakpointType::Conditional {
                        file: file.clone(),
                        line,
                        condition: cond.clone(),
                    },
                    None => BreakpointType::Line {
                        file: file.clone(),
                        line,
                    },
                });
                return (
                    format!(
                        "Breakpoint {} set at {}:{}{}",
                        id,
                        file,
                        line,
                        condition
                            .as_ref()
                            .map(|c| format!(" if {c}"))
                            .unwrap_or_default()
                    ),
                    CommandAction::Stay,
                );
            }
        }

        // Otherwise treat as function name
        let id = self
            .state
            .borrow_mut()
            .set_breakpoint(BreakpointType::Function {
                name: loc.to_string(),
            });
        (
            format!("Breakpoint {} set at function '{}'", id, loc),
            CommandAction::Stay,
        )
    }

    fn cmd_delete(&mut self, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            let count = self.state.borrow().list_breakpoints().len();
            self.state.borrow_mut().remove_all_breakpoints();
            return (
                format!("All breakpoints deleted ({})", count),
                CommandAction::Stay,
            );
        }

        if let Ok(id) = args[0].parse::<usize>() {
            if self.state.borrow_mut().remove_breakpoint(id) {
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
            if self.state.borrow_mut().toggle_breakpoint(id, false) {
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
            if self.state.borrow_mut().toggle_breakpoint(id, true) {
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
                let state = self.state.borrow();
                let breakpoints = state.list_breakpoints();
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

        self.state
            .borrow_mut()
            .set_execution_mode(ExecutionMode::StepInto);
        ("Stepping...".to_string(), CommandAction::Continue)
    }

    fn cmd_next(&mut self, ev: &mut Evaluator, args: &[&str]) -> (String, CommandAction) {
        let _count = if args.is_empty() {
            1
        } else {
            args[0].parse::<usize>().unwrap_or(1)
        };

        let depth = ev.get_call_stack_depth();

        let mut state = self.state.borrow_mut();
        state.set_execution_mode(ExecutionMode::StepOver);
        state.set_step_over_depth(depth);
        ("Next...".to_string(), CommandAction::Continue)
    }

    fn cmd_finish(&mut self, ev: &mut Evaluator) -> (String, CommandAction) {
        let depth = ev.get_call_stack_depth();

        let mut state = self.state.borrow_mut();
        state.set_execution_mode(ExecutionMode::StepOut);
        state.set_step_over_depth(depth);
        (
            "Running until current function returns...".to_string(),
            CommandAction::Continue,
        )
    }

    fn cmd_continue(&mut self) -> (String, CommandAction) {
        self.state
            .borrow_mut()
            .set_execution_mode(ExecutionMode::Continue);
        ("Continuing...".to_string(), CommandAction::Continue)
    }

    fn cmd_print(&mut self, ev: &mut Evaluator, args: &[&str]) -> (String, CommandAction) {
        if args.is_empty() {
            return (
                "Usage: print <variable_name>".to_string(),
                CommandAction::Stay,
            );
        }

        let var_name = args[0];

        // 走作用域链查找：暂停在函数内时也能看到局部变量与参数
        match ev.lookup_variable(var_name) {
            Some(value) => (format!("{} = {}", var_name, value), CommandAction::Stay),
            None => (
                format!("Variable '{}' not found", var_name),
                CommandAction::Stay,
            ),
        }
    }

    fn cmd_backtrace(&mut self, ev: &mut Evaluator, args: &[&str]) -> (String, CommandAction) {
        let call_stack = ev.get_call_stack();

        if call_stack.is_empty() {
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

        (self.listing(count), CommandAction::Stay)
    }

    /// 围绕当前行格式化源码片段（`=>` 标记当前行）。
    ///
    /// 暂停在别的文件（如 Import 的模块）时没有对应源码，明确提示而非错列主文件
    fn listing(&self, count: usize) -> String {
        let Some(source) = &self.source_code else {
            return "No source code available".to_string();
        };

        let location = self.state.borrow().current_location().cloned();
        if let Some((file, _)) = &location
            && let Some(loaded) = &self.source_file
        {
            fn base(p: &str) -> &str {
                p.rsplit(['/', '\\']).next().unwrap_or(p)
            }
            if base(file) != base(loaded) {
                return format!("No source available for {}\n", file);
            }
        }

        let current_line = location.map(|(_, line)| line).unwrap_or(1);

        let lines: Vec<&str> = source.lines().collect();
        let start = current_line.saturating_sub(count / 2);
        let end = (start + count).min(lines.len());

        let mut result = String::new();
        for (idx, line) in lines.iter().enumerate().take(end).skip(start) {
            let marker = if idx + 1 == current_line { "=>" } else { "  " };
            result.push_str(&format!("{} {:4}: {}\n", marker, idx + 1, line));
        }
        result
    }

    fn cmd_help(&mut self) -> (String, CommandAction) {
        (HELP_TEXT.to_string(), CommandAction::Stay)
    }

    fn cmd_quit(&mut self) -> (String, CommandAction) {
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

    fn create_test_session() -> DebuggerSession {
        let state = Rc::new(RefCell::new(DebuggerState::new()));
        let mut session = DebuggerSession::new(state);
        session.set_source(
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n".to_string(),
            "test.aether".to_string(),
        );
        session
    }

    fn create_test_evaluator() -> Evaluator {
        let env = Rc::new(RefCell::new(Environment::new()));
        Evaluator::with_env(env)
    }

    #[test]
    fn test_break_command() {
        let mut session = create_test_session();
        let mut ev = create_test_evaluator();

        let (result, _) = session.handle_command(&mut ev, "break 10");
        assert!(result.contains("Breakpoint"));
    }

    #[test]
    fn test_break_command_warns_on_unbreakable_line() {
        let mut session = create_test_session();
        session.set_breakable_lines(HashSet::from([1, 3]));
        let mut ev = create_test_evaluator();

        let (result, _) = session.handle_command(&mut ev, "break 2");
        assert!(result.contains("Breakpoint"));
        assert!(result.contains("may never trigger"));

        let (result, _) = session.handle_command(&mut ev, "break 3");
        assert!(result.contains("Breakpoint"));
        assert!(!result.contains("may never trigger"));
    }

    #[test]
    fn test_break_uses_evaluator_source_file() {
        let mut session = create_test_session();
        let mut ev = create_test_evaluator();
        ev.set_source_file("main.aether".to_string());

        let (result, _) = session.handle_command(&mut ev, "break 4");
        assert!(result.contains("main.aether:4"));
    }

    #[test]
    fn test_step_command() {
        let mut session = create_test_session();
        let mut ev = create_test_evaluator();

        let (_, action) = session.handle_command(&mut ev, "step");
        assert_eq!(action, CommandAction::Continue);
        assert_eq!(
            session.state.borrow().execution_mode(),
            &ExecutionMode::StepInto
        );
    }

    #[test]
    fn test_next_command() {
        let mut session = create_test_session();
        let mut ev = create_test_evaluator();

        let (_, action) = session.handle_command(&mut ev, "next");
        assert_eq!(action, CommandAction::Continue);
        assert_eq!(
            session.state.borrow().execution_mode(),
            &ExecutionMode::StepOver
        );
    }

    #[test]
    fn test_continue_command() {
        let mut session = create_test_session();
        let mut ev = create_test_evaluator();

        let (_, action) = session.handle_command(&mut ev, "continue");
        assert_eq!(action, CommandAction::Continue);
        assert_eq!(
            session.state.borrow().execution_mode(),
            &ExecutionMode::Continue
        );
    }

    #[test]
    fn test_print_variable_via_scope_chain() {
        let mut session = create_test_session();
        let mut ev = create_test_evaluator();
        ev.set_global("X".to_string(), crate::value::Value::Number(42.0));

        let (result, _) = session.handle_command(&mut ev, "print X");
        assert_eq!(result, "X = 42");

        let (result, _) = session.handle_command(&mut ev, "print Y");
        assert!(result.contains("not found"));
    }

    #[test]
    fn test_shared_state_visible_to_both_sides() {
        // 会话与求值器共享同一状态实例：attach 后命令修改立即可见
        let mut session = create_test_session();
        let mut ev = create_test_evaluator();
        let shared = session.shared_state();
        ev.attach_debugger(Rc::clone(&shared), Box::new(|_ev| false));

        assert!(ev.debugger_attached());
        session.handle_command(&mut ev, "break 2");
        assert!(shared.borrow().is_active());
        assert_eq!(shared.borrow().list_breakpoints().len(), 1);

        ev.detach_debugger();
        assert!(!ev.debugger_attached());
        assert!(!shared.borrow().is_active());
    }
}
