//mod memory;
//mod alu;
mod args;

use args::Args;
use clap::Parser;

// Main function
fn main() {
    let args: Args = Args::parse();
    println!("{:?}", args.read_bits);
    println!("{:?}", args.target);
}