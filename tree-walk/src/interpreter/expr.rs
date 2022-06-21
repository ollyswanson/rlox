use lox_syntax::ast::expr::*;

use crate::interpreter::error::{RuntimeError, TypeError};
use crate::interpreter::value::RuntimeValue;
use crate::interpreter::{CFResult, ControlFlow, Interpreter};

impl Interpreter {
    pub fn evaluate_expr(&mut self, expr: &Expr) -> CFResult<RuntimeValue> {
        use Expr::*;
        match expr {
            Literal(literal) => Ok(RuntimeValue::from(&literal.value)),
            Var(v) => self.evaluate_var_expr(v),
            Grouping(g) => self.evaluate_expr(&g.expr),
            Binary(b) => self.evaluate_binary_expression(b),
            Logical(l) => self.evaluate_logical_expression(l),
            Unary(u) => self.evaluate_unary_expression(u),
            Assign(a) => self.evaluate_assign(a),
            Call(c) => self.evaluate_call(c),
            Get(g) => todo!(),
        }
    }

    fn evaluate_var_expr(&self, var_expr: &Var) -> CFResult<RuntimeValue> {
        self.get_variable(&var_expr.id)
    }

    fn evaluate_unary_expression(&mut self, unary: &Unary) -> CFResult<RuntimeValue> {
        use RuntimeValue::*;
        use UnOp::*;

        let value = self.evaluate_expr(&unary.expr)?;

        match (unary.op, value) {
            (Minus, Number(n)) => Ok(Number(-n)),
            (Minus, v) => Err(RuntimeError::TypeError(TypeError {
                message: format!("Expected number found {}", v).into(),
            })
            .into()),
            (Bang, v) => Ok(Boolean(v.is_truthy())),
        }
    }

    fn evaluate_binary_expression(&mut self, binary: &Binary) -> CFResult<RuntimeValue> {
        use BinOp::*;
        use RuntimeValue::*;

        let lvalue = self.evaluate_expr(&binary.lhs)?;
        let rvalue = self.evaluate_expr(&binary.rhs)?;

        match (lvalue, rvalue, binary.op) {
            (Number(l), Number(r), op) => evaluate_arithmetic_expression(l, r, op),
            (String(l), String(r), Add) => Ok(String(format!("{}{}", l, r))),
            (l, r, op) => Err(RuntimeError::TypeError(TypeError {
                message: format!("Illegal operation {} {} {}", l, op, r).into(),
            })
            .into()),
        }
    }

    fn evaluate_logical_expression(&mut self, logical: &Logical) -> CFResult<RuntimeValue> {
        use LogicalOp::*;

        let lhs = self.evaluate_expr(&logical.lhs)?;

        match (lhs.is_truthy(), logical.op) {
            (true, Or) => Ok(lhs),
            (false, And) => Ok(lhs),
            _ => self.evaluate_expr(&logical.rhs),
        }
    }

    fn evaluate_assign(&mut self, assign: &Assign) -> CFResult<RuntimeValue> {
        let value = self.evaluate_expr(&assign.expr)?;

        if let Some(&depth) = self.locals.get(&assign.var.id) {
            Ok(self.environment.assign_at(&assign.var.name, value, depth))
        } else {
            self.globals.assign(&assign.var.name, value)
        }
    }

    fn evaluate_call(&mut self, call: &Call) -> CFResult<RuntimeValue> {
        let callee = self.evaluate_expr(call.callee.as_ref())?;

        let callee = match callee {
            RuntimeValue::Function(callee) => callee,
            RuntimeValue::Class(callee) => callee,
            _ => {
                return Err(RuntimeError::TypeError(TypeError {
                    message: "can only call functions and classes".into(),
                })
                .into())
            }
        };

        let args: Vec<RuntimeValue> = call
            .args
            .iter()
            .map(|arg| self.evaluate_expr(arg))
            .collect::<CFResult<_>>()?;

        if callee.arity() == args.len() {
            callee.call(self, args)
        } else {
            Err(RuntimeError::TypeError(TypeError {
                message: format!(
                    "expected {} arguments but got {}",
                    callee.arity(),
                    args.len()
                )
                .into(),
            })
            .into())
        }
    }
}

fn evaluate_arithmetic_expression(l: f64, r: f64, op: BinOp) -> CFResult<RuntimeValue> {
    use BinOp::*;
    use RuntimeValue::*;

    match (l, r, op) {
        (_l, r, Divide) if r == 0f64 => {
            Err(ControlFlow::RuntimeError(RuntimeError::DivisionByZero))
        }
        (l, r, Divide) => Ok(Number(l / r)),
        (l, r, Multiply) => Ok(Number(l * r)),
        (l, r, Add) => Ok(Number(l + r)),
        (l, r, Subtract) => Ok(Number(l - r)),
        (l, r, Equal) => Ok(Boolean(l == r)),
        (l, r, NotEqual) => Ok(Boolean(l != r)),
        (l, r, Greater) => Ok(Boolean(l > r)),
        (l, r, GreaterEqual) => Ok(Boolean(l >= r)),
        (l, r, Less) => Ok(Boolean(l < r)),
        (l, r, LessEqual) => Ok(Boolean(l <= r)),
    }
}
