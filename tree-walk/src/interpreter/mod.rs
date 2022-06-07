use lox_syntax::ast::stmt::Stmt;

use crate::interpreter::error::RResult;

mod error;
mod expr;
mod stmt;
mod value;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> RResult<()> {
        for stmt in statements {
            self.execute_stmt(stmt)?;
        }

        Ok(())
    }
}
