use std::rc::Rc;

use lox_syntax::ast::expr::*;

use crate::interpreter::error::{RuntimeError, TypeError, Undefined};
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
            Get(g) => self.evaluate_get(g),
            Set(s) => self.evaluate_set(s),
            This(t) => self.evaluate_this(t),
            Super(s) => self.evaluate_super(s),
        }
    }

    fn evaluate_var_expr(&self, var_expr: &Var) -> CFResult<RuntimeValue> {
        self.get_variable(&var_expr.id)
    }

    fn evaluate_this(&self, this_expr: &This) -> CFResult<RuntimeValue> {
        self.get_variable(&this_expr.id)
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
            (l, r, Equal) => Ok(Boolean(l == r)),
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

    fn evaluate_get(&mut self, get: &Get) -> CFResult<RuntimeValue> {
        let object = self.evaluate_expr(&get.object)?;

        match object {
            RuntimeValue::Object(instance) => instance.get(&get.property.name),
            _ => Err(ControlFlow::RuntimeError(RuntimeError::TypeError(
                TypeError {
                    message: "only instances have properties".into(),
                },
            ))),
        }
    }

    fn evaluate_set(&mut self, set: &Set) -> CFResult<RuntimeValue> {
        let object = self.evaluate_expr(&set.object)?;

        match object {
            RuntimeValue::Object(instance) => {
                let value = self.evaluate_expr(&set.value)?;
                instance.set(&set.property.name, value)
            }
            _ => Err(ControlFlow::RuntimeError(RuntimeError::TypeError(
                TypeError {
                    message: "only instances have properties".into(),
                },
            ))),
        }
    }

    fn evaluate_super(&mut self, super_expr: &Super) -> CFResult<RuntimeValue> {
        use RuntimeValue as RV;
        let depth = self
            .locals
            .get(&super_expr.id.id)
            .copied()
            .expect("Resolution step statically guarantees the super lookup");
        let super_class = self.environment.get_with_depth("super", depth);

        // hacky based on knowing the distance to `this`
        let this = self.environment.get_with_depth("this", depth - 1);

        if let RV::Class(super_class) = super_class {
            super_class
                .find_method(&super_expr.method.name)
                .map(|mtd| RV::Function(Rc::new(mtd.bind(this))))
                .ok_or_else(|| {
                    ControlFlow::RuntimeError(RuntimeError::Undefined(Undefined {
                        message: format!("undefined property {}", super_expr.method.name).into(),
                    }))
                })
        } else {
            unreachable!()
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
