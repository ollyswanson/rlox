use std::borrow::Cow;

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
