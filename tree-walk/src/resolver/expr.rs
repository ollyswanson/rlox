use lox_syntax::ast::expr::{Assign, Binary, Call, Expr, Logical, Var};

use crate::resolver::error::ResolverError;
use crate::resolver::BindingState;

use super::Resolver;

impl Resolver<'_> {
    pub(super) fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Var(v) => self.resolve_var_expr(v),
            Expr::Assign(a) => self.resolve_assign_expr(a),
            Expr::Literal(_) => {}
            Expr::Binary(b) => self.resolve_binary_expr(b),
            Expr::Grouping(g) => self.resolve_expr(&g.expr),
            Expr::Unary(u) => self.resolve_expr(&u.expr),
            Expr::Logical(l) => self.resolve_logical_expr(l),
            Expr::Call(c) => self.resolve_call_expr(c),
            Expr::Get(g) => self.resolve_expr(&g.object),
            Expr::Set(s) => todo!(),
        }
    }

    fn resolve_var_expr(&mut self, var: &Var) {
        match self.scopes.last().and_then(|scope| scope.get(&var.id.name)) {
            // If variable is referenced before it has been defined but after it has been declared
            // then it is being used in a situation such as var a = a;
            Some(BindingState::Declared) => {
                self.error(ResolverError::InitializeFromSelf { span: var.span })
            }
            _ => self.resolve_binding(&var.id),
        }
    }

    fn resolve_assign_expr(&mut self, assign: &Assign) {
        self.resolve_expr(&assign.expr);
        self.resolve_binding(&assign.var);
    }

    fn resolve_binary_expr(&mut self, binary: &Binary) {
        self.resolve_expr(&binary.lhs);
        self.resolve_expr(&binary.rhs);
    }

    fn resolve_call_expr(&mut self, call: &Call) {
        self.resolve_expr(&call.callee);
        for arg in call.args.iter() {
            self.resolve_expr(arg);
        }
    }

    fn resolve_logical_expr(&mut self, logical: &Logical) {
        self.resolve_expr(&logical.lhs);
        self.resolve_expr(&logical.rhs);
    }
}
