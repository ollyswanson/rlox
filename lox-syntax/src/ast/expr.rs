use std::fmt::{Display, Formatter, Write};

use crate::span::Span;
use crate::token::{Token, TokenKind};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnOp {
    Bang,
    Minus,
}

impl UnOp {
    pub fn from_token(token: &Token) -> Option<Self> {
        use TokenKind::*;

        match token.kind {
            Bang => Some(UnOp::Bang),
            Minus => Some(UnOp::Minus),
            _ => None,
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub span: Span,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub span: Span,
    pub op: UnOp,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub span: Span,
    pub op: BinOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl Expr {
    pub fn span(&self) -> Span {
        use Expr::*;

        match self {
            Literal(p) => p.span,
            Unary(u) => u.span,
            Binary(b) => b.span,
            Grouping(g) => g.span,
        }
    }
}

impl Literal {
    pub fn new(span: Span, value: Value) -> Self {
        Self { value, span }
    }

    pub fn from_token(token: &Token) -> Option<Self> {
        use TokenKind as T;
        use Value as V;

        match &token.kind {
            T::String(s) => Some(Self::new(token.span, V::String(s.clone()))),
            T::Number(n) => Some(Self::new(token.span, V::Number(*n))),
            T::True => Some(Self::new(token.span, V::Boolean(true))),
            T::False => Some(Self::new(token.span, V::Boolean(false))),
            T::Nil => Some(Self::new(token.span, V::Nil)),
            _ => None,
        }
    }
}

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
            Literal(p) => write!(f, "{}", p.value),
            Unary(u) => write!(f, "{}", u),
            Binary(b) => write!(f, "{}", b),
            Grouping(g) => write!(f, "{}", g),
        }
    }
}
