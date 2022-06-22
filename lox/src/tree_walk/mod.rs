use std::fs;
use std::path::Path;

use lox_syntax::Parser;
use tree_walk::{Interpreter, Resolver};

mod repl;

pub fn run_source(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;

    let mut interpreter = Interpreter::new();
    let mut parser = Parser::new(&source);
    let statements = parser.parse();
    let diagnostics = parser.diagnostics();

    if !parser.diagnostics().is_empty() {
        for diagnostic in diagnostics.iter() {
            eprintln!("{}", diagnostic);
        }
        return Ok(());
    }

    let mut resolver = Resolver::new(&mut interpreter);
    resolver.resolve(&statements);

    if !resolver.diagnostics().is_empty() {
        for diagnostic in resolver.diagnostics().iter() {
            eprintln!("{}", diagnostic)
        }
        return Ok(());
    }

    match interpreter.interpret(&statements) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

pub fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = repl::Repl::new();
    repl.start()?;

    Ok(())
}
