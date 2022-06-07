use lox_syntax::ast::expr::Expr;
use lox_syntax::ast::stmt::Stmt;

use crate::interpreter::error::RResult;

use super::Interpreter;

impl Interpreter {
    pub fn execute_stmt(&mut self, stmt: &Stmt) -> RResult<()> {
        use Stmt::*;

        match stmt {
            Print(p) => self.execute_print(&p.expr),
            _ => todo!(),
        }
    }

    pub fn execute_print(&mut self, expr: &Expr) -> RResult<()> {
        let value = self.evaluate_expr(expr)?;
        // Should probably be replaced with something that passes value to a Printer rather
        // than printing to stdout directly
        println!("{}", value);
        Ok(())
    }
}
