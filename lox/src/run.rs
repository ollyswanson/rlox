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
        run(&line?, &mut interpreter)?;

        stdout.lock().write_all("> ".as_ref())?;
        stdout.lock().flush()?;
    }

    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new(source);
    let expr = parser.parse_expr();

    match expr {
        Ok(e) => {
            let value = interpreter.evaluate_expr(&e);
            println!("{}", value.unwrap())
        }
        Err(e) => {}
    }

    Ok(())
}
