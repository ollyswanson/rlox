use std::fmt::{Display, Formatter};

use lox_syntax::ast::expr::Value;

pub enum RuntimeValue {
    Nil,
    String(String),
    Number(f64),
    Boolean(bool),
}

impl RuntimeValue {
    pub fn is_truthy(&self) -> bool {
        use RuntimeValue::*;

        match self {
            Nil => false,
            Boolean(b) => *b,
            _ => true,
        }
    }
}

impl From<&Value> for RuntimeValue {
    fn from(ast_value: &Value) -> Self {
        match ast_value {
            Value::Nil => RuntimeValue::Nil,
            Value::String(s) => RuntimeValue::String(s.clone()),
            Value::Boolean(b) => RuntimeValue::Boolean(*b),
            Value::Number(n) => RuntimeValue::Number(*n),
        }
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use RuntimeValue::*;
        match self {
            Nil => f.write_str("nil"),
            String(s) => write!(f, "\"{}\"", s),
            Number(n) => write!(f, "{}", n),
            Boolean(b) => write!(f, "{}", b),
        }
    }
}
