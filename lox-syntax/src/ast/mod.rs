use crate::span::Span;

pub mod expr;
pub mod stmt;
pub(crate) mod util;

pub type IdentifierId = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub id: IdentifierId,
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(span: Span, name: impl Into<String>, id: IdentifierId) -> Self {
        Self {
            id,
            name: name.into(),
            span,
        }
    }
}
