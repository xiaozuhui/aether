// src/cli/debugger.rs
//! Debugger CLI implementation

use aether::ast::collect_breakable_lines;
use aether::debugger::{CommandAction, DebuggerSession, DebuggerState};
use aether::{Aether, FileSystemModuleResolver, Parser};
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

    // 与普通运行一致的引擎（标准库 + 文件系统模块解析）
    let mut engine = match Aether::with_stdlib() {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("警告: 标准库加载失败: {}", e);
            eprintln!("继续运行但不加载标准库...");
            Aether::with_all_permissions()
        }
    };
    engine.set_module_resolver(Box::new(FileSystemModuleResolver::default()));
    engine.set_source_file(filename.to_string());

    // 解析源码收集可命中行（优化 pass 保留行号，此集合对优化后的 AST 同样成立）
    let breakable_lines = Parser::new(&source)
        .parse_program()
        .map(|program| collect_breakable_lines(&program))
        .unwrap_or_default();

    let mut session = DebuggerSession::new(Rc::new(RefCell::new(DebuggerState::new())));
    session.set_source(source.clone(), filename.to_string());
    session.set_breakable_lines(breakable_lines);
    session.start();

    // 挂载调试钩子：求值器命中断点/步进时回调，进入暂停交互循环；
    // 返回 true 表示用户 quit，程序以 DebugPause 终止
    let session_rc = Rc::new(RefCell::new(session));
    let hook_session = Rc::clone(&session_rc);
    engine.attach_debugger(
        session_rc.borrow().shared_state(),
        Box::new(move |ev| hook_session.borrow_mut().run_pause_loop(ev)),
    );

    // 运行前的首层 REPL：先设置断点，continue/step 启动执行
    loop {
        print!("(aether-debug) ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)：退出且不执行程序
                println!("\nExiting debugger without running the program.");
                return;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let (msg, action) = {
            let mut session = session_rc.borrow_mut();
            let ev = engine.evaluator_mut();
            session.handle_command(ev, input)
        };

        if !msg.is_empty() {
            println!("{}", msg);
        }

        match action {
            CommandAction::Continue => break,
            CommandAction::Quit => {
                println!("Exiting debugger...");
                return;
            }
            CommandAction::Stay => {}
        }
    }

    // 执行程序：断点/步进暂停发生在钩子内的交互循环，返回后从暂停语句自然继续
    match engine.eval_file(filename) {
        Ok(result) => {
            if result != aether::Value::Null {
                println!("{}", result);
            }
            println!("\nProgram finished.");
        }
        Err(e) => {
            if session_rc.borrow().is_terminated() {
                println!("\nProgram terminated by debugger.");
            } else {
                eprintln!("✗ 运行时错误:");
                crate::cli::error_context::print_detailed_error(&source, &e);
                std::process::exit(1);
            }
        }
    }
}
