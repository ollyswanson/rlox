use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type RResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    TypeError(TypeError),
    DivisionByZero,
    Undefined(Undefined),
    ReturnOutsideFunction,
}

#[derive(Debug)]
pub struct TypeError {
    pub(crate) message: Cow<'static, str>,
}

#[derive(Debug)]
pub struct Undefined {
    pub(crate) message: Cow<'static, str>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError(TypeError { message }) => f.write_str(message),
            RuntimeError::DivisionByZero => f.write_str("division by zero"),
            RuntimeError::Undefined(Undefined { message }) => f.write_str(message),
            RuntimeError::ReturnOutsideFunction => {
                f.write_str("return must be used within a function")
            }
        }
    }
}

impl Error for RuntimeError {}
