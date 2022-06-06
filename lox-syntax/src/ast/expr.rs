use std::fmt::{Display, Formatter, Write};

use crate::span::Span;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnOp {
    Bang,
    Minus,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinOp {
    Multiply,
    Divide,
    Add,
    Subtract,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
    Equal,
    NotEqual,
}

pub enum Expr {
    Value(Value),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

pub struct Unary {
    span: Span,
    op: UnOp,
    expr: Box<Expr>,
}

pub struct Binary {
    span: Span,
    op: BinOp,
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct Grouping {
    span: Span,
    expr: Box<Expr>,
}

// impl new
impl Unary {
    pub fn new(span: Span, op: UnOp, expr: impl Into<Box<Expr>>) -> Self {
        Self {
            span,
            op,
            expr: expr.into(),
        }
    }
}

impl Binary {
    pub fn new(
        span: Span,
        op: BinOp,
        left: impl Into<Box<Expr>>,
        right: impl Into<Box<Expr>>,
    ) -> Self {
        Self {
            span,
            op,
            left: left.into(),
            right: right.into(),
        }
    }
}

impl Grouping {
    pub fn new(span: Span, expr: impl Into<Box<Expr>>) -> Self {
        Self {
            span,
            expr: expr.into(),
        }
    }
}

// impl Display

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use UnOp::*;

        match self {
            Bang => f.write_str("!"),
            Minus => f.write_str("-"),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use BinOp::*;
        match self {
            Equal => f.write_str("=="),
            NotEqual => f.write_str("!="),
            Less => f.write_str("<"),
            LessEqual => f.write_str("<="),
            Greater => f.write_str(">"),
            GreaterEqual => f.write_str(">="),
            Add => f.write_str("+"),
            Subtract => f.write_str("-"),
            Multiply => f.write_str("*"),
            Divide => f.write_str("/"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Value::*;

        match self {
            Nil => f.write_str("nil"),
            Boolean(b) => write!(f, "{}", b),
            Number(n) => write!(f, "{}", n),
            String(s) => write!(f, "{}", s),
        }
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.op, self.expr)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.expr)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expr::*;
        match self {
            Value(v) => write!(f, "{}", v),
            Unary(u) => write!(f, "{}", u),
            Binary(b) => write!(f, "{}", b),
            Grouping(g) => write!(f, "{}", g),
        }
    }
}
