//! Parser for CatLang
//! Optimized for performance with reduced allocations and cloning

use crate::ast::*;
use crate::lexer::tokenize;
use crate::token::{Span, Token, TokenKind};
use std::collections::HashMap;

/// Parser error types
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

/// The CatLang Parser - optimized with keyword cache
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    eof_token: Token,
    #[allow(dead_code)]
    keyword_cache: HashMap<String, TokenKind>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Pre-build keyword cache for faster lookups
        let mut keyword_cache = HashMap::with_capacity(45);
        let keywords = [
            ("struct", TokenKind::KwStruct),
            ("impl", TokenKind::KwImpl),
            ("fn", TokenKind::KwFn),
            ("return", TokenKind::KwReturn),
            ("new", TokenKind::KwNew),
            ("cpy", TokenKind::KwCpy),
            ("unsafe", TokenKind::KwUnsafe),
            ("close", TokenKind::KwClose),
            ("keep", TokenKind::KwKeep),
            ("all", TokenKind::KwAll),
            ("m", TokenKind::KwM),
            ("ia", TokenKind::KwIa),
            ("fa", TokenKind::KwFa),
            ("sa", TokenKind::KwSa),
            // Any-width types
            ("a8", TokenKind::KwA8),
            ("a16", TokenKind::KwA16),
            ("a32", TokenKind::KwA32),
            ("a64", TokenKind::KwA64),
            ("aa", TokenKind::KwAa),
            ("timer", TokenKind::KwTimer),
            ("if", TokenKind::KwIf),
            ("else", TokenKind::KwElse),
            ("while", TokenKind::KwWhile),
            ("for", TokenKind::KwFor),
            ("switch", TokenKind::KwSwitch),
            ("case", TokenKind::KwCase),
            ("default", TokenKind::KwDefault),
            ("try", TokenKind::KwTry),
            ("catch", TokenKind::KwCatch),
            ("throw", TokenKind::KwThrow),
            ("async", TokenKind::KwAsync),
            ("await", TokenKind::KwAwait),
            ("spawn", TokenKind::KwSpawn),
            ("import", TokenKind::KwImport),
            ("from", TokenKind::KwFrom),
            ("as", TokenKind::KwAs),
            ("true", TokenKind::KwTrue),
            ("false", TokenKind::KwFalse),
        ];
        for (s, kind) in keywords.iter() {
            keyword_cache.insert(s.to_string(), kind.clone());
        }

        Self {
            tokens,
            current: 0,
            eof_token: Token::new(TokenKind::Eof, Span::new(0, 0), 1, 1),
            keyword_cache,
        }
    }

    pub fn parse(mut self) -> ParseResult<Program> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            // Skip leading comments
            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.advance();
                continue;
            }
            
            let decl = self.parse_top_level_decl()?;
            if let Some(d) = decl {
                declarations.push(d);
            }
        }

        Ok(Program { declarations })
    }

    fn parse_top_level_decl(&mut self) -> ParseResult<Option<TopLevelDecl>> {
        match &self.peek().kind {
            TokenKind::KwImport => {
                let import = self.parse_import()?;
                Ok(Some(TopLevelDecl::Import(import)))
            }
            TokenKind::KwStruct => {
                let struct_def = self.parse_struct_def()?;
                Ok(Some(TopLevelDecl::Struct(struct_def)))
            }
            TokenKind::KwImpl => {
                let impl_def = self.parse_impl_def()?;
                Ok(Some(TopLevelDecl::Impl(impl_def)))
            }
            TokenKind::KwAsync => {
                let fn_def = self.parse_async_fn_def()?;
                Ok(Some(TopLevelDecl::AsyncFunction(fn_def)))
            }
            TokenKind::KwFn => {
                let fn_def = self.parse_fn_def()?;
                Ok(Some(TopLevelDecl::Function(fn_def)))
            }
            TokenKind::KwNew => {
                // At top level, treat as sequential statement (not global var)
                let var_decl = self.parse_var_decl()?;
                let block = Block {
                    statements: vec![Stmt::VarDecl(var_decl)],
                    span: Span::new(0, 0),
                };
                Ok(Some(TopLevelDecl::Statements(block)))
            }
            TokenKind::LBracket => {
                // Block at top level - treat as sequential statements
                // Skip optional block name (e.g., [main] or [Main])
                self.advance(); // consume '['
                if let TokenKind::Identifier(_) = &self.peek().kind {
                    self.advance(); // skip block name
                }
                let block = self.parse_block_body()?;
                Ok(Some(TopLevelDecl::Statements(block)))
            }
            TokenKind::Eof => Ok(None),
            _ => {
                // Try to parse as statement and collect all following statements
                let mut statements = Vec::new();
                
                // Parse the first statement
                let stmt = self.parse_statement()?;
                match stmt {
                    Stmt::Block(b) => statements.extend(b.statements),
                    _ => statements.push(stmt),
                };
                
                // Continue parsing statements until we hit a declaration or EOF
                while !self.is_at_end() {
                    match &self.peek().kind {
                        TokenKind::KwImport | TokenKind::KwStruct | TokenKind::KwImpl |
                        TokenKind::KwAsync | TokenKind::KwFn | TokenKind::KwNew |
                        TokenKind::LBracket | TokenKind::Eof => break,
                        TokenKind::Comment(_) => {
                            self.advance();
                        }
                        _ => {
                            let stmt = self.parse_statement()?;
                            match stmt {
                                Stmt::Block(b) => statements.extend(b.statements),
                                _ => statements.push(stmt),
                            };
                        }
                    }
                }
                
                if statements.is_empty() {
                    Ok(None)
                } else {
                    let block = Block {
                        statements,
                        span: Span::new(0, 0),
                    };
                    Ok(Some(TopLevelDecl::Statements(block)))
                }
            }
        }
    }

    fn parse_import(&mut self) -> ParseResult<ImportStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'import'

        if self.check(&TokenKind::KwFrom) {
            // from module_path import names
            self.advance(); // consume 'from'
            let module_path = self.parse_module_path()?;
            self.expect(&TokenKind::KwImport, "Expected 'import'")?;
            
            let mut names = Vec::new();
            names.push(self.parse_import_name()?);
            
            while self.check(&TokenKind::Comma) {
                self.advance();
                names.push(self.parse_import_name()?);
            }
            
            Ok(ImportStmt::From { module_path, names, span })
        } else {
            // import module_path as alias
            let module_path = self.parse_module_path()?;
            let mut alias = None;
            
            if self.check(&TokenKind::KwAs) {
                self.advance();
                if let TokenKind::Identifier(name) = &self.peek().kind {
                    alias = Some(name.clone());
                    self.advance();
                }
            }
            
            Ok(ImportStmt::Simple { module_path, alias, span })
        }
    }

    fn parse_module_path(&mut self) -> ParseResult<Vec<String>> {
        let mut path = Vec::new();
        
        if let TokenKind::Identifier(name) = &self.peek().kind {
            path.push(name.clone());
            self.advance();
        } else {
            return Err(self.error("Expected identifier in module path"));
        }
        
        while self.check(&TokenKind::Dot) {
            self.advance();
            if let TokenKind::Identifier(name) = &self.peek().kind {
                path.push(name.clone());
                self.advance();
            } else {
                return Err(self.error("Expected identifier after '.' in module path"));
            }
        }
        
        Ok(path)
    }

    fn parse_import_name(&mut self) -> ParseResult<(String, Option<String>)> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            n.clone()
        } else {
            return Err(self.error("Expected identifier in import list"));
        };
        self.advance();
        
        let mut alias = None;
        if self.check(&TokenKind::KwAs) {
            self.advance();
            if let TokenKind::Identifier(n) = &self.peek().kind {
                alias = Some(n.clone());
                self.advance();
            }
        }
        
        Ok((name, alias))
    }

    fn parse_struct_def(&mut self) -> ParseResult<StructDef> {
        let span = self.peek().span;
        self.advance(); // consume 'struct'
        
        let name = self.parse_identifier()?;
        let fields = self.parse_struct_fields()?;
        
        Ok(StructDef { name, fields, span })
    }

    fn parse_struct_fields(&mut self) -> ParseResult<Vec<StructField>> {
        let mut fields = Vec::new();
        
        if !self.check(&TokenKind::LBracket) {
            return Err(self.error("Expected '[' for struct body"));
        }
        self.advance();
        
        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            // Skip comments
            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.advance();
                continue;
            }
            
            let field = self.parse_struct_field()?;
            fields.push(field);
        }
        
        self.expect(&TokenKind::RBracket, "Expected ']' to close struct")?;
        Ok(fields)
    }

    fn parse_struct_field(&mut self) -> ParseResult<StructField> {
        let name = self.parse_identifier()?;
        self.expect(&TokenKind::Colon, "Expected ':' after field name")?;
        let field_type = self.parse_type_expr()?;
        
        let span = Span::new(name.len(), name.len());
        Ok(StructField { name, field_type, span })
    }

    fn parse_impl_def(&mut self) -> ParseResult<ImplDef> {
        let span = self.peek().span;
        self.advance(); // consume 'impl'
        
        let type_name = self.parse_identifier()?;
        let mut methods = Vec::new();
        
        if !self.check(&TokenKind::LBracket) {
            return Err(self.error("Expected '[' for impl body"));
        }
        self.advance();
        
        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.advance();
                continue;
            }
            
            if self.check(&TokenKind::KwFn) || self.check(&TokenKind::KwAsync) {
                let method = self.parse_fn_def()?;
                methods.push(method);
            } else {
                return Err(self.error("Expected function definition in impl block"));
            }
        }
        
        self.expect(&TokenKind::RBracket, "Expected ']' to close impl")?;
        Ok(ImplDef { type_name, methods, span })
    }

    fn parse_fn_def(&mut self) -> ParseResult<FunctionDef> {
        let span = self.peek().span;
        self.advance(); // consume 'fn'
        
        let name = self.parse_identifier()?;
        let params = self.parse_params()?;
        
        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        
        let body = self.parse_block()?;
        
        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
            is_async: false,
            span,
        })
    }

    fn parse_async_fn_def(&mut self) -> ParseResult<FunctionDef> {
        let span = self.peek().span;
        self.advance(); // consume 'async'
        self.expect(&TokenKind::KwFn, "Expected 'fn' after 'async'")?;
        
        let name = self.parse_identifier()?;
        let params = self.parse_params()?;
        
        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        
        let body = self.parse_block()?;
        
        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
            is_async: true,
            span,
        })
    }

    fn parse_params(&mut self) -> ParseResult<Vec<Param>> {
        let mut params = Vec::new();
        
        if !self.check(&TokenKind::LParen) {
            return Err(self.error("Expected '(' for parameters"));
        }
        self.advance();
        
        if self.check(&TokenKind::RParen) {
            self.advance();
            return Ok(params);
        }
        
        params.push(self.parse_param()?);
        
        while self.check(&TokenKind::Comma) {
            self.advance();
            if self.check(&TokenKind::RParen) {
                break;
            }
            params.push(self.parse_param()?);
        }
        
        self.expect(&TokenKind::RParen, "Expected ')' to close parameters")?;
        Ok(params)
    }

    fn parse_param(&mut self) -> ParseResult<Param> {
        let span = self.peek().span;
        let name = self.parse_identifier()?;
        self.expect(&TokenKind::Colon, "Expected ':' after parameter name")?;
        let param_type = self.parse_type_expr()?;
        
        Ok(Param { name, param_type, span })
    }

    fn parse_type_expr(&mut self) -> ParseResult<TypeExpr> {
        // Check for m+ prefix (memory cast)
        if self.check(&TokenKind::KwM) {
            self.advance();
            if self.check(&TokenKind::Plus) {
                self.advance();
            }
            let inner = self.parse_type_expr()?;
            return Ok(TypeExpr::MemoryCast(Box::new(inner)));
        }

        // Check for pointer type
        if self.check(&TokenKind::Star) {
            self.advance();
            let inner = self.parse_type_expr()?;
            return Ok(TypeExpr::Pointer(Box::new(inner)));
        }

        // Check for array type
        if self.check(&TokenKind::LBracket) {
            self.advance();
            let inner = self.parse_type_expr()?;

            if self.check(&TokenKind::Semicolon) {
                // Fixed size array: [Type; N]
                self.advance();
                let size = self.parse_int_literal()?;
                self.expect(&TokenKind::RBracket, "Expected ']' to close array type")?;
                return Ok(TypeExpr::Array(Box::new(inner), Some(size)));
            } else {
                // Dynamic array: [Type]
                self.expect(&TokenKind::RBracket, "Expected ']' to close array type")?;
                return Ok(TypeExpr::Array(Box::new(inner), None));
            }
        }

        // Check for arbitrary length types: ia, fa, sa
        if self.check(&TokenKind::KwIa) {
            self.advance();
            return Ok(TypeExpr::Ia);
        }
        if self.check(&TokenKind::KwFa) {
            self.advance();
            return Ok(TypeExpr::Fa);
        }
        if self.check(&TokenKind::KwSa) {
            self.advance();
            return Ok(TypeExpr::Sa);
        }
        if self.check(&TokenKind::KwTimer) {
            self.advance();
            return Ok(TypeExpr::Timer);
        }

        // Check for any-width types: a8, a16, a32, a64, aa
        if self.check(&TokenKind::KwA8) {
            self.advance();
            return Ok(TypeExpr::AnyWidth(8));
        }
        if self.check(&TokenKind::KwA16) {
            self.advance();
            return Ok(TypeExpr::AnyWidth(16));
        }
        if self.check(&TokenKind::KwA32) {
            self.advance();
            return Ok(TypeExpr::AnyWidth(32));
        }
        if self.check(&TokenKind::KwA64) {
            self.advance();
            return Ok(TypeExpr::AnyWidth(64));
        }
        if self.check(&TokenKind::KwAa) {
            self.advance();
            return Ok(TypeExpr::AnyWidthArbitrary);
        }

        // Base type or generic type
        let name = self.parse_identifier()?;

        // Check for Result or Future
        match name.as_str() {
            "Result" => Ok(TypeExpr::Result),
            "Future" => Ok(TypeExpr::Future),
            _ => {
                // Check for generic type parameters: Type<T1, T2, ...>
                if self.check(&TokenKind::Less) {
                    self.advance(); // consume '<'
                    let mut params = Vec::new();
                    
                    // Parse first type parameter
                    if !self.check(&TokenKind::Greater) {
                        params.push(self.parse_type_expr()?);
                        
                        // Parse remaining type parameters
                        while self.check(&TokenKind::Comma) {
                            self.advance();
                            if self.check(&TokenKind::Greater) {
                                break;
                            }
                            params.push(self.parse_type_expr()?);
                        }
                    }
                    
                    self.expect(&TokenKind::Greater, "Expected '>' to close generic type")?;
                    Ok(TypeExpr::Generic(Box::new(TypeExpr::Base(name)), params))
                } else {
                    Ok(TypeExpr::Base(name))
                }
            }
        }
    }

    fn parse_int_literal(&mut self) -> ParseResult<usize> {
        if let TokenKind::IntLiteral(s) = &self.peek().kind {
            let value: usize = s.parse().map_err(|_| self.error("Invalid integer"))?;
            self.advance();
            Ok(value)
        } else {
            Err(self.error("Expected integer literal"))
        }
    }

    fn parse_block(&mut self) -> ParseResult<Block> {
        let span = self.peek().span;

        if !self.check(&TokenKind::LBracket) {
            return Err(self.error("Expected '[' to start block"));
        }
        self.advance();

        let mut statements = Vec::new();

        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            // Skip comments
            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.advance();
                continue;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        self.expect(&TokenKind::RBracket, "Expected ']' to close block")?;
        Ok(Block { statements, span })
    }

    fn parse_block_body(&mut self) -> ParseResult<Block> {
        // Parse block body assuming '[' was already consumed
        let span = Span::new(0, 0);
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            // Skip comments
            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.advance();
                continue;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        self.expect(&TokenKind::RBracket, "Expected ']' to close block")?;
        Ok(Block { statements, span })
    }

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        match &self.peek().kind {
            TokenKind::KwNew => {
                let var_decl = self.parse_var_decl()?;
                Ok(Stmt::VarDecl(var_decl))
            }
            TokenKind::KwCpy => {
                let copy = self.parse_copy_stmt()?;
                Ok(Stmt::CopyStmt(copy))
            }
            TokenKind::KwUnsafe => {
                let unsafe_block = self.parse_unsafe_block()?;
                Ok(Stmt::UnsafeBlock(unsafe_block))
            }
            TokenKind::KwIf => {
                let if_stmt = self.parse_if_stmt()?;
                Ok(Stmt::If(if_stmt))
            }
            TokenKind::KwWhile => {
                let while_stmt = self.parse_while_stmt()?;
                Ok(Stmt::While(while_stmt))
            }
            TokenKind::KwFor => {
                let for_stmt = self.parse_for_stmt()?;
                Ok(Stmt::For(for_stmt))
            }
            TokenKind::KwSwitch => {
                let switch_stmt = self.parse_switch_stmt()?;
                Ok(Stmt::Switch(switch_stmt))
            }
            TokenKind::KwTry => {
                let try_stmt = self.parse_try_stmt()?;
                Ok(Stmt::Try(try_stmt))
            }
            TokenKind::KwThrow => {
                let throw_stmt = self.parse_throw_stmt()?;
                Ok(Stmt::Throw(throw_stmt))
            }
            TokenKind::KwAsync => {
                let async_stmt = self.parse_async_stmt()?;
                Ok(Stmt::AsyncStmt(async_stmt))
            }
            TokenKind::KwReturn => {
                let return_stmt = self.parse_return_stmt()?;
                Ok(Stmt::Return(return_stmt))
            }
            TokenKind::LBracket => {
                let block = self.parse_block()?;
                Ok(Stmt::Block(block))
            }
            TokenKind::KwAwait => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Stmt::AsyncStmt(AsyncStmt::Await(expr)))
            }
            TokenKind::KwSpawn => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Stmt::AsyncStmt(AsyncStmt::Spawn(expr)))
            }
            _ => {
                // Try assignment or expression
                if self.is_assignment() {
                    let assignment = self.parse_assignment()?;
                    Ok(Stmt::Assignment(assignment))
                } else {
                    let expr = self.parse_expression()?;
                    Ok(Stmt::Expr(expr))
                }
            }
        }
    }

    fn is_assignment(&mut self) -> bool {
        // Check for simple identifier assignment: ident = ...
        // Or field access: ident.field = ...
        let current = self.peek();
        let next = self.peek_next();
        
        if !matches!(&current.kind, TokenKind::Identifier(_)) {
            return false;
        }
        
        if matches!(
            &next.kind,
            TokenKind::Equal
                | TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
                | TokenKind::AndEqual
                | TokenKind::OrEqual
                | TokenKind::XorEqual
        ) {
            return true;
        }
        
        // Check for field access assignment: ident.field = ...
        if matches!(&next.kind, TokenKind::Dot) {
            if let Some(after_dot) = self.tokens.get(self.current + 2) {
                if matches!(&after_dot.kind, TokenKind::Identifier(_)) {
                    if let Some(after_field) = self.tokens.get(self.current + 3) {
                        if matches!(
                            &after_field.kind,
                            TokenKind::Equal
                                | TokenKind::PlusEqual
                                | TokenKind::MinusEqual
                                | TokenKind::StarEqual
                                | TokenKind::SlashEqual
                                | TokenKind::PercentEqual
                                | TokenKind::AndEqual
                                | TokenKind::OrEqual
                                | TokenKind::XorEqual
                        ) {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }

    fn parse_var_decl(&mut self) -> ParseResult<VarDecl> {
        let span = self.peek().span;
        self.advance(); // consume 'new'

        let name = self.parse_identifier()?;

        // Check for type expression
        // Supports: new name<type> or new name type
        let var_type = if self.check(&TokenKind::Less) {
            // <type> syntax
            self.advance(); // consume '<'
            let ty = self.parse_type_expr()?;
            self.expect(&TokenKind::Greater, "Expected '>' to close type")?;
            ty
        } else if self.check(&TokenKind::Equal) {
            // Type inference - use a placeholder type
            TypeExpr::Base("auto".to_string())
        } else {
            self.parse_type_expr()?
        };

        let init = if self.check(&TokenKind::Equal) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(VarDecl { name, var_type, init, span })
    }

    fn parse_copy_stmt(&mut self) -> ParseResult<CopyStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'cpy'
        
        let dest = self.parse_identifier()?;
        let size_type = self.parse_type_expr()?;
        
        self.expect(&TokenKind::LParen, "Expected '(' in cpy statement")?;
        // The expression inside is optional for now
        if !self.check(&TokenKind::RParen) {
            let _expr = self.parse_expression()?;
        }
        self.expect(&TokenKind::RParen, "Expected ')' in cpy statement")?;
        
        Ok(CopyStmt { dest, size_type, span })
    }

    fn parse_unsafe_block(&mut self) -> ParseResult<UnsafeBlock> {
        let span = self.peek().span;
        self.advance(); // consume 'unsafe'
        
        let scope_modifier = self.parse_scope_modifier()?;
        let body = self.parse_block()?;
        
        Ok(UnsafeBlock { scope_modifier, body, span })
    }

    fn parse_scope_modifier(&mut self) -> ParseResult<ScopeModifier> {
        if self.check(&TokenKind::KwAll) {
            self.advance();
            return Ok(ScopeModifier::All);
        }
        
        if self.check(&TokenKind::KwClose) {
            self.advance();
            let target = self.parse_safety_target()?;
            return Ok(ScopeModifier::Close(target));
        }
        
        if self.check(&TokenKind::KwKeep) {
            self.advance();
            let target = self.parse_safety_target()?;
            return Ok(ScopeModifier::Keep(target));
        }
        
        Err(self.error("Expected scope modifier: 'all', 'close(...)', or 'keep(...)'"))
    }

    fn parse_safety_target(&mut self) -> ParseResult<SafetyTarget> {
        self.expect(&TokenKind::LParen, "Expected '(' after scope modifier")?;
        
        let target = if self.check(&TokenKind::Identifier("init".to_string())) {
            SafetyTarget::Init
        } else if self.check(&TokenKind::Identifier("bounds".to_string())) {
            SafetyTarget::Bounds
        } else if self.check(&TokenKind::Identifier("lifetime".to_string())) {
            SafetyTarget::Lifetime
        } else if self.check(&TokenKind::Identifier("null".to_string())) {
            SafetyTarget::Null
        } else if let TokenKind::Identifier(name) = &self.peek().kind {
            SafetyTarget::Identifier(name.clone())
        } else {
            return Err(self.error("Invalid safety target"));
        };
        
        self.advance();
        self.expect(&TokenKind::RParen, "Expected ')' after safety target")?;
        Ok(target)
    }

    fn parse_if_stmt(&mut self) -> ParseResult<IfStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'if'
        
        self.expect(&TokenKind::LParen, "Expected '(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RParen, "Expected ')' after condition")?;
        
        let then_block = self.parse_block()?;
        
        let else_branch = if self.check(&TokenKind::KwElse) {
            self.advance();
            if self.check(&TokenKind::KwIf) {
                let else_if = self.parse_if_stmt()?;
                Some(ElseBranch::ElseIf(Box::new(else_if)))
            } else {
                let else_block = self.parse_block()?;
                Some(ElseBranch::Block(else_block))
            }
        } else {
            None
        };
        
        Ok(IfStmt { condition, then_block, else_branch, span })
    }

    fn parse_while_stmt(&mut self) -> ParseResult<WhileStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'while'
        
        self.expect(&TokenKind::LParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RParen, "Expected ')' after condition")?;
        
        let body = self.parse_block()?;
        
        Ok(WhileStmt { condition, body, span })
    }

    fn parse_for_stmt(&mut self) -> ParseResult<ForStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'for'
        
        self.expect(&TokenKind::LParen, "Expected '(' after 'for'")?;
        
        let init = self.parse_var_decl()?;
        self.expect(&TokenKind::Comma, "Expected ',' after init")?;
        
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::Comma, "Expected ',' after condition")?;
        
        let update = self.parse_assignment()?;
        self.expect(&TokenKind::RParen, "Expected ')' after update")?;
        
        let body = self.parse_block()?;
        
        Ok(ForStmt { init, condition, update, body, span })
    }

    fn parse_switch_stmt(&mut self) -> ParseResult<SwitchStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'switch'

        // Parse the switch expression (primary expression only, not full expression with postfix)
        // to avoid confusion with array indexing
        let expr = self.parse_primary_expr()?;

        self.expect(&TokenKind::LBracket, "Expected '[' for switch cases")?;

        let mut cases = Vec::new();
        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.advance();
                continue;
            }
            cases.push(self.parse_case_clause()?);
        }

        self.expect(&TokenKind::RBracket, "Expected ']' to close switch")?;

        Ok(SwitchStmt { expr, cases, span })
    }

    fn parse_case_clause(&mut self) -> ParseResult<CaseClause> {
        let span = self.peek().span;

        // Check for default case
        if self.check(&TokenKind::KwDefault) {
            self.advance();
            self.expect(&TokenKind::LBracket, "Expected '[' for default block")?;
            let mut statements = Vec::new();
            while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
                if matches!(self.peek().kind, TokenKind::Comment(_)) {
                    self.advance();
                    continue;
                }
                statements.push(self.parse_statement()?);
            }
            self.expect(&TokenKind::RBracket, "Expected ']' to close default block")?;
            
            Ok(CaseClause {
                pattern: Pattern::Underscore,
                body: Block { statements, span },
                span,
            })
        } else {
            // Parse pattern (literal or identifier)
            let pattern = self.parse_pattern()?;
            self.expect(&TokenKind::LBracket, "Expected '[' for case block")?;
            let mut statements = Vec::new();
            while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
                if matches!(self.peek().kind, TokenKind::Comment(_)) {
                    self.advance();
                    continue;
                }
                statements.push(self.parse_statement()?);
            }
            self.expect(&TokenKind::RBracket, "Expected ']' to close case block")?;
            
            Ok(CaseClause { pattern, body: Block { statements, span }, span })
        }
    }

    fn parse_pattern(&mut self) -> ParseResult<Pattern> {
        match &self.peek().kind {
            TokenKind::IntLiteral(_)
            | TokenKind::FloatLiteral(_)
            | TokenKind::StringLiteral(_) => {
                let lit = self.parse_literal()?;
                Ok(Pattern::Literal(lit))
            }
            TokenKind::KwTrue | TokenKind::KwFalse => {
                let lit = self.parse_literal()?;
                Ok(Pattern::Literal(lit))
            }
            TokenKind::Identifier(_) => {
                let name = self.parse_identifier()?;
                Ok(Pattern::Identifier(name))
            }
            _ => Err(self.error("Invalid pattern")),
        }
    }

    fn parse_try_stmt(&mut self) -> ParseResult<TryStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'try'
        
        let try_block = self.parse_block()?;
        let mut catch_clauses = Vec::new();
        
        while self.check(&TokenKind::KwCatch) {
            catch_clauses.push(self.parse_catch_clause()?);
        }
        
        Ok(TryStmt { try_block, catch_clauses, span })
    }

    fn parse_catch_clause(&mut self) -> ParseResult<CatchClause> {
        let span = self.peek().span;
        self.advance(); // consume 'catch'
        
        self.expect(&TokenKind::LParen, "Expected '(' after 'catch'")?;
        let var_name = self.parse_identifier()?;
        let error_type = self.parse_type_expr()?;
        self.expect(&TokenKind::RParen, "Expected ')' after error type")?;
        
        let body = self.parse_block()?;
        
        Ok(CatchClause { var_name, error_type, body, span })
    }

    fn parse_throw_stmt(&mut self) -> ParseResult<ThrowStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'throw'
        
        let expr = self.parse_expression()?;
        Ok(ThrowStmt { expr, span })
    }

    fn parse_async_stmt(&mut self) -> ParseResult<AsyncStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'async'
        
        if self.check(&TokenKind::LBracket) {
            let block = self.parse_block()?;
            Ok(AsyncStmt::Spawn(Expr {
                kind: ExprKind::Block(block),
                span,
            }))
        } else if self.check(&TokenKind::KwFn) {
            // This should be handled at top level
            Err(self.error("async fn should be at top level"))
        } else {
            Err(self.error("Expected block or fn after 'async'"))
        }
    }

    fn parse_return_stmt(&mut self) -> ParseResult<ReturnStmt> {
        let span = self.peek().span;
        self.advance(); // consume 'return'
        
        let expr = if !self.check(&TokenKind::RBracket)
            && !self.check(&TokenKind::Eof)
            && !self.check(&TokenKind::Semicolon)
        {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(ReturnStmt { expr, span })
    }

    fn parse_assignment(&mut self) -> ParseResult<Assignment> {
        let span = self.peek().span;
        
        // Parse target: could be identifier or field access
        let target = if self.is_field_access_assignment() {
            let obj = self.parse_identifier()?;
            self.expect(&TokenKind::Dot, "Expected '.' for field access")?;
            let field = self.parse_identifier()?;
            AssignmentTarget::FieldAccess(obj, field)
        } else {
            let ident = self.parse_identifier()?;
            AssignmentTarget::Identifier(ident)
        };

        let op = match &self.peek().kind {
            TokenKind::Equal => AssignOp::Equal,
            TokenKind::PlusEqual => AssignOp::PlusEqual,
            TokenKind::MinusEqual => AssignOp::MinusEqual,
            TokenKind::StarEqual => AssignOp::StarEqual,
            TokenKind::SlashEqual => AssignOp::SlashEqual,
            TokenKind::PercentEqual => AssignOp::PercentEqual,
            TokenKind::AndEqual => AssignOp::AndEqual,
            TokenKind::OrEqual => AssignOp::OrEqual,
            TokenKind::XorEqual => AssignOp::XorEqual,
            _ => return Err(self.error("Expected assignment operator")),
        };
        self.advance();

        let value = self.parse_expression()?;

        Ok(Assignment { target, op, value, span })
    }

    fn is_field_access_assignment(&mut self) -> bool {
        // Check if this is ident.field = ...
        if let TokenKind::Identifier(_) = &self.peek().kind {
            if let Some(TokenKind::Dot) = self.tokens.get(self.current + 1).map(|t| &t.kind) {
                if let Some(TokenKind::Identifier(_)) = self.tokens.get(self.current + 2).map(|t| &t.kind) {
                    if let Some(op) = self.tokens.get(self.current + 3).map(|t| &t.kind) {
                        return matches!(
                            op,
                            TokenKind::Equal
                                | TokenKind::PlusEqual
                                | TokenKind::MinusEqual
                                | TokenKind::StarEqual
                                | TokenKind::SlashEqual
                                | TokenKind::PercentEqual
                                | TokenKind::AndEqual
                                | TokenKind::OrEqual
                                | TokenKind::XorEqual
                        );
                    }
                }
            }
        }
        false
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_and_expr()?;
        
        while self.check(&TokenKind::OrOr) {
            let op_span = self.peek().span;
            self.advance();
            let right = self.parse_and_expr()?;
            left = Expr {
                kind: ExprKind::Binary(Box::new(left), BinaryOp::Or, Box::new(right)),
                span: op_span,
            };
        }
        
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_equality_expr()?;
        
        while self.check(&TokenKind::AndAnd) {
            let op_span = self.peek().span;
            self.advance();
            let right = self.parse_equality_expr()?;
            left = Expr {
                kind: ExprKind::Binary(Box::new(left), BinaryOp::And, Box::new(right)),
                span: op_span,
            };
        }
        
        Ok(left)
    }

    fn parse_equality_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_comparison_expr()?;
        
        loop {
            let op = if self.check(&TokenKind::EqualEqual) {
                Some(BinaryOp::Eq)
            } else if self.check(&TokenKind::NotEqual) {
                Some(BinaryOp::Ne)
            } else {
                None
            };
            
            if let Some(op) = op {
                let op_span = self.peek().span;
                self.advance();
                let right = self.parse_comparison_expr()?;
                left = Expr {
                    kind: ExprKind::Binary(Box::new(left), op, Box::new(right)),
                    span: op_span,
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }

    fn parse_comparison_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_term_expr()?;
        
        loop {
            let op = if self.check(&TokenKind::Greater) {
                Some(BinaryOp::Gt)
            } else if self.check(&TokenKind::Less) {
                Some(BinaryOp::Lt)
            } else if self.check(&TokenKind::GreaterEqual) {
                Some(BinaryOp::Ge)
            } else if self.check(&TokenKind::LessEqual) {
                Some(BinaryOp::Le)
            } else {
                None
            };
            
            if let Some(op) = op {
                let op_span = self.peek().span;
                self.advance();
                let right = self.parse_term_expr()?;
                left = Expr {
                    kind: ExprKind::Binary(Box::new(left), op, Box::new(right)),
                    span: op_span,
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }

    fn parse_term_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_factor_expr()?;
        
        loop {
            let op = if self.check(&TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.check(&TokenKind::Minus) {
                Some(BinaryOp::Sub)
            } else {
                None
            };
            
            if let Some(op) = op {
                let op_span = self.peek().span;
                self.advance();
                let right = self.parse_factor_expr()?;
                left = Expr {
                    kind: ExprKind::Binary(Box::new(left), op, Box::new(right)),
                    span: op_span,
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }

    fn parse_factor_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_unary_expr()?;
        
        loop {
            let op = if self.check(&TokenKind::Star) {
                Some(BinaryOp::Mul)
            } else if self.check(&TokenKind::Slash) {
                Some(BinaryOp::Div)
            } else if self.check(&TokenKind::Percent) {
                Some(BinaryOp::Rem)
            } else {
                None
            };
            
            if let Some(op) = op {
                let op_span = self.peek().span;
                self.advance();
                let right = self.parse_unary_expr()?;
                left = Expr {
                    kind: ExprKind::Binary(Box::new(left), op, Box::new(right)),
                    span: op_span,
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> ParseResult<Expr> {
        let span = self.peek().span;
        
        if self.check(&TokenKind::Minus) {
            self.advance();
            let expr = self.parse_unary_expr()?;
            return Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::Neg, Box::new(expr)),
                span,
            });
        }
        
        if self.check(&TokenKind::Star) {
            self.advance();
            let expr = self.parse_unary_expr()?;
            return Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::Deref, Box::new(expr)),
                span,
            });
        }
        
        self.parse_postfix_expr()
    }

    fn parse_postfix_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_primary_expr()?;
        
        loop {
            if self.check(&TokenKind::LParen) {
                // Function call
                self.advance();
                let args = self.parse_arg_list()?;
                let span = expr.span;
                expr = Expr {
                    kind: ExprKind::Call(Box::new(expr.clone()), args),
                    span,
                };
            } else if self.check(&TokenKind::Dot) {
                // Field access
                self.advance();
                let field = self.parse_identifier()?;
                let span = expr.span;
                expr = Expr {
                    kind: ExprKind::FieldAccess(Box::new(expr.clone()), field),
                    span,
                };
            } else if self.check(&TokenKind::LBracket) {
                // Array index
                self.advance();
                let index = self.parse_expression()?;
                self.expect(&TokenKind::RBracket, "Expected ']' after index")?;
                let span = expr.span;
                expr = Expr {
                    kind: ExprKind::ArrayIndex(Box::new(expr.clone()), Box::new(index)),
                    span,
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn parse_arg_list(&mut self) -> ParseResult<Vec<Expr>> {
        let mut args = Vec::new();
        
        if self.check(&TokenKind::RParen) {
            self.advance();
            return Ok(args);
        }
        
        args.push(self.parse_expression()?);
        
        while self.check(&TokenKind::Comma) {
            self.advance();
            if self.check(&TokenKind::RParen) {
                break;
            }
            args.push(self.parse_expression()?);
        }
        
        self.expect(&TokenKind::RParen, "Expected ')' to close arguments")?;
        Ok(args)
    }

    fn parse_primary_expr(&mut self) -> ParseResult<Expr> {
        let span = self.peek().span;
        
        let kind = match &self.peek().kind {
            TokenKind::IntLiteral(_)
            | TokenKind::FloatLiteral(_)
            | TokenKind::StringLiteral(_)
            | TokenKind::BoolLiteral(_) => {
                let lit = self.parse_literal()?;
                ExprKind::Literal(lit)
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                
                // Check for m+Type memory cast
                if name == "m" && self.check(&TokenKind::Plus) {
                    self.advance();
                    let cast_type = self.parse_type_expr()?;
                    let inner = self.parse_primary_expr()?;
                    ExprKind::MemoryCast(cast_type, Box::new(inner))
                } else {
                    ExprKind::Identifier(name)
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RParen, "Expected ')' to close expression")?;
                return Ok(expr);
            }
            TokenKind::LBracket => {
                let block = self.parse_block()?;
                ExprKind::Block(block)
            }
            TokenKind::KwAsync => {
                self.advance();
                let block = self.parse_block()?;
                ExprKind::AsyncBlock(block)
            }
            TokenKind::KwTry => {
                self.advance();
                let block = self.parse_block()?;
                ExprKind::TryBlock(block)
            }
            _ => {
                return Err(self.error("Expected expression"));
            }
        };
        
        Ok(Expr { kind, span })
    }

    fn parse_literal(&mut self) -> ParseResult<Literal> {
        match &self.peek().kind {
            TokenKind::IntLiteral(s) => {
                let s = s.clone();
                self.advance();
                Ok(Literal::Int(s))
            }
            TokenKind::FloatLiteral(s) => {
                let s = s.clone();
                self.advance();
                Ok(Literal::Float(s))
            }
            TokenKind::StringLiteral(s) => {
                let s = s.clone();
                self.advance();
                if s.starts_with("i:") {
                    Ok(Literal::InterpolatedString(s[2..].to_string()))
                } else {
                    Ok(Literal::String(s))
                }
            }
            TokenKind::BoolLiteral(b) => {
                let b = *b;
                self.advance();
                Ok(Literal::Bool(b))
            }
            _ => Err(self.error("Expected literal")),
        }
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        if let TokenKind::Identifier(name) = &self.peek().kind {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(self.error("Expected identifier"))
        }
    }

    // Helper methods
    fn check(&self, kind: &TokenKind) -> bool {
        matches!(&self.peek().kind, k if std::mem::discriminant(k) == std::mem::discriminant(kind))
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&self.eof_token)
    }

    fn peek_next(&self) -> &Token {
        self.tokens.get(self.current + 1).unwrap_or(&self.eof_token)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn expect(&mut self, kind: &TokenKind, message: &str) -> ParseResult<()> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> ParseError {
        let token = self.peek();
        ParseError {
            message: message.to_string(),
            span: token.span,
            line: token.line,
            column: token.column,
        }
    }
}

/// Public function to parse tokenized input
pub fn parse(tokens: Vec<Token>) -> ParseResult<Program> {
    Parser::new(tokens).parse()
}

/// Public function to parse source code directly
pub fn parse_source(source: &str) -> Result<Program, ParseError> {
    let tokens = tokenize(source).map_err(|e| ParseError {
        message: e.to_string(),
        span: Span::new(0, 0),
        line: 1,
        column: 1,
    })?;
    parse(tokens)
}
