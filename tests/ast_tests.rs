use aether::{
    Expr,
    ast::{BinOp, UnaryOp},
};

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
