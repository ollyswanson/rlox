use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use lox_syntax::span::Span;

#[derive(Debug)]
pub enum ResolverError {
    InitializeFromSelf {
        span: Span,
    },
    AlreadyDeclared {
        span: Span,
    },
    Undeclared {
        span: Span,
        message: String,
    },
    ReturnOutsideFn {
        span: Span,
    },
    ThisOutsideClass {
        span: Span,
    },
    ReturnValueFromInit {
        span: Span,
    },
    InheritFromSelf {
        span: Span,
    },
    InvalidSuper {
        span: Span,
        message: Cow<'static, str>,
    },
}

impl Display for ResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ResolverError::*;

        match self {
            InitializeFromSelf { .. } => f.write_str("defined variable from self in declaration"),
            AlreadyDeclared { .. } => f.write_str("variable has already been declared"),
            Undeclared { message, .. } => f.write_str(message),
            ReturnOutsideFn { .. } => f.write_str("can't return outside of function body"),
            ThisOutsideClass { .. } => f.write_str("can't use `this` outside of a class"),
            ReturnValueFromInit { .. } => f.write_str("can't return a value inside `init` method"),
            InheritFromSelf { .. } => f.write_str("a class can't inherit from itself"),
            InvalidSuper { message, .. } => f.write_str(message),
        }
    }
}
