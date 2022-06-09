use std::collections::HashMap;
use std::mem;

use crate::interpreter::error::{RResult, RuntimeError, Undefined};
use crate::interpreter::value::RuntimeValue;

#[derive(Debug)]
pub struct Environment {
    inner: HashMap<String, RuntimeValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn enter_scope(&mut self) {
        let mut scope = Environment::new();
        mem::swap(self, &mut scope);
        self.enclosing = Some(Box::new(scope));
    }

    pub fn exit_scope(&mut self) {
        if let Some(mut enclosing) = self.enclosing.take() {
            mem::swap(self, &mut *enclosing);
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.inner.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: RuntimeValue) -> RResult<RuntimeValue> {
        let current_value = self.get_mut(name)?;
        *current_value = value;
        Ok(current_value.clone())
    }

    pub fn get_ref(&self, name: &str) -> RResult<&RuntimeValue> {
        match &self.enclosing {
            None => self.inner.get(name).ok_or_else(|| {
                RuntimeError::Undefined(Undefined {
                    message: format!("undefined variable {}", name).into(),
                })
            }),
            Some(enclosing) => self
                .inner
                .get(name)
                .map(Ok)
                .unwrap_or_else(|| enclosing.get_ref(name)),
        }
    }

    pub fn get_mut(&mut self, name: &str) -> RResult<&mut RuntimeValue> {
        match &mut self.enclosing {
            None => self.inner.get_mut(name).ok_or_else(|| {
                RuntimeError::Undefined(Undefined {
                    message: format!("undefined variable {}", name).into(),
                })
            }),
            Some(enclosing) => self
                .inner
                .get_mut(name)
                .map(Ok)
                .unwrap_or_else(|| enclosing.get_mut(name)),
        }
    }

    // For primitive types we will choose to clone them and pass owned copies back to the caller
    // for more complex types we will use reference counted smart pointers to hand an owned copy of
    // the pointer back to the caller.
    pub fn get(&self, name: &str) -> RResult<RuntimeValue> {
        self.get_ref(name).map(|value| value.clone())
    }
}
