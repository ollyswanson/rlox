use bytecode::{Chunk, OpCode};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut chunk = Chunk::new();
    chunk.push_op(OpCode::Return);
    chunk.disassemble("test chunk");

    Ok(())
}
