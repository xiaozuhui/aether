#[cfg(feature = "pytranspile")]
use pyo3::{prelude::*, types::PyList};

use crate::pytranspile::diagnostics::{Diagnostic, Diagnostics};
#[cfg(feature = "pytranspile")]
use crate::pytranspile::ir::{Expr, Stmt};
use crate::pytranspile::ir::{Module, Span};
use crate::pytranspile::options::TranspileOptions;

#[derive(Debug)]
pub struct PythonIrResult {
    pub module: Option<Module>,
    pub diagnostics: Diagnostics,
    pub numpy_used: bool,
    pub io_used: bool,
    pub console_used: bool,
}

impl PythonIrResult {
    fn fail(diagnostics: Diagnostics) -> Self {
        Self {
            module: None,
            diagnostics,
            numpy_used: false,
            io_used: false,
            console_used: false,
        }
    }
}

#[cfg(feature = "pytranspile")]
pub fn python_to_ir(source: &str, opts: &TranspileOptions) -> PythonIrResult {
    Python::attach(|py| {
        let mut diagnostics = Diagnostics::new();

        let ast = match py.import("ast") {
            Ok(m) => m,
            Err(err) => {
                diagnostics.push(Diagnostic::error(
                    "PY_IMPORT_AST_FAILED",
                    format!("failed to import python ast module: {err}"),
                    Span::default(),
                ));
                return PythonIrResult::fail(diagnostics);
            }
        };

        let tree = match ast.call_method1("parse", (source,)) {
            Ok(t) => t,
            Err(err) => {
                let msg = err.to_string();
                diagnostics.push(Diagnostic::error("PY_SYNTAX_ERROR", msg, Span::default()));
                return PythonIrResult::fail(diagnostics);
            }
        };

        let mut builder = IrBuilder::new(py, ast, opts, diagnostics);
        let module = builder.emit_module(tree.as_ref());

        // Enforce hard rejections as early diagnostics.
        if opts.reject_numpy && builder.numpy_used {
            builder.diagnostics.push(Diagnostic::error(
                "PY_NUMPY_REJECTED",
                "numpy usage is rejected by transpile options",
                Span::default(),
            ));
        }
        if opts.reject_io && builder.io_used {
            builder.diagnostics.push(Diagnostic::error(
                "PY_IO_REJECTED",
                "filesystem/network usage is rejected by transpile options",
                Span::default(),
            ));
        }
        if opts.reject_console && builder.console_used {
            builder.diagnostics.push(Diagnostic::error(
                "PY_CONSOLE_REJECTED",
                "console IO (print/input) is rejected by transpile options",
                Span::default(),
            ));
        }

        let (module, diagnostics, numpy_used, io_used, console_used) = (
            module,
            builder.diagnostics,
            builder.numpy_used,
            builder.io_used,
            builder.console_used,
        );

        PythonIrResult {
            module,
            diagnostics,
            numpy_used,
            io_used,
            console_used,
        }
    })
}

#[cfg(not(feature = "pytranspile"))]
pub fn python_to_ir(_source: &str, _opts: &TranspileOptions) -> PythonIrResult {
    let mut diagnostics = Diagnostics::new();
    diagnostics.push(Diagnostic::error(
        "PYTRANSPILE_FEATURE_DISABLED",
        "enable cargo feature `pytranspile` to use python_to_ir",
        Span::default(),
    ));
    PythonIrResult::fail(diagnostics)
}

#[cfg(feature = "pytranspile")]
struct IrBuilder {
    diagnostics: Diagnostics,
    numpy_used: bool,
    io_used: bool,
    console_used: bool,
    aliases: std::collections::HashMap<String, String>,
}

#[cfg(feature = "pytranspile")]
impl IrBuilder {
    fn new(
        _py: Python<'_>,
        _ast: Bound<'_, PyModule>,
        _opts: &TranspileOptions,
        diagnostics: Diagnostics,
    ) -> Self {
        Self {
            diagnostics,
            numpy_used: false,
            io_used: false,
            console_used: false,
            aliases: std::collections::HashMap::new(),
        }
    }

