use aether::{EvalResult, Evaluator, Parser, Value};

// 帮助函数
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
