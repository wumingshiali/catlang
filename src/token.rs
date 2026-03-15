//! Token definitions for CatLang lexer

use std::fmt;

/// Source location for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn point(pos: usize) -> Self {
        Self { start: pos, end: pos }
    }
}

/// Token types for CatLang
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    IntLiteral(String),
    FloatLiteral(String),
    StringLiteral(String),
    BoolLiteral(bool),

    // Identifiers & Keywords
    Identifier(String),
    
    // Type keywords
    KwStruct,
    KwImpl,
    KwFn,
    KwReturn,
    KwNew,
    KwCpy,
    KwUnsafe,
    KwClose,
    KwKeep,
    KwAll,
    KwM,

    // Arbitrary precision/length types
    KwIa,  // arbitrary length int
    KwFa,  // arbitrary length float
    KwSa,  // arbitrary length string

    // Built-in types
    KwTimer,  // timer type

    // Control flow
    KwIf,
    KwElse,
    KwWhile,
    KwFor,
    KwSwitch,
    KwCase,
    KwDefault,
    
    // Error handling
    KwTry,
    KwCatch,
    KwThrow,
    
    // Async & concurrency
    KwAsync,
    KwAwait,
    KwSpawn,
    
    // Import (third-party only)
    KwImport,
    KwFrom,
    KwAs,
    
    // Literals keywords
    KwTrue,
    KwFalse,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AndEqual,
    OrEqual,
    XorEqual,
    AndAnd,
    OrOr,
    
    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Arrow,
    
    // Special
    At,      // @ for attributes if needed
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    
    // End of file
    Eof,
    
    // Comments (ignored but tracked)
    Comment(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::IntLiteral(s) => write!(f, "int({})", s),
            TokenKind::FloatLiteral(s) => write!(f, "float({})", s),
            TokenKind::StringLiteral(s) => write!(f, "string({})", s),
            TokenKind::BoolLiteral(b) => write!(f, "bool({})", b),
            TokenKind::Identifier(s) => write!(f, "ident({})", s),
            
            TokenKind::KwStruct => write!(f, "struct"),
            TokenKind::KwImpl => write!(f, "impl"),
            TokenKind::KwFn => write!(f, "fn"),
            TokenKind::KwReturn => write!(f, "return"),
            TokenKind::KwNew => write!(f, "new"),
            TokenKind::KwCpy => write!(f, "cpy"),
            TokenKind::KwUnsafe => write!(f, "unsafe"),
            TokenKind::KwClose => write!(f, "close"),
            TokenKind::KwKeep => write!(f, "keep"),
            TokenKind::KwAll => write!(f, "all"),
            TokenKind::KwM => write!(f, "m"),

            TokenKind::KwIa => write!(f, "ia"),
            TokenKind::KwFa => write!(f, "fa"),
            TokenKind::KwSa => write!(f, "sa"),

            TokenKind::KwTimer => write!(f, "timer"),

            TokenKind::KwIf => write!(f, "if"),
            TokenKind::KwElse => write!(f, "else"),
            TokenKind::KwWhile => write!(f, "while"),
            TokenKind::KwFor => write!(f, "for"),
            TokenKind::KwSwitch => write!(f, "switch"),
            TokenKind::KwCase => write!(f, "case"),
            TokenKind::KwDefault => write!(f, "default"),
            
            TokenKind::KwTry => write!(f, "try"),
            TokenKind::KwCatch => write!(f, "catch"),
            TokenKind::KwThrow => write!(f, "throw"),
            
            TokenKind::KwAsync => write!(f, "async"),
            TokenKind::KwAwait => write!(f, "await"),
            TokenKind::KwSpawn => write!(f, "spawn"),
            
            TokenKind::KwImport => write!(f, "import"),
            TokenKind::KwFrom => write!(f, "from"),
            TokenKind::KwAs => write!(f, "as"),
            
            TokenKind::KwTrue => write!(f, "true"),
            TokenKind::KwFalse => write!(f, "false"),
            
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::Less => write!(f, "<"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::PlusEqual => write!(f, "+="),
            TokenKind::MinusEqual => write!(f, "-="),
            TokenKind::StarEqual => write!(f, "*="),
            TokenKind::SlashEqual => write!(f, "/="),
            TokenKind::PercentEqual => write!(f, "%="),
            TokenKind::AndEqual => write!(f, "&="),
            TokenKind::OrEqual => write!(f, "|="),
            TokenKind::XorEqual => write!(f, "^="),
            TokenKind::AndAnd => write!(f, "&&"),
            TokenKind::OrOr => write!(f, "||"),
            
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Arrow => write!(f, "->"),
            
            TokenKind::At => write!(f, "@"),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Tilde => write!(f, "~"),
            
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Comment(s) => write!(f, ";{}", s),
        }
    }
}

/// A token with its kind, span, and line/column information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, line: usize, column: usize) -> Self {
        Self { kind, span, line, column }
    }
}
