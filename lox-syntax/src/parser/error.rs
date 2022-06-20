use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub use crate::token::ScanError;
use crate::{span::Span, token::TokenKind};

pub type PResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    ScanError {
        error: ScanError,
        span: Span,
    },
    UnexpectedToken {
        span: Span,
        message: Cow<'static, str>,
        kind: TokenKind,
    },
    InvalidAssignment {
        span: Span,
        message: Cow<'static, str>,
    },
}

impl ParseError {
    pub fn allows_continuation(&self) -> bool {
        match self {
            ParseError::ScanError { error, .. } => matches!(error, ScanError::UnterminatedString),
            ParseError::UnexpectedToken { kind, .. } => matches!(kind, &TokenKind::Eof),
            ParseError::InvalidAssignment { .. } => false,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;
        match self {
            ScanError { error, .. } => write!(f, "{}", error),
            UnexpectedToken { message, .. } => f.write_str(message),
            InvalidAssignment { message, .. } => f.write_str(message),
        }
    }
}
