use aether::Token;

#[test]
fn test_keyword_lookup() {
    assert_eq!(Token::lookup_keyword("Set"), Token::Set);
    assert_eq!(Token::lookup_keyword("Func"), Token::Func);
    assert_eq!(Token::lookup_keyword("If"), Token::If);
    assert_eq!(Token::lookup_keyword("True"), Token::Boolean(true));
    assert_eq!(Token::lookup_keyword("False"), Token::Boolean(false));
    assert_eq!(Token::lookup_keyword("Null"), Token::Null);

    // Non-keyword should return identifier
    match Token::lookup_keyword("MY_VAR") {
        Token::Identifier(s) => assert_eq!(s, "MY_VAR"),
        _ => panic!("Expected identifier"),
    }
}

#[test]
fn test_token_type() {
    assert_eq!(Token::Set.token_type(), "Set");
    assert_eq!(Token::Plus.token_type(), "+");
    assert_eq!(Token::Equal.token_type(), "==");
    assert_eq!(Token::LeftParen.token_type(), "(");
    assert_eq!(Token::EOF.token_type(), "EOF");
}
