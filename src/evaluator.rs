// src/evaluator.rs
//! Evaluator for executing Aether AST

use crate::ast::{BinOp, Expr, Program, Stmt, UnaryOp};
use crate::builtins::BuiltInRegistry;
use crate::environment::Environment;
use crate::module_system::{DisabledModuleResolver, ModuleContext, ModuleResolver, ResolvedModule};
use crate::value::{GeneratorState, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

/// Runtime errors
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    /// Variable not found
    UndefinedVariable(String),

    /// Type mismatch - simple message
    TypeError(String),

    /// Type mismatch - detailed version
    TypeErrorDetailed { expected: String, got: String },

    /// Invalid operation
    InvalidOperation(String),

    /// Division by zero
    DivisionByZero,

    /// Function not found or not callable
    NotCallable(String),

    /// Wrong number of arguments
    WrongArity { expected: usize, got: usize },

    /// Return statement (used for control flow)
    Return(Value),

    /// Yield statement (used for generators)
    Yield(Value),

    /// Break statement (used for loop control)
    Break,

    /// Continue statement (used for loop control)
    Continue,

    /// Throw statement (user-thrown error)
    Throw(Value),

    /// Custom error message (用于IO操作等)
    CustomError(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            RuntimeError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RuntimeError::TypeErrorDetailed { expected, got } => {
                write!(f, "Type error: expected {}, got {}", expected, got)
            }
            RuntimeError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::NotCallable(name) => write!(f, "Not callable: {}", name),
            RuntimeError::WrongArity { expected, got } => {
                write!(
                    f,
                    "Wrong number of arguments: expected {}, got {}",
                    expected, got
                )
            }
            RuntimeError::Return(val) => write!(f, "Return: {}", val),
            RuntimeError::Yield(val) => write!(f, "Yield: {}", val),
            RuntimeError::Break => write!(f, "Break outside of loop"),
            RuntimeError::Continue => write!(f, "Continue outside of loop"),
            RuntimeError::Throw(val) => write!(f, "Throw: {}", val),
            RuntimeError::CustomError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

pub type EvalResult = Result<Value, RuntimeError>;

/// Evaluator for Aether programs
pub struct Evaluator {
    /// Global environment
    env: Rc<RefCell<Environment>>,
    /// Built-in function registry
    registry: BuiltInRegistry,
    /// In-memory trace buffer (for DSL-safe debugging; no stdout/files/network)
    trace: VecDeque<String>,
    /// Monotonic sequence for trace entries (starts at 1)
    trace_seq: u64,

    /// Module resolver (Import/Export). Defaults to disabled for DSL safety.
    module_resolver: Box<dyn ModuleResolver>,
    /// Module export cache: module_id -> exports
    module_cache: HashMap<String, HashMap<String, Value>>,
    /// Module load stack for cycle detection
    module_stack: Vec<String>,
    /// Current module export table stack (only when evaluating an imported module)
    export_stack: Vec<HashMap<String, Value>>,
    /// Optional base directory context for resolving relative imports (e.g. eval_file)
    import_base_stack: Vec<ModuleContext>,
}

impl Evaluator {
    const TRACE_MAX_ENTRIES: usize = 1024;

    /// Create a new evaluator (默认禁用IO)
    pub fn new() -> Self {
        Self::with_permissions(crate::builtins::IOPermissions::default())
    }

    /// Create a new evaluator with custom IO permissions
    pub fn with_permissions(permissions: crate::builtins::IOPermissions) -> Self {
        let env = Rc::new(RefCell::new(Environment::new()));

        // Register built-in functions with permissions
        let registry = BuiltInRegistry::with_permissions(permissions);
        for name in registry.names() {
            env.borrow_mut()
                .set(name.clone(), Value::BuiltIn { name, arity: 0 });
        }

        Evaluator {
            env,
            registry,
            trace: VecDeque::new(),
            trace_seq: 0,

            module_resolver: Box::new(DisabledModuleResolver),
            module_cache: HashMap::new(),
            module_stack: Vec::new(),
            export_stack: Vec::new(),
            import_base_stack: Vec::new(),
        }
    }

    /// Create evaluator with custom environment
    pub fn with_env(env: Rc<RefCell<Environment>>) -> Self {
        let registry = BuiltInRegistry::new();
        Evaluator {
            env,
            registry,
            trace: VecDeque::new(),
            trace_seq: 0,

            module_resolver: Box::new(DisabledModuleResolver),
            module_cache: HashMap::new(),
            module_stack: Vec::new(),
            export_stack: Vec::new(),
            import_base_stack: Vec::new(),
        }
    }

    /// Configure the module resolver used for `Import/Export`.
    pub fn set_module_resolver(&mut self, resolver: Box<dyn ModuleResolver>) {
        self.module_resolver = resolver;
    }

    /// Push a base directory context for resolving relative imports.
    ///
    /// This is typically used by CLI `eval_file()` wrappers.
    pub fn push_import_base(&mut self, module_id: String, base_dir: Option<std::path::PathBuf>) {
        self.import_base_stack.push(ModuleContext {
            module_id,
            base_dir,
        });
    }

    /// Pop the most recent base directory context.
    pub fn pop_import_base(&mut self) {
        self.import_base_stack.pop();
    }

    /// Append a trace entry (host-readable; no IO side effects).
    pub fn trace_push(&mut self, msg: String) {
        self.trace_seq = self.trace_seq.saturating_add(1);
        let entry = format!("#{} {}", self.trace_seq, msg);

        if self.trace.len() >= Self::TRACE_MAX_ENTRIES {
            self.trace.pop_front();
        }
        self.trace.push_back(entry);
    }

    /// Drain the trace buffer.
    pub fn take_trace(&mut self) -> Vec<String> {
        std::mem::take(&mut self.trace).into_iter().collect()
    }

    /// Clear the trace buffer.
    pub fn clear_trace(&mut self) {
        self.trace.clear();
        self.trace_seq = 0;
    }

    /// Reset the environment (clear all variables and re-register built-ins)
    ///
    /// This is useful for engine pooling and global singleton patterns
    /// where you want to reuse an engine instance but ensure isolation.
    pub fn reset_env(&mut self) {
        // Create new environment
        self.env = Rc::new(RefCell::new(Environment::new()));

        // Avoid leaking trace across pooled executions
        self.trace.clear();
        self.trace_seq = 0;

        // Reset module contexts (cache is kept; can be cleared explicitly by host if needed)
        self.import_base_stack.clear();
        self.export_stack.clear();
        self.module_stack.clear();

        // Re-register built-in functions
        for name in self.registry.names() {
            self.env
                .borrow_mut()
                .set(name.clone(), Value::BuiltIn { name, arity: 0 });
        }
    }

    /// Set a global variable from the host (without requiring `eval`).
    pub fn set_global(&mut self, name: impl Into<String>, value: Value) {
        self.env.borrow_mut().set(name.into(), value);
    }

    /// Enter a child scope (new environment whose parent is the current env).
    ///
    /// Returns the previous environment handle; pass it back to `restore_env()`.
    pub fn enter_child_scope(&mut self) -> Rc<RefCell<Environment>> {
        let prev = Rc::clone(&self.env);
        let child = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&prev))));
        self.env = child;
        prev
    }

    /// Restore a previously saved environment handle (typically from `enter_child_scope()`).
    pub fn restore_env(&mut self, prev: Rc<RefCell<Environment>>) {
        self.env = prev;
    }

    /// Evaluate a program
    pub fn eval_program(&mut self, program: &Program) -> EvalResult {
        let mut result = Value::Null;

        for stmt in program {
            result = self.eval_statement(stmt)?;
        }

        Ok(result)
    }

    /// Evaluate a statement
    pub fn eval_statement(&mut self, stmt: &Stmt) -> EvalResult {
        match stmt {
            Stmt::Set { name, value } => {
                let val = self.eval_expression(value)?;
                self.env.borrow_mut().set(name.clone(), val.clone());
                Ok(val)
            }

            Stmt::SetIndex {
                object,
                index,
                value,
            } => {
                // Evaluate the value to be assigned
                let val = self.eval_expression(value)?;

                // For simple identifier objects, we can modify in place
                if let Expr::Identifier(name) = object.as_ref() {
                    // Get the object from environment
                    let obj = self
                        .env
                        .borrow()
                        .get(name)
                        .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?;

                    // Evaluate the index
                    let idx_val = self.eval_expression(index)?;

                    // Modify based on object type
                    let new_obj = match (obj, idx_val) {
                        (Value::Array(mut arr), Value::Number(n)) => {
                            let idx = n as usize;
                            if idx >= arr.len() {
                                return Err(RuntimeError::InvalidOperation(format!(
                                    "Index {} out of bounds (array length: {})",
                                    idx,
                                    arr.len()
                                )));
                            }
                            arr[idx] = val.clone();
                            Value::Array(arr)
                        }
                        (Value::Dict(mut dict), Value::String(key)) => {
                            dict.insert(key, val.clone());
                            Value::Dict(dict)
                        }
                        (obj, idx) => {
                            return Err(RuntimeError::TypeError(format!(
                                "Cannot index {} with {}",
                                obj.type_name(),
                                idx.type_name()
                            )));
                        }
                    };

                    // Update the variable in environment
                    self.env.borrow_mut().set(name.clone(), new_obj);
                    Ok(val)
                } else {
                    // For complex expressions, we can't modify in place
                    Err(RuntimeError::InvalidOperation(
                        "Can only assign to simple variable indices (e.g., dict[key], not expr[key])"
                            .to_string(),
                    ))
                }
            }

            Stmt::FuncDef { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    env: Rc::clone(&self.env),
                };
                self.env.borrow_mut().set(name.clone(), func.clone());
                Ok(func)
            }

            Stmt::GeneratorDef { name, params, body } => {
                let r#gen = Value::Generator {
                    params: params.clone(),
                    body: body.clone(),
                    env: Rc::clone(&self.env),
                    state: GeneratorState::NotStarted,
                };
                self.env.borrow_mut().set(name.clone(), r#gen.clone());
                Ok(r#gen)
            }

            Stmt::LazyDef { name, expr } => {
                let lazy = Value::Lazy {
                    expr: expr.clone(),
                    env: Rc::clone(&self.env),
                    cached: None,
                };
                self.env.borrow_mut().set(name.clone(), lazy.clone());
                Ok(lazy)
            }

            Stmt::Return(expr) => {
                let val = self.eval_expression(expr)?;
                Err(RuntimeError::Return(val))
            }

            Stmt::Yield(expr) => {
                let val = self.eval_expression(expr)?;
                Err(RuntimeError::Yield(val))
            }

            Stmt::Break => Err(RuntimeError::Break),

            Stmt::Continue => Err(RuntimeError::Continue),

            Stmt::While { condition, body } => {
                let mut result = Value::Null;

                loop {
                    let cond = self.eval_expression(condition)?;
                    if !cond.is_truthy() {
                        break;
                    }

                    let mut should_break = false;
                    for stmt in body {
                        match self.eval_statement(stmt) {
                            Ok(val) => result = val,
                            Err(RuntimeError::Break) => {
                                should_break = true;
                                break;
                            }
                            Err(RuntimeError::Continue) => break,
                            Err(e) => return Err(e),
                        }
                    }

                    if should_break {
                        break;
                    }
                }

                Ok(result)
            }

            Stmt::For {
                var,
                iterable,
                body,
            } => {
                let iter_val = self.eval_expression(iterable)?;
                let mut result = Value::Null;

                match iter_val {
                    Value::Array(arr) => {
                        let mut should_break = false;
                        for item in arr {
                            self.env.borrow_mut().set(var.clone(), item);
                            for stmt in body {
                                match self.eval_statement(stmt) {
                                    Ok(val) => result = val,
                                    Err(RuntimeError::Break) => {
                                        should_break = true;
                                        break;
                                    }
                                    Err(RuntimeError::Continue) => break,
                                    Err(e) => return Err(e),
                                }
                            }
                            if should_break {
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot iterate over {}",
                            iter_val.type_name()
                        )));
                    }
                }

                Ok(result)
            }

            Stmt::ForIndexed {
                index_var,
                value_var,
                iterable,
                body,
            } => {
                let iter_val = self.eval_expression(iterable)?;
                let mut result = Value::Null;

                match iter_val {
                    Value::Array(arr) => {
                        let mut should_break = false;
                        for (idx, item) in arr.iter().enumerate() {
                            self.env
                                .borrow_mut()
                                .set(index_var.clone(), Value::Number(idx as f64));
                            self.env.borrow_mut().set(value_var.clone(), item.clone());
                            for stmt in body {
                                match self.eval_statement(stmt) {
                                    Ok(val) => result = val,
                                    Err(RuntimeError::Break) => {
                                        should_break = true;
                                        break;
                                    }
                                    Err(RuntimeError::Continue) => break,
                                    Err(e) => return Err(e),
                                }
                            }
                            if should_break {
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot iterate over {}",
                            iter_val.type_name()
                        )));
                    }
                }

                Ok(result)
            }

            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                let val = self.eval_expression(expr)?;

                for (case_expr, case_body) in cases {
                    let case_val = self.eval_expression(case_expr)?;
                    if val.equals(&case_val) {
                        let mut result = Value::Null;
                        for stmt in case_body {
                            result = self.eval_statement(stmt)?;
                        }
                        return Ok(result);
                    }
                }

                if let Some(default_body) = default {
                    let mut result = Value::Null;
                    for stmt in default_body {
                        result = self.eval_statement(stmt)?;
                    }
                    return Ok(result);
                }

                Ok(Value::Null)
            }

            Stmt::Import {
                names,
                path,
                aliases,
            } => self.eval_import(names, path, aliases),

            Stmt::Export(name) => self.eval_export(name),

            Stmt::Throw(expr) => {
                let val = self.eval_expression(expr)?;
                Err(RuntimeError::Throw(val))
            }

            Stmt::Expression(expr) => self.eval_expression(expr),
        }
    }

    /// Evaluate an expression
    pub fn eval_expression(&mut self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),

            Expr::BigInteger(s) => {
                // 将大整数字符串转换为 Fraction (分母为1的分数)
                use num_bigint::BigInt;
                use num_rational::Ratio;

                match s.parse::<BigInt>() {
                    Ok(big_int) => Ok(Value::Fraction(Ratio::new(big_int, BigInt::from(1)))),
                    Err(_) => Err(RuntimeError::InvalidOperation(format!(
                        "Invalid big integer: {}",
                        s
                    ))),
                }
            }

            Expr::String(s) => Ok(Value::String(s.clone())),

            Expr::Boolean(b) => Ok(Value::Boolean(*b)),

            Expr::Null => Ok(Value::Null),

            Expr::Identifier(name) => self
                .env
                .borrow()
                .get(name)
                .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone())),

            Expr::Binary { left, op, right } => {
                // Short-circuit evaluation for And and Or
                match op {
                    BinOp::And => {
                        let left_val = self.eval_expression(left)?;
                        if !left_val.is_truthy() {
                            // Short-circuit: left is falsy, return left without evaluating right
                            Ok(left_val)
                        } else {
                            // left is truthy, return right value
                            self.eval_expression(right)
                        }
                    }
                    BinOp::Or => {
                        let left_val = self.eval_expression(left)?;
                        if left_val.is_truthy() {
                            // Short-circuit: left is truthy, return left without evaluating right
                            Ok(left_val)
                        } else {
                            // left is falsy, return right value
                            self.eval_expression(right)
                        }
                    }
                    // For other operators, evaluate both sides
                    _ => {
                        let left_val = self.eval_expression(left)?;
                        let right_val = self.eval_expression(right)?;
                        self.eval_binary_op(&left_val, op, &right_val)
                    }
                }
            }

            Expr::Unary { op, expr } => {
                let val = self.eval_expression(expr)?;
                self.eval_unary_op(op, &val)
            }

            Expr::Call { func, args } => {
                let func_val = self.eval_expression(func)?;
                let arg_vals: Result<Vec<_>, _> =
                    args.iter().map(|arg| self.eval_expression(arg)).collect();
                let arg_vals = arg_vals?;

                self.call_function(&func_val, arg_vals)
            }

            Expr::Array(elements) => {
                let vals: Result<Vec<_>, _> =
                    elements.iter().map(|e| self.eval_expression(e)).collect();
                Ok(Value::Array(vals?))
            }

            Expr::Dict(pairs) => {
                let mut map = std::collections::HashMap::new();
                for (key, value_expr) in pairs {
                    let value = self.eval_expression(value_expr)?;
                    map.insert(key.clone(), value);
                }
                Ok(Value::Dict(map))
            }

            Expr::Index { object, index } => {
                let obj_val = self.eval_expression(object)?;
                let idx_val = self.eval_expression(index)?;

                match (obj_val, idx_val) {
                    (Value::Array(arr), Value::Number(n)) => {
                        let idx = n as usize;
                        arr.get(idx).cloned().ok_or_else(|| {
                            RuntimeError::InvalidOperation(format!("Index {} out of bounds", idx))
                        })
                    }
                    (Value::String(s), Value::Number(n)) => {
                        let idx = n as usize;
                        let chars: Vec<char> = s.chars().collect();
                        chars
                            .get(idx)
                            .cloned()
                            .map(|ch| Value::String(ch.to_string()))
                            .ok_or_else(|| {
                                RuntimeError::InvalidOperation(format!(
                                    "Index {} out of bounds (string length: {})",
                                    idx,
                                    chars.len()
                                ))
                            })
                    }
                    (Value::Dict(dict), Value::String(key)) => {
                        dict.get(&key).cloned().ok_or_else(|| {
                            RuntimeError::InvalidOperation(format!("Key '{}' not found", key))
                        })
                    }
                    (obj, idx) => Err(RuntimeError::TypeError(format!(
                        "Cannot index {} with {}",
                        obj.type_name(),
                        idx.type_name()
                    ))),
                }
            }

            Expr::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let cond = self.eval_expression(condition)?;

                if cond.is_truthy() {
                    let mut result = Value::Null;
                    for stmt in then_branch {
                        result = self.eval_statement(stmt)?;
                    }
                    return Ok(result);
                }

                for (elif_cond, elif_body) in elif_branches {
                    let cond = self.eval_expression(elif_cond)?;
                    if cond.is_truthy() {
                        let mut result = Value::Null;
                        for stmt in elif_body {
                            result = self.eval_statement(stmt)?;
                        }
                        return Ok(result);
                    }
                }

                if let Some(else_body) = else_branch {
                    let mut result = Value::Null;
                    for stmt in else_body {
                        result = self.eval_statement(stmt)?;
                    }
                    return Ok(result);
                }

                Ok(Value::Null)
            }

            Expr::Lambda { params, body } => {
                // Create a closure by capturing the current environment
                Ok(Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    env: Rc::clone(&self.env),
                })
            }
        }
    }

    /// Evaluate binary operation
    fn eval_binary_op(&self, left: &Value, op: &BinOp, right: &Value) -> EvalResult {
        match op {
            BinOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::Fraction(a), Value::Fraction(b)) => Ok(Value::Fraction(a + b)),
                (Value::Number(a), Value::Fraction(b)) | (Value::Fraction(b), Value::Number(a)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac + b))
                    } else {
                        // 浮点数和分数混合运算，转换为浮点数
                        use num_traits::ToPrimitive;
                        let b_float =
                            b.numer().to_f64().unwrap_or(0.0) / b.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a + b_float))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot add {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Subtract => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                (Value::Fraction(a), Value::Fraction(b)) => Ok(Value::Fraction(a - b)),
                (Value::Number(a), Value::Fraction(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac - b))
                    } else {
                        use num_traits::ToPrimitive;
                        let b_float =
                            b.numer().to_f64().unwrap_or(0.0) / b.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a - b_float))
                    }
                }
                (Value::Fraction(a), Value::Number(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if b.fract() == 0.0 {
                        let b_frac = Ratio::new(BigInt::from(*b as i64), BigInt::from(1));
                        Ok(Value::Fraction(a - b_frac))
                    } else {
                        use num_traits::ToPrimitive;
                        let a_float =
                            a.numer().to_f64().unwrap_or(0.0) / a.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a_float - b))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot subtract {} from {}",
                    right.type_name(),
                    left.type_name()
                ))),
            },

            BinOp::Multiply => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    // 如果两个数都是整数，且足够大，使用精确计算
                    if a.fract() == 0.0 && b.fract() == 0.0 {
                        // 检查是否超过 f64 的安全整数范围 (2^53)
                        let max_safe = 9007199254740992.0; // 2^53
                        if a.abs() > max_safe || b.abs() > max_safe {
                            // 使用 Fraction (BigInt) 进行精确计算
                            use num_bigint::BigInt;
                            use num_rational::Ratio;

                            // 将 f64 转换为字符串再转为 BigInt，避免精度损失
                            let a_str = format!("{:.0}", a);
                            let b_str = format!("{:.0}", b);

                            if let (Ok(a_big), Ok(b_big)) =
                                (a_str.parse::<BigInt>(), b_str.parse::<BigInt>())
                            {
                                let result_big = a_big * b_big;
                                let frac = Ratio::new(result_big, BigInt::from(1));
                                return Ok(Value::Fraction(frac));
                            }
                        }
                    }
                    Ok(Value::Number(a * b))
                }
                (Value::Fraction(a), Value::Fraction(b)) => Ok(Value::Fraction(a * b)),
                (Value::Number(a), Value::Fraction(b)) | (Value::Fraction(b), Value::Number(a)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac * b))
                    } else {
                        Err(RuntimeError::TypeError(
                            "Cannot multiply non-integer Number with Fraction".to_string(),
                        ))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot multiply {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Divide => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                (Value::Fraction(a), Value::Fraction(b)) => {
                    use num_traits::Zero;
                    if b.is_zero() {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Fraction(a / b))
                    }
                }
                (Value::Number(a), Value::Fraction(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    use num_traits::Zero;
                    if b.is_zero() {
                        Err(RuntimeError::DivisionByZero)
                    } else if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac / b))
                    } else {
                        use num_traits::ToPrimitive;
                        let b_float =
                            b.numer().to_f64().unwrap_or(0.0) / b.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a / b_float))
                    }
                }
                (Value::Fraction(a), Value::Number(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if *b == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else if b.fract() == 0.0 {
                        let b_frac = Ratio::new(BigInt::from(*b as i64), BigInt::from(1));
                        Ok(Value::Fraction(a / b_frac))
                    } else {
                        use num_traits::ToPrimitive;
                        let a_float =
                            a.numer().to_f64().unwrap_or(0.0) / a.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a_float / b))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot divide {} by {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Modulo => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot modulo {} by {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Equal => Ok(Value::Boolean(left.equals(right))),

            BinOp::NotEqual => Ok(Value::Boolean(!left.equals(right))),

            BinOp::Less => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord == std::cmp::Ordering::Less)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::LessEqual => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord != std::cmp::Ordering::Greater)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Greater => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord == std::cmp::Ordering::Greater)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::GreaterEqual => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord != std::cmp::Ordering::Less)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::And => {
                if !left.is_truthy() {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }

            BinOp::Or => {
                if left.is_truthy() {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }
        }
    }

    /// Evaluate unary operation
    fn eval_unary_op(&self, op: &UnaryOp, val: &Value) -> EvalResult {
        match op {
            UnaryOp::Minus => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot negate {}",
                    val.type_name()
                ))),
            },

            UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
        }
    }

    /// Call a function with arguments
    fn call_function(&mut self, func: &Value, args: Vec<Value>) -> EvalResult {
        match func {
            Value::Function { params, body, env } => {
                if params.len() != args.len() {
                    return Err(RuntimeError::WrongArity {
                        expected: params.len(),
                        got: args.len(),
                    });
                }

                // Create new environment for function execution
                let func_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));

                // Bind parameters
                for (param, arg) in params.iter().zip(args.iter()) {
                    func_env.borrow_mut().set(param.clone(), arg.clone());
                }

                // Execute function body
                let prev_env = Rc::clone(&self.env);
                self.env = func_env;

                let mut result = Value::Null;
                for stmt in body {
                    match self.eval_statement(stmt) {
                        Ok(val) => result = val,
                        Err(RuntimeError::Return(val)) => {
                            result = val;
                            break;
                        }
                        Err(e) => {
                            self.env = prev_env;
                            return Err(e);
                        }
                    }
                }

                self.env = prev_env;
                Ok(result)
            }

            Value::BuiltIn { name, .. } => {
                // Special handling for MAP, FILTER, and REDUCE
                match name.as_str() {
                    "TRACE" => {
                        if args.is_empty() {
                            return Err(RuntimeError::WrongArity {
                                expected: 1,
                                got: 0,
                            });
                        }

                        // Optional label: TRACE("label", x, y)
                        // If only one argument is provided, treat it as the payload (backward compatible).
                        let (label, payload_args) = if args.len() >= 2 {
                            match &args[0] {
                                Value::String(s) => (Some(s.as_str()), &args[1..]),
                                _ => (None, args.as_slice()),
                            }
                        } else {
                            (None, args.as_slice())
                        };

                        let payload = payload_args
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");

                        let msg = match label {
                            Some(l) => format!("[{}] {}", l, payload),
                            None => payload,
                        };

                        self.trace_push(msg);
                        Ok(Value::Null)
                    }
                    "MAP" => self.builtin_map(&args),
                    "FILTER" => self.builtin_filter(&args),
                    "REDUCE" => self.builtin_reduce(&args),
                    _ => {
                        // Get the built-in function from the registry
                        if let Some((func, _arity)) = self.registry.get(name) {
                            // Call the built-in function
                            func(&args)
                        } else {
                            Err(RuntimeError::NotCallable(format!(
                                "Built-in function '{}' not found",
                                name
                            )))
                        }
                    }
                }
            }

            _ => Err(RuntimeError::NotCallable(func.type_name().to_string())),
        }
    }

    // 实现 MAP 内置函数
    fn builtin_map(&mut self, args: &[Value]) -> EvalResult {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArity {
                expected: 2,
                got: args.len(),
            });
        }

        let arr = match &args[0] {
            Value::Array(a) => a,
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Array".to_string(),
                    got: format!("{:?}", other),
                });
            }
        };

        let func = &args[1];

        let mut result = Vec::new();
        for item in arr {
            let mapped = self.call_function(func, vec![item.clone()])?;
            result.push(mapped);
        }

        Ok(Value::Array(result))
    }

    // 实现 FILTER 内置函数
    fn builtin_filter(&mut self, args: &[Value]) -> EvalResult {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArity {
                expected: 2,
                got: args.len(),
            });
        }

        let arr = match &args[0] {
            Value::Array(a) => a,
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Array".to_string(),
                    got: format!("{:?}", other),
                });
            }
        };

        let predicate = &args[1];

        let mut result = Vec::new();
        for item in arr {
            let test_result = self.call_function(predicate, vec![item.clone()])?;
            if test_result.is_truthy() {
                result.push(item.clone());
            }
        }

        Ok(Value::Array(result))
    }

    // 实现 REDUCE 内置函数
    fn builtin_reduce(&mut self, args: &[Value]) -> EvalResult {
        if args.len() != 3 {
            return Err(RuntimeError::WrongArity {
                expected: 3,
                got: args.len(),
            });
        }

        let arr = match &args[0] {
            Value::Array(a) => a,
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Array".to_string(),
                    got: format!("{:?}", other),
                });
            }
        };

        let mut accumulator = args[1].clone();
        let func = &args[2];

        for item in arr {
            accumulator = self.call_function(func, vec![accumulator, item.clone()])?;
        }

        Ok(accumulator)
    }
}

