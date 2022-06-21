use std::{fmt::Display, rc::Rc};

use super::{CFResult, Callable, RuntimeValue};

#[derive(Debug)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        self: Rc<Self>,
        _interpreter: &mut crate::Interpreter,
        _args: Vec<RuntimeValue>,
    ) -> CFResult<RuntimeValue> {
        Ok(RuntimeValue::Object(Rc::new(Instance::new(self))))
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Instance {
    class: Rc<Class>,
}

impl Instance {
    fn new(class: Rc<Class>) -> Self {
        Self { class }
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
