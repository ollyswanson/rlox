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

#[derive(Debug)]
pub struct LoxFunction {
    decl: FunDecl,
    closure: Environment,
}

impl LoxFunction {
    pub fn new(decl: &FunDecl, closure: Environment) -> Self {
        Self {
            decl: decl.clone(),
            closure,
        }
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
                        Err(ControlFlow::Return(v)) => return Ok(v),
                    }
                }
                Ok(RuntimeValue::Nil)
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
