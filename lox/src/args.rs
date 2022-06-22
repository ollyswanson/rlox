use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "rlox")]
#[clap(author = "Olly Swanson <olly.swanson95@gmail.com")]
#[clap(
    about = "Lox interpreter written in Rust",
    long_about = "A Rust-based implementation of the tree-walk and bytecode interpreters for the \
    programming language Lox (\"Crafting Interpreters\")"
)]
pub struct Args {
    /// Script to run
    pub script: Option<PathBuf>,
}

pub fn get_args() -> Args {
    Args::parse()
}
