use lox_syntax::ast::stmt::{Block, ExprStmt, FunDecl, If, Print, Return, Stmt, Var, While};

use super::Resolver;

impl Resolver<'_> {
    pub(super) fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FunDecl(f) => self.resolve_fun_decl(f),
            Stmt::Var(v) => self.resolve_var_decl(v),
            Stmt::Expr(e) => self.resolve_expr_stmt(e),
            Stmt::Block(b) => self.resolve_block_stmt(b),
            Stmt::While(w) => self.resolve_while_stmt(w),
            Stmt::Print(p) => self.resolve_print_stmt(p),
            Stmt::If(i) => self.resolve_if_stmt(i),
            Stmt::Return(r) => self.resolve_return_stmt(r),
        }
    }

    fn resolve_var_decl(&mut self, var_decl: &Var) {
        self.declare(&var_decl.id);
        self.resolve_expr(&var_decl.expr);
        self.define(&var_decl.id);
    }

    fn resolve_fun_decl(&mut self, fun_decl: &FunDecl) {
        self.declare(&fun_decl.id);
        self.define(&fun_decl.id);
        self.scoped(|this| {
            for param in fun_decl.params.iter() {
                this.declare(param);
                this.define(param);
            }

            this.resolve(&fun_decl.body);
        });
    }

    fn resolve_expr_stmt(&mut self, expr_stmt: &ExprStmt) {
        self.resolve_expr(&expr_stmt.expr);
    }

    fn resolve_if_stmt(&mut self, if_stmt: &If) {
        self.resolve_expr(&if_stmt.cond);
        self.resolve_stmt(&if_stmt.then_stmt);
        if let Some(else_stmt) = &if_stmt.else_stmt {
            self.resolve_stmt(else_stmt);
        }
    }

    fn resolve_print_stmt(&mut self, print_stmt: &Print) {
        self.resolve_expr(&print_stmt.expr);
    }

    fn resolve_return_stmt(&mut self, return_stmt: &Return) {
        self.resolve_expr(&return_stmt.expr);
    }

    fn resolve_while_stmt(&mut self, while_stmt: &While) {
        self.resolve_expr(&while_stmt.cond);
        self.resolve_stmt(&while_stmt.stmt);
    }

    fn resolve_block_stmt(&mut self, block_stmt: &Block) {
        self.scoped(|this| {
            for stmt in block_stmt.stmts.iter() {
                this.resolve_stmt(stmt);
            }
        })
    }
}
