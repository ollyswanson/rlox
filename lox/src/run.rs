use crate::bytecode;
use crate::tree_walk;

use crate::args::Args;

pub fn run_lox(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    match (args.tree_walk, args.script) {
        (true, Some(path)) => tree_walk::run_source(&path),
        (true, None) => tree_walk::run_repl(),
        (false, _) => bytecode::run(),
    }
}
