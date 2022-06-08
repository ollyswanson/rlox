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
            Unary(u) => self.evaluate_unary_expression(u),
            Assign(a) => self.evaluate_assign(a),
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

        let lvalue = self.evaluate_expr(&binary.left)?;
        let rvalue = self.evaluate_expr(&binary.right)?;

        match (lvalue, rvalue, binary.op) {
            (Number(l), Number(r), op) => evaluate_arithmetic_expression(l, r, op),
            (String(l), String(r), Add) => Ok(String(format!("{}{}", l, r))),
            (l, r, op) => Err(RuntimeError::TypeError(TypeError {
                message: format!("Illegal operation {} {} {}", l, op, r).into(),
            })),
        }
    }

    fn evaluate_assign(&mut self, assign: &Assign) -> RResult<RuntimeValue> {
        let value = self.evaluate_expr(&assign.expr)?;
        self.environment.define(&assign.var.name, value.clone());
        Ok(value)
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
