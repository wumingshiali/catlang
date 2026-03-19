//! Abstract Syntax Tree definitions for CatLang
//! Optimized with better derive macros for performance

use crate::token::Span;

/// Program - the root AST node
#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<TopLevelDecl>,
}

/// Top-level declarations
#[derive(Debug, Clone)]
pub enum TopLevelDecl {
    Import(ImportStmt),
    Function(FunctionDef),
    AsyncFunction(FunctionDef),
    Struct(StructDef),
    Impl(ImplDef),
    GlobalVar(GlobalVarDecl),
    Statements(Block), // Top-level statements for sequential execution
}

/// Import statement (third-party only)
#[derive(Debug, Clone)]
pub enum ImportStmt {
    /// import module_path as alias
    Simple {
        module_path: Vec<String>,
        alias: Option<String>,
        span: Span,
    },
    /// from module_path import name1 as alias1, name2 as alias2
    From {
        module_path: Vec<String>,
        names: Vec<(String, Option<String>)>,
        span: Span,
    },
}

/// Function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeExpr>,
    pub body: Block,
    pub is_async: bool,
    pub span: Span,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub param_type: TypeExpr,
    pub span: Span,
}

/// Struct definition
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
    pub span: Span,
}

/// Struct field
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: TypeExpr,
    pub span: Span,
}

/// Impl block
#[derive(Debug, Clone)]
pub struct ImplDef {
    pub type_name: String,
    pub methods: Vec<FunctionDef>,
    pub span: Span,
}

/// Global variable declaration
#[derive(Debug, Clone)]
pub struct GlobalVarDecl {
    pub name: String,
    pub var_type: TypeExpr,
    pub init: Option<Expr>,
    pub span: Span,
}

/// Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(VarDecl),
    Assignment(Assignment),
    CopyStmt(CopyStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Switch(SwitchStmt),
    Try(TryStmt),
    Throw(ThrowStmt),
    AsyncStmt(AsyncStmt),
    Block(Block),
    UnsafeBlock(UnsafeBlock),
    Expr(Expr),
    Return(ReturnStmt),
}

/// Variable declaration: new identifier type_expr = expression
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub var_type: TypeExpr,
    pub init: Option<Expr>,
    pub span: Span,
}

/// Assignment statement
#[derive(Debug, Clone)]
pub struct Assignment {
    pub target: AssignmentTarget,
    pub op: AssignOp,
    pub value: Expr,
    pub span: Span,
}

/// Assignment target (identifier or field access)
#[derive(Debug, Clone)]
pub enum AssignmentTarget {
    Identifier(String),
    FieldAccess(String, String), // object.field
    ArrayIndex(String, Box<Expr>), // array[index]
}

impl AssignmentTarget {
    pub fn to_zig(&self) -> String {
        match self {
            AssignmentTarget::Identifier(name) => name.clone(),
            AssignmentTarget::FieldAccess(obj, field) => format!("{}.{}", obj, field),
            AssignmentTarget::ArrayIndex(_, _) => "[index]".to_string(), // Placeholder, actual index handled in codegen
        }
    }
}

/// Assignment operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssignOp {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AndEqual,
    OrEqual,
    XorEqual,
}

/// Copy statement: cpy dest(bytes)
#[derive(Debug, Clone)]
pub struct CopyStmt {
    pub dest: String,
    pub size_type: TypeExpr,
    pub span: Span,
}

/// Block of statements: [ ... ]
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

/// If statement
#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub else_branch: Option<ElseBranch>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ElseBranch {
    Block(Block),
    ElseIf(Box<IfStmt>),
}

/// While statement
#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
    pub span: Span,
}

/// For statement: for (var_decl, condition, update) block
#[derive(Debug, Clone)]
pub struct ForStmt {
    pub init: VarDecl,
    pub condition: Expr,
    pub update: Assignment,
    pub body: Block,
    pub span: Span,
}

/// Switch statement with pattern matching
#[derive(Debug, Clone)]
pub struct SwitchStmt {
    pub expr: Expr,
    pub cases: Vec<CaseClause>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CaseClause {
    pub pattern: Pattern,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    Underscore,
}

/// Try/Catch statement
#[derive(Debug, Clone)]
pub struct TryStmt {
    pub try_block: Block,
    pub catch_clauses: Vec<CatchClause>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CatchClause {
    pub var_name: String,
    pub error_type: TypeExpr,
    pub body: Block,
    pub span: Span,
}

/// Throw statement
#[derive(Debug, Clone)]
pub struct ThrowStmt {
    pub expr: Expr,
    pub span: Span,
}

/// Async statements (async fn, await, spawn)
#[derive(Debug, Clone)]
pub enum AsyncStmt {
    Await(Expr),
    Spawn(Expr),
}

/// Unsafe block
#[derive(Debug, Clone)]
pub struct UnsafeBlock {
    pub scope_modifier: ScopeModifier,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ScopeModifier {
    Close(SafetyTarget),
    Keep(SafetyTarget),
    All,
}

#[derive(Debug, Clone)]
pub enum SafetyTarget {
    Init,
    Bounds,
    Lifetime,
    Null,
    Identifier(String),
}

/// Return statement
#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub expr: Option<Expr>,
    pub span: Span,
}

/// Expressions
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(Literal),
    Identifier(String),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    FieldAccess(Box<Expr>, String),
    Array(Vec<Expr>),
    ArrayIndex(Box<Expr>, Box<Expr>),
    Block(Block),
    AsyncBlock(Block),
    TryBlock(Block),
    Cast(Box<Expr>, TypeExpr),
    MemoryCast(TypeExpr, Box<Expr>), // m+Type cast
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
}

/// Literals
#[derive(Debug, Clone)]
pub enum Literal {
    Int(String),
    Float(String),
    String(String),
    InterpolatedString(String),
    Bool(bool),
}

/// Type expressions
#[derive(Debug, Clone)]
pub enum TypeExpr {
    Base(String),
    Pointer(Box<TypeExpr>),
    Array(Box<TypeExpr>, Option<usize>), // None = dynamic size
    MemoryCast(Box<TypeExpr>),           // m+Type
    Result,
    Future,
    Ia,    // arbitrary length int
    Fa,    // arbitrary length float
    Sa,    // arbitrary length string
    Timer, // timer type

