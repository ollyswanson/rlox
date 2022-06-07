use std::borrow::Cow;

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
}
