// src/parser.rs
//! Parser for the Aether language
//!
//! Converts a stream of tokens into an Abstract Syntax Tree (AST)

use crate::ast::{BinOp, Expr, Program, Stmt, UnaryOp};
use crate::lexer::Lexer;
use crate::token::Token;

/// Parse errors with location information
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: Token,
        line: usize,
        column: usize,
    },
    UnexpectedEOF {
        line: usize,
        column: usize,
    },
    InvalidNumber(String),
    InvalidExpression {
        message: String,
        line: usize,
        column: usize,
    },
    InvalidStatement {
        message: String,
        line: usize,
        column: usize,
    },
    InvalidIdentifier {
        name: String,
        reason: String,
        line: usize,
        column: usize,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                line,
                column,
            } => {
                write!(
                    f,
                    "Parse error at line {}, column {}: Expected {}, found {:?}",
                    line, column, expected, found
                )
            }
            ParseError::UnexpectedEOF { line, column } => {
                write!(
                    f,
                    "Parse error at line {}, column {}: Unexpected end of file",
                    line, column
                )
            }
            ParseError::InvalidNumber(s) => write!(f, "Parse error: Invalid number: {}", s),
            ParseError::InvalidExpression {
                message,
                line,
                column,
            } => {
                write!(
                    f,
                    "Parse error at line {}, column {}: Invalid expression - {}",
                    line, column, message
                )
            }
            ParseError::InvalidStatement {
                message,
                line,
                column,
            } => {
                write!(
                    f,
                    "Parse error at line {}, column {}: Invalid statement - {}",
                    line, column, message
                )
            }
            ParseError::InvalidIdentifier {
                name,
                reason,
                line,
                column,
            } => {
                write!(
                    f,
                    "Parse error at line {}, column {}: Invalid identifier '{}' - {}",
                    line, column, name, reason
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Operator precedence (higher number = higher precedence)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 0,
    Or = 1,         // ||
    And = 2,        // &&
    Equals = 3,     // ==, !=
    Comparison = 4, // <, <=, >, >=
    Sum = 5,        // +, -
    Product = 6,    // *, /, %
    Prefix = 7,     // -, !
    Call = 8,       // func()
    Index = 9,      // array[index]
}

/// Parser state
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    current_line: usize,
    current_column: usize,
    current_had_whitespace: bool, // whether whitespace preceded current_token
    peek_had_whitespace: bool,    // whether whitespace preceded peek_token
}

impl Parser {
    /// Create a new parser from source code
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        let current_ws = lexer.had_whitespace();
        let peek = lexer.next_token();
        let peek_ws = lexer.had_whitespace();
        let line = lexer.line();
        let column = lexer.column();

