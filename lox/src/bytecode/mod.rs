use bytecode::{Chunk, OpCode, Value};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut chunk = Chunk::new();
    let i = chunk.push_constant(Value::Number(1.0));
    chunk.push_op(OpCode::Constant(i));
    chunk.push_op(OpCode::Return);
    chunk.disassemble("test chunk");

    Ok(())
}
