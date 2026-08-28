//! 调试器集成测试：断点触发、单步执行、暂停/恢复与变量检查
//!
//! 这些测试通过 `Aether::attach_debugger` 挂载自定义钩子驱动调试状态机，
//! 不经过 stdin 交互（交互循环由 CLI 层负责）。

use aether::ast::collect_breakable_lines;
use aether::debugger::{BreakpointType, DebuggerState, ExecutionMode};
use aether::{Aether, Parser, Value};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

fn debug_engine(
    state: Rc<RefCell<DebuggerState>>,
    hook: Box<dyn FnMut(&mut aether::evaluator::Evaluator) -> bool>,
) -> Aether {
    let mut engine = Aether::new();
    engine.set_source_file("test.aether".to_string());
    engine.attach_debugger(state, hook);
    engine
}

/// 记录暂停行号的钩子：每次暂停记录 `ev.get_current_line()`，然后按 `next_mode` 继续
fn recording_hook(
    state: &Rc<RefCell<DebuggerState>>,
    paused: &Rc<RefCell<Vec<usize>>>,
    next_mode: ExecutionMode,
) -> Box<dyn FnMut(&mut aether::evaluator::Evaluator) -> bool> {
    let state = Rc::clone(state);
    let paused = Rc::clone(paused);
    Box::new(move |ev| {
        paused.borrow_mut().push(ev.get_current_line());
        state.borrow_mut().set_execution_mode(next_mode.clone());
        false
    })
}

#[test]
fn line_breakpoint_pauses_at_correct_line_and_resumes() {
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_breakpoint(BreakpointType::Line {
        file: "test.aether".to_string(),
        line: 2,
    });

    let paused = Rc::new(RefCell::new(Vec::new()));
    let mut engine = debug_engine(Rc::clone(&state), recording_hook(&state, &paused, ExecutionMode::Continue));

    let result = engine.eval("Set X 10\nSet Y (X + 5)\nY").unwrap();

    assert_eq!(result, Value::Number(15.0));
    // 只应暂停一次：第 2 行命中断点，之后以 Continue 模式放行
    assert_eq!(*paused.borrow(), vec![2]);
}

#[test]
fn breakpoint_file_matching_falls_back_to_file_name() {
    // 求值器报告 /abs/path/test.aether，断点只写文件名，仍应命中
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_breakpoint(BreakpointType::Line {
        file: "test.aether".to_string(),
        line: 1,
    });

    let paused = Rc::new(RefCell::new(Vec::new()));
    let mut engine = {
        let mut engine = Aether::new();
        engine.set_source_file("/abs/path/test.aether".to_string());
        let s = Rc::clone(&state);
        let p = Rc::clone(&paused);
        engine.attach_debugger(
            state,
            Box::new(move |ev| {
                p.borrow_mut().push(ev.get_current_line());
                s.borrow_mut().set_execution_mode(ExecutionMode::Continue);
                false
            }),
        );
        engine
    };

    let result = engine.eval("Set X 1\nX").unwrap();
    assert_eq!(result, Value::Number(1.0));
    assert_eq!(*paused.borrow(), vec![1]);
}

#[test]
fn step_into_records_every_statement() {
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_execution_mode(ExecutionMode::StepInto);

    let paused = Rc::new(RefCell::new(Vec::new()));
    let mut engine = debug_engine(Rc::clone(&state), recording_hook(&state, &paused, ExecutionMode::StepInto));

    let result = engine
        .eval("Set X 1\nIf (X > 0) {\n  Set Y 2\n}\nSet Z (X + Y)\nZ")
        .unwrap();

    assert_eq!(result, Value::Number(3.0));
    // 顶层语句 1、2（If 表达式语句）、5、6 + If 体内第 3 行，按执行顺序
    assert_eq!(*paused.borrow(), vec![1, 2, 3, 5, 6]);
}

#[test]
fn step_over_skips_function_body() {
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_breakpoint(BreakpointType::Line {
        file: "test.aether".to_string(),
        line: 5,
    });

    // 断点命中（第 5 行调用 DOUBLE）后切 StepOver：函数体内两行（2、3）被跳过，
    // 下一次暂停回到顶层第 6 行
    let s = Rc::clone(&state);
    let paused = Rc::new(RefCell::new(Vec::new()));
    let p = Rc::clone(&paused);
    let mut engine = debug_engine(
        state,
        Box::new(move |ev| {
            let line = ev.get_current_line();
            p.borrow_mut().push(line);
            let mut st = s.borrow_mut();
            st.set_execution_mode(ExecutionMode::StepOver);
            st.set_step_over_depth(ev.get_call_stack_depth());
            false
        }),
    );

    let result = engine
        .eval("Func DOUBLE(X) {\n  Set R (X * 2)\n  Return R\n}\nSet B DOUBLE(5)\nB")
        .unwrap();

    assert_eq!(result, Value::Number(10.0));
    assert_eq!(*paused.borrow(), vec![5, 6]);
}

