use crate::tree_walk;

use crate::args::Args;

pub fn run_lox(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = args.script {
        tree_walk::run_source(&path)
    } else {
        tree_walk::run_repl()
    }
}
