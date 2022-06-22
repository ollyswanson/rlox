use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use lox_syntax::ast::expr::Value;

use crate::interpreter::CFResult;
use crate::Interpreter;

pub mod class;
pub mod function;

use class::{Class, Instance};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Nil,
    String(String),
    Number(f64),
    Boolean(bool),
    Function(Rc<dyn Callable>),
    Class(Rc<Class>),
    Object(Rc<Instance>),
}

impl RuntimeValue {
    pub fn is_truthy(&self) -> bool {
        use RuntimeValue::*;

        match self {
            Nil => false,
            Boolean(b) => *b,
            _ => true,
        }
    }
}

impl From<&Value> for RuntimeValue {
    fn from(ast_value: &Value) -> Self {
        match ast_value {
            Value::Nil => RuntimeValue::Nil,
            Value::String(s) => RuntimeValue::String(s.clone()),
            Value::Boolean(b) => RuntimeValue::Boolean(*b),
            Value::Number(n) => RuntimeValue::Number(*n),
        }
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use RuntimeValue::*;
        match self {
            Nil => f.write_str("nil"),
            String(s) => write!(f, "\"{}\"", s),
            Number(n) => write!(f, "{}", n),
            Boolean(b) => write!(f, "{}", b),
            Function(fun) => write!(f, "{}", fun),
            Class(class) => write!(f, "{}", class),
            Object(instance) => write!(f, "{}", instance),
        }
    }
}

pub trait Callable: Debug + Display {
    fn arity(&self) -> usize;
    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        args: Vec<RuntimeValue>,
    ) -> CFResult<RuntimeValue>;
}
