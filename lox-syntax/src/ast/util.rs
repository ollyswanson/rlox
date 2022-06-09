use crate::ast::expr::{BinOp, LogicalOp};
use crate::token::{Token, TokenKind};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AssocOp {
    Assign,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
}

pub enum Fixity {
    /// The operator has left associativity
    Left,
    /// The operator has right associativity
    Right,
}

impl AssocOp {
    pub fn from_token(t: &Token) -> Option<AssocOp> {
        use AssocOp::*;

        match t.kind {
            TokenKind::EqualEqual => Some(Equal),
            TokenKind::BangEqual => Some(NotEqual),
            TokenKind::Less => Some(Less),
            TokenKind::LessEqual => Some(LessEqual),
            TokenKind::Greater => Some(Greater),
            TokenKind::GreaterEqual => Some(GreaterEqual),
            TokenKind::Plus => Some(Add),
            TokenKind::Minus => Some(Subtract),
            TokenKind::Star => Some(Multiply),
            TokenKind::Slash => Some(Divide),
            TokenKind::Equal => Some(Assign),
            TokenKind::And => Some(And),
            TokenKind::Or => Some(Or),
            _ => None,
        }
    }

    pub fn precedence(&self) -> u8 {
        use AssocOp::*;

        match self {
            Assign => 1,
            Or => 2,
            And => 3,
            Equal | NotEqual => 4,
            Less | LessEqual | Greater | GreaterEqual => 5,
            Add | Subtract => 6,
            Multiply | Divide => 7,
        }
    }

    pub fn fixity(&self) -> Fixity {
        use AssocOp::*;
        use Fixity::*;

        match self {
            Assign => Right,
            _ => Left,
        }
    }

    pub fn to_bin_op(self) -> Option<BinOp> {
        use AssocOp::*;

        match self {
            Equal => Some(BinOp::Equal),
            NotEqual => Some(BinOp::NotEqual),
            Less => Some(BinOp::Less),
            LessEqual => Some(BinOp::LessEqual),
            Greater => Some(BinOp::Greater),
            GreaterEqual => Some(BinOp::GreaterEqual),
            Add => Some(BinOp::Add),
            Subtract => Some(BinOp::Subtract),
            Multiply => Some(BinOp::Multiply),
            Divide => Some(BinOp::Divide),
            _ => None,
        }
    }

    pub fn to_logical_op(self) -> Option<LogicalOp> {
        use AssocOp::*;

        match self {
            And => Some(LogicalOp::And),
            Or => Some(LogicalOp::Or),
            _ => None,
        }
    }
}