impl Evaluator {
    fn current_import_context(&self) -> Option<&ModuleContext> {
        self.import_base_stack.last()
    }

    fn eval_import(
        &mut self,
        names: &[String],
        specifier: &str,
        aliases: &[Option<String>],
    ) -> EvalResult {
        let from_ctx = self.current_import_context();

        let resolved = self
            .module_resolver
            .resolve(specifier, from_ctx)
            .map_err(|e| RuntimeError::CustomError(format!("Import error: {e}")))?;

        let exports = self.load_module(resolved)?;

        for (i, name) in names.iter().enumerate() {
            let alias = aliases
                .get(i)
                .and_then(|a| a.clone())
                .unwrap_or_else(|| name.clone());
            let v = exports.get(name).cloned().ok_or_else(|| {
                RuntimeError::CustomError(format!(
                    "Import error: '{}' is not exported by module {}",
                    name, specifier
                ))
            })?;
            self.env.borrow_mut().set(alias, v);
        }

        Ok(Value::Null)
    }

    fn eval_export(&mut self, name: &str) -> EvalResult {
        let exports = self.export_stack.last_mut().ok_or_else(|| {
            RuntimeError::CustomError("Export error: Export used outside of a module".to_string())
        })?;

        let val = self.env.borrow().get(name).ok_or_else(|| {
            RuntimeError::CustomError(format!("Export error: '{}' is not defined", name))
        })?;

        exports.insert(name.to_string(), val);
        Ok(Value::Null)
    }

