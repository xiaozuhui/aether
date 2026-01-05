#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub span: Span,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assign {
        span: Span,
        target: Expr,
        value: Expr,
    },
    ExprStmt {
        span: Span,
        value: Expr,
    },
    Return {
        span: Span,
        value: Option<Expr>,
    },
    If {
        span: Span,
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },
    While {
        span: Span,
        test: Expr,
        body: Vec<Stmt>,
    },
    For {
        span: Span,
        target: Expr,
        iter: Expr,
        body: Vec<Stmt>,
    },
    FunctionDef {
        span: Span,
        name: String,
        args: Vec<String>,
        body: Vec<Stmt>,
    },
    Import {
        span: Span,
        module: String,
        asname: Option<String>,
    },
    ImportFrom {
        span: Span,
        module: String,
        name: String,
        asname: Option<String>,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Pass {
        span: Span,
    },
    Unsupported {
        span: Span,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Name {
        span: Span,
        id: String,
    },
    Number {
        span: Span,
        value: f64,
    },
    String {
        span: Span,
        value: String,
    },
    Bool {
        span: Span,
        value: bool,
    },
    None {
        span: Span,
    },
    List {
        span: Span,
        elts: Vec<Expr>,
    },
    Dict {
        span: Span,
        items: Vec<(Expr, Expr)>,
    },

    BinOp {
        span: Span,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        span: Span,
        op: String,
        operand: Box<Expr>,
    },
    BoolOp {
        span: Span,
        op: String,
        values: Vec<Expr>,
    },
    Compare {
        span: Span,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    Attribute {
        span: Span,
        value: Box<Expr>,
        attr: String,
    },
    Call {
        span: Span,
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Subscript {
        span: Span,
        value: Box<Expr>,
        index: Box<Expr>,
    },

    Unsupported {
        span: Span,
        reason: String,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Name { span, .. }
            | Expr::Number { span, .. }
            | Expr::String { span, .. }
            | Expr::Bool { span, .. }
            | Expr::None { span }
            | Expr::List { span, .. }
            | Expr::Dict { span, .. }
            | Expr::BinOp { span, .. }
            | Expr::UnaryOp { span, .. }
            | Expr::BoolOp { span, .. }
            | Expr::Compare { span, .. }
            | Expr::Attribute { span, .. }
            | Expr::Call { span, .. }
            | Expr::Subscript { span, .. }
            | Expr::Unsupported { span, .. } => *span,
        }
    }
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Assign { span, .. }
            | Stmt::ExprStmt { span, .. }
            | Stmt::Return { span, .. }
            | Stmt::If { span, .. }
            | Stmt::While { span, .. }
            | Stmt::For { span, .. }
            | Stmt::FunctionDef { span, .. }
            | Stmt::Import { span, .. }
            | Stmt::ImportFrom { span, .. }
            | Stmt::Break { span }
            | Stmt::Continue { span }
            | Stmt::Pass { span }
            | Stmt::Unsupported { span, .. } => *span,
        }
    }
}
