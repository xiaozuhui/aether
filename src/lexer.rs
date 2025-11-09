// src/lexer.rs
//! Lexer for the Aether language
//!
//! Converts source code into a stream of tokens

use crate::token::Token;

/// Lexer state
pub struct Lexer {
    input: Vec<char>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
    line: usize,          // current line number (for error reporting)
    column: usize,        // current column number (for error reporting)
    had_whitespace_before_token: bool, // whether whitespace was skipped before current token
}

impl Lexer {
    /// Create a new lexer from input string
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
            had_whitespace_before_token: false,
        };
        lexer.read_char(); // Initialize by reading the first character
        lexer
    }

    /// Get current line number
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get current column number
    pub fn column(&self) -> usize {
        self.column
    }

    /// Check if whitespace was skipped before the last token
    pub fn had_whitespace(&self) -> bool {
        self.had_whitespace_before_token
    }

    /// Read the next character and advance position
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // EOF
        } else {
            self.ch = self.input[self.read_position];
        }

        // Update line and column tracking
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Peek at the next character without advancing
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    /// Peek at the character n positions ahead without advancing
    fn peek_char_n(&self, n: usize) -> char {
        let pos = self.position + n;
        if pos >= self.input.len() {
            '\0'
        } else {
            self.input[pos]
        }
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        let had_ws = self.skip_whitespace();
        self.had_whitespace_before_token = had_ws;

        let token = match self.ch {
            // Operators
            '+' => Token::Plus,
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Multiply,
            '/' => {
                // Check for comments
                if self.peek_char() == '/' {
                    self.skip_line_comment();
                    return self.next_token();
                } else if self.peek_char() == '*' {
                    self.skip_block_comment();
                    return self.next_token();
                } else {
                    Token::Divide
                }
            }
            '%' => Token::Modulo,

            // Comparison and logical
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::And
                } else {
                    Token::Illegal('&')
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    Token::Or
                } else {
                    Token::Illegal('|')
                }
            }

            // Delimiters
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,

            // String literals
            '"' => {
                // Check if it's a multiline string (""")
                if self.peek_char() == '"' && self.peek_char_n(2) == '"' {
                    return self.read_multiline_string();
                } else {
                    return self.read_string();
                }
            }

            // Newline (statement separator)
            '\n' => Token::Newline,

            // EOF
            '\0' => Token::EOF,

            // Identifiers, keywords, and numbers
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    return self.read_identifier();
                } else if self.ch.is_numeric() {
                    return self.read_number();
                } else {
                    Token::Illegal(self.ch)
                }
            }
        };

        self.read_char();
        token
    }

    /// Skip whitespace (except newlines, which are significant)
    /// Returns true if any whitespace was skipped
    fn skip_whitespace(&mut self) -> bool {
        let mut skipped = false;
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' {
            skipped = true;
            self.read_char();
        }
        skipped
    }

    /// Skip single-line comment (// ...)
    fn skip_line_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
    }

    /// Skip block comment (/* ... */)
    fn skip_block_comment(&mut self) {
        self.read_char(); // skip '/'
        self.read_char(); // skip '*'

        while !(self.ch == '*' && self.peek_char() == '/') && self.ch != '\0' {
            if self.ch == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.read_char();
        }

        if self.ch != '\0' {
            self.read_char(); // skip '*'
            self.read_char(); // skip '/'
        }
    }

    /// Read an identifier or keyword
    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        // Aether 标识符: 大写字母、数字、下划线
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }

        let ident: String = self.input[start..self.position].iter().collect();
        Token::lookup_keyword(&ident)
    }

    /// Read a number (integer or float)
    fn read_number(&mut self) -> Token {
        let start = self.position;
        let mut has_dot = false;

        while self.ch.is_numeric() || (self.ch == '.' && !has_dot) {
            if self.ch == '.' {
                // Check if next character is a digit
                if !self.peek_char().is_numeric() {
                    break;
                }
                has_dot = true;
            }
            self.read_char();
        }

        let num_str: String = self.input[start..self.position].iter().collect();

        // 如果是整数且位数较多（超过15位，接近f64精度极限），作为大整数处理
        if !has_dot && num_str.len() > 15 {
            return Token::BigInteger(num_str);
        }

        match num_str.parse::<f64>() {
            Ok(num) => Token::Number(num),
            Err(_) => Token::Illegal('0'), // Invalid number
        }
    }

    /// Read a string literal
    fn read_string(&mut self) -> Token {
        self.read_char(); // Skip opening quote
        let start = self.position;

        while self.ch != '"' && self.ch != '\0' {
            // Handle escape sequences
            if self.ch == '\\' {
                self.read_char(); // Skip backslash
                if self.ch != '\0' {
                    self.read_char(); // Skip escaped character
                }
            } else {
                if self.ch == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.read_char();
            }
        }

        if self.ch == '\0' {
            return Token::Illegal('"'); // Unterminated string
        }

        let string: String = self.input[start..self.position].iter().collect();
        self.read_char(); // Skip closing quote

        // Process escape sequences
        Token::String(self.process_escapes(&string))
    }

    /// Read a multiline string literal (""" ... """)
    fn read_multiline_string(&mut self) -> Token {
        // Skip the opening """
        self.read_char(); // Skip first "
        self.read_char(); // Skip second "
        self.read_char(); // Skip third "
        
        let start = self.position;
        
        // Read until we find closing """
        loop {
            if self.ch == '\0' {
                return Token::Illegal('"'); // Unterminated multiline string
            }
            
            // Check if we found closing """
            if self.ch == '"' && self.peek_char() == '"' && self.peek_char_n(2) == '"' {
                let string: String = self.input[start..self.position].iter().collect();
                
                // Skip the closing """
                self.read_char(); // Skip first "
                self.read_char(); // Skip second "
                self.read_char(); // Skip third "
                
                // Process escape sequences
                return Token::String(self.process_escapes(&string));
            }
            
            // Handle newlines for line tracking
            if self.ch == '\n' {
                self.line += 1;
                self.column = 0;
            }
            
            self.read_char();
        }
    }

    /// Process escape sequences in strings
    fn process_escapes(&self, s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some(c) => {
                        result.push('\\');
                        result.push(c);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
