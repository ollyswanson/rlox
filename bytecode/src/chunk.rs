use std::{fmt::Display, ops::Deref};

#[derive(Default)]
pub struct Chunk(Vec<OpCode>);

impl Chunk {
    pub fn new() -> Chunk {
        Default::default()
    }
    pub fn push_op(&mut self, op: OpCode) {
        self.0.push(op);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        print!("{}", self);
    }
}

impl Deref for Chunk {
    type Target = [OpCode];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[OpCode]> for Chunk {
    fn as_ref(&self) -> &[OpCode] {
        &self.0
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, op) in self.iter().enumerate() {
            writeln!(f, "{:04} {}", i, op)?
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Return,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpCode::*;

        match self {
            Return => f.write_str("OP_RETURN"),
        }
    }
}
