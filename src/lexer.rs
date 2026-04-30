//! Lexer for CatLang
//! Optimized for performance with reduced allocations and faster string handling

use crate::token::{Span, Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

/// Lexer error types
#[derive(Debug, Clone)]
pub enum LexError {
    UnexpectedCharacter { char: char, line: usize, column: usize },
    UnterminatedString { line: usize, column: usize },
    InvalidNumber { value: String, line: usize, column: usize },
    InvalidEscape { escape: String, line: usize, column: usize },
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedCharacter { char, line, column } => {
                write!(f, "Unexpected character '{}' at line {}, column {}", char, line, column)
            }
            LexError::UnterminatedString { line, column } => {
                write!(f, "Unterminated string at line {}, column {}", line, column)
            }
            LexError::InvalidNumber { value, line, column } => {
                write!(f, "Invalid number '{}' at line {}, column {}", value, line, column)
            }
            LexError::InvalidEscape { escape, line, column } => {
                write!(f, "Invalid escape sequence '{}' at line {}, column {}", escape, line, column)
            }
        }
    }
}

pub type LexResult<T> = Result<T, LexError>;

/// The CatLang Lexer - optimized with byte-based scanning
pub struct Lexer<'a> {
    #[allow(dead_code)]
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    pos: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        // Pre-allocate tokens vector with estimated capacity
        let estimated_tokens = input.len() / 5 + 1;
        Self {
            input,
            chars: input.chars().peekable(),
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::with_capacity(estimated_tokens),
        }
    }

    pub fn tokenize(mut self) -> LexResult<Vec<Token>> {
        while let Some(ch) = self.peek_char() {
            match ch {
                // Whitespace
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                
                // Comment (semicolon to end of line)
                ';' => {
                    self.scan_comment();
                }
                
                // String literal
                '"' => {
                    let token = self.scan_string()?;
                    self.tokens.push(token);
                }
                
                // Number literal
                c if c.is_ascii_digit() => {
                    let token = self.scan_number()?;
                    self.tokens.push(token);
                }
                
                // Identifier or keyword
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let token = self.scan_identifier();
                    self.tokens.push(token);
                }
                
                // Operators and delimiters
                '+' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::PlusEqual);
                    } else {
                        self.push_token(TokenKind::Plus);
                    }
                }
                
                '-' => {
                    self.advance();
                    if self.peek_char() == Some('>') {
                        self.advance();
                        self.push_token(TokenKind::Arrow);
                    } else if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::MinusEqual);
                    } else {
                        self.push_token(TokenKind::Minus);
                    }
                }
                
                '*' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::StarEqual);
                    } else {
                        self.push_token(TokenKind::Star);
                    }
                }
                
                '/' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::SlashEqual);
                    } else {
                        self.push_token(TokenKind::Slash);
                    }
                }
                
                '%' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::PercentEqual);
                    } else {
                        self.push_token(TokenKind::Percent);
                    }
                }
                
                '=' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::EqualEqual);
                    } else {
                        self.push_token(TokenKind::Equal);
                    }
                }
                
                '!' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::NotEqual);
                    } else {
                        return Err(LexError::UnexpectedCharacter {
                            char: '!',
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                
                '>' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::GreaterEqual);
                    } else {
                        self.push_token(TokenKind::Greater);
                    }
                }
                
                '<' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::LessEqual);
                    } else {
                        self.push_token(TokenKind::Less);
                    }
                }
                
                '&' => {
                    self.advance();
                    if self.peek_char() == Some('&') {
                        self.advance();
                        self.push_token(TokenKind::AndAnd);
                    } else if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::AndEqual);
                    } else {
                        self.push_token(TokenKind::Ampersand);
                    }
                }
                
                '|' => {
                    self.advance();
                    if self.peek_char() == Some('|') {
                        self.advance();
                        self.push_token(TokenKind::OrOr);
                    } else if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::OrEqual);
                    } else {
                        self.push_token(TokenKind::Pipe);
                    }
                }
                
                '^' => {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.push_token(TokenKind::XorEqual);
                    } else {
                        self.push_token(TokenKind::Caret);
                    }
                }
                
                '(' => {
                    self.advance();
                    self.push_token(TokenKind::LParen);
                }
                
                ')' => {
                    self.advance();
                    self.push_token(TokenKind::RParen);
                }
                
                '[' => {
                    self.advance();
                    self.push_token(TokenKind::LBracket);
                }
                
                ']' => {
                    self.advance();
                    self.push_token(TokenKind::RBracket);
                }
                
                '{' => {
                    self.advance();
                    self.push_token(TokenKind::LBrace);
                }
                
                '}' => {
                    self.advance();
                    self.push_token(TokenKind::RBrace);
                }
                
                ':' => {
                    self.advance();
                    self.push_token(TokenKind::Colon);
                }
                
                ',' => {
                    self.advance();
                    self.push_token(TokenKind::Comma);
                }
                
                '.' => {
                    self.advance();
                    self.push_token(TokenKind::Dot);
                }
                
                '@' => {
                    self.advance();
                    self.push_token(TokenKind::At);
                }
                
                '~' => {
                    self.advance();
                    self.push_token(TokenKind::Tilde);
                }
                
                _ => {
                    return Err(LexError::UnexpectedCharacter {
                        char: ch,
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        }
        
        self.tokens.push(Token::new(
            TokenKind::Eof,
            Span::point(self.pos),
            self.line,
            self.column,
        ));
        
        Ok(self.tokens)
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if ch.is_some() {
            self.pos += ch.unwrap().len_utf8();
            self.column += 1;
        }
        ch
    }

    fn push_token(&mut self, kind: TokenKind) {
        let span = Span::point(self.pos);
        self.tokens.push(Token::new(kind, span, self.line, self.column));
    }

    fn scan_comment(&mut self) {
        // Consume the semicolon
        self.advance();
        
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;
        
        // Read until end of line
        let mut content = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }
            content.push(ch);
            self.advance();
        }
        
        // Add comment token (for tracking, but parser may ignore)
        let span = Span::new(start, self.pos);
        self.tokens.push(Token::new(
            TokenKind::Comment(content),
            span,
            start_line,
            start_col,
        ));
    }

    fn scan_string(&mut self) -> LexResult<Token> {
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;

        // Consume opening quote
        self.advance();

        let mut value = String::with_capacity(64); // Pre-allocate reasonable capacity
        let mut interpolated = false;

        while let Some(ch) = self.peek_char() {
            match ch {
                '"' => {
                    self.advance();
                    break;
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.peek_char() {
                        match escaped {
                            'n' => value.push('\n'),
                            'r' => value.push('\r'),
                            't' => value.push('\t'),
                            '\\' => value.push('\\'),
                            '"' => value.push('"'),
                            '{' => value.push('{'),
                            _ => {
                                return Err(LexError::InvalidEscape {
                                    escape: format!("\\{}", escaped),
                                    line: self.line,
                                    column: self.column,
                                });
                            }
                        }
                        self.advance();
                    }
                }
                '{' => {
                    // String interpolation start - include the brace and expression
                    interpolated = true;
                    value.push('{');
                    self.advance();

                    // Read until closing brace - optimized with depth counting
                    let mut depth = 1;
                    while let Some(inner_ch) = self.peek_char() {
                        if inner_ch == '{' {
                            depth += 1;
                        } else if inner_ch == '}' {
                            depth -= 1;
                            if depth == 0 {
                                value.push(inner_ch);
                                self.advance();
                                break;
                            }
                        }
                        value.push(inner_ch);
                        self.advance();
                    }
                }
                _ => {
                    value.push(ch);
                    self.advance();
                }
            }
        }

        let span = Span::new(start, self.pos);
        Ok(Token::new(
            if interpolated {
                TokenKind::StringLiteral(format!("i:{}", value))
            } else {
                TokenKind::StringLiteral(value)
            },
            span,
            start_line,
            start_col,
        ))
    }

    fn scan_number(&mut self) -> LexResult<Token> {
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;

        let mut value = String::with_capacity(16); // Pre-allocate for typical numbers
        let mut is_float = false;

        // Check for hex literal
        if self.peek_char() == Some('0') {
            self.advance();
            if self.peek_char() == Some('x') || self.peek_char() == Some('X') {
                self.advance();
                value.push_str("0x");

                // Read hex digits
                while let Some(ch) = self.peek_char() {
                    if ch.is_ascii_hexdigit() {
                        value.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                let span = Span::new(start, self.pos);
                return Ok(Token::new(
                    TokenKind::IntLiteral(value),
                    span,
                    start_line,
                    start_col,
                ));
            } else {
                value.push('0');
            }
        }

        // Read integer part - optimized loop
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for float
        if self.peek_char() == Some('.') {
            // Check if next character is a digit without cloning
            let mut temp_chars = self.chars.clone();
            temp_chars.next(); // skip '.'
            if let Some(next) = temp_chars.next() {
                if next.is_ascii_digit() {
                    is_float = true;
                    value.push('.');
                    self.advance();

                    while let Some(ch) = self.peek_char() {
                        if ch.is_ascii_digit() {
                            value.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Check for exponent
        if self.peek_char() == Some('e') || self.peek_char() == Some('E') {
            is_float = true;
            value.push('e');
            self.advance();

            if let Some(sign) = self.peek_char() {
                if sign == '+' || sign == '-' {
                    value.push(sign);
                    self.advance();
                }
            }

            while let Some(ch) = self.peek_char() {
                if ch.is_ascii_digit() {
                    value.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let span = Span::new(start, self.pos);
        Ok(Token::new(
            if is_float {
                TokenKind::FloatLiteral(value)
            } else {
                TokenKind::IntLiteral(value)
            },
            span,
            start_line,
            start_col,
        ))
    }

    fn scan_identifier(&mut self) -> Token {
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;

        let mut value = String::with_capacity(32); // Pre-allocate for typical identifiers

        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let kind = self.ident_to_keyword(&value);
        let span = Span::new(start, self.pos);
        Token::new(kind, span, start_line, start_col)
    }

    fn ident_to_keyword(&self, value: &str) -> TokenKind {
        match value {
            "struct" => TokenKind::KwStruct,
            "impl" => TokenKind::KwImpl,
            "fn" => TokenKind::KwFn,
            "return" => TokenKind::KwReturn,
            "new" => TokenKind::KwNew,
            "cpy" => TokenKind::KwCpy,
            "unsafe" => TokenKind::KwUnsafe,
            "close" => TokenKind::KwClose,
            "keep" => TokenKind::KwKeep,
            "all" => TokenKind::KwAll,
            "m" => TokenKind::KwM,
            "ia" => TokenKind::KwIa,
            "fa" => TokenKind::KwFa,
            "sa" => TokenKind::KwSa,
            "arr" => TokenKind::KwArr,
            // Any-width types
            "a8" => TokenKind::KwA8,
            "a16" => TokenKind::KwA16,
            "a32" => TokenKind::KwA32,
            "a64" => TokenKind::KwA64,
            "aa" => TokenKind::KwAa,
            "timer" => TokenKind::KwTimer,
            "if" => TokenKind::KwIf,
            "else" => TokenKind::KwElse,
            "while" => TokenKind::KwWhile,
            "for" => TokenKind::KwFor,
            "switch" => TokenKind::KwSwitch,
            "case" => TokenKind::KwCase,
            "default" => TokenKind::KwDefault,
            "try" => TokenKind::KwTry,
            "catch" => TokenKind::KwCatch,
            "throw" => TokenKind::KwThrow,
            "async" => TokenKind::KwAsync,
            "await" => TokenKind::KwAwait,
            "spawn" => TokenKind::KwSpawn,
            "import" => TokenKind::KwImport,
            "from" => TokenKind::KwFrom,
            "as" => TokenKind::KwAs,
            "true" => TokenKind::KwTrue,
            "false" => TokenKind::KwFalse,
            _ => TokenKind::Identifier(value.to_string()),
        }
    }
}

/// Public function to tokenize input
pub fn tokenize(input: &str) -> LexResult<Vec<Token>> {
    Lexer::new(input).tokenize()
}
