use std::fmt::{Display, Formatter};

use crate::ast::expr::Expr;
use crate::span::Span;
use crate::Identifier;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Print),
    Expr(ExprStmt),
    Var(Var),
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

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Stmt::*;
        match self {
            Var(v) => write!(f, "{}", v),
            Print(p) => write!(f, "{}", p),
            Expr(e) => write!(f, "{}", e),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.id.name)
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "print {};", self.expr)
    }
}

impl Display for ExprStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.expr)
    }
}
