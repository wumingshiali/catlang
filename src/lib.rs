//! CatLang Compiler - A compiler from CatLang to Zig
//!
//! This library provides the core functionality for compiling CatLang source code
//! to Zig source code.

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod token;

pub use codegen::{generate_zig, CodeGenError, CodeGenResult};
pub use lexer::{tokenize, LexError, LexResult};
pub use optimizer::Optimizer;
pub use parser::{parse, parse_source, ParseError, ParseResult};

/// Compile CatLang source code to Zig source code
pub fn compile(source: &str) -> Result<String, CompileError> {
    compile_with_opts(source, 2)
}

/// Compile CatLang source code to Zig source code with optimization level
pub fn compile_with_opts(source: &str, opt_level: u8) -> Result<String, CompileError> {
    // Tokenize
    let tokens = tokenize(source).map_err(|e| CompileError::Lexical(e.to_string()))?;

    // Parse
    let mut program = parse(tokens).map_err(|e| CompileError::Syntax(e.to_string()))?;

    // Optimize
    if opt_level > 0 {
        let mut optimizer = Optimizer::new(opt_level);
        optimizer.optimize(&mut program);
    }

    // Generate Zig code
    let zig_code =
        generate_zig(&program).map_err(|e| CompileError::CodeGeneration(e.to_string()))?;

    Ok(zig_code)
}

/// Compilation error combining all error types
#[derive(Debug, Clone)]
pub enum CompileError {
    Lexical(String),
    Syntax(String),
    CodeGeneration(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Lexical(msg) => write!(f, "Lexical error: {}", msg),
            CompileError::Syntax(msg) => write!(f, "Syntax error: {}", msg),
            CompileError::CodeGeneration(msg) => write!(f, "Code generation error: {}", msg),
        }
    }
}

impl std::error::Error for CompileError {}
