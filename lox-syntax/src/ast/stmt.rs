use std::fmt::{Display, Formatter};

use crate::ast::expr::Expr;
use crate::span::Span;
use crate::Identifier;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Print),
    Expr(ExprStmt),
    Var(Var),
    Block(Block),
    If(If),
}

#[derive(Debug, PartialEq)]
pub struct Var {
    pub span: Span,
    pub id: Identifier,
    pub expr: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Print {
    pub span: Span,
    pub expr: Expr,
}

#[derive(Debug, PartialEq)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub span: Span,
    pub cond: Expr,
    pub then_stmt: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Print(p) => p.span,
            Stmt::Expr(e) => e.span,
            Stmt::Var(v) => v.span,
            Stmt::Block(b) => b.span,
            Stmt::If(i) => i.span,
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
