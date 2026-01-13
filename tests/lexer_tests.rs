use aether::{Lexer, Token};

#[test]
fn test_basic_tokens() {
    let input = "Set X 10";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Set);
    assert_eq!(lexer.next_token(), Token::Identifier("X".to_string()));
    assert_eq!(lexer.next_token(), Token::Number(10.0));
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_operators() {
    let input = "+ - * / % == != < <= > >= && || !";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Plus);
    assert_eq!(lexer.next_token(), Token::Minus);
    assert_eq!(lexer.next_token(), Token::Multiply);
    assert_eq!(lexer.next_token(), Token::Divide);
    assert_eq!(lexer.next_token(), Token::Modulo);
    assert_eq!(lexer.next_token(), Token::Equal);
    assert_eq!(lexer.next_token(), Token::NotEqual);
    assert_eq!(lexer.next_token(), Token::Less);
    assert_eq!(lexer.next_token(), Token::LessEqual);
    assert_eq!(lexer.next_token(), Token::Greater);
    assert_eq!(lexer.next_token(), Token::GreaterEqual);
    assert_eq!(lexer.next_token(), Token::And);
    assert_eq!(lexer.next_token(), Token::Or);
    assert_eq!(lexer.next_token(), Token::Not);
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_string_literal() {
    let input = r#"Set MSG "Hello World""#;
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Set);
    assert_eq!(lexer.next_token(), Token::Identifier("MSG".to_string()));
    assert_eq!(lexer.next_token(), Token::String("Hello World".to_string()));
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_string_with_escapes() {
    let input = r#""Hello\nWorld\t!""#;
    let mut lexer = Lexer::new(input);

    assert_eq!(
        lexer.next_token(),
        Token::String("Hello\nWorld\t!".to_string())
    );
}

#[test]
fn test_numbers() {
    let input = "123 45.67 0.5";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Number(123.0));
    assert_eq!(lexer.next_token(), Token::Number(45.67));
    assert_eq!(lexer.next_token(), Token::Number(0.5));
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_keywords() {
    let input = "Set Func If Else While For Return True False Null";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Set);
    assert_eq!(lexer.next_token(), Token::Func);
    assert_eq!(lexer.next_token(), Token::If);
    assert_eq!(lexer.next_token(), Token::Else);
    assert_eq!(lexer.next_token(), Token::While);
    assert_eq!(lexer.next_token(), Token::For);
    assert_eq!(lexer.next_token(), Token::Return);
    assert_eq!(lexer.next_token(), Token::Boolean(true));
    assert_eq!(lexer.next_token(), Token::Boolean(false));
    assert_eq!(lexer.next_token(), Token::Null);
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_identifiers() {
    let input = "USER_NAME CALCULATE_TOTAL MY_VAR";
    let mut lexer = Lexer::new(input);

    assert_eq!(
        lexer.next_token(),
        Token::Identifier("USER_NAME".to_string())
    );
    assert_eq!(
        lexer.next_token(),
        Token::Identifier("CALCULATE_TOTAL".to_string())
    );
    assert_eq!(lexer.next_token(), Token::Identifier("MY_VAR".to_string()));
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_delimiters() {
    let input = "( ) { } [ ] , : ;";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::LeftParen);
    assert_eq!(lexer.next_token(), Token::RightParen);
    assert_eq!(lexer.next_token(), Token::LeftBrace);
    assert_eq!(lexer.next_token(), Token::RightBrace);
    assert_eq!(lexer.next_token(), Token::LeftBracket);
    assert_eq!(lexer.next_token(), Token::RightBracket);
    assert_eq!(lexer.next_token(), Token::Comma);
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::Semicolon);
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_line_comment() {
    let input = "Set X 10 // This is a comment\nSet Y 20";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Set);
    assert_eq!(lexer.next_token(), Token::Identifier("X".to_string()));
    assert_eq!(lexer.next_token(), Token::Number(10.0));
    assert_eq!(lexer.next_token(), Token::Newline);
    assert_eq!(lexer.next_token(), Token::Set);
    assert_eq!(lexer.next_token(), Token::Identifier("Y".to_string()));
    assert_eq!(lexer.next_token(), Token::Number(20.0));
}

#[test]
fn test_block_comment() {
    let input = "Set X /* block comment */ 10";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Set);
    assert_eq!(lexer.next_token(), Token::Identifier("X".to_string()));
    assert_eq!(lexer.next_token(), Token::Number(10.0));
}

#[test]
fn test_newlines() {
    let input = "Set X 10\nSet Y 20";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Set);
    assert_eq!(lexer.next_token(), Token::Identifier("X".to_string()));
    assert_eq!(lexer.next_token(), Token::Number(10.0));
    assert_eq!(lexer.next_token(), Token::Newline);
    assert_eq!(lexer.line(), 2);
    assert_eq!(lexer.next_token(), Token::Set);
}

#[test]
fn test_complex_expression() {
    let input = r#"
            Func ADD (A, B) {
                Return (A + B)
            }
        "#;
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Newline);
    assert_eq!(lexer.next_token(), Token::Func);
    assert_eq!(lexer.next_token(), Token::Identifier("ADD".to_string()));
    assert_eq!(lexer.next_token(), Token::LeftParen);
    assert_eq!(lexer.next_token(), Token::Identifier("A".to_string()));
    assert_eq!(lexer.next_token(), Token::Comma);
    assert_eq!(lexer.next_token(), Token::Identifier("B".to_string()));
    assert_eq!(lexer.next_token(), Token::RightParen);
    assert_eq!(lexer.next_token(), Token::LeftBrace);
    assert_eq!(lexer.next_token(), Token::Newline);
    assert_eq!(lexer.next_token(), Token::Return);
    assert_eq!(lexer.next_token(), Token::LeftParen);
    assert_eq!(lexer.next_token(), Token::Identifier("A".to_string()));
    assert_eq!(lexer.next_token(), Token::Plus);
    assert_eq!(lexer.next_token(), Token::Identifier("B".to_string()));
    assert_eq!(lexer.next_token(), Token::RightParen);
    assert_eq!(lexer.next_token(), Token::Newline);
    assert_eq!(lexer.next_token(), Token::RightBrace);
}
