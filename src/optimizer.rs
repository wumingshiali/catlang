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
    /// Optimization level (0-3)
    opt_level: u8,
}

impl Optimizer {
    /// Create a new optimizer with given optimization level
    pub fn new(opt_level: u8) -> Self {
        Optimizer {
            constants: HashMap::with_capacity(32), // Pre-allocate for typical use
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

                        // Try to fold constants
                        if let ExprKind::Literal(lit) = &init.kind {
                            if var_decl.var_type.is_numeric() {
                                self.constants.insert(var_decl.name.clone(), lit.clone());
                            }
                        }
                    }
                    new_statements.push(Stmt::VarDecl(var_decl));
                }
                Stmt::Assignment(mut assignment) => {
                    self.optimize_expr(&mut assignment.value);

                    // Invalidate constant if reassigned
                    if let AssignmentTarget::Identifier(name) = &assignment.target {
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
                            self.optimize_block(&mut if_stmt.then_block);
                            for s in if_stmt.then_block.statements.drain(..) {
                                new_statements.push(s);
                            }
                        } else if let Some(else_branch) = if_stmt.else_branch {
                            // Condition is always false - keep only else branch
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
                        }
                        // If false and no else, skip entirely
                    } else {
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
                    }
                }
                Stmt::While(mut while_stmt) => {
                    self.optimize_expr(&mut while_stmt.condition);
                    self.optimize_block(&mut while_stmt.body);
                    new_statements.push(Stmt::While(while_stmt));
                }
                Stmt::For(mut for_stmt) => {
                    if let Some(init) = &mut for_stmt.init.init {
                        self.optimize_expr(init);
                    }
                    self.optimize_expr(&mut for_stmt.condition);
                    self.optimize_expr(&mut for_stmt.update.value);
                    self.optimize_block(&mut for_stmt.body);
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
                    self.optimize_block(&mut block);
                    new_statements.push(Stmt::Block(block));
                }
                // Pass through other statements
                Stmt::CopyStmt(_)
                | Stmt::Switch(_)
                | Stmt::Try(_)
                | Stmt::Throw(_)
                | Stmt::AsyncStmt(_)
                | Stmt::UnsafeBlock(_) => {
                    new_statements.push(stmt);
                }
            }
        }

        block.statements = new_statements;
    }

    /// Optimize an expression - uses a two-pass approach to avoid borrow conflicts
    pub fn optimize_expr(&mut self, expr: &mut Expr) {
        // Pass 1: Recursively optimize child expressions
        self.optimize_children(expr);

        // Pass 2: Apply local optimizations
        if self.opt_level >= 2 {
            self.apply_local_optimizations(expr);
        }

        // Pass 3: Constant propagation (opt level 1+)
        if self.opt_level >= 1 {
            if let ExprKind::Identifier(name) = &expr.kind {
                if let Some(lit) = self.constants.get(name) {
                    expr.kind = ExprKind::Literal(lit.clone());
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
                for arg in args {
                    self.optimize_expr(arg);
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
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(2)
    }
}
