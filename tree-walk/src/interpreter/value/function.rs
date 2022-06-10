use std::time::{SystemTime, UNIX_EPOCH};

use super::Interpreter;
use super::{Callable, RResult, RuntimeValue};

#[derive(Debug)]
pub struct Clock {}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _args: Vec<RuntimeValue>,
    ) -> RResult<RuntimeValue> {
        Ok(RuntimeValue::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        ))
    }
}
