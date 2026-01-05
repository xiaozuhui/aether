pub mod diagnostics;
pub mod emitter;
pub mod ir;
pub mod options;
pub mod python;

pub use diagnostics::{Diagnostic, Diagnostics, Severity};
pub use ir::{Expr, Module, Span, Stmt};
pub use options::{DecimalMode, TranspileOptions};

#[derive(Debug)]
pub struct PythonToAetherResult {
    pub aether: Option<String>,
    pub diagnostics: Diagnostics,
    pub numpy_used: bool,
    pub io_used: bool,
    pub console_used: bool,
}

/// Transpile Python source into Aether source.
///
/// - By default (`TranspileOptions::default()`), numpy / filesystem / network / console IO are rejected.
/// - When the `pytranspile` cargo feature is disabled, this always returns an error diagnostic.
pub fn python_to_aether(source: &str, opts: &TranspileOptions) -> PythonToAetherResult {
    let ir_res = python::python_to_ir(source, opts);

    // If python parsing already failed, return early.
    if ir_res.diagnostics.has_errors() || ir_res.module.is_none() {
        return PythonToAetherResult {
            aether: None,
            diagnostics: ir_res.diagnostics,
            numpy_used: ir_res.numpy_used,
            io_used: ir_res.io_used,
            console_used: ir_res.console_used,
        };
    }

    // Sanity: module present.
    let module = ir_res.module.unwrap();

    let emit_res = emitter::ir_to_aether(&module, opts);

    // Combine diagnostics (python stage + emit stage).
    let mut diags = ir_res.diagnostics;
    for d in emit_res.diagnostics.0 {
        diags.push(d);
    }

    if diags.has_errors() {
        PythonToAetherResult {
            aether: None,
            diagnostics: diags,
            numpy_used: ir_res.numpy_used,
            io_used: ir_res.io_used,
            console_used: ir_res.console_used,
        }
    } else {
        PythonToAetherResult {
            aether: emit_res.code,
            diagnostics: diags,
            numpy_used: ir_res.numpy_used,
            io_used: ir_res.io_used,
            console_used: ir_res.console_used,
        }
    }
}

/// Convenience: returns Ok(code) if transpilation succeeded, Err(diagnostics) otherwise.
pub fn python_to_aether_checked(
    source: &str,
    opts: &TranspileOptions,
) -> Result<String, Diagnostics> {
    let res = python_to_aether(source, opts);
    match res.aether {
        Some(code) if !res.diagnostics.has_errors() => Ok(code),
        _ => Err(res.diagnostics),
    }
}

pub fn aether_parse_ast(code: &str) -> Result<crate::ast::Program, crate::parser::ParseError> {
    let mut parser = crate::parser::Parser::new(code);
    parser.parse_program()
}

/// Parse Aether code into a JSON AST (best-effort, stable enough for debugging/tooling).
pub fn aether_parse_ast_json(code: &str) -> Result<serde_json::Value, Diagnostics> {
    let program = aether_parse_ast(code).map_err(|e| {
        let mut d = Diagnostics::new();
        d.push(Diagnostic::error(
            "AETHER_PARSE_ERROR",
            e.to_string(),
            Span::default(),
        ));
        d
    })?;

    Ok(program_to_json(&program))
}

/// Evaluate Aether code in DSL-safe mode (no filesystem/network permissions) and with
/// static pre-checks enforced (console/io rejection).
pub fn aether_eval_safe(
    code: &str,
    opts: &TranspileOptions,
) -> Result<crate::value::Value, Diagnostics> {
    let diags = aether_check(code, opts);
    if diags.has_errors() {
        return Err(diags);
    }

    let mut engine = crate::Aether::new();
    engine.eval(code).map_err(|e| {
        let mut d = Diagnostics::new();
        d.push(Diagnostic::error("AETHER_EVAL_ERROR", e, Span::default()));
        d
    })
}

