use std::rc::Rc;

use environment::Environment;
use error::RResult;
use lox_syntax::ast::stmt::Stmt;
use value::function::Clock;
use value::RuntimeValue;

mod environment;
mod error;
mod expr;
mod stmt;
mod value;

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
            self.execute_stmt(stmt)?;
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
