use crate::pytranspile::diagnostics::{Diagnostic, Diagnostics};
use crate::pytranspile::ir::{Expr, Module, Span, Stmt};
use crate::pytranspile::options::{DecimalMode, TranspileOptions};

#[derive(Debug)]
pub struct EmitResult {
    pub code: Option<String>,
    pub diagnostics: Diagnostics,
}

pub fn ir_to_aether(module: &Module, opts: &TranspileOptions) -> EmitResult {
    let mut diagnostics = Diagnostics::new();
    let mut out = String::new();

    let mut emitter = Emitter {
        indent: 0,
        diagnostics: &mut diagnostics,
        opts,
    };

    for (i, stmt) in module.body.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        emitter.emit_stmt(stmt, &mut out);
    }

    if diagnostics.has_errors() {
        EmitResult {
            code: None,
            diagnostics,
        }
    } else {
        EmitResult {
            code: Some(out),
            diagnostics,
        }
    }
}

struct Emitter<'a> {
    indent: usize,
    diagnostics: &'a mut Diagnostics,
    opts: &'a TranspileOptions,
}

impl<'a> Emitter<'a> {
    fn emit_stmt(&mut self, stmt: &Stmt, out: &mut String) {
        match stmt {
            Stmt::Assign {
                span,
                target,
                value,
            } => {
                self.write_indent(out);
                match target {
                    Expr::Name { id, .. } => {
                        out.push_str("Set ");
                        out.push_str(&to_aether_ident(id));
                        out.push(' ');
                        self.emit_expr(value, out);
                    }
                    Expr::Subscript {
                        value: base, index, ..
                    } => {
                        out.push_str("Set ");
                        self.emit_subscript_lhs(base, index, out);
                        out.push(' ');
                        self.emit_expr(value, out);
                    }
                    Expr::Unsupported { .. } => {
                        self.diagnostics.push(Diagnostic::error(
                            "PY_UNSUPPORTED",
                            "unsupported assignment target",
                            *span,
                        ));
                    }
                    _ => {
                        self.diagnostics.push(Diagnostic::error(
                            "PY_UNSUPPORTED",
                            "unsupported assignment target",
                            *span,
                        ));
                    }
                }
            }
            Stmt::ExprStmt { value, .. } => {
                self.write_indent(out);
                self.emit_expr(value, out);
            }
            Stmt::Return { value, .. } => {
                self.write_indent(out);
                out.push_str("Return");
                if let Some(v) = value {
                    out.push(' ');
                    self.emit_expr(v, out);
                }
            }
            Stmt::If {
                span,
                test,
                body,
                orelse,
            } => {
                self.write_indent(out);
                out.push_str("If (");
                self.emit_expr(test, out);
                out.push_str(") {");
                out.push('\n');

                self.indent += 1;
                for s in body {
                    self.emit_stmt(s, out);
                    out.push('\n');
                }
                self.indent -= 1;

                self.write_indent(out);
                out.push('}');

                if !orelse.is_empty() {
                    out.push_str(" Else {");
                    out.push('\n');
                    self.indent += 1;
                    for s in orelse {
                        self.emit_stmt(s, out);
                        out.push('\n');
                    }
                    self.indent -= 1;
                    self.write_indent(out);
                    out.push('}');
                }

                // remove trailing newline(s) emitted by caller loops
                let _ = span;
            }
            Stmt::While { test, body, .. } => {
                self.write_indent(out);
                out.push_str("While (");
                self.emit_expr(test, out);
                out.push_str(") {");
                out.push('\n');

                self.indent += 1;
                for s in body {
                    self.emit_stmt(s, out);
                    out.push('\n');
                }
                self.indent -= 1;
                self.write_indent(out);
                out.push('}');
            }
            Stmt::For {
                span,
                target,
                iter,
                body,
            } => {
                self.write_indent(out);
                out.push_str("For ");
                match target {
                    Expr::Name { id, .. } => out.push_str(&to_aether_ident(id)),
                    _ => {
                        self.diagnostics.push(Diagnostic::error(
                            "PY_UNSUPPORTED",
                            "unsupported for-loop target",
                            *span,
                        ));
                        out.push_str("_UNSUPPORTED_");
                    }
                }
                out.push_str(" In ");
                self.emit_expr(iter, out);
                out.push_str(" {");
                out.push('\n');

                self.indent += 1;
                for s in body {
                    self.emit_stmt(s, out);
                    out.push('\n');
                }
                self.indent -= 1;
                self.write_indent(out);
                out.push('}');
            }
            Stmt::FunctionDef {
                name, args, body, ..
            } => {
                self.write_indent(out);
                out.push_str("Func ");
                out.push_str(&to_aether_ident(name));
                out.push('(');
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&to_aether_ident(a));
                }
                out.push_str(") {");
                out.push('\n');

                self.indent += 1;
                for s in body {
                    self.emit_stmt(s, out);
                    out.push('\n');
                }
                self.indent -= 1;

                self.write_indent(out);
                out.push('}');
            }
            Stmt::Import { span, .. } | Stmt::ImportFrom { span, .. } => {
                // Python imports don't translate to Aether; treat as warning.
                self.diagnostics.push(Diagnostic::warning(
                    "PY_IMPORT_IGNORED",
                    "python import statements are ignored during transpilation",
                    *span,
                ));
                self.write_indent(out);
                out.push_str("// Import ignored");
            }
            Stmt::Break { .. } => {
                self.write_indent(out);
                out.push_str("Break");
            }
            Stmt::Continue { .. } => {
                self.write_indent(out);
                out.push_str("Continue");
            }
            Stmt::Pass { .. } => {
                self.write_indent(out);
                out.push_str("// Pass");
            }
            Stmt::Unsupported { span, reason } => {
                self.diagnostics.push(Diagnostic::error(
                    "PY_UNSUPPORTED",
                    format!("unsupported statement: {reason}"),
                    *span,
                ));
                self.write_indent(out);
                out.push_str("// Unsupported statement");
            }
        }
    }

    fn emit_expr(&mut self, expr: &Expr, out: &mut String) {
        match expr {
            Expr::Name { id, .. } => out.push_str(&to_aether_ident(id)),
            Expr::Number { value, .. } => {
                self.emit_number(*value, out);
            }
            Expr::String { value, .. } => {
                out.push('"');
                out.push_str(&escape_string(value));
                out.push('"');
            }
            Expr::Bool { value, .. } => out.push_str(if *value { "True" } else { "False" }),
            Expr::None { .. } => out.push_str("Null"),
            Expr::List { elts, .. } => {
                out.push('[');
                for (i, e) in elts.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    self.emit_expr(e, out);
                }
                out.push(']');
            }
            Expr::Dict { items, .. } => {
                out.push('{');
                for (i, (k, v)) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    self.emit_expr(k, out);
                    out.push_str(": ");
                    self.emit_expr(v, out);
                }
                out.push('}');
            }
            Expr::BinOp {
                op, left, right, ..
            } => {
                out.push('(');
                self.emit_expr(left, out);
                out.push(' ');
                out.push_str(&map_binop(op));
                out.push(' ');
                self.emit_expr(right, out);
                out.push(')');
            }
            Expr::UnaryOp { op, operand, .. } => {
                out.push('(');
                out.push_str(&map_unaryop(op));
                self.emit_expr(operand, out);
                out.push(')');
            }
            Expr::BoolOp { op, values, .. } => {
                out.push('(');
                let aop = map_boolop(op);
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                        out.push_str(aop);
                        out.push(' ');
                    }
                    self.emit_expr(v, out);
                }
                out.push(')');
            }
            Expr::Compare {
                op, left, right, ..
            } => {
                out.push('(');
                self.emit_expr(left, out);
                out.push(' ');
                out.push_str(&map_cmp(op));
                out.push(' ');
                self.emit_expr(right, out);
                out.push(')');
            }
            Expr::Call { func, args, span } => {
                // Reject attribute calls for now (no dot operator in Aether).
                match func.as_ref() {
                    Expr::Name { id, .. } => {
                        out.push_str(&to_aether_ident(id));
                        out.push('(');
                        for (i, a) in args.iter().enumerate() {
                            if i > 0 {
                                out.push_str(", ");
                            }
                            self.emit_expr(a, out);
                        }
                        out.push(')');
                    }
                    _ => {
                        self.diagnostics.push(Diagnostic::error(
                            "PY_UNSUPPORTED",
                            "unsupported call target (only direct function calls are supported)",
                            *span,
                        ));
                        out.push_str("_UNSUPPORTED_CALL_()");
                    }
                }
            }
            Expr::Subscript { value, index, .. } => {
                self.emit_expr(value, out);
                out.push('[');
                self.emit_expr(index, out);
                out.push(']');
            }
            Expr::Attribute { span, .. } => {
                self.diagnostics.push(Diagnostic::error(
                    "PY_UNSUPPORTED",
                    "python attribute access is not supported in aether output",
                    *span,
                ));
                out.push_str("_UNSUPPORTED_ATTR_");
            }
            Expr::Unsupported { span, reason } => {
                self.diagnostics.push(Diagnostic::error(
                    "PY_UNSUPPORTED",
                    format!("unsupported expression: {reason}"),
                    *span,
                ));
                out.push_str("_UNSUPPORTED_EXPR_");
            }
        }
    }

    fn emit_subscript_lhs(&mut self, base: &Expr, index: &Expr, out: &mut String) {
        // Keep tight `A[B]` formatting.
        self.emit_expr(base, out);
        out.push('[');
        self.emit_expr(index, out);
        out.push(']');
    }

    fn write_indent(&self, out: &mut String) {
        for _ in 0..self.indent {
            out.push_str("    ");
        }
    }

    fn emit_number(&mut self, value: f64, out: &mut String) {
        if value.fract() == 0.0 {
            out.push_str(&format!("{}", value as i64));
            return;
        }

        match self.opts.decimal_mode {
            DecimalMode::FixedPrecision => {
                let scale = self.opts.calc_scale;
                if scale == 0 {
                    out.push_str(&format!("{}", value.trunc() as i64));
                    return;
                }

                let denom = 10_f64.powi(scale as i32);
                let scaled = (value * denom).round();

                // If scaling collapsed to an integer, keep the simpler literal.
                if (scaled / denom).fract() == 0.0 {
                    out.push_str(&format!("{}", (scaled / denom) as i64));
                    return;
                }

                // Use Fraction builtins to preserve decimal intent in evaluation.
                // FRAC_DIV(int, int) -> Fraction
                let denom_i128 = 10_i128.pow(scale);
                let scaled_i128 = scaled as i128;
                out.push_str("FRAC_DIV(");
                out.push_str(&scaled_i128.to_string());
                out.push_str(", ");
                out.push_str(&denom_i128.to_string());
                out.push(')');
            }
        }
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn to_aether_ident(py_ident: &str) -> String {
    // Convert python-ish names to Aether's required UPPER_SNAKE_CASE.
    // This is intentionally conservative: alnum + '_' only.
    let mut out = String::new();
    let mut prev_is_underscore = false;

    for (i, ch) in py_ident.chars().enumerate() {
        if ch.is_ascii_alphanumeric() {
            // Insert underscore before an uppercase letter in camelCase/PascalCase.
            if ch.is_ascii_uppercase() && i > 0 && !prev_is_underscore {
                out.push('_');
            }
            out.push(ch.to_ascii_uppercase());
            prev_is_underscore = false;
        } else if ch == '_' {
            if !prev_is_underscore && !out.is_empty() {
                out.push('_');
                prev_is_underscore = true;
            }
        } else {
            // Drop other characters.
            if !prev_is_underscore && !out.is_empty() {
                out.push('_');
                prev_is_underscore = true;
            }
        }
    }

    if out.is_empty() {
        "_".to_string()
    } else {
        out.trim_matches('_').to_string()
    }
}

fn map_binop(op: &str) -> String {
    match op {
        "Add" => "+".to_string(),
        "Sub" => "-".to_string(),
        "Mult" => "*".to_string(),
        "Div" => "/".to_string(),
        "Mod" => "%".to_string(),
        _ => format!("/*{op}*/"),
    }
}

fn map_unaryop(op: &str) -> String {
    match op {
        "USub" => "-".to_string(),
        "UAdd" => "+".to_string(),
        "Not" => "!".to_string(),
        _ => format!("/*{op}*/"),
    }
}

fn map_boolop(op: &str) -> &'static str {
    match op {
        "And" => "And",
        "Or" => "Or",
        _ => "And",
    }
}

fn map_cmp(op: &str) -> String {
    match op {
        "Eq" => "==".to_string(),
        "NotEq" => "!=".to_string(),
        "Lt" => "<".to_string(),
        "LtE" => "<=".to_string(),
        "Gt" => ">".to_string(),
        "GtE" => ">=".to_string(),
        _ => format!("/*{op}*/"),
    }
}

#[allow(dead_code)]
fn span_default() -> Span {
    Span::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pytranspile::ir::{Expr, Module, Span, Stmt};

    #[test]
    fn emits_fixed_precision_decimal_as_fraction() {
        let module = Module {
            span: Span::default(),
            body: vec![Stmt::ExprStmt {
                span: Span::default(),
                value: Expr::Number {
                    span: Span::default(),
                    value: 12.34,
                },
            }],
        };

        let opts = TranspileOptions {
            calc_scale: 2,
            ..Default::default()
        };

        let res = ir_to_aether(&module, &opts);
        assert!(!res.diagnostics.has_errors());
        let code = res.code.expect("expected emitted code");
        assert!(code.contains("FRAC_DIV("));
        assert!(code.contains("1234"));
        assert!(code.contains("100"));
    }
}