/// Aether-side static checker for disallowed builtins (best-effort).
///
/// Note: Aether AST nodes currently don't carry spans, so diagnostics use default spans.
pub fn aether_check(code: &str, opts: &TranspileOptions) -> Diagnostics {
    let mut diagnostics = Diagnostics::new();

    let program = match aether_parse_ast(code) {
        Ok(p) => p,
        Err(e) => {
            diagnostics.push(Diagnostic::error(
                "AETHER_PARSE_ERROR",
                e.to_string(),
                crate::pytranspile::ir::Span::default(),
            ));
            return diagnostics;
        }
    };

    for stmt in &program {
        check_stmt(stmt, opts, &mut diagnostics);
    }

    diagnostics
}

fn check_stmt(stmt: &crate::ast::Stmt, opts: &TranspileOptions, d: &mut Diagnostics) {
    use crate::ast::Stmt;
    match stmt {
        Stmt::Set { value, .. } => check_expr(value, opts, d),
        Stmt::SetIndex {
            object,
            index,
            value,
        } => {
            check_expr(object, opts, d);
            check_expr(index, opts, d);
            check_expr(value, opts, d);
        }
        Stmt::FuncDef { body, .. } | Stmt::GeneratorDef { body, .. } => {
            for s in body {
                check_stmt(s, opts, d);
            }
        }
        Stmt::LazyDef { expr, .. } => check_expr(expr, opts, d),
        Stmt::Return(expr) | Stmt::Yield(expr) => check_expr(expr, opts, d),
        Stmt::While { condition, body } => {
            check_expr(condition, opts, d);
            for s in body {
                check_stmt(s, opts, d);
            }
        }
        Stmt::For {
            var,
            iterable,
            body,
        } => {
            let _ = var;
            check_expr(iterable, opts, d);
            for s in body {
                check_stmt(s, opts, d);
            }
        }
        Stmt::ForIndexed {
            index_var,
            value_var,
            iterable,
            body,
        } => {
            let _ = (index_var, value_var);
            check_expr(iterable, opts, d);
            for s in body {
                check_stmt(s, opts, d);
            }
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            check_expr(expr, opts, d);
            for (c, b) in cases {
                check_expr(c, opts, d);
                for s in b {
                    check_stmt(s, opts, d);
                }
            }
            if let Some(b) = default {
                for s in b {
                    check_stmt(s, opts, d);
                }
            }
        }
        Stmt::Expression(expr) => check_expr(expr, opts, d),
        Stmt::Import { .. } | Stmt::Export(_) => {}
        Stmt::Throw(expr) => check_expr(expr, opts, d),
        Stmt::Break | Stmt::Continue => {}
    }
}

fn check_expr(expr: &crate::ast::Expr, opts: &TranspileOptions, d: &mut Diagnostics) {
    use crate::ast::Expr;
    match expr {
        Expr::Binary { left, right, .. } => {
            check_expr(left, opts, d);
            check_expr(right, opts, d);
        }
        Expr::Unary { expr, .. } => check_expr(expr, opts, d),
        Expr::Call { func, args } => {
            if let Expr::Identifier(name) = func.as_ref() {
                if opts.reject_console && is_console_builtin(name) {
                    d.push(Diagnostic::error(
                        "AETHER_CONSOLE_REJECTED",
                        format!("console builtin '{name}' is rejected"),
                        crate::pytranspile::ir::Span::default(),
                    ));
                }
                if opts.reject_io && is_io_builtin(name) {
                    d.push(Diagnostic::error(
                        "AETHER_IO_REJECTED",
                        format!("io builtin '{name}' is rejected"),
                        crate::pytranspile::ir::Span::default(),
                    ));
                }
            }
            check_expr(func, opts, d);
            for a in args {
                check_expr(a, opts, d);
            }
        }
        Expr::Array(items) => {
            for e in items {
                check_expr(e, opts, d);
            }
        }
        Expr::Dict(items) => {
            for (_k, v) in items {
                check_expr(v, opts, d);
            }
        }
        Expr::Index { object, index } => {
            check_expr(object, opts, d);
            check_expr(index, opts, d);
        }
        Expr::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
        } => {
            check_expr(condition, opts, d);
            for s in then_branch {
                check_stmt(s, opts, d);
            }
            for (c, b) in elif_branches {
                check_expr(c, opts, d);
                for s in b {
                    check_stmt(s, opts, d);
                }
            }
            if let Some(b) = else_branch {
                for s in b {
                    check_stmt(s, opts, d);
                }
            }
        }
        Expr::Lambda { body, .. } => {
            for s in body {
                check_stmt(s, opts, d);
            }
        }
        Expr::Number(_)
        | Expr::BigInteger(_)
        | Expr::String(_)
        | Expr::Boolean(_)
        | Expr::Null
        | Expr::Identifier(_) => {}
    }
}

