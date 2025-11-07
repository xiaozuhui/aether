// src/ast.rs
//! Abstract Syntax Tree (AST) definitions for Aether
//!
//! This module defines the structure of Aether programs as a tree of nodes.

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
    BigInteger(String), // 大整数字面量
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
        then_branch: Vec<Stmt>,
        elif_branches: Vec<(Expr, Vec<Stmt>)>, // (condition, body) pairs
        else_branch: Option<Vec<Stmt>>,
    },

    // Anonymous function
    Lambda {
        params: Vec<String>,
        body: Vec<Stmt>,
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

    // Function definition: Func NAME (params) { body }
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },

    // Generator definition: Generator NAME (params) { body }
    GeneratorDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
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

    // While loop: While (condition) { body }
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },

    // For loop: For VAR In ITERABLE { body }
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },

    // For loop with index: For INDEX, VAR In ITERABLE { body }
    ForIndexed {
        index_var: String,
        value_var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },

    // Switch statement: Switch (expr) { Case val: body ... Default: body }
    Switch {
        expr: Expr,
        cases: Vec<(Expr, Vec<Stmt>)>,
        default: Option<Vec<Stmt>>,
    },

    // Import statement: Import NAME From PATH
    Import {
        names: Vec<String>,
        path: String,
        aliases: Vec<Option<String>>, // Optional aliases (as NAME)
    },

    // Export statement: Export NAME
    Export(String),

    // Throw statement: Throw message
    Throw(Expr),

    // Expression statement (expression as statement)
    Expression(Expr),
}

/// A complete program is a list of statements
pub type Program = Vec<Stmt>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_helpers() {
        // Binary expression: 1 + 2
        let expr = Expr::binary(Expr::Number(1.0), BinOp::Add, Expr::Number(2.0));
        match expr {
            Expr::Binary { left, op, right } => {
                assert_eq!(*left, Expr::Number(1.0));
                assert_eq!(op, BinOp::Add);
                assert_eq!(*right, Expr::Number(2.0));
            }
            _ => panic!("Expected Binary expression"),
        }

        // Unary expression: -5
        let expr = Expr::unary(UnaryOp::Minus, Expr::Number(5.0));
        match expr {
            Expr::Unary { op, expr } => {
                assert_eq!(op, UnaryOp::Minus);
                assert_eq!(*expr, Expr::Number(5.0));
            }
            _ => panic!("Expected Unary expression"),
        }

        // Function call: ADD(1, 2)
        let expr = Expr::call(
            Expr::Identifier("ADD".to_string()),
            vec![Expr::Number(1.0), Expr::Number(2.0)],
        );
        match expr {
            Expr::Call { func, args } => {
                assert_eq!(*func, Expr::Identifier("ADD".to_string()));
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected Call expression"),
        }
    }

    #[test]
    fn test_binop_display() {
        assert_eq!(format!("{}", BinOp::Add), "+");
        assert_eq!(format!("{}", BinOp::Equal), "==");
        assert_eq!(format!("{}", BinOp::And), "&&");
    }

    #[test]
    fn test_unaryop_display() {
        assert_eq!(format!("{}", UnaryOp::Minus), "-");
        assert_eq!(format!("{}", UnaryOp::Not), "!");
    }
}
