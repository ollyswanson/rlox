use lox_syntax::ast::expr::*;

use crate::interpreter::error::{RResult, RuntimeError, TypeError};
use crate::interpreter::value::RuntimeValue;
use crate::interpreter::Interpreter;

impl Interpreter {
    pub fn evaluate_expr(&mut self, expr: &Expr) -> RResult<RuntimeValue> {
        use Expr::*;
        match expr {
            Literal(literal) => Ok(RuntimeValue::from(&literal.value)),
            Var(v) => self.environment.get(&v.id.name),
            Grouping(g) => self.evaluate_expr(&g.expr),
            Binary(b) => self.evaluate_binary_expression(b),
            Logical(l) => self.evaluate_logical_expression(l),
            Unary(u) => self.evaluate_unary_expression(u),
            Assign(a) => self.evaluate_assign(a),
            Call(c) => self.evaluate_call(c),
        }
    }

    fn evaluate_unary_expression(&mut self, unary: &Unary) -> RResult<RuntimeValue> {
        use RuntimeValue::*;
        use UnOp::*;

        let value = self.evaluate_expr(&unary.expr)?;

        match (unary.op, value) {
            (Minus, Number(n)) => Ok(Number(-n)),
            (Minus, v) => Err(RuntimeError::TypeError(TypeError {
                message: format!("Expected number found {}", v).into(),
            })),
            (Bang, v) => Ok(Boolean(v.is_truthy())),
        }
    }

    fn evaluate_binary_expression(&mut self, binary: &Binary) -> RResult<RuntimeValue> {
        use BinOp::*;
        use RuntimeValue::*;

        let lvalue = self.evaluate_expr(&binary.lhs)?;
        let rvalue = self.evaluate_expr(&binary.rhs)?;

        match (lvalue, rvalue, binary.op) {
            (Number(l), Number(r), op) => evaluate_arithmetic_expression(l, r, op),
            (String(l), String(r), Add) => Ok(String(format!("{}{}", l, r))),
            (l, r, op) => Err(RuntimeError::TypeError(TypeError {
                message: format!("Illegal operation {} {} {}", l, op, r).into(),
            })),
        }
    }

    fn evaluate_logical_expression(&mut self, logical: &Logical) -> RResult<RuntimeValue> {
        use LogicalOp::*;

        let lhs = self.evaluate_expr(&logical.lhs)?;

        match (lhs.is_truthy(), logical.op) {
            (true, Or) => Ok(lhs),
            (false, And) => Ok(lhs),
            _ => self.evaluate_expr(&logical.rhs),
        }
    }

    fn evaluate_assign(&mut self, assign: &Assign) -> RResult<RuntimeValue> {
        let value = self.evaluate_expr(&assign.expr)?;
        self.environment.assign(&assign.var.name, value)
    }

    fn evaluate_call(&mut self, call: &Call) -> RResult<RuntimeValue> {
        let callee = self.evaluate_expr(call.callee.as_ref())?;

        if let RuntimeValue::Function(callee) = callee {
            let args: Vec<RuntimeValue> = call
                .args
                .iter()
                .map(|arg| self.evaluate_expr(arg))
                .collect::<RResult<_>>()?;
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
                }))
            }
        } else {
            Err(RuntimeError::TypeError(TypeError {
                message: "can only call functions and classes".into(),
            }))
        }
    }
}

fn evaluate_arithmetic_expression(l: f64, r: f64, op: BinOp) -> RResult<RuntimeValue> {
    use BinOp::*;
    use RuntimeValue::*;

    match (l, r, op) {
        (_l, r, Divide) if r == 0f64 => Err(RuntimeError::DivisionByZero),
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
