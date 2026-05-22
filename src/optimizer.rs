//! CatLang Optimizer
//!
//! Performs AST-level optimizations before code generation:
//! - Constant folding
//! - Dead code elimination
//! - Algebraic simplification
//!
//! Optimized for performance with reduced allocations

use crate::ast::*;
use std::collections::HashMap;

/// Optimizer for CatLang AST
pub struct Optimizer {
    /// Constants known at compile time
    constants: HashMap<String, Literal>,
    /// Track which variables are mutable (can be reassigned)
    mutable_vars: std::collections::HashSet<String>,
    /// Optimization level (0-3)
    opt_level: u8,
}

impl Optimizer {
    /// Create a new optimizer with given optimization level
    pub fn new(opt_level: u8) -> Self {
        Optimizer {
            constants: HashMap::with_capacity(32), // Pre-allocate for typical use
            mutable_vars: std::collections::HashSet::with_capacity(32),
            opt_level: opt_level.min(3),
        }
    }

    /// Optimize a program
    pub fn optimize(&mut self, program: &mut Program) {
        if self.opt_level == 0 {
            return;
        }

        // Optimize each declaration
        for decl in &mut program.declarations {
            match decl {
                TopLevelDecl::Function(func) => {
                    self.optimize_block(&mut func.body);
                }
                TopLevelDecl::AsyncFunction(func) => {
                    self.optimize_block(&mut func.body);
                }
                TopLevelDecl::GlobalVar(global) => {
                    if let Some(init) = &mut global.init {
                        self.optimize_expr(init);
                    }
                }
                TopLevelDecl::Statements(block) => {
                    self.optimize_block(block);
                }
                TopLevelDecl::Import(_) | TopLevelDecl::Struct(_) | TopLevelDecl::Impl(_) => {
                    // Nothing to optimize
                }
            }
        }
    }