    fn span_of(&self, node: &Bound<'_, PyAny>) -> Span {
        let line = node
            .getattr("lineno")
            .ok()
            .and_then(|v| v.extract().ok())
            .unwrap_or(0);
        let col = node
            .getattr("col_offset")
            .ok()
            .and_then(|v| v.extract().ok())
            .unwrap_or(0);
        let end_line = node
            .getattr("end_lineno")
            .ok()
            .and_then(|v| v.extract().ok())
            .unwrap_or(0);
        let end_col = node
            .getattr("end_col_offset")
            .ok()
            .and_then(|v| v.extract().ok())
            .unwrap_or(0);

        Span {
            line,
            col,
            end_line,
            end_col,
        }
    }

    fn node_type(&self, node: &Bound<'_, PyAny>) -> Option<String> {
        node.get_type()
            .name()
            .ok()
            .map(|s| s.to_string_lossy().into_owned())
    }

    fn type_name(&self, node: &Bound<'_, PyAny>, fallback: &str) -> String {
        node.get_type()
            .name()
            .ok()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| fallback.to_string())
    }

    fn emit_module(&mut self, node: &Bound<'_, PyAny>) -> Option<Module> {
        let span = self.span_of(node);
        let ty = self.node_type(node)?;
        if ty != "Module" {
            self.diagnostics.push(Diagnostic::error(
                "PY_UNEXPECTED_ROOT",
                format!("expected ast.Module, found {ty}"),
                span,
            ));
            return None;
        }

        let body_list = node.getattr("body").ok()?.cast_into::<PyList>().ok()?;
        let body: Vec<Bound<'_, PyAny>> = body_list.iter().collect();

        let mut out = Vec::new();
        for stmt in body {
            out.push(self.emit_stmt(&stmt));
        }

        Some(Module { span, body: out })
    }

    fn emit_stmt(&mut self, node: &Bound<'_, PyAny>) -> Stmt {
        let span = self.span_of(node);
        let ty = self
            .node_type(node)
            .unwrap_or_else(|| "<unknown>".to_string());

        match ty.as_str() {
            "Assign" => {
                let targets = node.getattr("targets").ok();
                if let Some(targets) = targets {
                    if let Ok(list) = targets.cast_into::<PyList>() {
                        if list.len() != 1 {
                            return Stmt::Unsupported {
                                span,
                                reason: "multiple assignment targets".to_string(),
                            };
                        }
                        let target = self.emit_expr(&list.get_item(0).unwrap());
                        let value = self.emit_expr(&node.getattr("value").unwrap());
                        return Stmt::Assign {
                            span,
                            target,
                            value,
                        };
                    }
                }
                Stmt::Unsupported {
                    span,
                    reason: "invalid Assign".to_string(),
                }
            }
            "AugAssign" => {
                // target += value  ==>  target = (target + value)
                let target = self.emit_expr(&node.getattr("target").unwrap());
                let value = self.emit_expr(&node.getattr("value").unwrap());
                let op = node.getattr("op").unwrap();
                let op_name = self.type_name(&op, "<op>");
                let combined = Expr::BinOp {
                    span,
                    op: op_name,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                };
                Stmt::Assign {
                    span,
                    target,
                    value: combined,
                }
            }
            "Expr" => {
                let value = self.emit_expr(&node.getattr("value").unwrap());
                // detect python print()
                if let Expr::Call { func, .. } = &value {
                    if let Expr::Name { id, .. } = func.as_ref() {
                        if id == "print" {
                            self.console_used = true;
                        }
                    }
                }
                Stmt::ExprStmt { span, value }
            }
            "Return" => {
                let value = node.getattr("value").ok().and_then(|v| {
                    if v.is_none() {
                        None
                    } else {
                        Some(self.emit_expr(&v))
                    }
                });
                Stmt::Return { span, value }
            }
            "Pass" => Stmt::Pass { span },
            "Break" => Stmt::Break { span },
            "Continue" => Stmt::Continue { span },
            "If" => {
                let test = self.emit_expr(&node.getattr("test").unwrap());
                let body = self.emit_stmt_list(node.getattr("body").unwrap());
                let orelse = self.emit_stmt_list(node.getattr("orelse").unwrap());
                Stmt::If {
                    span,
                    test,
                    body,
                    orelse,
                }
            }
            "While" => {
                let test = self.emit_expr(&node.getattr("test").unwrap());
                let body = self.emit_stmt_list(node.getattr("body").unwrap());
                Stmt::While { span, test, body }
            }
            "For" => {
                let target = self.emit_expr(&node.getattr("target").unwrap());
                let iter = self.emit_expr(&node.getattr("iter").unwrap());
                let body = self.emit_stmt_list(node.getattr("body").unwrap());
                Stmt::For {
                    span,
                    target,
                    iter,
                    body,
                }
            }
            "FunctionDef" => {
                let name: String = node.getattr("name").unwrap().extract().unwrap_or_default();
                let args = node.getattr("args").unwrap();
                let args_list = args.getattr("args").unwrap();
                let mut params = Vec::new();
                if let Ok(list) = args_list.cast_into::<PyList>() {
                    for item in list.iter() {
                        let arg_name: String =
                            item.getattr("arg").unwrap().extract().unwrap_or_default();
                        params.push(arg_name);
                    }
                }
                let body = self.emit_stmt_list(node.getattr("body").unwrap());
                Stmt::FunctionDef {
                    span,
                    name,
                    args: params,
                    body,
                }
            }
            "Import" => {
                let names = node.getattr("names").unwrap();
                if let Ok(list) = names.cast_into::<PyList>() {
                    // one per stmt; expand to multiple Stmt::Import for simplicity
                    let mut out: Vec<Stmt> = Vec::new();
                    for alias in list.iter() {
                        let name: String =
                            alias.getattr("name").unwrap().extract().unwrap_or_default();
                        let asname: Option<String> =
                            alias.getattr("asname").ok().and_then(|v| v.extract().ok());
                        if name.split('.').next() == Some("numpy")
                            || asname.as_deref() == Some("np")
                        {
                            self.numpy_used = true;
                        }
                        if let Some(a) = asname.clone() {
                            self.aliases.insert(a, name.clone());
                        }
                        out.push(Stmt::Import {
                            span,
                            module: name,
                            asname,
                        });
                    }
                    if out.len() == 1 {
                        return out.remove(0);
                    }
                    return Stmt::Unsupported {
                        span,
                        reason: "multiple imports in single statement".to_string(),
                    };
                }
                Stmt::Unsupported {
                    span,
                    reason: "invalid Import".to_string(),
                }
            }
            "ImportFrom" => {
                let module: String = node
                    .getattr("module")
                    .ok()
                    .and_then(|v| v.extract().ok())
                    .unwrap_or_default();
                if module.split('.').next() == Some("numpy") {
                    self.numpy_used = true;
                }
                let names = node.getattr("names").unwrap();
                if let Ok(list) = names.cast_into::<PyList>() {
                    if list.len() != 1 {
                        return Stmt::Unsupported {
                            span,
                            reason: "multiple from-import names".to_string(),
                        };
                    }
                    let alias = list.get_item(0).unwrap();
                    let name: String = alias.getattr("name").unwrap().extract().unwrap_or_default();
                    let asname: Option<String> =
                        alias.getattr("asname").ok().and_then(|v| v.extract().ok());
                    if let Some(a) = asname.clone() {
                        self.aliases.insert(a, format!("{module}.{name}"));
                    }
                    return Stmt::ImportFrom {
                        span,
                        module,
                        name,
                        asname,
                    };
                }
                Stmt::Unsupported {
                    span,
                    reason: "invalid ImportFrom".to_string(),
                }
            }
            _ => Stmt::Unsupported { span, reason: ty },
        }
    }

    fn emit_stmt_list(&mut self, list_any: Bound<'_, PyAny>) -> Vec<Stmt> {
        if let Ok(list) = list_any.cast_into::<PyList>() {
            list.iter().map(|n| self.emit_stmt(&n)).collect()
        } else {
            vec![Stmt::Unsupported {
                span: Span::default(),
                reason: "expected list".to_string(),
            }]
        }
    }

    fn emit_expr(&mut self, node: &Bound<'_, PyAny>) -> Expr {
        let span = self.span_of(node);
        let ty = self
            .node_type(node)
            .unwrap_or_else(|| "<unknown>".to_string());

        match ty.as_str() {
            "Name" => {
                let id: String = node.getattr("id").unwrap().extract().unwrap_or_default();
                Expr::Name { span, id }
            }
            "Constant" => {
                if node.is_none() {
                    return Expr::None { span };
                }
                let value = node.getattr("value").ok();
                if let Some(value) = value {
                    if value.is_none() {
                        return Expr::None { span };
                    }
                    if let Ok(v) = value.extract::<bool>() {
                        return Expr::Bool { span, value: v };
                    }
                    if let Ok(v) = value.extract::<i64>() {
                        return Expr::Number {
                            span,
                            value: v as f64,
                        };
                    }
                    if let Ok(v) = value.extract::<f64>() {
                        return Expr::Number { span, value: v };
                    }
                    if let Ok(v) = value.extract::<String>() {
                        return Expr::String { span, value: v };
                    }
                }
                Expr::Unsupported {
                    span,
                    reason: "unsupported constant".to_string(),
                }
            }
            "BinOp" => {
                let op = node.getattr("op").unwrap();
                let op_name = self.type_name(&op, "<op>");
                let left = self.emit_expr(&node.getattr("left").unwrap());
                let right = self.emit_expr(&node.getattr("right").unwrap());
                Expr::BinOp {
                    span,
                    op: op_name,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            "UnaryOp" => {
                let op = node.getattr("op").unwrap();
                let op_name = self.type_name(&op, "<op>");
                let operand = self.emit_expr(&node.getattr("operand").unwrap());
                Expr::UnaryOp {
                    span,
                    op: op_name,
                    operand: Box::new(operand),
                }
            }
            "BoolOp" => {
                let op = node.getattr("op").unwrap();
                let op_name = self.type_name(&op, "<op>");
                let values_any = node.getattr("values").unwrap();
                if let Ok(list) = values_any.cast_into::<PyList>() {
                    let values = list.iter().map(|n| self.emit_expr(&n)).collect();
                    Expr::BoolOp {
                        span,
                        op: op_name,
                        values,
                    }
                } else {
                    Expr::Unsupported {
                        span,
                        reason: "invalid BoolOp".to_string(),
                    }
                }
            }
            "Compare" => {
                let ops_any = node.getattr("ops").unwrap();
                let comps_any = node.getattr("comparators").unwrap();
                if let (Ok(ops), Ok(comps)) = (
                    ops_any.cast_into::<PyList>(),
                    comps_any.cast_into::<PyList>(),
                ) {
                    if ops.len() != 1 || comps.len() != 1 {
                        return Expr::Unsupported {
                            span,
                            reason: "chained comparison".to_string(),
                        };
                    }
                    let op_name = self.type_name(&ops.get_item(0).unwrap(), "<op>");
                    let left = self.emit_expr(&node.getattr("left").unwrap());
                    let right = self.emit_expr(&comps.get_item(0).unwrap());
                    return Expr::Compare {
                        span,
                        op: op_name,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Expr::Unsupported {
                    span,
                    reason: "invalid Compare".to_string(),
                }
            }
            "Call" => {
                let func = self.emit_expr(&node.getattr("func").unwrap());
                let args_any = node.getattr("args").unwrap();
                let mut args = Vec::new();
                if let Ok(list) = args_any.cast_into::<PyList>() {
                    for item in list.iter() {
                        args.push(self.emit_expr(&item));
                    }
                }

                // Precheck: filesystem/network based on callee
                self.inspect_call_for_io(span, &func);

                Expr::Call {
                    span,
                    func: Box::new(func),
                    args,
                }
            }
            "Attribute" => {
                let value = self.emit_expr(&node.getattr("value").unwrap());
                let attr: String = node.getattr("attr").unwrap().extract().unwrap_or_default();

                // Detect numpy usage via alias `np.*`
                if let Expr::Name { id, .. } = &value {
                    if id == "np" {
                        self.numpy_used = true;
                    }
                    if let Some(resolved) = self.aliases.get(id) {
                        if resolved.split('.').next() == Some("numpy") {
                            self.numpy_used = true;
                        }
                    }
                }

                Expr::Attribute {
                    span,
                    value: Box::new(value),
                    attr,
                }
            }
            "List" => {
                let elts_any = node.getattr("elts").unwrap();
                if let Ok(list) = elts_any.cast_into::<PyList>() {
                    let elts = list.iter().map(|n| self.emit_expr(&n)).collect();
                    Expr::List { span, elts }
                } else {
                    Expr::Unsupported {
                        span,
                        reason: "invalid List".to_string(),
                    }
                }
            }
            "Dict" => {
                let keys_any = node.getattr("keys").unwrap();
                let values_any = node.getattr("values").unwrap();
                if let (Ok(keys), Ok(values)) = (
                    keys_any.cast_into::<PyList>(),
                    values_any.cast_into::<PyList>(),
                ) {
                    let mut items = Vec::new();
                    for (k, v) in keys.iter().zip(values.iter()) {
                        // Dict unpacking like {**x} appears as a None key in Python AST.
                        if k.is_none() {
                            return Expr::Unsupported {
                                span,
                                reason: "dict unpack is not supported".to_string(),
                            };
                        }
                        items.push((self.emit_expr(&k), self.emit_expr(&v)));
                    }
                    Expr::Dict { span, items }
                } else {
                    Expr::Unsupported {
                        span,
                        reason: "invalid Dict".to_string(),
                    }
                }
            }
            "Subscript" => {
                let value = self.emit_expr(&node.getattr("value").unwrap());
                let slice = node.getattr("slice").unwrap();
                // python 3.9+: slice is Expr; we reject Slice objects
                let idx_ty = self.node_type(&slice).unwrap_or_default();
                if idx_ty == "Slice" {
                    return Expr::Unsupported {
                        span,
                        reason: "slice is not supported".to_string(),
                    };
                }
                let index = self.emit_expr(&slice);
                Expr::Subscript {
                    span,
                    value: Box::new(value),
                    index: Box::new(index),
                }
            }
            _ => Expr::Unsupported { span, reason: ty },
        }
    }

    fn inspect_call_for_io(&mut self, _span: Span, func: &Expr) {
        fn leftmost_name(expr: &Expr) -> Option<&str> {
            match expr {
                Expr::Name { id, .. } => Some(id.as_str()),
                Expr::Attribute { value, .. } => leftmost_name(value.as_ref()),
                _ => None,
            }
        }

        if let Some(base) = leftmost_name(func) {
            if base == "open" {
                self.io_used = true;
            }
            if base == "socket" || base == "urllib" || base == "requests" || base == "http" {
                self.io_used = true;
            }
            if base == "print" {
                self.console_used = true;
            }
            if base == "os" || base == "pathlib" || base == "shutil" {
                self.io_used = true;
            }
        }
    }
}
