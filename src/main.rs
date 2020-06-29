//! # Proposed command syntax
//! dnd d4
//! dnd [1-9]dxx
//! dnd [1-9]dxx+x
//! dnd [1-9]dxx+x advantage

use args::CLIArgs;
use structopt::StructOpt;

mod args;

fn main() {
    let args = CLIArgs::from_args();
    println!("args: {:?}", args);
}