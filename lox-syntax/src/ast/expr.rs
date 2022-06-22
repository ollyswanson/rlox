use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::span::Span;
use crate::token::{Token, TokenKind};

use super::Identifier;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Var(Var),
    Unary(Unary),
    Binary(Binary),
    Logical(Logical),
    Grouping(Grouping),
    Assign(Assign),
    Call(Call),
    Get(Get),
    Set(Set),
    This(This),
    Super(Super),
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
pub struct Var {
    pub span: Span,
    pub id: Identifier,
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
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logical {
    pub span: Span,
    pub op: LogicalOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub span: Span,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub span: Span,
    pub var: Identifier,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub span: Span,
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Get {
    pub span: Span,
    pub object: Box<Expr>,
    pub property: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Set {
    pub span: Span,
    pub object: Box<Expr>,
    pub property: Identifier,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct This {
    pub span: Span,
    pub id: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Super {
    pub span: Span,
    pub id: Identifier,
    pub method: Identifier,
}

impl Expr {
    pub fn span(&self) -> Span {
        use Expr::*;

        match self {
            Literal(p) => p.span,
            Unary(u) => u.span,
            Binary(b) => b.span,
            Var(v) => v.span,
            Grouping(g) => g.span,
            Assign(a) => a.span,
            Logical(l) => l.span,
            Call(c) => c.span,
            Get(g) => g.span,
            Set(s) => s.span,
            This(t) => t.span,
            Super(s) => s.span,
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

impl Var {
    pub fn new(span: Span, id: Identifier) -> Self {
        Self { span, id }
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
        lhs: impl Into<Box<Expr>>,
        rhs: impl Into<Box<Expr>>,
    ) -> Self {
        Self {
            span,
            op,
            lhs: lhs.into(),
            rhs: rhs.into(),
        }
    }
}

impl Logical {
    pub fn new(
        span: Span,
        op: LogicalOp,
        lhs: impl Into<Box<Expr>>,
        rhs: impl Into<Box<Expr>>,
    ) -> Self {
        Self {
            span,
            op,
            lhs: lhs.into(),
            rhs: rhs.into(),
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

impl Assign {
    pub fn new(span: Span, var: Identifier, expr: impl Into<Box<Expr>>) -> Self {
        Self {
            span,
            var,
            expr: expr.into(),
        }
    }
}

impl Call {
    pub fn new(span: Span, callee: impl Into<Box<Expr>>, args: Vec<Expr>) -> Self {
        Self {
            span,
            callee: callee.into(),
            args,
        }
    }
}

impl Get {
    pub fn new(span: Span, object: impl Into<Box<Expr>>, property: Identifier) -> Self {
        Self {
            span,
            object: object.into(),
            property,
        }
    }
}

impl Set {
    pub fn new(
        span: Span,
        object: impl Into<Box<Expr>>,
        property: Identifier,
        value: impl Into<Box<Expr>>,
    ) -> Self {
        Self {
            span,
            object: object.into(),
            property,
            value: value.into(),
        }
    }
}

impl This {
    pub fn new(span: Span, identifier: Identifier) -> Self {
        Self {
            span,
            id: identifier,
        }
    }
}

impl Super {
    pub fn new(span: Span, id: Identifier, method: Identifier) -> Self {
        Self { span, id, method }
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

impl Display for LogicalOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LogicalOp::*;

        match self {
            And => f.write_str("and"),
            Or => f.write_str("or"),
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

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id.name)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", &self.op, &self.expr)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.op, &self.lhs, &self.rhs)
    }
}

impl Display for Logical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.op, self.lhs, self.rhs)
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", &self.expr)
    }
}

impl Display for Assign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(assign {} {})", &self.var.name, &self.expr)
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.callee, self.args.iter().join(", "))
    }
}

impl Display for Get {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(get {} {})", self.object, self.property.name)
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(set {} {} {})",
            self.object, self.property.name, self.value
        )
    }
}

impl Display for This {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("this")
    }
}

impl Display for Super {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(super {})", self.method.name)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expr::*;
        match self {
            Literal(p) => write!(f, "{}", p.value),
            Var(v) => write!(f, "{}", v.id.name),
            Unary(u) => write!(f, "{}", u),
            Binary(b) => write!(f, "{}", b),
            Logical(l) => write!(f, "{}", l),
            Grouping(g) => write!(f, "{}", g),
            Assign(a) => write!(f, "{}", a),
            Call(c) => write!(f, "{}", c),
            Get(g) => write!(f, "{}", g),
            Set(s) => write!(f, "{}", s),
            This(t) => write!(f, "{}", t),
            Super(s) => write!(f, "{}", s),
        }
    }
}
