use lox::args::get_args;
use lox::run::run_lox;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = get_args();
    run_lox(args)
}
