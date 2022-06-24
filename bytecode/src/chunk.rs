use crate::value::Value;
use std::fmt::Display;

#[derive(Default)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Default::default()
    }
    pub fn push_op(&mut self, op: OpCode) {
        self.code.push(op);
    }

    pub fn push_constant(&mut self, constant: Value) -> u8 {
        assert!(self.constants.len() < 256);

        self.constants.push(constant);
        self.constants.len() as u8 - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        print!("{}", self);
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpCode::*;
        for (i, op) in self.code.iter().enumerate() {
            write!(f, "{:04} ", i)?;
            match op {
                Return => writeln!(f, "OP_RETURN"),
                Constant(i) => writeln!(
                    f,
                    "{:16}{:4} '{}'",
                    "OP_CONSTANT", i, self.constants[*i as usize]
                ),
            }?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Return,
    Constant(u8),
}
