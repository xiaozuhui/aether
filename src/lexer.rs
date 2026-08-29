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
    /// 整数字面量超过该位数后切换为 BigInteger（默认 15，接近 f64 精度极限）
    bigint_threshold: usize,
}

/// 默认的大整数切换阈值（f64 安全整数位数为 15-16 位）
pub const DEFAULT_BIGINT_THRESHOLD: usize = 15;

impl Lexer {
    /// Create a new lexer from input string
    pub fn new(input: &str) -> Self {
        Self::with_bigint_threshold(input, DEFAULT_BIGINT_THRESHOLD)
    }

    /// Create a new lexer with a custom big-integer threshold
    pub fn with_bigint_threshold(input: &str, bigint_threshold: usize) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
            had_whitespace_before_token: false,
            bigint_threshold,
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
                    // Aether 用 Set 赋值，孤立 '=' 不是合法 token
                    Token::Illegal('=')
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
                } else if self.peek_char() == '<' {
                    self.read_char();
                    Token::Shl
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GreaterEqual
                } else if self.peek_char() == '>' {
                    self.read_char();
                    Token::Shr
                } else {
                    Token::Greater
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::And
                } else {
                    Token::BitAnd
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    Token::Or
                } else {
                    Token::BitOr
                }
            }
            '^' => Token::BitXor,

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

    /// Read a number (integer, float, or scientific notation)
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

        // 科学计数法：e/E 后跟数字，或 +/- 后跟数字（如 1e30、1.5E-3）
        if (self.ch == 'e' || self.ch == 'E')
            && (self.peek_char().is_numeric()
                || ((self.peek_char() == '+' || self.peek_char() == '-')
                    && self.peek_char_n(2).is_numeric()))
        {
            self.read_char(); // skip e/E
            if self.ch == '+' || self.ch == '-' {
                self.read_char(); // skip sign
            }
            while self.ch.is_numeric() {
                self.read_char();
            }

            let num_str: String = self.input[start..self.position].iter().collect();
            return self.tokenize_scientific(&num_str);
        }

        let num_str: String = self.input[start..self.position].iter().collect();

        // 如果是整数且位数较多（超过阈值，接近f64精度极限），作为大整数处理
        if !has_dot && num_str.len() > self.bigint_threshold {
            return Token::BigInteger(num_str);
        }

        match num_str.parse::<f64>() {
            Ok(num) => Token::Number(num),
            Err(_) => Token::Illegal('0'), // Invalid number
        }
    }

    /// 将科学计数法字面量转换为 token。
    ///
    /// 语义（0.6.0 冻结）：科学计数法一律是 f64（`1e15`、`1e-7` 均为
    /// Number，与「小数保持 f64」一致）；需要精确大整数请书写完整数字
    /// （超阈值自动 BigInteger）或显式 `TO_FRACTION`。
    fn tokenize_scientific(&self, num_str: &str) -> Token {
        match num_str.parse::<f64>() {
            Ok(num) => Token::Number(num),
            Err(_) => Token::Illegal('0'),
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
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('u') => {
                        // Handle \uXXXX Unicode escape sequences
                        let mut hex = String::new();
                        for _ in 0..4 {
                            if let Some(c) = chars.next() {
                                hex.push(c);
                            } else {
                                // Invalid escape sequence, keep as is
                                result.push_str("\\u");
                                result.push_str(&hex);
                                break;
                            }
                        }
                        if hex.len() == 4 {
                            if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                if let Some(unicode_char) = char::from_u32(code) {
                                    result.push(unicode_char);
                                } else {
                                    // Invalid Unicode code point, keep as is
                                    result.push_str("\\u");
                                    result.push_str(&hex);
                                }
                            } else {
                                // Invalid hex, keep as is
                                result.push_str("\\u");
                                result.push_str(&hex);
                            }
                        }
                    }
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