    // Generic type with type parameters: <T> or <T1, T2, ...>
    Generic(Box<TypeExpr>, Vec<TypeExpr>),

    // Arbitrary width types: a8, a16, a32, a64, aa
    // a8/a16/a32/a64 = any type with specific bit width
    // aa = any type with arbitrary/infinite length
    AnyWidth(u16),  // 8, 16, 32, 64
    AnyWidthArbitrary, // aa - arbitrary length

    // List type with length specifier: arr name<long<type>>
    // long can be a number (fixed capacity) or 'a' (arbitrary/infinite length)
    List(Box<TypeExpr>, ListLength),
}

/// List length specifier
#[derive(Debug, Clone)]
pub enum ListLength {
    Fixed(usize),      // Fixed capacity: long(10)
    Arbitrary,         // Arbitrary/infinite length: long(a)
}

impl TypeExpr {
    pub fn to_zig_type(&self) -> String {
        match self {
            TypeExpr::Base(name) => match name.as_str() {
                "i8" => "i8".to_string(),
                "i16" => "i16".to_string(),
                "i32" => "i32".to_string(),
                "i64" => "i64".to_string(),
                "u8" => "u8".to_string(),
                "u16" => "u16".to_string(),
                "u32" => "u32".to_string(),
                "u64" => "u64".to_string(),
                "f32" => "f32".to_string(),
                "f64" => "f64".to_string(),
                "bool" => "bool".to_string(),
                "str" => "[]const u8".to_string(),
                "void" => "void".to_string(),
                _ => name.to_string(), // User-defined types
            },
            TypeExpr::Pointer(inner) => format!("*{}", inner.to_zig_type()),
            TypeExpr::Array(inner, None) => format!("[]{}", inner.to_zig_type()),
            TypeExpr::Array(inner, Some(size)) => format!("[{}]{}", size, inner.to_zig_type()),
            TypeExpr::MemoryCast(inner) => inner.to_zig_type(),
            TypeExpr::Result => "anyerror!void".to_string(),
            TypeExpr::Future => "anyerror!void".to_string(),
            TypeExpr::Ia => "i128".to_string(), // arbitrary length int -> i128
            TypeExpr::Fa => "f128".to_string(), // arbitrary length float -> f128
            TypeExpr::Sa => "[]const u8".to_string(), // arbitrary length string
            TypeExpr::Timer => "Timer".to_string(), // timer type
            
            // Generic types: Map<T>, Option<T>, Result<T, E>, etc.
            TypeExpr::Generic(base, params) => {
                let base_str = base.to_zig_type();
                let params_str: Vec<String> = params.iter().map(|p| p.to_zig_type()).collect();
                format!("{}({})", base_str, params_str.join(", "))
            }
            
            // Arbitrary width types
            TypeExpr::AnyWidth(bits) => {
                // For Zig, we map to the closest standard type
                match bits {
                    8 => "u8".to_string(),
                    16 => "u16".to_string(),
                    32 => "u32".to_string(),
                    64 => "u64".to_string(),
                    _ => "u128".to_string(), // Fallback for unknown sizes
                }
            }
            TypeExpr::AnyWidthArbitrary => "u128".to_string(), // Use u128 for arbitrary length

            // List type: std.ArrayList(T) for arbitrary length, or fixed-size array
            TypeExpr::List(inner, length) => {
                let inner_type = inner.to_zig_type();
                match length {
                    ListLength::Fixed(capacity) => format!("[{}]{}", capacity, inner_type),
                    ListLength::Arbitrary => format!("std.ArrayList({})", inner_type),
                }
            }
        }
    }

    /// Check if this is a numeric type (integer or float)
    pub fn is_numeric(&self) -> bool {
        matches!(self,
            TypeExpr::Base(name) if matches!(name.as_str(),
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" |
                "f32" | "f64" | "ia" | "fa" | "bool"
            )
            || matches!(self, TypeExpr::AnyWidth(_) | TypeExpr::AnyWidthArbitrary)
        )
    }
    
    /// Check if this is an any-width type (a8, a16, a32, a64, aa)
    pub fn is_any_width(&self) -> bool {
        matches!(self, TypeExpr::AnyWidth(_) | TypeExpr::AnyWidthArbitrary)
    }
}