        Parser {
            lexer,
            current_token: current,
            peek_token: peek,
            current_line: line,
            current_column: column,
            current_had_whitespace: current_ws,
            peek_had_whitespace: peek_ws,
        }
    }

    /// Advance to the next token
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.current_had_whitespace = self.peek_had_whitespace;
        self.peek_token = self.lexer.next_token();
        self.peek_had_whitespace = self.lexer.had_whitespace();
        self.current_line = self.lexer.line();
        self.current_column = self.lexer.column();
    }

    /// Skip newline tokens (they're optional in many places)
    fn skip_newlines(&mut self) {
        while self.current_token == Token::Newline {
            self.next_token();
        }
    }

    /// Check if current token matches expected, advance if true
    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.next_token();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: self.current_token.clone(),
                line: self.current_line,
                column: self.current_column,
            })
        }
    }

    /// Helper to check if identifier follows naming convention (UPPER_SNAKE_CASE)
    fn validate_identifier(&self, name: &str) -> Result<(), ParseError> {
        self.validate_identifier_internal(name, false)
    }

    /// Helper to check if identifier follows naming convention
    /// For function parameters, we allow more flexible naming (can use lowercase)
    fn validate_identifier_internal(&self, name: &str, is_param: bool) -> Result<(), ParseError> {
        // Check it doesn't start with a number
        if name.chars().next().is_some_and(|c| c.is_numeric()) {
            return Err(ParseError::InvalidIdentifier {
                name: name.to_string(),
                reason: "标识符不能以数字开头".to_string(),
                line: self.current_line,
                column: self.current_column,
            });
        }

        // For function parameters, allow lowercase letters
        if is_param {
            let is_valid = name
                .chars()
                .all(|c| c.is_alphabetic() || c.is_numeric() || c == '_');

            if !is_valid {
                return Err(ParseError::InvalidIdentifier {
                    name: name.to_string(),
                    reason: "参数名只能包含字母、数字和下划线".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        } else {
            // For variables and function names, require uppercase
            let is_valid = name
                .chars()
                .all(|c| c.is_uppercase() || c.is_numeric() || c == '_');

            if !is_valid {
                return Err(ParseError::InvalidIdentifier {
                    name: name.to_string(),
                    reason:
                        "变量名和函数名必须使用全大写字母和下划线（例如：MY_VAR, CALCULATE_SUM）"
                            .to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        }

        Ok(())
    }

    /// Get precedence of current token
    fn current_precedence(&self) -> Precedence {
        self.token_precedence(&self.current_token)
    }

    /// Get precedence of peek token
    #[allow(dead_code)]
    fn peek_precedence(&self) -> Precedence {
        self.token_precedence(&self.peek_token)
    }

    /// Get precedence of a token
    fn token_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Or => Precedence::Or,
            Token::And => Precedence::And,
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => {
                Precedence::Comparison
            }
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Multiply | Token::Divide | Token::Modulo => Precedence::Product,
            Token::LeftParen => Precedence::Call,
            Token::LeftBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }

    /// Parse a complete program
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        self.skip_newlines();

        while self.current_token != Token::EOF {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.skip_newlines();
        }

        Ok(statements)
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match &self.current_token {
            Token::Set => self.parse_set_statement(),
            Token::Func => self.parse_func_definition(),
            Token::Generator => self.parse_generator_definition(),
            Token::Lazy => self.parse_lazy_definition(),
            Token::Return => self.parse_return_statement(),
            Token::Yield => self.parse_yield_statement(),
            Token::Break => self.parse_break_statement(),
            Token::Continue => self.parse_continue_statement(),
            Token::While => self.parse_while_statement(),
            Token::For => self.parse_for_statement(),
            Token::Switch => self.parse_switch_statement(),
            Token::Import => self.parse_import_statement(),
            Token::Export => self.parse_export_statement(),
            Token::Throw => self.parse_throw_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Parse: Set NAME value
    fn parse_set_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Set'

        // Parse the left-hand side (target)
        // This can be either an identifier or an index expression
        // We manually parse this to avoid consuming array literals as part of the target

        let name = match &self.current_token {
            Token::Identifier(n) => {
                self.validate_identifier(n)?;
                n.clone()
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        self.next_token(); // move past identifier

        // Check if followed by '[' for index access
        // CRITICAL: Distinguish between:
        // 1. Set NAME[index] value  -> index assignment (NO space before '[')
        // 2. Set NAME [array]       -> array literal assignment (space before '[')
        //
        // We check if there was whitespace before the '[' token
        if self.current_token == Token::LeftBracket {
            // IMPORTANT: Check whitespace BEFORE calling next_token()
            // because had_whitespace() reflects the whitespace before current_token
            let has_space_before_bracket = self.current_had_whitespace;

            if has_space_before_bracket {
                // Space before '[' means this is: Set NAME [array_literal]
                // Parse the whole thing as a value expression
                let value = self.parse_expression(Precedence::Lowest)?;
                if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
                    self.next_token();
                }
                return Ok(Stmt::Set { name, value });
            }

            // No space before '[' means this is: Set NAME[index] value
            // This is an index assignment
            self.next_token(); // skip '['            // Parse the index expression
            let index = self.parse_expression(Precedence::Lowest)?;

            // Expect ']'
            if self.current_token != Token::RightBracket {
                return Err(ParseError::UnexpectedToken {
                    expected: "']' for index access".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }

            self.next_token(); // skip ']'

            // Now parse the value to assign
            let value = self.parse_expression(Precedence::Lowest)?;

            if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
                self.next_token();
            }

            return Ok(Stmt::SetIndex {
                object: Box::new(Expr::Identifier(name)),
                index: Box::new(index),
                value,
            });
        }

        // Regular Set statement: Set NAME value
        let value = self.parse_expression(Precedence::Lowest)?;

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Set { name, value })
    }

    /// Parse: Func NAME (params) { body }
    fn parse_func_definition(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Func'

        let name = match &self.current_token {
            Token::Identifier(name) => {
                // Validate function name
                self.validate_identifier(name)?;
                name.clone()
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        self.next_token(); // move to '('
        self.expect_token(Token::LeftParen)?;

        let params = self.parse_parameter_list()?;

        self.expect_token(Token::RightParen)?;
        self.skip_newlines();
        self.expect_token(Token::LeftBrace)?;

        let body = self.parse_block()?;

        self.expect_token(Token::RightBrace)?;

        Ok(Stmt::FuncDef { name, params, body })
    }

    /// Parse: Generator NAME (params) { body }
    fn parse_generator_definition(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Generator'

        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        self.next_token();
        self.expect_token(Token::LeftParen)?;

        let params = self.parse_parameter_list()?;

        self.expect_token(Token::RightParen)?;
        self.skip_newlines();
        self.expect_token(Token::LeftBrace)?;

        let body = self.parse_block()?;

        self.expect_token(Token::RightBrace)?;

        Ok(Stmt::GeneratorDef { name, params, body })
    }

    /// Parse: Lazy NAME (expr)
    fn parse_lazy_definition(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Lazy'

        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        self.next_token();
        self.expect_token(Token::LeftParen)?;

        let expr = self.parse_expression(Precedence::Lowest)?;

        self.expect_token(Token::RightParen)?;

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::LazyDef { name, expr })
    }

    /// Parse: Return expr
    fn parse_return_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Return'

        let expr =
            if self.current_token == Token::Newline || self.current_token == Token::RightBrace {
                Expr::Null
            } else {
                self.parse_expression(Precedence::Lowest)?
            };

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Return(expr))
    }

    /// Parse: Yield expr
    fn parse_yield_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Yield'

        let expr =
            if self.current_token == Token::Newline || self.current_token == Token::RightBrace {
                Expr::Null
            } else {
                self.parse_expression(Precedence::Lowest)?
            };

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Yield(expr))
    }

    /// Parse: Break
    fn parse_break_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Break'

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Break)
    }

    /// Parse: Continue
    fn parse_continue_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Continue'

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Continue)
    }

    /// Parse: While (condition) { body }
    fn parse_while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'While'
        self.expect_token(Token::LeftParen)?;

        let condition = self.parse_expression(Precedence::Lowest)?;

        self.expect_token(Token::RightParen)?;
        self.skip_newlines();
        self.expect_token(Token::LeftBrace)?;

        let body = self.parse_block()?;

        self.expect_token(Token::RightBrace)?;

        Ok(Stmt::While { condition, body })
    }

    /// Parse: For VAR In ITERABLE { body }
    fn parse_for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'For'

        let first_var = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        self.next_token();

        // Check for indexed for loop: For INDEX, VALUE In ...
        if self.current_token == Token::Comma {
            self.next_token(); // skip comma

            let second_var = match &self.current_token {
                Token::Identifier(name) => name.clone(),
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "identifier".to_string(),
                        found: self.current_token.clone(),
                        line: self.current_line,
                        column: self.current_column,
                    });
                }
            };

            self.next_token();
            self.expect_token(Token::In)?;

            let iterable = self.parse_expression(Precedence::Lowest)?;

            self.skip_newlines();
            self.expect_token(Token::LeftBrace)?;

            let body = self.parse_block()?;

            self.expect_token(Token::RightBrace)?;

            return Ok(Stmt::ForIndexed {
                index_var: first_var,
                value_var: second_var,
                iterable,
                body,
            });
        }

        // Simple for loop: For VAR In ...
        self.expect_token(Token::In)?;

        let iterable = self.parse_expression(Precedence::Lowest)?;

        self.skip_newlines();
        self.expect_token(Token::LeftBrace)?;

        let body = self.parse_block()?;

        self.expect_token(Token::RightBrace)?;

        Ok(Stmt::For {
            var: first_var,
            iterable,
            body,
        })
    }

    /// Parse: Switch (expr) { Case val: ... Default: ... }
    fn parse_switch_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Switch'
        self.expect_token(Token::LeftParen)?;

        let expr = self.parse_expression(Precedence::Lowest)?;

        self.expect_token(Token::RightParen)?;
        self.skip_newlines();
        self.expect_token(Token::LeftBrace)?;
        self.skip_newlines();

        let mut cases = Vec::new();
        let mut default = None;

        while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
            if self.current_token == Token::Case {
                self.next_token();
                let case_expr = self.parse_expression(Precedence::Lowest)?;
                self.expect_token(Token::Colon)?;
                self.skip_newlines();

                let mut case_body = Vec::new();
                while self.current_token != Token::Case
                    && self.current_token != Token::Default
                    && self.current_token != Token::RightBrace
                    && self.current_token != Token::EOF
                {
                    case_body.push(self.parse_statement()?);
                    self.skip_newlines();
                }

                cases.push((case_expr, case_body));
            } else if self.current_token == Token::Default {
                self.next_token();
                self.expect_token(Token::Colon)?;
                self.skip_newlines();

                let mut default_body = Vec::new();
                while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
                    default_body.push(self.parse_statement()?);
                    self.skip_newlines();
                }

                default = Some(default_body);
                break;
            } else {
                self.next_token();
            }
        }

        self.expect_token(Token::RightBrace)?;

        Ok(Stmt::Switch {
            expr,
            cases,
            default,
        })
    }

    /// Parse: Import {NAME1, NAME2} From "path"
    fn parse_import_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Import'

        let mut names = Vec::new();
        let mut aliases = Vec::new();

        // Import {NAME1, NAME2, ...}
        if self.current_token == Token::LeftBrace {
            self.next_token();
            self.skip_newlines();

            while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
                let name = match &self.current_token {
                    Token::Identifier(n) => n.clone(),
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: "identifier".to_string(),
                            found: self.current_token.clone(),
                            line: self.current_line,
                            column: self.current_column,
                        });
                    }
                };

                self.next_token();

                // Check for alias: as ALIAS
                let alias = if self.current_token == Token::As {
                    self.next_token();
                    if let Token::Identifier(a) = &self.current_token.clone() {
                        let alias_name = a.clone();
                        self.next_token();
                        Some(alias_name)
                    } else {
                        None
                    }
                } else {
                    None
                };

                names.push(name);
                aliases.push(alias);

                if self.current_token == Token::Comma {
                    self.next_token();
                    self.skip_newlines();
                } else {
                    break;
                }
            }

            self.expect_token(Token::RightBrace)?;
        } else {
            // Import NAME
            let name = match &self.current_token {
                Token::Identifier(n) => n.clone(),
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "identifier".to_string(),
                        found: self.current_token.clone(),
                        line: self.current_line,
                        column: self.current_column,
                    });
                }
            };
            self.next_token();

            let alias = if self.current_token == Token::As {
                self.next_token();
                if let Token::Identifier(a) = &self.current_token.clone() {
                    let alias_name = a.clone();
                    self.next_token();
                    Some(alias_name)
                } else {
                    None
                }
            } else {
                None
            };

            names.push(name);
            aliases.push(alias);
        }

        self.expect_token(Token::From)?;

        let path = match &self.current_token {
            Token::String(p) => p.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "string".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        self.next_token();

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Import {
            names,
            path,
            aliases,
        })
    }

    /// Parse: Export NAME
    fn parse_export_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Export'

        let name = match &self.current_token {
            Token::Identifier(n) => n.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current_token.clone(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        self.next_token();

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Export(name))
    }

    /// Parse: Throw expr
    fn parse_throw_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // skip 'Throw'

        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Throw(expr))
    }

    /// Parse expression as statement
    fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.current_token == Token::Newline || self.current_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Stmt::Expression(expr))
    }

    /// Parse parameter list: (A, B, C)
    fn parse_parameter_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();

        if self.current_token == Token::RightParen {
            return Ok(params);
        }

        while let Token::Identifier(name) = &self.current_token {
            // Validate parameter name (allow flexible naming)
            self.validate_identifier_internal(name, true)?;
            params.push(name.clone());
            self.next_token();

            if self.current_token == Token::Comma {
                self.next_token();
            } else {
                break;
            }
        }

        Ok(params)
    }

    /// Parse a block of statements: { stmt1 stmt2 ... }
    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        self.skip_newlines();

        while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(statements)
    }

    /// Parse an expression using Pratt parsing
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr, ParseError> {
        let mut left = self.parse_prefix()?;

        // After parse_prefix, current_token is at the first token after the prefix expression
        while precedence < self.current_precedence()
            && self.current_token != Token::Newline
            && self.current_token != Token::Semicolon
            && self.current_token != Token::EOF
            && self.current_token != Token::RightParen
            && self.current_token != Token::RightBracket
            && self.current_token != Token::RightBrace
            && self.current_token != Token::Comma
            && self.current_token != Token::Colon
        {
            left = self.parse_infix(left)?;
        }

        Ok(left)
    }

    /// Parse prefix expressions
    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        match &self.current_token.clone() {
            Token::Number(n) => {
                let num = *n;
                self.next_token();
                Ok(Expr::Number(num))
            }
            Token::BigInteger(s) => {
                let big_int_str = s.clone();
                self.next_token();
                Ok(Expr::BigInteger(big_int_str))
            }
            Token::String(s) => {
                let string = s.clone();
                self.next_token();
                Ok(Expr::String(string))
            }
            Token::Boolean(b) => {
                let bool_val = *b;
                self.next_token();
                Ok(Expr::Boolean(bool_val))
            }
            Token::Null => {
                self.next_token();
                Ok(Expr::Null)
            }
            Token::Identifier(name) => {
                let ident = name.clone();
                self.next_token();
                Ok(Expr::Identifier(ident))
            }
            Token::LeftParen => self.parse_grouped_expression(),
            Token::LeftBracket => self.parse_array_literal(),
            Token::LeftBrace => self.parse_dict_literal(),
            Token::Minus => self.parse_unary_expression(UnaryOp::Minus),
            Token::Not => self.parse_unary_expression(UnaryOp::Not),
            Token::If => self.parse_if_expression(),
            Token::Func => self.parse_lambda_expression(),
            Token::Lambda => self.parse_lambda_arrow_expression(),
            _ => Err(ParseError::InvalidExpression {
                message: "Unexpected token in expression".to_string(),
                line: self.current_line,
                column: self.current_column,
            }),
        }
    }

    /// Parse infix expressions
    fn parse_infix(&mut self, left: Expr) -> Result<Expr, ParseError> {
        match &self.current_token {
            Token::Plus
            | Token::Minus
            | Token::Multiply
            | Token::Divide
            | Token::Modulo
            | Token::Equal
            | Token::NotEqual
            | Token::Less
            | Token::LessEqual
            | Token::Greater
            | Token::GreaterEqual
            | Token::And
            | Token::Or => self.parse_binary_expression(left),
            Token::LeftParen => self.parse_call_expression(left),
            Token::LeftBracket => self.parse_index_expression(left),
            _ => Ok(left),
        }
    }

    /// Parse grouped expression: (expr)
    fn parse_grouped_expression(&mut self) -> Result<Expr, ParseError> {
        self.next_token(); // skip '('

        let expr = self.parse_expression(Precedence::Lowest)?;

        // parse_expression returns with current_token at the first token after the expression
        // which should be ')'
        if self.current_token == Token::RightParen {
            self.next_token(); // move past ')'
            Ok(expr)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "RightParen".to_string(),
                found: self.current_token.clone(),
                line: self.current_line,
                column: self.current_column,
            })
        }
    }
    /// Parse array literal: [1, 2, 3]
    fn parse_array_literal(&mut self) -> Result<Expr, ParseError> {
        self.next_token(); // skip '['

        let mut elements = Vec::new();

        self.skip_newlines();

        while self.current_token != Token::RightBracket && self.current_token != Token::EOF {
            elements.push(self.parse_expression(Precedence::Lowest)?);

            self.skip_newlines();

            if self.current_token == Token::Comma {
                self.next_token();
                self.skip_newlines();
            } else if self.current_token == Token::RightBracket {
                break;
            }
        }

        self.expect_token(Token::RightBracket)?;

        Ok(Expr::Array(elements))
    }

    /// Parse dictionary literal: {key: value, ...}
    fn parse_dict_literal(&mut self) -> Result<Expr, ParseError> {
        self.next_token(); // skip '{'

        let mut pairs = Vec::new();

        self.skip_newlines();

        while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
            let key = match &self.current_token {
                Token::Identifier(k) => k.clone(),
                Token::String(k) => k.clone(),
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "identifier or string".to_string(),
                        found: self.current_token.clone(),
                        line: self.current_line,
                        column: self.current_column,
                    });
                }
            };

            self.next_token();
            self.expect_token(Token::Colon)?;

            let value = self.parse_expression(Precedence::Lowest)?;

            pairs.push((key, value));

            self.skip_newlines();

            if self.current_token == Token::Comma {
                self.next_token();
                self.skip_newlines();
            } else if self.current_token == Token::RightBrace {
                break;
            }
        }

        self.expect_token(Token::RightBrace)?;

        Ok(Expr::Dict(pairs))
    }

    /// Parse unary expression: -expr or !expr
    fn parse_unary_expression(&mut self, op: UnaryOp) -> Result<Expr, ParseError> {
        self.next_token(); // skip operator

        let expr = self.parse_expression(Precedence::Prefix)?;

        Ok(Expr::unary(op, expr))
    }

    /// Parse binary expression: left op right
    fn parse_binary_expression(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let op = match &self.current_token {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Subtract,
            Token::Multiply => BinOp::Multiply,
            Token::Divide => BinOp::Divide,
            Token::Modulo => BinOp::Modulo,
            Token::Equal => BinOp::Equal,
            Token::NotEqual => BinOp::NotEqual,
            Token::Less => BinOp::Less,
            Token::LessEqual => BinOp::LessEqual,
            Token::Greater => BinOp::Greater,
            Token::GreaterEqual => BinOp::GreaterEqual,
            Token::And => BinOp::And,
            Token::Or => BinOp::Or,
            _ => {
                return Err(ParseError::InvalidExpression {
                    message: "Invalid binary operator".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };

        let precedence = self.current_precedence();
        self.next_token();

        let right = self.parse_expression(precedence)?;

        Ok(Expr::binary(left, op, right))
    }

    /// Parse function call: func(arg1, arg2, ...)
    fn parse_call_expression(&mut self, func: Expr) -> Result<Expr, ParseError> {
        self.next_token(); // skip '('

        let mut args = Vec::new();

        self.skip_newlines();

        while self.current_token != Token::RightParen && self.current_token != Token::EOF {
            args.push(self.parse_expression(Precedence::Lowest)?);

            if self.current_token == Token::Comma {
                self.next_token();
                self.skip_newlines();
            } else {
                break;
            }
        }

        self.expect_token(Token::RightParen)?;

        Ok(Expr::call(func, args))
    }

    /// Parse index expression: object[index]
    fn parse_index_expression(&mut self, object: Expr) -> Result<Expr, ParseError> {
        self.next_token(); // skip '['

        let index = self.parse_expression(Precedence::Lowest)?;

        self.expect_token(Token::RightBracket)?;

        Ok(Expr::index(object, index))
    }

    /// Parse if expression: If (cond) { ... } Elif (cond) { ... } Else { ... }
    fn parse_if_expression(&mut self) -> Result<Expr, ParseError> {
        self.next_token(); // skip 'If'
        self.expect_token(Token::LeftParen)?;

        let condition = self.parse_expression(Precedence::Lowest)?;

        self.expect_token(Token::RightParen)?;
        self.skip_newlines();
        self.expect_token(Token::LeftBrace)?;

        let then_branch = self.parse_block()?;

        self.expect_token(Token::RightBrace)?;
        self.skip_newlines();

        let mut elif_branches = Vec::new();
        while self.current_token == Token::Elif {
            self.next_token();
            self.expect_token(Token::LeftParen)?;

            let elif_cond = self.parse_expression(Precedence::Lowest)?;

            self.expect_token(Token::RightParen)?;
            self.skip_newlines();
            self.expect_token(Token::LeftBrace)?;

            let elif_body = self.parse_block()?;

            self.expect_token(Token::RightBrace)?;
            self.skip_newlines();

            elif_branches.push((elif_cond, elif_body));
        }

        let else_branch = if self.current_token == Token::Else {
            self.next_token();
            self.skip_newlines();
            self.expect_token(Token::LeftBrace)?;

            let else_body = self.parse_block()?;

            self.expect_token(Token::RightBrace)?;

            Some(else_body)
        } else {
            None
        };

        Ok(Expr::If {
            condition: Box::new(condition),
            then_branch,
            elif_branches,
            else_branch,
        })
    }

    /// Parse lambda expression: Func(params) { body }
    fn parse_lambda_expression(&mut self) -> Result<Expr, ParseError> {
        self.next_token(); // skip 'Func'
        self.expect_token(Token::LeftParen)?;

        let params = self.parse_parameter_list()?;

        self.expect_token(Token::RightParen)?;
        self.skip_newlines();
        self.expect_token(Token::LeftBrace)?;

        let body = self.parse_block()?;

        self.expect_token(Token::RightBrace)?;

        Ok(Expr::Lambda { params, body })
    }

    /// Parse lambda arrow expression: Lambda X -> expr or Lambda (X, Y) -> expr
    fn parse_lambda_arrow_expression(&mut self) -> Result<Expr, ParseError> {
        self.next_token(); // skip 'Lambda'

        let params = if self.current_token == Token::LeftParen {
            // Multiple parameters: Lambda (X, Y) -> expr
            self.next_token(); // skip '('
            let params = self.parse_parameter_list()?;
            self.expect_token(Token::RightParen)?;
            params
        } else {
            // Single parameter: Lambda X -> expr
            match &self.current_token {
                Token::Identifier(name) => {
                    self.validate_identifier_internal(name, true)?;
                    let param = name.clone();
                    self.next_token();
                    vec![param]
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "identifier or '('".to_string(),
                        found: self.current_token.clone(),
                        line: self.current_line,
                        column: self.current_column,
                    });
                }
            }
        };

        // Expect arrow
        self.expect_token(Token::Arrow)?;

        // Parse the expression body
        let expr = self.parse_expression(Precedence::Lowest)?;

        // Wrap the expression in a Return statement
        let body = vec![Stmt::Return(expr)];

        Ok(Expr::Lambda { params, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
