use std::fs;
use std::io;
use std::io::{BufRead, Write};

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
    let mut interpreter = Interpreter::new();
    let stdout = io::stdout();
    stdout.lock().write_all("> ".as_ref())?;
    stdout.lock().flush()?;

    for line in io::stdin().lock().lines() {
        match run(&line?, &mut interpreter) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        };

        stdout.lock().write_all("> ".as_ref())?;
        stdout.lock().flush()?;
    }

    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new(source);
    let statements = parser.parse();
    interpreter.interpret(&statements)?;

    Ok(())
}
