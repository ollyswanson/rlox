use std::fs;
use std::io;
use std::io::{BufRead, Write};

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
    run(&source)
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    stdout.lock().write_all("> ".as_ref())?;
    stdout.lock().flush()?;

    for line in io::stdin().lock().lines() {
        run(&line?)?;

        stdout.lock().write_all("> ".as_ref())?;
        stdout.lock().flush()?;
    }

    Ok(())
}

fn run(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", source);
    Ok(())
}
