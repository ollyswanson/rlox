use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use crate::span::Span;
pub use crate::token::ScanError;

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
    },
    InvalidAssignment {
        span: Span,
        message: Cow<'static, str>,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;
        match self {
            ScanError { error, span: _ } => write!(f, "{}", error),
            UnexpectedToken { message, span: _ } => f.write_str(message),
            InvalidAssignment { message, span: _ } => f.write_str(message),
        }
    }
}