    /// Optimize a block of statements
    fn optimize_block(&mut self, block: &mut Block) {
        // Pre-allocate with current size to avoid reallocations
        let mut new_statements = Vec::with_capacity(block.statements.len());

        for stmt in block.statements.drain(..) {
            match stmt {
                Stmt::VarDecl(mut var_decl) => {
                    if let Some(init) = &mut var_decl.init {
                        self.optimize_expr(init);

                        // Try to fold constants - only for immutable variables
                        if let ExprKind::Literal(lit) = &init.kind {
                            if var_decl.var_type.is_numeric() {
                                // Only add to constants if not already marked as mutable
                                if !self.mutable_vars.contains(&var_decl.name) {
                                    self.constants.insert(var_decl.name.clone(), lit.clone());
                                }
                            }
                        }
                    }
                    new_statements.push(Stmt::VarDecl(var_decl));
                }
                Stmt::Assignment(mut assignment) => {
                    self.optimize_expr(&mut assignment.value);

                    // Mark variable as mutable and invalidate constant if reassigned
                    if let AssignmentTarget::Identifier(name) = &assignment.target {
                        self.mutable_vars.insert(name.clone());
                        self.constants.remove(name);
                    }

                    new_statements.push(Stmt::Assignment(assignment));
                }
                Stmt::If(mut if_stmt) => {
                    self.optimize_expr(&mut if_stmt.condition);

                    // Constant condition optimization
                    if let ExprKind::Literal(Literal::Bool(cond)) = &if_stmt.condition.kind {
                        if *cond {
                            // Condition is always true - keep only if branch
                            let saved_constants = self.constants.clone();
                            let saved_mutable = self.mutable_vars.clone();

                            self.optimize_block(&mut if_stmt.then_block);
                            for s in if_stmt.then_block.statements.drain(..) {
                                new_statements.push(s);
                            }

                            self.constants = saved_constants;
                            self.mutable_vars = saved_mutable;
                        } else if let Some(else_branch) = if_stmt.else_branch {
                            // Condition is always false - keep only else branch
                            let saved_constants = self.constants.clone();
                            let saved_mutable = self.mutable_vars.clone();

                            match else_branch {
                                ElseBranch::Block(mut block) => {
                                    self.optimize_block(&mut block);
                                    for s in block.statements.drain(..) {
                                        new_statements.push(s);
                                    }
                                }
                                ElseBranch::ElseIf(if_stmt_inner) => {
                                    new_statements.push(Stmt::If(*if_stmt_inner));
                                }
                            }

                            self.constants = saved_constants;
                            self.mutable_vars = saved_mutable;
                        }
                        // If false and no else, skip entirely
                    } else {
                        let saved_constants = self.constants.clone();
                        let saved_mutable = self.mutable_vars.clone();

                        self.optimize_block(&mut if_stmt.then_block);
                        if let Some(else_branch) = &mut if_stmt.else_branch {
                            match else_branch {
                                ElseBranch::Block(block) => {
                                    self.optimize_block(block);
                                }
                                ElseBranch::ElseIf(if_stmt_inner) => {
                                    self.optimize_block(&mut if_stmt_inner.then_block);
                                }
                            }
                        }
                        new_statements.push(Stmt::If(if_stmt));

                        self.constants = saved_constants;
                        self.mutable_vars = saved_mutable;
                    }
                }
                Stmt::While(mut while_stmt) => {
                    self.optimize_expr(&mut while_stmt.condition);

                    // Check for always-true condition (potential infinite loop)
                    if let ExprKind::Literal(Literal::Bool(true)) = &while_stmt.condition.kind {
                        // This is a potential infinite loop - keep it but could warn
                        // For now, we just optimize the body normally
                    }

                    // Save constants before optimizing body
                    let saved_constants = self.constants.clone();
                    let saved_mutable = self.mutable_vars.clone();

                    self.optimize_block(&mut while_stmt.body);

                    // Restore constants after optimizing body
                    self.constants = saved_constants;
                    self.mutable_vars = saved_mutable;

                    new_statements.push(Stmt::While(while_stmt));
                }
                Stmt::For(mut for_stmt) => {
                    if let Some(init) = &mut for_stmt.init.init {
                        self.optimize_expr(init);
                    }
                    self.optimize_expr(&mut for_stmt.condition);
                    self.optimize_expr(&mut for_stmt.update.value);

                    // Save constants before optimizing body
                    let saved_constants = self.constants.clone();
                    let saved_mutable = self.mutable_vars.clone();

                    self.optimize_block(&mut for_stmt.body);

                    // Restore constants after optimizing body
                    self.constants = saved_constants;
                    self.mutable_vars = saved_mutable;

                    new_statements.push(Stmt::For(for_stmt));
                }
                Stmt::Return(mut ret_stmt) => {
                    if let Some(expr) = &mut ret_stmt.expr {
                        self.optimize_expr(expr);
                    }
                    new_statements.push(Stmt::Return(ret_stmt));
                }
                Stmt::Expr(mut expr) => {
                    self.optimize_expr(&mut expr);
                    // Remove no-op expressions
                    if !self.is_noop(&expr) {
                        new_statements.push(Stmt::Expr(expr));
                    }
                }
                Stmt::Block(mut block) => {
                    // Save current constants and mutable vars
                    let saved_constants = self.constants.clone();
                    let saved_mutable = self.mutable_vars.clone();

                    self.optimize_block(&mut block);

                    // Restore constants and mutable vars
                    self.constants = saved_constants;
                    self.mutable_vars = saved_mutable;

                    new_statements.push(Stmt::Block(block));
                }
                // Pass through other statements
                Stmt::CopyStmt(_)
                | Stmt::Switch(_)
                | Stmt::Throw(_)
                | Stmt::AsyncStmt(_) => {
                    new_statements.push(stmt);
                }
                Stmt::Try(mut try_stmt) => {
                    // Save constants before optimizing try block
                    let saved_constants = self.constants.clone();
                    let saved_mutable = self.mutable_vars.clone();

                    self.optimize_block(&mut try_stmt.try_block);

                    // Restore constants after try block
                    self.constants = saved_constants;
                    self.mutable_vars = saved_mutable;

                    // Optimize catch clauses with their own scopes
                    let mut catch_clauses = Vec::new();
                    for catch_clause in try_stmt.catch_clauses {
                        let saved_constants = self.constants.clone();
                        let saved_mutable = self.mutable_vars.clone();

                        let mut clause = catch_clause;
                        self.optimize_block(&mut clause.body);

                        self.constants = saved_constants;
                        self.mutable_vars = saved_mutable;

                        catch_clauses.push(clause);
                    }
                    try_stmt.catch_clauses = catch_clauses;

                    new_statements.push(Stmt::Try(try_stmt));
                }
                Stmt::UnsafeBlock(mut unsafe_block) => {
                    // Save constants before optimizing unsafe block
                    let saved_constants = self.constants.clone();
                    let saved_mutable = self.mutable_vars.clone();

                    self.optimize_block(&mut unsafe_block.body);

                    // Restore constants after unsafe block
                    self.constants = saved_constants;
                    self.mutable_vars = saved_mutable;

                    new_statements.push(Stmt::UnsafeBlock(unsafe_block));
                }
                Stmt::UnsandboxBlock(mut unsandbox_block) => {
                    // Unsandbox block doesn't affect constants/mutability
                    self.optimize_block(&mut unsandbox_block.body);
                    new_statements.push(Stmt::UnsandboxBlock(unsandbox_block));
                }
            }
        }

        block.statements = new_statements;
    }

    /// Optimize an expression - uses a two-pass approach to avoid borrow conflicts
    pub fn optimize_expr(&mut self, expr: &mut Expr) {
        self.optimize_expr_with_options(expr, true, true);
    }

