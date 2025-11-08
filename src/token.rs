// src/token.rs
//! Token definitions for the Aether lexer

/// Position information for tokens
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithPos {
    pub token: Token,
    pub pos: Position,
}

impl TokenWithPos {
    pub fn new(token: Token, line: usize, column: usize) -> Self {
        TokenWithPos {
            token,
            pos: Position::new(line, column),
        }
    }
}

/// Represents all possible tokens in the Aether language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords - 首字母大写
    Set,
    Func,
    Generator,
    Lazy,
    If,
    Elif,
    Else,
    While,
    For,
    In,
    Switch,
    Case,
    Default,
    Return,
    Yield,
    Break,
    Continue,
    Import,
    From,
    As,
    Export,
    Throw,

    // Identifiers and literals - 全大写标识符
    Identifier(String),
    Number(f64),
    BigInteger(String), // 大整数字面量，保留原始字符串
    String(String),
    Boolean(bool),
    Null,

    // Operators
    Plus,     // +
    Minus,    // -
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
    Not, // !

    // Assignment
    Assign, // = (但 Aether 中用 Set，这里可能不需要)

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    Newline,      // \n (语句分隔符)

    // Special
    Arrow, // ->
    Illegal(char),
    EOF,
}

impl Token {
    /// Check if a string is a keyword
    pub fn lookup_keyword(ident: &str) -> Token {
        match ident {
            // Keywords
            "Set" => Token::Set,
            "Func" => Token::Func,
            "Generator" => Token::Generator,
            "Lazy" => Token::Lazy,
            "If" => Token::If,
            "Elif" => Token::Elif,
            "Else" => Token::Else,
            "While" => Token::While,
            "For" => Token::For,
            "In" => Token::In,
            "Switch" => Token::Switch,
            "Case" => Token::Case,
            "Default" => Token::Default,
            "Return" => Token::Return,
            "Yield" => Token::Yield,
            "Break" => Token::Break,
            "Continue" => Token::Continue,
            "Import" => Token::Import,
            "From" => Token::From,
            "as" => Token::As,
            "Export" => Token::Export,
            "Throw" => Token::Throw,

            // Logical operators as keywords
            "And" => Token::And,
            "Or" => Token::Or,
            "Not" => Token::Not,

            // Boolean literals (uppercase per Aether design)
            "True" => Token::Boolean(true),
            "False" => Token::Boolean(false),

            // Null
            "Null" => Token::Null,

            // Not a keyword, return identifier
            _ => Token::Identifier(ident.to_string()),
        }
    }

    /// Get a human-readable representation of the token
    pub fn token_type(&self) -> &str {
        match self {
            Token::Set => "Set",
            Token::Func => "Func",
            Token::Generator => "Generator",
            Token::Lazy => "Lazy",
            Token::If => "If",
            Token::Elif => "Elif",
            Token::Else => "Else",
            Token::While => "While",
            Token::For => "For",
            Token::In => "In",
            Token::Switch => "Switch",
            Token::Case => "Case",
            Token::Default => "Default",
            Token::Return => "Return",
            Token::Yield => "Yield",
            Token::Break => "Break",
            Token::Continue => "Continue",
            Token::Import => "Import",
            Token::From => "From",
            Token::As => "as",
            Token::Export => "Export",
            Token::Throw => "Throw",
            Token::Identifier(_) => "Identifier",
            Token::Number(_) => "Number",
            Token::BigInteger(_) => "BigInteger",
            Token::String(_) => "String",
            Token::Boolean(_) => "Boolean",
            Token::Null => "nil",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Multiply => "*",
            Token::Divide => "/",
            Token::Modulo => "%",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::Less => "<",
            Token::LessEqual => "<=",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            Token::And => "&&",
            Token::Or => "||",
            Token::Not => "!",
            Token::Assign => "=",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::LeftBracket => "[",
            Token::RightBracket => "]",
            Token::Comma => ",",
            Token::Colon => ":",
            Token::Semicolon => ";",
            Token::Newline => "\\n",
            Token::Arrow => "->",
            Token::Illegal(_) => "Illegal",
            Token::EOF => "EOF",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
