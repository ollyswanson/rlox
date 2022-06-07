use std::fmt::{Display, Formatter};

use crate::ast::expr::Expr;
use crate::span::Span;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Print),
    Expr(ExprStmt),
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
            Print(p) => write!(f, "{}", p),
            Expr(e) => write!(f, "{}", e),
        }
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
