use std::rc::Rc;

use environment::Environment;
use error::RResult;
use error::RuntimeError;
use lox_syntax::ast::stmt::Stmt;
use value::function::Clock;
use value::RuntimeValue;

mod environment;
mod error;
mod expr;
mod stmt;
mod value;

pub type CFResult<T> = Result<T, ControlFlow>;

pub enum ControlFlow {
    Return(RuntimeValue),
    RuntimeError(RuntimeError),
}

impl From<RuntimeError> for ControlFlow {
    fn from(e: RuntimeError) -> Self {
        ControlFlow::RuntimeError(e)
    }
}

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
    globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> RResult<()> {
        for stmt in statements {
            match self.execute_stmt(stmt) {
                Ok(_) => {}
                Err(ControlFlow::Return(_)) => {
                    return Err(RuntimeError::ReturnOutsideFunction);
                }
                Err(ControlFlow::RuntimeError(e)) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut globals = Environment::new();
        let environment = globals.clone();
        globals.define("clock", RuntimeValue::Function(Rc::new(Clock {})));

        Self {
            environment,
            globals,
        }
    }
}