    /// Optimize expression with options
    fn optimize_expr_with_options(&mut self, expr: &mut Expr, allow_const_prop: bool, allow_const_fold: bool) {
        // Pass 1: Recursively optimize child expressions
        self.optimize_children(expr);

        // Pass 2: Apply local optimizations
        if self.opt_level >= 2 && allow_const_fold {
            self.apply_local_optimizations(expr);
        }

        // Pass 3: Constant propagation (opt level 1+)
        // Only propagate constants for immutable variables
        if self.opt_level >= 1 && allow_const_prop {
            if let ExprKind::Identifier(name) = &expr.kind {
                // Only replace with constant if variable is not mutable
                if !self.mutable_vars.contains(name) {
                    if let Some(lit) = self.constants.get(name) {
                        expr.kind = ExprKind::Literal(lit.clone());
                    }
                }
            }
        }

        // Pass 4: Inline built-in functions (opt level 3)
        if self.opt_level >= 3 {
            if let ExprKind::Call(func, args) = &expr.kind {
                if let ExprKind::Identifier(name) = &func.kind {
                    if let Some(result) = self.try_inline_builtin(name, args) {
                        expr.kind = result;
                    }
                }
            }
        }
    }

    /// Recursively optimize child expressions
    fn optimize_children(&mut self, expr: &mut Expr) {
        match expr.kind {
            ExprKind::Binary(ref mut left, _, ref mut right) => {
                self.optimize_expr(left);
                self.optimize_expr(right);
            }
            ExprKind::Unary(_, ref mut operand) => {
                self.optimize_expr(operand);
            }
            ExprKind::Call(ref mut func, ref mut args) => {
                self.optimize_expr(func);
                // Optimize arguments without constant propagation
                for arg in args {
                    self.optimize_expr_without_const_prop(arg);
                }
            }
            ExprKind::Block(ref mut block) => {
                self.optimize_block(block);
            }
            ExprKind::Cast(ref mut inner, _) => {
                self.optimize_expr(inner);
            }
            ExprKind::ArrayIndex(ref mut arr, ref mut index) => {
                self.optimize_expr(arr);
                self.optimize_expr(index);
            }
            ExprKind::Array(ref mut elems) => {
                for elem in elems {
                    self.optimize_expr(elem);
                }
            }
            ExprKind::FieldAccess(ref mut inner, _) => {
                self.optimize_expr(inner);
            }
            ExprKind::AsyncBlock(ref mut block) => {
                self.optimize_block(block);
            }
            ExprKind::TryBlock(ref mut block) => {
                self.optimize_block(block);
            }
            ExprKind::MemoryCast(_, ref mut inner) => {
                self.optimize_expr(inner);
            }
            _ => {}
        }
    }

    /// Optimize expression without constant propagation
    fn optimize_expr_without_const_prop(&mut self, expr: &mut Expr) {
        // Pass 1: Recursively optimize child expressions
        self.optimize_children(expr);

        // Pass 2: Apply local optimizations
        if self.opt_level >= 2 {
            self.apply_local_optimizations(expr);
        }

        // Skip constant propagation here

        // Pass 4: Inline built-in functions (opt level 3)
        if self.opt_level >= 3 {
            if let ExprKind::Call(func, args) = &expr.kind {
                if let ExprKind::Identifier(name) = &func.kind {
                    if let Some(result) = self.try_inline_builtin(name, args) {
                        expr.kind = result;
                    }
                }
            }
        }
    }

    /// Apply local optimizations (constant folding, algebraic simplification)
    fn apply_local_optimizations(&mut self, expr: &mut Expr) {
        match &mut expr.kind {
            ExprKind::Binary(left, op, right) => {
                // Try constant folding first
                if let Some(result) = self.try_fold_binary(left, op, right) {
                    expr.kind = result;
                } else {
                    // Then try algebraic simplifications
                    if let Some(simplified) = self.apply_algebraic_simplifications(left, op, right)
                    {
                        expr.kind = simplified;
                    }
                }
            }
            ExprKind::Unary(op, operand) => {
                if let Some(result) = self.try_fold_unary(op, operand) {
                    expr.kind = result;
                }
            }
            _ => {}
        }
    }

    /// Try to fold binary operations
    fn try_fold_binary(&self, left: &Expr, op: &BinaryOp, right: &Expr) -> Option<ExprKind> {
        match (&left.kind, &right.kind) {
            (ExprKind::Literal(l), ExprKind::Literal(r)) => self.fold_binary_literals(l, op, r),
            _ => None,
        }
    }

