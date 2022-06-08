use std::collections::HashMap;

use crate::interpreter::error::{RResult, RuntimeError, Undefined};
use crate::interpreter::value::RuntimeValue;

#[derive(Debug)]
pub struct Environment {
    inner: HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.inner.insert(name.into(), value);
    }

    // For primitive types we will choose to clone them and pass owned copies back to the caller
    // for more complex types we will use reference counted smart pointers to hand an owned copy of
    // the pointer back to the caller.
    pub fn get(&self, name: &str) -> RResult<RuntimeValue> {
        self.inner.get(name).cloned().ok_or_else(|| {
            RuntimeError::Undefined(Undefined {
                message: format!("undefined variable {}", name).into(),
            })
        })
    }
}
