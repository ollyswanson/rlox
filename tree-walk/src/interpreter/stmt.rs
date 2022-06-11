use std::mem;
use std::rc::Rc;

use lox_syntax::ast::stmt::{Block, ExprStmt, FunDecl, If, Print, Return, Stmt, Var, While};

use crate::interpreter::environment::Environment;
use crate::interpreter::ControlFlow;

use super::value::function::LoxFunction;
use super::value::RuntimeValue;
use super::{CFResult, Interpreter};

impl Interpreter {
    pub fn execute_stmt(&mut self, stmt: &Stmt) -> CFResult<()> {
        use Stmt::*;

        match stmt {
            Var(v) => self.execute_var_stmt(v),
            Print(p) => self.execute_print_stmt(p),
            Expr(s) => self.execute_expr_stmt(s),
            Block(b) => self.execute_block_stmt(b),
            If(i) => self.execute_if_stmt(i),
            While(w) => self.execute_while_stmt(w),
            FunDecl(f) => self.execute_fun_decl(f),
            Return(r) => self.execute_return_stmt(r),
        }
    }

    fn execute_block_stmt(&mut self, block: &Block) -> CFResult<()> {
        self.scoped_statement(
            |i| {
                for stmt in block.stmts.iter() {
                    i.execute_stmt(stmt)?
                }
                Ok(())
            },
            Environment::from_enclosing(self.environment.clone()),
        )
    }

    fn execute_if_stmt(&mut self, if_stmt: &If) -> CFResult<()> {
        if self.evaluate_expr(&if_stmt.cond)?.is_truthy() {
            self.execute_stmt(&if_stmt.then_stmt)
        } else {
            if_stmt
                .else_stmt
                .as_deref()
                .map(|else_stmt| self.execute_stmt(else_stmt))
                .unwrap_or(Ok(()))
        }
    }

    fn execute_while_stmt(&mut self, while_stmt: &While) -> CFResult<()> {
        while self.evaluate_expr(&while_stmt.cond)?.is_truthy() {
            self.execute_stmt(&while_stmt.stmt)?;
        }

        Ok(())
    }

    fn execute_var_stmt(&mut self, var: &Var) -> CFResult<()> {
        let value = self.evaluate_expr(&var.expr)?;
        self.environment.define(&var.id.name, value);
        Ok(())
    }

    fn execute_print_stmt(&mut self, print: &Print) -> CFResult<()> {
        let value = self.evaluate_expr(&print.expr)?;
        // Should probably be replaced with something that passes value to a Printer rather
        // than printing to stdout directly
        println!("{}", value);
        Ok(())
    }

    fn execute_expr_stmt(&mut self, expr_stmt: &ExprStmt) -> CFResult<()> {
        self.evaluate_expr(&expr_stmt.expr)?;
        Ok(())
    }

    fn execute_fun_decl(&mut self, fun_decl: &FunDecl) -> CFResult<()> {
        self.environment.define(
            &fun_decl.id.name,
            RuntimeValue::Function(Rc::new(LoxFunction::new(fun_decl))),
        );
        Ok(())
    }

    fn execute_return_stmt(&mut self, return_stmt: &Return) -> CFResult<()> {
        let value = self.evaluate_expr(&return_stmt.expr)?;
        Err(ControlFlow::Return(value))
    }

    pub fn scoped_statement<F, T>(&mut self, f: F, environment: Environment) -> CFResult<T>
    where
        F: FnOnce(&mut Self) -> CFResult<T>,
    {
        let old_env = mem::replace(&mut self.environment, environment);
        let result = f(self);
        self.environment = old_env;
        result
    }
}