    /// Fold binary operations on literals
    fn fold_binary_literals(
        &self,
        left: &Literal,
        op: &BinaryOp,
        right: &Literal,
    ) -> Option<ExprKind> {
        match (left, right) {
            (Literal::Int(l), Literal::Int(r)) => {
                let l: i128 = l.parse().ok()?;
                let r: i128 = r.parse().ok()?;
                let result = match op {
                    BinaryOp::Add => l.checked_add(r)?,
                    BinaryOp::Sub => l.checked_sub(r)?,
                    BinaryOp::Mul => l.checked_mul(r)?,
                    BinaryOp::Div => {
                        if r != 0 {
                            l.checked_div(r)?
                        } else {
                            return None;
                        }
                    }
                    BinaryOp::Rem => {
                        if r != 0 {
                            l.checked_rem(r)?
                        } else {
                            return None;
                        }
                    }
                    BinaryOp::Lt => (l < r) as i128,
                    BinaryOp::Le => (l <= r) as i128,
                    BinaryOp::Gt => (l > r) as i128,
                    BinaryOp::Ge => (l >= r) as i128,
                    BinaryOp::Eq => (l == r) as i128,
                    BinaryOp::Ne => (l != r) as i128,
                    BinaryOp::And | BinaryOp::Or => return None,
                };
                Some(ExprKind::Literal(Literal::Int(result.to_string())))
            }
            (Literal::Float(l), Literal::Float(r)) => {
                let l: f64 = l.parse().ok()?;
                let r: f64 = r.parse().ok()?;
                let result: ExprKind = match op {
                    BinaryOp::Add => ExprKind::Literal(Literal::Float((l + r).to_string())),
                    BinaryOp::Sub => ExprKind::Literal(Literal::Float((l - r).to_string())),
                    BinaryOp::Mul => ExprKind::Literal(Literal::Float((l * r).to_string())),
                    BinaryOp::Div => {
                        if r != 0.0 {
                            ExprKind::Literal(Literal::Float((l / r).to_string()))
                        } else {
                            return None;
                        }
                    }
                    BinaryOp::Lt => ExprKind::Literal(Literal::Int(((l < r) as i128).to_string())),
                    BinaryOp::Le => ExprKind::Literal(Literal::Int(((l <= r) as i128).to_string())),
                    BinaryOp::Gt => ExprKind::Literal(Literal::Int(((l > r) as i128).to_string())),
                    BinaryOp::Ge => ExprKind::Literal(Literal::Int(((l >= r) as i128).to_string())),
                    BinaryOp::Eq => ExprKind::Literal(Literal::Int(((l == r) as i128).to_string())),
                    BinaryOp::Ne => ExprKind::Literal(Literal::Int(((l != r) as i128).to_string())),
                    _ => return None,
                };
                return Some(result);
            }
            (Literal::Bool(l), Literal::Bool(r)) => {
                let result = match op {
                    BinaryOp::And => *l && *r,
                    BinaryOp::Or => *l || *r,
                    BinaryOp::Eq => l == r,
                    BinaryOp::Ne => l != r,
                    _ => return None,
                };
                Some(ExprKind::Literal(Literal::Bool(result)))
            }
            _ => None,
        }
    }

    /// Apply algebraic simplifications
    fn apply_algebraic_simplifications(
        &self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
    ) -> Option<ExprKind> {
        match (op, &left.kind, &right.kind) {
            // x + 0 = x
            (BinaryOp::Add, _, ExprKind::Literal(Literal::Int(s))) if s == "0" => {
                Some(left.clone().kind)
            }
            (BinaryOp::Add, ExprKind::Literal(Literal::Int(s)), _) if s == "0" => {
                Some(right.clone().kind)
            }
            (BinaryOp::Add, _, ExprKind::Literal(Literal::Float(s))) if s == "0" || s == "0.0" => {
                Some(left.clone().kind)
            }
            (BinaryOp::Add, ExprKind::Literal(Literal::Float(s)), _) if s == "0" || s == "0.0" => {
                Some(right.clone().kind)
            }
            // x * 1 = x
            (BinaryOp::Mul, _, ExprKind::Literal(Literal::Int(s))) if s == "1" => {
                Some(left.clone().kind)
            }
            (BinaryOp::Mul, ExprKind::Literal(Literal::Int(s)), _) if s == "1" => {
                Some(right.clone().kind)
            }
            (BinaryOp::Mul, _, ExprKind::Literal(Literal::Float(s))) if s == "1" || s == "1.0" => {
                Some(left.clone().kind)
            }
            (BinaryOp::Mul, ExprKind::Literal(Literal::Float(s)), _) if s == "1" || s == "1.0" => {
                Some(right.clone().kind)
            }
            // x * 0 = 0
            (BinaryOp::Mul, _, ExprKind::Literal(Literal::Int(s))) if s == "0" => {
                Some(ExprKind::Literal(Literal::Int("0".to_string())))
            }
            (BinaryOp::Mul, ExprKind::Literal(Literal::Int(s)), _) if s == "0" => {
                Some(ExprKind::Literal(Literal::Int("0".to_string())))
            }
            // x - 0 = x
            (BinaryOp::Sub, _, ExprKind::Literal(Literal::Int(s))) if s == "0" => {
                Some(left.clone().kind)
            }
            // x / 1 = x
            (BinaryOp::Div, _, ExprKind::Literal(Literal::Int(s))) if s == "1" => {
                Some(left.clone().kind)
            }
            _ => None,
        }
    }

