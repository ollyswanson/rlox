use std::sync::atomic::{AtomicUsize, Ordering};

use crate::span::Span;

pub mod expr;
pub mod stmt;
pub(crate) mod util;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub id: usize,
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(span: Span, name: impl Into<String>, id: usize) -> Self {
        Self {
            id,
            name: name.into(),
            span,
        }
    }
}
