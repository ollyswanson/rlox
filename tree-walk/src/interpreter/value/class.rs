use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use crate::interpreter::{
    error::{RuntimeError, Undefined},
    value::function::LoxFunction,
    ControlFlow,
};

use super::{CFResult, Callable, RuntimeValue};

#[derive(Debug)]
pub struct Class {
    name: String,
    methods: HashMap<String, Rc<LoxFunction>>,
}

impl Class {
    pub fn new(name: impl Into<String>, methods: HashMap<String, Rc<LoxFunction>>) -> Self {
        Self {
            name: name.into(),
            methods,
        }
    }

    fn find_method(&self, name: &str) -> Option<Rc<LoxFunction>> {
        self.methods.get(name).cloned()
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
    properties: RefCell<HashMap<String, RuntimeValue>>,
}

impl Instance {
    fn new(class: Rc<Class>) -> Self {
        Self {
            class,
            properties: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(self: Rc<Self>, property_name: &str) -> CFResult<RuntimeValue> {
        if let Some(property) = self.properties.borrow().get(property_name).cloned() {
            return Ok(property);
        }

        self.class
            .find_method(property_name)
            .map(|mtd| RuntimeValue::Function(Rc::new(mtd.bind(RuntimeValue::Object(self)))))
            .ok_or_else(|| {
                ControlFlow::RuntimeError(RuntimeError::Undefined(Undefined {
                    message: format!("undefined property {}", property_name).into(),
                }))
            })
    }

    pub fn set(
        &self,
        property_name: impl Into<String>,
        value: RuntimeValue,
    ) -> CFResult<RuntimeValue> {
        self.properties
            .borrow_mut()
            .insert(property_name.into(), value.clone());

        Ok(value)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
