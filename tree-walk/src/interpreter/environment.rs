use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

use crate::interpreter::error::{RuntimeError, Undefined};
use crate::interpreter::value::RuntimeValue;
use crate::interpreter::CFResult;

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

    pub fn from_enclosing(enclosing: Environment) -> Environment {
        let environment = Environment::new();
        environment.inner.borrow_mut().enclosing = Some(enclosing);
        environment
    }

    pub fn define(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.inner.borrow_mut().locals.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: RuntimeValue) -> CFResult<RuntimeValue> {
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
                })
                .into()),
            },
        }
    }

    pub fn assign_at(&mut self, name: &str, value: RuntimeValue, depth: usize) -> RuntimeValue {
        let env = self.ancestor(depth);
        let mut inner = env.inner.borrow_mut();
        *inner.locals.get_mut(name).unwrap() = value.clone();
        value
    }

    pub fn get(&self, name: &str) -> CFResult<RuntimeValue> {
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
                    .into()
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

    // TODO: There's a dependence on use of the resolver to make static guarantees about the
    // presence of values. It might be better to still return a result and not unwrap here. As
    // otherwise there is a strict dependence on the Resolver by the Interpreter.
    pub fn get_with_depth(&self, name: &str, depth: usize) -> RuntimeValue {
        let env = self.ancestor(depth);
        let inner = env.inner.borrow();

        inner.locals.get(name).cloned().expect(
            "Semantic analysis by the resolver guarantees that this variable will be present",
        )
    }

    fn ancestor(&self, depth: usize) -> Environment {
        let mut env = self.clone();
        for _ in 0..depth {
            let enclosing = env.inner.borrow().enclosing.clone().unwrap();
            env = enclosing.clone();
        }
        env
    }
}
