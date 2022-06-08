use lox_syntax::ast::expr::Expr;
use lox_syntax::ast::stmt::Stmt;
use lox_syntax::Identifier;

use crate::interpreter::error::RResult;

use super::Interpreter;

impl Interpreter {
    pub fn execute_stmt(&mut self, stmt: &Stmt) -> RResult<()> {
        use Stmt::*;

        match stmt {
            Var(v) => self.execute_var_stmt(&v.id, &v.expr),
            Print(p) => self.execute_print(&p.expr),
            Expr(s) => self.execute_expr_statement(&s.expr),
        }
    }

    pub fn execute_var_stmt(&mut self, id: &Identifier, expr: &Expr) -> RResult<()> {
        let value = self.evaluate_expr(expr)?;
        self.environment.define(&id.name, value);
        Ok(())
    }

    pub fn execute_print(&mut self, expr: &Expr) -> RResult<()> {
        let value = self.evaluate_expr(expr)?;
        // Should probably be replaced with something that passes value to a Printer rather
        // than printing to stdout directly
        println!("{}", value);
        Ok(())
    }

    pub fn execute_expr_statement(&mut self, expr: &Expr) -> RResult<()> {
        self.evaluate_expr(expr)?;
        Ok(())
    }
}
