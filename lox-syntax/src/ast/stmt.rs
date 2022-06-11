use crate::ast::expr::Expr;
use crate::span::Span;

use super::Identifier;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Print(Print),
    Expr(ExprStmt),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
    FunDecl(FunDecl),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    pub span: Span,
    pub id: Identifier,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Print {
    pub span: Span,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub span: Span,
    pub cond: Expr,
    pub then_stmt: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct While {
    pub span: Span,
    pub cond: Expr,
    pub stmt: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunDecl {
    pub span: Span,
    pub id: Identifier,
    pub params: Vec<Identifier>,
    pub body: Vec<Stmt>,
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Print(p) => p.span,
            Stmt::Expr(e) => e.span,
            Stmt::Var(v) => v.span,
            Stmt::Block(b) => b.span,
            Stmt::If(i) => i.span,
            Stmt::While(w) => w.span,
            Stmt::FunDecl(f) => f.span,
        }
    }
}

impl Var {
    pub fn new(span: Span, id: Identifier, expr: Expr) -> Self {
        Self { span, id, expr }
    }
}

impl Print {
    pub fn new(span: Span, expr: Expr) -> Self {
        Self { span, expr }
    }
}

impl ExprStmt {
    pub fn new(span: Span, expr: Expr) -> Self {
        Self { span, expr }
    }
}

impl Block {
    pub fn new(span: Span, stmts: Vec<Stmt>) -> Self {
        Self { span, stmts }
    }
}

impl If {
    pub fn new(
        span: Span,
        cond: Expr,
        then_stmt: impl Into<Box<Stmt>>,
        else_stmt: Option<impl Into<Box<Stmt>>>,
    ) -> Self {
        Self {
            span,
            cond,
            then_stmt: then_stmt.into(),
            else_stmt: else_stmt.map(|s| s.into()),
        }
    }
}

impl While {
    pub fn new(span: Span, cond: Expr, stmt: impl Into<Box<Stmt>>) -> Self {
        Self {
            span,
            cond,
            stmt: stmt.into(),
        }
    }
}

impl FunDecl {
    pub fn new(span: Span, id: Identifier, params: Vec<Identifier>, body: Vec<Stmt>) -> Self {
        Self {
            span,
            id,
            params,
            body,
        }
    }
}
