use std::fmt::{Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use lox_syntax::ast::stmt::FunDecl;

use crate::interpreter::environment::Environment;

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

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<fn clock>")
    }
}

#[derive(Debug)]
pub struct LoxFunction {
    decl: FunDecl,
}

impl LoxFunction {
    pub fn new(decl: &FunDecl) -> Self {
        Self { decl: decl.clone() }
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.decl.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<RuntimeValue>,
    ) -> RResult<RuntimeValue> {
        let mut environment = Environment::from_enclosing(interpreter.globals.clone());

        for (param, arg) in self.decl.params.iter().cloned().zip(args) {
            environment.define(param.name, arg);
        }

        interpreter.scoped_statement(
            |this| {
                this.interpret(&self.decl.body)?;
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
