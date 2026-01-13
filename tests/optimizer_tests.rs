use aether::{Expr, Optimizer, Program, Stmt, ast::BinOp};

#[test]
fn test_optimize_program() {
    let optimizer = Optimizer::new();

    let program: Program = vec![Stmt::Set {
        name: "x".to_string(),
        value: Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinOp::Add,
            right: Box::new(Expr::Number(3.0)),
        },
    }];

    let optimized = optimizer.optimize_program(&program);

    // 验证常量折叠
    if let Some(Stmt::Set { value, .. }) = optimized.first() {
        assert_eq!(*value, Expr::Number(5.0));
    }
}