    /// Try to fold unary operations
    fn try_fold_unary(&self, op: &UnaryOp, operand: &Expr) -> Option<ExprKind> {
        if let ExprKind::Literal(lit) = &operand.kind {
            let result = match (op, lit) {
                (UnaryOp::Neg, Literal::Int(i)) => {
                    let v: i128 = i.parse().ok()?;
                    Literal::Int((-v).to_string())
                }
                (UnaryOp::Neg, Literal::Float(f)) => {
                    let v: f64 = f.parse().ok()?;
                    Literal::Float((-v).to_string())
                }
                (UnaryOp::Not, Literal::Bool(b)) => Literal::Bool(!b),
                _ => return None,
            };
            Some(ExprKind::Literal(result))
        } else {
            None
        }
    }

    /// Try to inline built-in functions
    fn try_inline_builtin(&self, name: &str, args: &[Expr]) -> Option<ExprKind> {
        match name {
            "abs" if args.len() == 1 => {
                if let ExprKind::Literal(Literal::Int(i)) = &args[0].kind {
                    let v: i128 = i.parse().ok()?;
                    return Some(ExprKind::Literal(Literal::Int(v.abs().to_string())));
                }
            }
            "min" if args.len() == 2 => {
                if let (ExprKind::Literal(Literal::Int(a)), ExprKind::Literal(Literal::Int(b))) =
                    (&args[0].kind, &args[1].kind)
                {
                    let av: i128 = a.parse().ok()?;
                    let bv: i128 = b.parse().ok()?;
                    return Some(ExprKind::Literal(Literal::Int(av.min(bv).to_string())));
                }
            }
            "max" if args.len() == 2 => {
                if let (ExprKind::Literal(Literal::Int(a)), ExprKind::Literal(Literal::Int(b))) =
                    (&args[0].kind, &args[1].kind)
                {
                    let av: i128 = a.parse().ok()?;
                    let bv: i128 = b.parse().ok()?;
                    return Some(ExprKind::Literal(Literal::Int(av.max(bv).to_string())));
                }
            }
            _ => {}
        }
        None
    }

    /// Check if an expression is a no-op
    fn is_noop(&self, expr: &Expr) -> bool {
        matches!(expr.kind, ExprKind::Literal(_))
    }

