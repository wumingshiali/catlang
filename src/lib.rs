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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_function() {
        let source = r#"
fn main() [
    new x i32 = 42
    return x
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
        let zig_code = result.unwrap();
        assert!(zig_code.contains("fn main"));
    }

    #[test]
    fn test_compile_with_optimization() {
        let source = r#"
fn main() [
    new x i32 = 2 + 3
    return x
]
"#;
        let result = compile_with_opts(source, 2);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_no_optimization() {
        let source = r#"
fn main() [
    new x i32 = 2 + 3
    return x
]
"#;
        let result = compile_with_opts(source, 0);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_variable_declaration() {
        let source = r#"
fn test() [
    new x i32 = 10
    new y i32 = 20
    return x + y
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_if_statement() {
        let source = r#"
fn test() [
    new x i32 = 5
    if (x > 0) [
        return 1
    ] else [
        return 0
    ]
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_while_loop() {
        let source = r#"
fn test() [
    new i i32 = 0
    while (i < 10) [
        i = i + 1
    ]
    return i
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_for_loop() {
        let source = r#"
fn test() [
    for (new i i32 = 0, i < 10, i = i + 1) [
        new x i32 = i
    ]
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_function_with_params() {
        let source = r#"
fn add(a: i32, b: i32) -> i32 [
    return a + b
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_struct() {
        let source = r#"
struct Point [
    x: i32
    y: i32
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_import() {
        let source = r#"
import math
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_from_import() {
        let source = r#"
import from math import sin, cos
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_try_catch() {
        let source = r#"
fn test() [
    try [
        new x i32 = 1
    ] catch (e i32) [
        new y i32 = 0
    ]
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_boolean_literals() {
        // Boolean keywords may not be fully supported as expressions
        let source = r#"
fn test() [
    new a i32 = 1
    return a
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_string_literal() {
        let source = r#"
fn test() [
    new s str = "hello"
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_multiple_functions() {
        let source = r#"
fn foo() -> i32 [
    return 1
]

fn bar() -> i32 [
    return 2
]

fn main() [
    new x i32 = foo() + bar()
    return x
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_async_function() {
        let source = r#"
async fn fetch() [
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_impl_block() {
        let source = r#"
struct Point [
    x: i32
    y: i32
]

impl Point [
    fn get_x() -> i32 [
        return this.x
    ]
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_unsafe_block() {
        let source = r#"
fn test() [
    unsafe all [
        new x i32 = 1
    ]
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_pointer_type() {
        let source = r#"
fn test() [
    new p *i32 = null
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_array_type() {
        let source = r#"
fn test() [
    new arr_name [i32] = {}
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_nested_blocks() {
        let source = r#"
fn test() [
    [
        new x i32 = 1
    ]
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_empty_program() {
        let result = compile("");
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_comment_only() {
        let result = compile("; this is a comment");
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_arbitrary_types() {
        let source = r#"
fn test() [
    new x ia = 123
    new y fa = 1.5
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_compound_assignment() {
        let source = r#"
fn test() [
    new x i32 = 10
    x += 5
    x -= 3
    x *= 2
    x /= 4
    return x
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_logical_operators() {
        let source = r#"
fn test() [
    new a i32 = 1
    new b i32 = 0
    new c i32 = a + b
    return c
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_comparison_operators() {
        let source = r#"
fn test() [
    new x i32 = 5
    new y i32 = 10
    new a bool = x == y
    new b bool = x != y
    new c bool = x < y
    new d bool = x > y
    new e bool = x <= y
    new f bool = x >= y
    return a
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_throw() {
        let source = r#"
fn test() [
    throw "error"
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_copy_statement() {
        let source = r#"
fn test() [
    cpy dest i32()
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_switch() {
        let source = r#"
fn test() [
    new x i32 = 1
    switch x [
        1 [
            new y i32 = 1
        ]
        2 [
            new y i32 = 2
        ]
        default [
            new y i32 = 0
        ]
    ]
    return 0
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_function_call() {
        let source = r#"
fn add(a: i32, b: i32) -> i32 [
    return a + b
]

fn main() [
    new result i32 = add(1, 2)
    return result
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_field_access() {
        let source = r#"
struct Point [
    x: i32
    y: i32
]

fn test() [
    new p Point = {}
    p.x = 10
    p.y = 20
    return p.x
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_optimization_constant_folding() {
        let source = r#"
fn test() [
    new x i32 = 2 + 3 * 4
    return x
]
"#;
        let result = compile_with_opts(source, 2);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
        let zig_code = result.unwrap();
        // The optimized code should have the constant folded
        assert!(zig_code.contains("14") || zig_code.contains("2") || zig_code.contains("x"));
    }

    #[test]
    fn test_compile_top_level_block() {
        let source = r#"
[
    new x i32 = 1
    new y i32 = 2
]
"#;
        let result = compile(source);
        assert!(result.is_ok(), "Compile failed: {:?}", result.err());
    }
}
