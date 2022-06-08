use lox_syntax::ast::stmt::Stmt;

use crate::interpreter::environment::Environment;
use crate::interpreter::error::RResult;

mod environment;
mod error;
mod expr;
mod stmt;
mod value;

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> RResult<()> {
        for stmt in statements {
            self.execute_stmt(stmt)?;
        }

        Ok(())
    }
}
