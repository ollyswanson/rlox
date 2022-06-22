use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use lox_syntax::ast::stmt::FunDecl;

use crate::interpreter::{CFResult, ControlFlow, Environment};

use super::Interpreter;
use super::{Callable, RuntimeValue};

#[derive(Debug)]
pub struct Clock {}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        self: Rc<Self>,
        _interpreter: &mut Interpreter,
        _args: Vec<RuntimeValue>,
    ) -> CFResult<RuntimeValue> {
        Ok(RuntimeValue::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        ))
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<fn clock>")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LoxFunctionType {
    Initializer,
    Function,
}

impl From<&str> for LoxFunctionType {
    fn from(s: &str) -> Self {
        match s {
            "init" => Self::Initializer,
            _ => Self::Function,
        }
    }
}

#[derive(Debug)]
pub struct LoxFunction {
    decl: FunDecl,
    closure: Environment,
    function_type: LoxFunctionType,
}

impl LoxFunction {
    pub fn new(decl: &FunDecl, closure: Environment, function_type: LoxFunctionType) -> Self {
        Self {
            decl: decl.clone(),
            closure,
            function_type,
        }
    }

    /// Binds `value` to a new inner scope and returns a new instance of the function
    pub fn bind(&self, value: RuntimeValue) -> Self {
        let mut bindings = Environment::from_enclosing(self.closure.clone());
        bindings.define("this", value);
        Self::new(&self.decl, bindings, self.function_type)
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.decl.params.len()
    }

    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        args: Vec<RuntimeValue>,
    ) -> CFResult<RuntimeValue> {
        let mut environment = Environment::from_enclosing(self.closure.clone());

        for (param, arg) in self.decl.params.iter().cloned().zip(args) {
            environment.define(param.name, arg);
        }

        interpreter.scoped_statement(
            |this| {
                for stmt in self.decl.body.iter() {
                    match this.execute_stmt(stmt) {
                        Ok(_) => {}
                        Err(e @ ControlFlow::RuntimeError(_)) => return Err(e),
                        Err(ControlFlow::Return(v)) => {
                            return match self.function_type {
                                LoxFunctionType::Function => Ok(v),
                                LoxFunctionType::Initializer => {
                                    Ok(self.closure.get("this").unwrap())
                                }
                            }
                        }
                    }
                }

                // For simplicity's sake we make sure that init methods always return the related
                // instance
                match self.function_type {
                    // init methods will always have a reference to "this" therefore we can unwrap
                    LoxFunctionType::Initializer => Ok(self.closure.get("this").unwrap()),
                    _ => Ok(RuntimeValue::Nil),
                }
            },
            environment,
        )
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", &self.decl.id.name)
    }
}