#[test]
fn function_breakpoint_triggers_at_entry_with_params_bound() {
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_breakpoint(BreakpointType::Function {
        name: "ADD".to_string(),
    });

    #[derive(Clone)]
    struct Observed {
        top_frame: String,
        param_a: Option<Value>,
    }
    let observed = Rc::new(RefCell::new(None::<Observed>));

    let s = Rc::clone(&state);
    let obs = Rc::clone(&observed);
    let mut engine = debug_engine(
        state,
        Box::new(move |ev| {
            let top_frame = ev
                .get_call_stack()
                .last()
                .map(|f| f.signature.clone())
                .unwrap_or_default();
            *obs.borrow_mut() = Some(Observed {
                top_frame,
                param_a: ev.lookup_variable("A"),
            });
            s.borrow_mut().set_execution_mode(ExecutionMode::Continue);
            false
        }),
    );

    let result = engine
        .eval("Func ADD(A, B) {\n  Return (A + B)\n}\nADD(1, 2)")
        .unwrap();

    assert_eq!(result, Value::Number(3.0));
    let obs = observed.borrow().clone().expect("function breakpoint never triggered");
    assert_eq!(obs.top_frame, "ADD(A, B)");
    // 暂停发生在参数绑定之后：入口即可打印参数
    assert_eq!(obs.param_a, Some(Value::Number(1.0)));
}

#[test]
fn hook_quit_terminates_program_before_execution() {
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_execution_mode(ExecutionMode::StepInto);

    let mut engine = debug_engine(state, Box::new(|_ev| true));

    let result = engine.eval("Set X 10\nSet Y 20");

    // 以 DebugPause 终止：对外的 eval 错误串含 "Debugger pause"
    assert!(result.unwrap_err().contains("Debugger pause"));
    // 第 1 条语句执行前即退出，X 从未被赋值
    assert_eq!(engine.lookup_variable("X"), None);
}

#[test]
fn loop_breakpoint_hits_each_iteration() {
    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_breakpoint(BreakpointType::Line {
        file: "test.aether".to_string(),
        line: 3,
    });

    let paused = Rc::new(RefCell::new(Vec::new()));
    let mut engine = debug_engine(Rc::clone(&state), recording_hook(&state, &paused, ExecutionMode::Continue));

    let result = engine
        .eval("Set S 0\nFor I In [1, 2, 3] {\n  Set S (S + I)\n}\nS")
        .unwrap();

    assert_eq!(result, Value::Number(6.0));
    assert_eq!(*paused.borrow(), vec![3, 3, 3]);
}

#[test]
fn collect_breakable_lines_includes_bodies_but_not_braces() {
    let src = "Set A 1\nFunc F(X) {\n  Return X\n}\nFor I In [1, 2] {\n  PRINT(I)\n}";
    let program = Parser::new(src).parse_program().unwrap();

    let lines = collect_breakable_lines(&program);

    // 顶层语句行（1、2、5）+ 函数体（3）+ 循环体（6）；第 4 行是 `}` 不可命中
    let expected: std::collections::HashSet<usize> = [1, 2, 3, 5, 6].into_iter().collect();
    assert_eq!(lines, expected);
}

// ---- 跨文件断点（Import 模块） ----

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut path = std::env::temp_dir();
        path.push(format!("{prefix}_{}_{nanos}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn write(&self, rel: &str, content: &str) -> PathBuf {
        let p = self.path.join(rel);
        std::fs::write(&p, content).unwrap();
        p
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// 模块内**函数体**的行断点：函数在调用时按其定义文件命中断点
#[test]
fn line_breakpoint_inside_imported_function_body() {
    let dir = TempDir::new("aether_debug_mod");
    let math = dir.write(
        "math.aether",
        "Func TRIPLE(X) {\n  Return (X * 3)\n}\nExport TRIPLE\n",
    );
    let main = dir.write(
        "main.aether",
        "Import {TRIPLE} From \"math.aether\"\nTRIPLE(4)\n",
    );

    let state = Rc::new(RefCell::new(DebuggerState::new()));
    state.borrow_mut().set_breakpoint(BreakpointType::Line {
        file: "math.aether".to_string(),
        line: 2,
    });

    #[derive(Clone)]
    struct Observed {
        file: String,
        param: Option<Value>,
    }
    let observed = Rc::new(RefCell::new(None::<Observed>));
    let s = Rc::clone(&state);
    let obs = Rc::clone(&observed);

    let mut engine = {
        let mut engine = Aether::new();
        engine.set_module_resolver(Box::new(aether::FileSystemModuleResolver::default()));
        let canon = main.canonicalize().unwrap();
        engine.push_import_base(
            canon.display().to_string(),
            canon.parent().map(|p: &Path| p.to_path_buf()),
        );
        engine.set_source_file(canon.display().to_string());
        let math_name = math.file_name().unwrap().to_string_lossy().to_string();
        engine.attach_debugger(
            state,
            Box::new(move |ev| {
                *obs.borrow_mut() = Some(Observed {
                    file: ev.get_source_file().unwrap_or_default().to_string(),
                    param: ev.lookup_variable("X"),
                });
                let _ = &math_name;
                s.borrow_mut().set_execution_mode(ExecutionMode::Continue);
                false
            }),
        );
        engine
    };

    let result = engine
        .eval(&std::fs::read_to_string(&main).unwrap())
        .unwrap();

    assert_eq!(result, Value::Number(12.0));
    let obs = observed.borrow().clone().expect("module body breakpoint never triggered");
    // 暂停文件是模块文件（按文件名兜底匹配断点），参数已绑定
    assert!(obs.file.ends_with("math.aether"), "was: {}", obs.file);
    assert_eq!(obs.param, Some(Value::Number(4.0)));
}