fn is_console_builtin(name: &str) -> bool {
    matches!(name, "PRINT" | "PRINTLN" | "INPUT")
}

fn is_io_builtin(name: &str) -> bool {
    matches!(
        name,
        "READ_FILE"
            | "WRITE_FILE"
            | "APPEND_FILE"
            | "DELETE_FILE"
            | "FILE_EXISTS"
            | "LIST_DIR"
            | "CREATE_DIR"
            | "HTTP_GET"
            | "HTTP_POST"
            | "HTTP_PUT"
            | "HTTP_DELETE"
    ) || name.starts_with("EXCEL_")
}

fn program_to_json(program: &crate::ast::Program) -> serde_json::Value {
    serde_json::Value::Array(program.iter().map(stmt_to_json).collect())
}

fn stmt_to_json(stmt: &crate::ast::Stmt) -> serde_json::Value {
    use crate::ast::Stmt;
    match stmt {
        Stmt::Set { name, value } => {
            serde_json::json!({"type":"Set","name":name,"value":expr_to_json(value)})
        }
        Stmt::SetIndex {
            object,
            index,
            value,
        } => {
            serde_json::json!({"type":"SetIndex","object":expr_to_json(object),"index":expr_to_json(index),"value":expr_to_json(value)})
        }
        Stmt::FuncDef { name, params, body } => {
            serde_json::json!({"type":"FuncDef","name":name,"params":params,"body": program_to_json(body)})
        }
        Stmt::GeneratorDef { name, params, body } => {
            serde_json::json!({"type":"GeneratorDef","name":name,"params":params,"body": program_to_json(body)})
        }
        Stmt::LazyDef { name, expr } => {
            serde_json::json!({"type":"LazyDef","name":name,"expr":expr_to_json(expr)})
        }
        Stmt::Return(expr) => serde_json::json!({"type":"Return","value":expr_to_json(expr)}),
        Stmt::Yield(expr) => serde_json::json!({"type":"Yield","value":expr_to_json(expr)}),
        Stmt::Break => serde_json::json!({"type":"Break"}),
        Stmt::Continue => serde_json::json!({"type":"Continue"}),
        Stmt::While { condition, body } => {
            serde_json::json!({"type":"While","condition":expr_to_json(condition),"body": program_to_json(body)})
        }
        Stmt::For {
            var,
            iterable,
            body,
        } => {
            serde_json::json!({"type":"For","var":var,"iterable":expr_to_json(iterable),"body": program_to_json(body)})
        }
        Stmt::ForIndexed {
            index_var,
            value_var,
            iterable,
            body,
        } => {
            serde_json::json!({"type":"ForIndexed","index_var":index_var,"value_var":value_var,"iterable":expr_to_json(iterable),"body": program_to_json(body)})
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            let cases_json: Vec<_> = cases
                .iter()
                .map(|(c, b)| serde_json::json!({"when":expr_to_json(c),"body":program_to_json(b)}))
                .collect();
            let default_json = default.as_ref().map(program_to_json);
            serde_json::json!({"type":"Switch","expr":expr_to_json(expr),"cases":cases_json,"default":default_json})
        }
        Stmt::Import {
            names,
            path,
            aliases,
        } => serde_json::json!({"type":"Import","names":names,"path":path,"aliases":aliases}),
        Stmt::Export(name) => serde_json::json!({"type":"Export","name":name}),
        Stmt::Throw(expr) => serde_json::json!({"type":"Throw","value":expr_to_json(expr)}),
        Stmt::Expression(expr) => {
            serde_json::json!({"type":"Expression","value":expr_to_json(expr)})
        }
    }
}

