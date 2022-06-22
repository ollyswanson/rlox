use lox_syntax::ast::expr::{Assign, Binary, Call, Expr, Logical, Set, Super, This, Var};

use super::{BindingState, ClassType, Resolver, ResolverError};

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
            Expr::Set(s) => self.resolve_set_expr(s),
            Expr::This(t) => self.resolve_this_expr(t),
            Expr::Super(s) => self.resolve_super_expr(s),
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

    fn resolve_this_expr(&mut self, this: &This) {
        if matches!(self.class_type, ClassType::Class) {
            self.resolve_binding(&this.id);
        } else {
            self.error(ResolverError::ThisOutsideClass { span: this.span });
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

    fn resolve_set_expr(&mut self, set: &Set) {
        self.resolve_expr(&set.object);
        self.resolve_expr(&set.value);
    }

    fn resolve_super_expr(&mut self, super_expr: &Super) {
        match self.class_type {
            ClassType::None => {
                self.error(ResolverError::InvalidSuper {
                    span: super_expr.span,
                    message: "can't user 'super' outside of a class".into(),
                });
            }
            ClassType::Class => {
                self.error(ResolverError::InvalidSuper {
                    span: super_expr.span,
                    message: "can't user 'super' in a class with no superclass".into(),
                });
            }
            _ => {}
        }

        self.resolve_binding(&super_expr.id);
    }
}
