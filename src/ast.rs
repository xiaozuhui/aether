// src/ast.rs
//! Abstract Syntax Tree (AST) definitions for Aether
//!
//! This module defines the structure of Aether programs as a tree of nodes.

use std::collections::HashSet;

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    // Arithmetic
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // Comparison
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // Logical
    And, // &&
    Or,  // ||

    // Bitwise
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    Shl,    // <<
    Shr,    // >>
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus, // -
    Not,   // !
}

/// Expressions - things that evaluate to values
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Number(f64),
    BigInteger(num_bigint::BigInt), // 大整数字面量（解析期一次性构造）
    String(String),
    Boolean(bool),
    Null,

    // Identifier (variable reference)
    Identifier(String),

    // Binary operation: (left op right)
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    // Unary operation: (op expr)
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    // Function call: FUNC(arg1, arg2, ...)
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },

    // Array literal: [1, 2, 3]
    Array(Vec<Expr>),

    // Dictionary literal: {key: value, ...}
    Dict(Vec<(String, Expr)>),

    // Array/Dict access: array[index] or dict[key]
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    // If expression (can return value)
    If {
        condition: Box<Expr>,
        then_branch: Vec<Located>,
        elif_branches: Vec<(Expr, Vec<Located>)>, // (condition, body) pairs
        else_branch: Option<Vec<Located>>,
    },

    // Anonymous function
    Lambda {
        params: Vec<String>,
        body: Vec<Located>,
    },
}

/// Statements - things that perform actions
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Variable assignment: Set NAME value
    Set {
        name: String,
        value: Expr,
    },

    // Index assignment: Set OBJECT[INDEX] value (for arrays and dicts)
    SetIndex {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Expr,
    },

    // Function definition: Func NAME (params) { body }
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Located>,
    },

    // Generator definition: Generator NAME (params) { body }
    GeneratorDef {
        name: String,
        params: Vec<String>,
        body: Vec<Located>,
    },

    // Lazy variable: Lazy NAME (expr)
    LazyDef {
        name: String,
        expr: Expr,
    },

    // Return statement: Return expr
    Return(Expr),

    // Yield statement (for generators): Yield expr
    Yield(Expr),

    // Break statement: Break (exit loop)
    Break,

    // Continue statement: Continue (skip to next iteration)
    Continue,

    // While loop: While (condition) { body }
    While {
        condition: Expr,
        body: Vec<Located>,
    },

    // For loop: For VAR In ITERABLE { body }
    For {
        var: String,
        iterable: Expr,
        body: Vec<Located>,
    },

    // For loop with index: For INDEX, VAR In ITERABLE { body }
    ForIndexed {
        index_var: String,
        value_var: String,
        iterable: Expr,
        body: Vec<Located>,
    },

    // Switch statement: Switch (expr) { Case val: body ... Default: body }
    Switch {
        expr: Expr,
        cases: Vec<(Expr, Vec<Located>)>,
        default: Option<Vec<Located>>,
    },

    // Import statement:
    // - Named imports: Import {NAME1, NAME2} From PATH
    // - Named import with alias: Import NAME As ALIAS From PATH
    // - Namespace import: Import NS From PATH  (bind module exports as Dict to NS)
    Import {
        names: Vec<String>,
        path: String,
        aliases: Vec<Option<String>>, // Optional aliases (As NAME)
        namespace: Option<String>,    // Namespace binding name
    },

    // Export statement: Export NAME
    Export(String),

    // Throw statement: Throw message
    Throw(Expr),

    // Expression statement (expression as statement)
    Expression(Expr),
}

/// 带源码行号的语句包装（行号为语句首 token 所在行，供调试器断点定位）
#[derive(Debug, Clone)]
pub struct Located {
    pub line: usize,
    pub stmt: Stmt,
}

// 手工实现并忽略 line：行号是位置元数据而非程序语义，
// 优化器改写或测试构造的 AST 不应因行号差异而不相等
impl PartialEq for Located {
    fn eq(&self, other: &Self) -> bool {
        self.stmt == other.stmt
    }
}
impl Eq for Located {}

/// 包装语句并标注行号的便捷构造
pub fn located(line: usize, stmt: Stmt) -> Located {
    Located { line, stmt }
}

/// A complete program is a list of statements
pub type Program = Vec<Located>;

/// 递归收集程序中所有语句起始行号（供调试器校验断点行是否可命中）
pub fn collect_breakable_lines(program: &Program) -> HashSet<usize> {
    let mut lines = HashSet::new();
    for l in program {
        lines.insert(l.line);
        collect_stmt_lines(&l.stmt, &mut lines);
    }
    lines
}

fn collect_stmt_lines(stmt: &Stmt, lines: &mut HashSet<usize>) {
    let body_of = |body: &Vec<Located>, lines: &mut HashSet<usize>| {
        for l in body {
            lines.insert(l.line);
            collect_stmt_lines(&l.stmt, lines);
        }
    };
    match stmt {
        Stmt::FuncDef { body, .. } | Stmt::GeneratorDef { body, .. } => body_of(body, lines),
        Stmt::While { body, .. } | Stmt::For { body, .. } | Stmt::ForIndexed { body, .. } => {
            body_of(body, lines)
        }
        Stmt::Switch { cases, default, .. } => {
            for (_, body) in cases {
                body_of(body, lines);
            }
            if let Some(body) = default {
                body_of(body, lines);
            }
        }
        Stmt::Expression(Expr::If {
            then_branch,
            elif_branches,
            else_branch,
            ..
        }) => {
            body_of(then_branch, lines);
            for (_, body) in elif_branches {
                body_of(body, lines);
            }
            if let Some(body) = else_branch {
                body_of(body, lines);
            }
        }
        _ => {}
    }
}

impl Expr {
    /// Helper to create a binary expression
    pub fn binary(left: Expr, op: BinOp, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    /// Helper to create a unary expression
    pub fn unary(op: UnaryOp, expr: Expr) -> Self {
        Expr::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    /// Helper to create a function call
    pub fn call(func: Expr, args: Vec<Expr>) -> Self {
        Expr::Call {
            func: Box::new(func),
            args,
        }
    }

    /// Helper to create an index expression
    pub fn index(object: Expr, index: Expr) -> Self {
        Expr::Index {
            object: Box::new(object),
            index: Box::new(index),
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Subtract => write!(f, "-"),
            BinOp::Multiply => write!(f, "*"),
            BinOp::Divide => write!(f, "/"),
            BinOp::Modulo => write!(f, "%"),
            BinOp::BitAnd => write!(f, "&"),
            BinOp::BitOr => write!(f, "|"),
            BinOp::BitXor => write!(f, "^"),
            BinOp::Shl => write!(f, "<<"),
            BinOp::Shr => write!(f, ">>"),
            BinOp::Equal => write!(f, "=="),
            BinOp::NotEqual => write!(f, "!="),
            BinOp::Less => write!(f, "<"),
            BinOp::LessEqual => write!(f, "<="),
            BinOp::Greater => write!(f, ">"),
            BinOp::GreaterEqual => write!(f, ">="),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}
