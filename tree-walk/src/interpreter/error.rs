use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type RResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    TypeError(TypeError),
    DivisionByZero,
}

#[derive(Debug)]
pub struct TypeError {
    pub(crate) message: Cow<'static, str>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