    fn load_module(
        &mut self,
        resolved: ResolvedModule,
    ) -> Result<HashMap<String, Value>, RuntimeError> {
        if let Some(cached) = self.module_cache.get(&resolved.module_id) {
            return Ok(cached.clone());
        }

        if self.module_stack.contains(&resolved.module_id) {
            let mut chain = self.module_stack.clone();
            chain.push(resolved.module_id.clone());
            return Err(RuntimeError::CustomError(format!(
                "Import error: circular import detected: {}",
                chain.join(" -> ")
            )));
        }

        self.module_stack.push(resolved.module_id.clone());

        // Parse module
        let mut parser = crate::parser::Parser::new(&resolved.source);
        let program = parser.parse_program().map_err(|e| {
            RuntimeError::CustomError(format!(
                "Import error: parse failed for module {}: {}",
                resolved.module_id, e
            ))
        })?;

        // Evaluate in an isolated environment with builtins registered.
        let prev_env = Rc::clone(&self.env);
        let module_env = Rc::new(RefCell::new(Environment::new()));
        for name in self.registry.names() {
            module_env
                .borrow_mut()
                .set(name.clone(), Value::BuiltIn { name, arity: 0 });
        }
        self.env = module_env;

        // Push module import base (for relative imports inside the module)
        self.import_base_stack.push(ModuleContext {
            module_id: resolved.module_id.clone(),
            base_dir: resolved.base_dir.clone(),
        });

        // Push export table
        self.export_stack.push(HashMap::new());

        let _ = self.eval_program(&program)?;

        // Pop stacks and restore env
        let exports = self.export_stack.pop().unwrap_or_default();
        self.import_base_stack.pop();
        self.env = prev_env;

        // Pop module stack
        let _ = self.module_stack.pop();

        self.module_cache
            .insert(resolved.module_id.clone(), exports.clone());
        Ok(exports)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn eval(code: &str) -> EvalResult {
        let mut parser = Parser::new(code);
        let program = parser.parse_program().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.eval_program(&program)
    }

    #[test]
    fn test_eval_numbers() {
        assert_eq!(eval("42").unwrap(), Value::Number(42.0));
        #[allow(clippy::approx_constant)]
        {
            assert_eq!(eval("3.14").unwrap(), Value::Number(3.14));
        }
    }

    #[test]
    fn test_eval_strings() {
        assert_eq!(
            eval(r#""hello""#).unwrap(),
            Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_eval_booleans() {
        assert_eq!(eval("True").unwrap(), Value::Boolean(true));
        assert_eq!(eval("False").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_eval_arithmetic() {
        assert_eq!(eval("(5 + 3)").unwrap(), Value::Number(8.0));
        assert_eq!(eval("(10 - 3)").unwrap(), Value::Number(7.0));
        assert_eq!(eval("(4 * 3)").unwrap(), Value::Number(12.0));
        assert_eq!(eval("(10 / 2)").unwrap(), Value::Number(5.0));
        assert_eq!(eval("(10 % 3)").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_eval_arithmetic_precedence() {
        assert_eq!(eval("(5 + 3 * 2)").unwrap(), Value::Number(11.0));
        assert_eq!(eval("((5 + 3) * 2)").unwrap(), Value::Number(16.0));
    }

    #[test]
    fn test_eval_comparison() {
        assert_eq!(eval("(5 < 10)").unwrap(), Value::Boolean(true));
        assert_eq!(eval("(10 < 5)").unwrap(), Value::Boolean(false));
        assert_eq!(eval("(5 == 5)").unwrap(), Value::Boolean(true));
        assert_eq!(eval("(5 != 3)").unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_eval_logical() {
        assert_eq!(eval("(True && False)").unwrap(), Value::Boolean(false));
        assert_eq!(eval("(True || False)").unwrap(), Value::Boolean(true));
        assert_eq!(eval("(!True)").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_eval_set() {
        let code = r#"
            Set X 42
            X
        "#;
        assert_eq!(eval(code).unwrap(), Value::Number(42.0));
    }

    #[test]
    fn test_eval_function() {
        let code = r#"
            Func ADD (A, B) {
                Return (A + B)
            }
            ADD(5, 3)
        "#;
        assert_eq!(eval(code).unwrap(), Value::Number(8.0));
    }

    #[test]
    fn test_eval_array() {
        let code = "[1, 2, 3]";
        let result = eval(code).unwrap();
        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::Number(1.0));
                assert_eq!(arr[1], Value::Number(2.0));
                assert_eq!(arr[2], Value::Number(3.0));
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_eval_array_index() {
        let code = r#"
            Set ARR [10, 20, 30]
            ARR[1]
        "#;
        assert_eq!(eval(code).unwrap(), Value::Number(20.0));
    }

    #[test]
    fn test_eval_if() {
        let code = r#"
            If (True) {
                Set X 42
            } Else {
                Set X 0
            }
            X
        "#;
        assert_eq!(eval(code).unwrap(), Value::Number(42.0));
    }

    #[test]
    fn test_eval_for() {
        let code = r#"
            Set SUM 0
            For I In [1, 2, 3] {
                Set SUM (SUM + I)
            }
            SUM
        "#;
        assert_eq!(eval(code).unwrap(), Value::Number(6.0));
    }
}