fn expr_to_json(expr: &crate::ast::Expr) -> serde_json::Value {
    use crate::ast::Expr;
    match expr {
        Expr::Number(n) => serde_json::json!({"type":"Number","value":n}),
        Expr::BigInteger(s) => serde_json::json!({"type":"BigInteger","value":s}),
        Expr::String(s) => serde_json::json!({"type":"String","value":s}),
        Expr::Boolean(b) => serde_json::json!({"type":"Boolean","value":b}),
        Expr::Null => serde_json::json!({"type":"Null"}),
        Expr::Identifier(name) => serde_json::json!({"type":"Identifier","name":name}),
        Expr::Binary { left, op, right } => {
            serde_json::json!({"type":"Binary","op":binop_to_str(op),"left":expr_to_json(left),"right":expr_to_json(right)})
        }
        Expr::Unary { op, expr } => {
            serde_json::json!({"type":"Unary","op":unaryop_to_str(op),"expr":expr_to_json(expr)})
        }
        Expr::Call { func, args } => {
            serde_json::json!({"type":"Call","func":expr_to_json(func),"args": serde_json::Value::Array(args.iter().map(expr_to_json).collect())})
        }
        Expr::Array(items) => {
            serde_json::json!({"type":"Array","items": serde_json::Value::Array(items.iter().map(expr_to_json).collect())})
        }
        Expr::Dict(items) => {
            let pairs: Vec<_> = items
                .iter()
                .map(|(k, v)| serde_json::json!({"key":k,"value":expr_to_json(v)}))
                .collect();
            serde_json::json!({"type":"Dict","items":pairs})
        }
        Expr::Index { object, index } => {
            serde_json::json!({"type":"Index","object":expr_to_json(object),"index":expr_to_json(index)})
        }
        Expr::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
        } => {
            let elifs: Vec<_> = elif_branches.iter().map(|(c, b)| serde_json::json!({"condition":expr_to_json(c),"body":program_to_json(b)})).collect();
            let else_json = else_branch.as_ref().map(program_to_json);
            serde_json::json!({"type":"If","condition":expr_to_json(condition),"then":program_to_json(then_branch),"elifs":elifs,"else":else_json})
        }
        Expr::Lambda { params, body } => {
            serde_json::json!({"type":"Lambda","params":params,"body":program_to_json(body)})
        }
    }
}

fn binop_to_str(op: &crate::ast::BinOp) -> &'static str {
    use crate::ast::BinOp;
    match op {
        BinOp::Add => "+",
        BinOp::Subtract => "-",
        BinOp::Multiply => "*",
        BinOp::Divide => "/",
        BinOp::Modulo => "%",
        BinOp::Equal => "==",
        BinOp::NotEqual => "!=",
        BinOp::Less => "<",
        BinOp::LessEqual => "<=",
        BinOp::Greater => ">",
        BinOp::GreaterEqual => ">=",
        BinOp::And => "&&",
        BinOp::Or => "||",
    }
}

fn unaryop_to_str(op: &crate::ast::UnaryOp) -> &'static str {
    use crate::ast::UnaryOp;
    match op {
        UnaryOp::Minus => "-",
        UnaryOp::Not => "!",
    }
}
