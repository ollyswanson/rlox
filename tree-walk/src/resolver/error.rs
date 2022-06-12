use std::fmt::{Display, Formatter};

use lox_syntax::span::Span;

pub type ResolveResult<T> = Result<T, ResolverError>;

#[derive(Debug)]
pub enum ResolverError {
    InitializeFromSelf { span: Span },
    AlreadyDeclared { span: Span },
    Undeclared { span: Span, message: String },
}

impl Display for ResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ResolverError::*;

        match self {
            InitializeFromSelf { span: _ } => {
                f.write_str("defined variable from self in declaration")
            }
            AlreadyDeclared { span: _ } => f.write_str("variable has already been declared"),
            Undeclared { span: _, message } => f.write_str(message),
        }
    }
}
