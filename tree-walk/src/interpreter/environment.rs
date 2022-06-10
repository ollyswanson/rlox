use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

use crate::interpreter::error::{RResult, RuntimeError, Undefined};
use crate::interpreter::value::RuntimeValue;

#[derive(Debug, Default)]
struct EnvironmentInner {
    locals: HashMap<String, RuntimeValue>,
    enclosing: Option<Environment>,
}

#[derive(Debug, Clone, Default)]
pub struct Environment {
    inner: Rc<RefCell<EnvironmentInner>>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn enter_scope(&mut self) {
        let mut scope = Environment::new();
        mem::swap(self, &mut scope);
        self.inner.borrow_mut().enclosing = Some(scope);
    }

    pub fn exit_scope(&mut self) {
        let enclosing = self.inner.borrow_mut().enclosing.take();
        if let Some(mut enclosing) = enclosing {
            mem::swap(self, &mut enclosing);
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.inner.borrow_mut().locals.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: RuntimeValue) -> RResult<RuntimeValue> {
        let mut inner = self.inner.borrow_mut();
        match inner.locals.get_mut(name) {
            Some(current) => {
                *current = value.clone();
                Ok(value)
            }
            None => match &mut inner.enclosing {
                Some(enclosing) => enclosing.assign(name, value),
                None => Err(RuntimeError::Undefined(Undefined {
                    message: format!("cannot assign undefined variable {}", name).into(),
                })),
            },
        }
    }

    pub fn get(&self, name: &str) -> RResult<RuntimeValue> {
        match &self.inner.borrow().enclosing {
            None => self
                .inner
                .borrow()
                .locals
                .get(name)
                .cloned()
                .ok_or_else(|| {
                    RuntimeError::Undefined(Undefined {
                        message: format!("undefined variable {}", name).into(),
                    })
                }),
            Some(enclosing) => self
                .inner
                .borrow()
                .locals
                .get(name)
                .cloned()
                .map(Ok)
                .unwrap_or_else(|| enclosing.get(name)),
        }
    }
}
