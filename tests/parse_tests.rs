use aether::{Expr, Parser, Stmt, ast::BinOp};

#[test]
fn test_parse_set_statement() {
    let input = "Set X 10";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Set { name, value } => {
            assert_eq!(name, "X");
            assert_eq!(*value, Expr::Number(10.0));
        }
        _ => panic!("Expected Set statement"),
    }
}

#[test]
fn test_parse_arithmetic() {
    let input = "Set X (5 + 3 * 2)";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Set { name, value } => {
            assert_eq!(name, "X");
            // Should be: 5 + (3 * 2) due to precedence
            match value {
                Expr::Binary { left, op, right } => {
                    assert_eq!(**left, Expr::Number(5.0));
                    assert_eq!(*op, BinOp::Add);
                    match &**right {
                        Expr::Binary { left, op, right } => {
                            assert_eq!(**left, Expr::Number(3.0));
                            assert_eq!(*op, BinOp::Multiply);
                            assert_eq!(**right, Expr::Number(2.0));
                        }
                        _ => panic!("Expected binary expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected Set statement"),
    }
}

#[test]
fn test_parse_function_definition() {
    let input = r#"
            Func ADD (A, B) {
                Return (A + B)
            }
        "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::FuncDef { name, params, body } => {
            assert_eq!(name, "ADD");
            assert_eq!(params, &vec!["A".to_string(), "B".to_string()]);
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected FuncDef"),
    }
}

#[test]
fn test_parse_function_call() {
    let input = "ADD(5, 3)";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Expression(Expr::Call { func, args }) => {
            assert_eq!(**func, Expr::Identifier("ADD".to_string()));
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], Expr::Number(5.0));
            assert_eq!(args[1], Expr::Number(3.0));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_parse_array_literal() {
    let input = "Set ARR [1, 2, 3]";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Set { name, value } => {
            assert_eq!(name, "ARR");
            match value {
                Expr::Array(elements) => {
                    assert_eq!(elements.len(), 3);
                    assert_eq!(elements[0], Expr::Number(1.0));
                    assert_eq!(elements[1], Expr::Number(2.0));
                    assert_eq!(elements[2], Expr::Number(3.0));
                }
                _ => panic!("Expected array"),
            }
        }
        _ => panic!("Expected Set statement"),
    }
}

#[test]
fn test_parse_if_expression() {
    let input = r#"
            If (X > 0) {
                Set Y 1
            } Else {
                Set Y 0
            }
        "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Expression(Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        }) => {
            assert!(matches!(**condition, Expr::Binary { .. }));
            assert_eq!(then_branch.len(), 1);
            assert!(else_branch.is_some());
        }
        _ => panic!("Expected If expression"),
    }
}

#[test]
fn test_parse_for_loop() {
    let input = r#"
            For I In RANGE(0, 10) {
                PRINT(I)
            }
        "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    // Debug: print what we got
    eprintln!("Program length: {}", program.len());
    for (i, stmt) in program.iter().enumerate() {
        eprintln!("Statement {}: {:?}", i, stmt);
    }

    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::For {
            var,
            iterable,
            body,
        } => {
            assert_eq!(var, "I");
            assert!(matches!(iterable, Expr::Call { .. }));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected For statement"),
    }
}
