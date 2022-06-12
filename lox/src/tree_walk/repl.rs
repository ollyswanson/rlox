use std::io;
use std::io::{BufRead, Write};

use lox_syntax::parser::ParserState;
use lox_syntax::Parser;
use tree_walk::{Interpreter, Resolver};

#[derive(Debug, Default)]
pub struct Repl {
    interpreter: Interpreter,
    /// src accumulator to allow multiline input
    curr_src: String,
    parser_state: ParserState,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            curr_src: "".to_owned(),
            parser_state: ParserState::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stdout = io::stdout();
        stdout.lock().write_all(">> ".as_ref())?;
        stdout.lock().flush()?;

        for line in io::stdin().lock().lines() {
            self.run_line(&line?);

            stdout.lock().write_all(">> ".as_ref())?;
            stdout.lock().flush()?;
        }
        Ok(())
    }

    fn run_line(&mut self, line: &str) {
        let mut parser = Parser::new(line).with_state(self.parser_state);
        let statements = parser.parse();
        let diagnostics = parser.diagnostics();

        if !parser.diagnostics().is_empty() {
            for diagnostic in diagnostics.iter() {
                eprintln!("{}", diagnostic);
            }
            return;
        }

        self.parser_state = parser.state();

        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve(&statements);

        if !resolver.diagnostics().is_empty() {
            for diagnostic in resolver.diagnostics().iter() {
                eprintln!("{}", diagnostic)
            }
            return;
        }

        match self.interpreter.interpret(&statements) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        }
    }
}
