use std::sync::atomic::{AtomicUsize, Ordering};

use crate::span::Span;

pub mod expr;
pub(crate) mod util;

//  A unique id for the Identifiers is needed for the following case:
//
// var a = 1;
// fun() {
//   print a;
//   var a = 2;
//   print a;
//
// Without a unique id and just using the name, during resolution `a` would first be pointed at the
// `a` in the global scope, and would then be pointed at the `a` in the function scope, such that
// instead of `1` and `2` being printed to stdout, there would be an error due to printing an
// undeclared variable.
static ID_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

pub struct Identifier {
    id: usize,
    name: String,
    span: Span,
}

impl Identifier {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        let id = ID_SEQUENCE.fetch_add(1, Ordering::SeqCst);

        Self {
            id,
            name: name.into(),
            span,
        }
    }
}