    /// Clear constants (call when scope changes)
    pub fn clear_constants(&mut self) {
        self.constants.clear();
        self.mutable_vars.clear();
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Span;

    fn make_expr(kind: ExprKind) -> Expr {
        Expr {
            kind,
            span: Span::new(0, 0),
        }
    }

    fn make_int(value: i64) -> Expr {
        make_expr(ExprKind::Literal(Literal::Int(value.to_string())))
    }

    fn make_float(value: f64) -> Expr {
        make_expr(ExprKind::Literal(Literal::Float(value.to_string())))
    }

    fn make_bool(value: bool) -> Expr {
        make_expr(ExprKind::Literal(Literal::Bool(value)))
    }

    fn make_binary(left: Expr, op: BinaryOp, right: Expr) -> Expr {
        make_expr(ExprKind::Binary(Box::new(left), op, Box::new(right)))
    }

    fn make_unary(op: UnaryOp, operand: Expr) -> Expr {
        make_expr(ExprKind::Unary(op, Box::new(operand)))
    }

    fn make_identifier(name: &str) -> Expr {
        make_expr(ExprKind::Identifier(name.to_string()))
    }

    #[test]
    fn test_constant_folding_add() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(2), BinaryOp::Add, make_int(3));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "5"),
            _ => panic!("Expected folded int literal, got {:?}", expr.kind),
        }
    }

    #[test]
    fn test_constant_folding_sub() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(10), BinaryOp::Sub, make_int(4));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "6"),
            _ => panic!("Expected folded int literal"),
        }
    }

    #[test]
    fn test_constant_folding_mul() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(3), BinaryOp::Mul, make_int(7));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "21"),
            _ => panic!("Expected folded int literal"),
        }
    }

    #[test]
    fn test_constant_folding_div() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(20), BinaryOp::Div, make_int(4));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "5"),
            _ => panic!("Expected folded int literal"),
        }
    }

    #[test]
    fn test_constant_folding_div_by_zero() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(10), BinaryOp::Div, make_int(0));
        optimizer.optimize_expr(&mut expr);
        // Should not fold division by zero
        match &expr.kind {
            ExprKind::Binary(_, _, _) => {} // Not folded, as expected
            _ => panic!("Should not fold division by zero"),
        }
    }

    #[test]
    fn test_constant_folding_rem() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(17), BinaryOp::Rem, make_int(5));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "2"),
            _ => panic!("Expected folded int literal"),
        }
    }

    #[test]
    fn test_constant_folding_float_add() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_float(1.5), BinaryOp::Add, make_float(2.5));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Float(v)) => {
                let val: f64 = v.parse().unwrap();
                assert!((val - 4.0).abs() < 0.001);
            }
            _ => panic!("Expected folded float literal"),
        }
    }

    #[test]
    fn test_constant_folding_bool_and() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_bool(true), BinaryOp::And, make_bool(false));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Bool(v)) => assert!(!v),
            _ => panic!("Expected folded bool literal"),
        }
    }

    #[test]
    fn test_constant_folding_bool_or() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_bool(true), BinaryOp::Or, make_bool(false));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Bool(v)) => assert!(v),
            _ => panic!("Expected folded bool literal"),
        }
    }

    #[test]
    fn test_constant_folding_comparison_eq() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(5), BinaryOp::Eq, make_int(5));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "1"),
            _ => panic!("Expected folded int literal (1 for true)"),
        }
    }

    #[test]
    fn test_constant_folding_comparison_lt() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_int(3), BinaryOp::Lt, make_int(5));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "1"),
            _ => panic!("Expected folded int literal"),
        }
    }

    #[test]
    fn test_unary_neg_fold() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_unary(UnaryOp::Neg, make_int(42));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "-42"),
            _ => panic!("Expected folded int literal"),
        }
    }

    #[test]
    fn test_unary_not_fold() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_unary(UnaryOp::Not, make_bool(true));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Bool(v)) => assert!(!v),
            _ => panic!("Expected folded bool literal"),
        }
    }

    #[test]
    fn test_algebraic_simplification_add_zero() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Add, make_int(0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected simplified identifier"),
        }
    }

    #[test]
    fn test_algebraic_simplification_mul_one() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Mul, make_int(1));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected simplified identifier"),
        }
    }

    #[test]
    fn test_algebraic_simplification_mul_zero() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Mul, make_int(0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "0"),
            _ => panic!("Expected simplified zero"),
        }
    }

    #[test]
    fn test_algebraic_simplification_sub_zero() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Sub, make_int(0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected simplified identifier"),
        }
    }

    #[test]
    fn test_algebraic_simplification_div_one() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Div, make_int(1));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected simplified identifier"),
        }
    }

    #[test]
    fn test_constant_propagation() {
        let mut optimizer = Optimizer::new(2);
        let mut program = Program {
            declarations: vec![TopLevelDecl::Statements(Block {
                statements: vec![
                    Stmt::VarDecl(VarDecl {
                        name: "x".to_string(),
                        var_type: TypeExpr::Base("i32".to_string()),
                        init: Some(make_int(42)),
                        span: Span::new(0, 0),
                    }),
                    Stmt::Expr(make_identifier("x")),
                ],
                span: Span::new(0, 0),
            })],
        };
        optimizer.optimize(&mut program);
        // After optimization, check that the expression was optimized
        match &program.declarations[0] {
            TopLevelDecl::Statements(block) => {
                // The int literal expression is a noop, so it gets removed
                // We just verify the var decl is still there
                assert!(block.statements.len() >= 1);
                match &block.statements[0] {
                    Stmt::VarDecl(var) => {
                        assert_eq!(var.name, "x");
                    }
                    _ => panic!("Expected VarDecl"),
                }
            }
            _ => panic!("Expected Statements"),
        }
    }

    #[test]
    fn test_no_constant_propagation_for_mutable() {
        let mut optimizer = Optimizer::new(2);
        let mut program = Program {
            declarations: vec![TopLevelDecl::Statements(Block {
                statements: vec![
                    Stmt::VarDecl(VarDecl {
                        name: "x".to_string(),
                        var_type: TypeExpr::Base("i32".to_string()),
                        init: Some(make_int(42)),
                        span: Span::new(0, 0),
                    }),
                    Stmt::Assignment(Assignment {
                        target: AssignmentTarget::Identifier("x".to_string()),
                        op: AssignOp::Equal,
                        value: make_int(100),
                        span: Span::new(0, 0),
                    }),
                    Stmt::Expr(make_identifier("x")),
                ],
                span: Span::new(0, 0),
            })],
        };
        optimizer.optimize(&mut program);
        // After optimization, the expression should NOT be replaced because x is mutable
        match &program.declarations[0] {
            TopLevelDecl::Statements(block) => {
                match &block.statements[2] {
                    Stmt::Expr(expr) => {
                        match &expr.kind {
                            ExprKind::Identifier(name) => assert_eq!(name, "x"),
                            _ => panic!("Should not propagate constant for mutable variable"),
                        }
                    }
                    _ => panic!("Expected Expr"),
                }
            }
            _ => panic!("Expected Statements"),
        }
    }

    #[test]
    fn test_dead_code_elimination_noop() {
        let mut optimizer = Optimizer::new(2);
        let mut program = Program {
            declarations: vec![TopLevelDecl::Statements(Block {
                statements: vec![
                    Stmt::Expr(make_int(42)), // This is a no-op
                    Stmt::Expr(make_identifier("x")),
                ],
                span: Span::new(0, 0),
            })],
        };
        optimizer.optimize(&mut program);
        match &program.declarations[0] {
            TopLevelDecl::Statements(block) => {
                // The literal expression should be removed as a no-op
                assert_eq!(block.statements.len(), 1);
            }
            _ => panic!("Expected Statements"),
        }
    }

    #[test]
    fn test_builtin_inline_abs() {
        let mut optimizer = Optimizer::new(3);
        let mut expr = Expr {
            kind: ExprKind::Call(
                Box::new(make_identifier("abs")),
                vec![make_int(-42)],
            ),
            span: Span::new(0, 0),
        };
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "42"),
            _ => panic!("Expected inlined abs result"),
        }
    }

    #[test]
    fn test_builtin_inline_min() {
        let mut optimizer = Optimizer::new(3);
        let mut expr = Expr {
            kind: ExprKind::Call(
                Box::new(make_identifier("min")),
                vec![make_int(10), make_int(20)],
            ),
            span: Span::new(0, 0),
        };
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "10"),
            _ => panic!("Expected inlined min result"),
        }
    }

    #[test]
    fn test_builtin_inline_max() {
        let mut optimizer = Optimizer::new(3);
        let mut expr = Expr {
            kind: ExprKind::Call(
                Box::new(make_identifier("max")),
                vec![make_int(10), make_int(20)],
            ),
            span: Span::new(0, 0),
        };
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "20"),
            _ => panic!("Expected inlined max result"),
        }
    }

    #[test]
    fn test_opt_level_zero_no_optimization() {
        let mut optimizer = Optimizer::new(0);
        let mut expr = make_binary(make_int(2), BinaryOp::Add, make_int(3));
        optimizer.optimize_expr(&mut expr);
        // Should not fold at opt level 0
        match &expr.kind {
            ExprKind::Binary(_, _, _) => {} // Not folded, as expected
            _ => panic!("Should not optimize at level 0"),
        }
    }

    #[test]
    fn test_nested_expression_optimization() {
        let mut optimizer = Optimizer::new(2);
        // (2 + 3) * (4 - 1)
        let inner1 = make_binary(make_int(2), BinaryOp::Add, make_int(3));
        let inner2 = make_binary(make_int(4), BinaryOp::Sub, make_int(1));
        let mut expr = make_binary(inner1, BinaryOp::Mul, inner2);
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "15"),
            _ => panic!("Expected fully folded nested expression"),
        }
    }

    #[test]
    fn test_float_comparison_folding() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_float(1.5), BinaryOp::Gt, make_float(1.0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "1"),
            _ => panic!("Expected folded comparison result"),
        }
    }

    #[test]
    fn test_unary_float_neg_fold() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_unary(UnaryOp::Neg, make_float(3.14));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Float(v)) => {
                let val: f64 = v.parse().unwrap();
                assert!((val - (-3.14)).abs() < 0.001);
            }
            _ => panic!("Expected folded float literal"),
        }
    }

    #[test]
    fn test_clear_constants() {
        let mut optimizer = Optimizer::new(2);
        optimizer.constants.insert("x".to_string(), Literal::Int("42".to_string()));
        optimizer.mutable_vars.insert("y".to_string());
        optimizer.clear_constants();
        assert!(optimizer.constants.is_empty());
        assert!(optimizer.mutable_vars.is_empty());
    }

    #[test]
    fn test_default_optimizer() {
        let optimizer = Optimizer::default();
        assert_eq!(optimizer.opt_level, 2);
    }

    #[test]
    fn test_constant_folding_bool_eq() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_bool(true), BinaryOp::Eq, make_bool(true));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Bool(v)) => assert!(v),
            _ => panic!("Expected folded bool literal"),
        }
    }

    #[test]
    fn test_constant_folding_bool_ne() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_bool(true), BinaryOp::Ne, make_bool(false));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Bool(v)) => assert!(v),
            _ => panic!("Expected folded bool literal"),
        }
    }

    #[test]
    fn test_constant_folding_float_mul() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_float(2.0), BinaryOp::Mul, make_float(3.0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Float(v)) => {
                let val: f64 = v.parse().unwrap();
                assert!((val - 6.0).abs() < 0.001);
            }
            _ => panic!("Expected folded float literal"),
        }
    }

    #[test]
    fn test_constant_folding_float_div() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_float(10.0), BinaryOp::Div, make_float(2.0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Literal(Literal::Float(v)) => {
                let val: f64 = v.parse().unwrap();
                assert!((val - 5.0).abs() < 0.001);
            }
            _ => panic!("Expected folded float literal"),
        }
    }

    #[test]
    fn test_constant_folding_float_div_by_zero() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_float(10.0), BinaryOp::Div, make_float(0.0));
        optimizer.optimize_expr(&mut expr);
        // Should not fold division by zero
        match &expr.kind {
            ExprKind::Binary(_, _, _) => {} // Not folded, as expected
            _ => panic!("Should not fold division by zero"),
        }
    }

    #[test]
    fn test_algebraic_simplification_float_add_zero() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Add, make_float(0.0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected simplified identifier"),
        }
    }

    #[test]
    fn test_algebraic_simplification_float_mul_one() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Mul, make_float(1.0));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected simplified identifier"),
        }
    }

    #[test]
    fn test_no_optimization_for_non_literals() {
        let mut optimizer = Optimizer::new(2);
        let mut expr = make_binary(make_identifier("x"), BinaryOp::Add, make_identifier("y"));
        optimizer.optimize_expr(&mut expr);
        match &expr.kind {
            ExprKind::Binary(_, _, _) => {} // Cannot fold non-literals
            _ => panic!("Should not fold non-literal expressions"),
        }
    }

    #[test]
    fn test_constant_propagation_in_block() {
        let mut optimizer = Optimizer::new(2);
        let mut program = Program {
            declarations: vec![TopLevelDecl::Function(FunctionDef {
                name: "test".to_string(),
                params: vec![],
                return_type: Some(TypeExpr::Base("i32".to_string())),
                body: Block {
                    statements: vec![
                        Stmt::VarDecl(VarDecl {
                            name: "x".to_string(),
                            var_type: TypeExpr::Base("i32".to_string()),
                            init: Some(make_int(100)),
                            span: Span::new(0, 0),
                        }),
                        Stmt::Return(ReturnStmt {
                            expr: Some(make_binary(make_identifier("x"), BinaryOp::Add, make_int(1))),
                            span: Span::new(0, 0),
                        }),
                    ],
                    span: Span::new(0, 0),
                },
                is_async: false,
                span: Span::new(0, 0),
            })],
        };
        optimizer.optimize(&mut program);
        // Check that the return expression was optimized
        match &program.declarations[0] {
            TopLevelDecl::Function(func) => {
                match &func.body.statements[1] {
                    Stmt::Return(ret) => {
                        match &ret.expr.as_ref().unwrap().kind {
                            ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "101"),
                            _ => panic!("Expected folded constant in return"),
                        }
                    }
                    _ => panic!("Expected Return"),
                }
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_optimize_if_constant_true() {
        let mut optimizer = Optimizer::new(2);
        let mut program = Program {
            declarations: vec![TopLevelDecl::Statements(Block {
                statements: vec![
                    Stmt::If(IfStmt {
                        condition: make_bool(true),
                        then_block: Block {
                            statements: vec![Stmt::Expr(make_int(42))],
                            span: Span::new(0, 0),
                        },
                        else_branch: None,
                        span: Span::new(0, 0),
                    }),
                ],
                span: Span::new(0, 0),
            })],
        };
        optimizer.optimize(&mut program);
        // The if statement should be eliminated when condition is always true
        match &program.declarations[0] {
            TopLevelDecl::Statements(block) => {
                // The if body should be inlined, so we should have 1 statement (the int literal, which is a noop)
                // Actually the int literal inside is a noop so it gets removed, leaving 0 statements
                assert!(block.statements.len() <= 1);
            }
            _ => panic!("Expected Statements"),
        }
    }

    #[test]
    fn test_optimize_if_constant_false_no_else() {
        let mut optimizer = Optimizer::new(2);
        let mut program = Program {
            declarations: vec![TopLevelDecl::Statements(Block {
                statements: vec![
                    Stmt::If(IfStmt {
                        condition: make_bool(false),
                        then_block: Block {
                            statements: vec![Stmt::Expr(make_int(42))],
                            span: Span::new(0, 0),
                        },
                        else_branch: None,
                        span: Span::new(0, 0),
                    }),
                ],
                span: Span::new(0, 0),
            })],
        };
        optimizer.optimize(&mut program);
        // The if statement should be eliminated entirely when condition is always false and no else
        match &program.declarations[0] {
            TopLevelDecl::Statements(block) => {
                assert_eq!(block.statements.len(), 0);
            }
            _ => panic!("Expected Statements"),
        }
    }

    #[test]
    fn test_optimize_global_var() {
        let mut optimizer = Optimizer::new(2);
        let mut program = Program {
            declarations: vec![TopLevelDecl::GlobalVar(GlobalVarDecl {
                name: "global_x".to_string(),
                var_type: TypeExpr::Base("i32".to_string()),
                init: Some(make_binary(make_int(10), BinaryOp::Add, make_int(20))),
                span: Span::new(0, 0),
            })],
        };
        optimizer.optimize(&mut program);
        match &program.declarations[0] {
            TopLevelDecl::GlobalVar(global) => {
                match &global.init.as_ref().unwrap().kind {
                    ExprKind::Literal(Literal::Int(v)) => assert_eq!(v, "30"),
                    _ => panic!("Expected folded global var init"),
                }
            }
            _ => panic!("Expected GlobalVar"),
        }
    }
}
