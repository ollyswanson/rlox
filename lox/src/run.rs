use std::fs;

use lox_syntax::Parser;
use tree_walk::Interpreter;

use crate::args::Args;

pub fn run_lox(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = args.script {
        run_source(&path)
    } else {
        run_repl()
    }
}

fn run_source(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let mut interpreter = Interpreter::new();

    run(&source, &mut interpreter)
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = crate::tree_walk::Repl::new();
    repl.start()?;

    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new(source);
    let statements = parser.parse();
    interpreter.interpret(&statements)?;

    Ok(())
}
