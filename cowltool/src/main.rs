mod args;
mod memory;
mod alu;

use args::Args;
use clap::Parser;
use std::fs;

use crate::memory::{Variable};

// Reads the target file, 
fn read_file(target: &str) -> Vec<u8> {
    return fs::read(target).expect("Something went wrong reading the file");
}

// Will print out all the individual bits
fn print_bits(bytes: &Vec<u8>) {
    let mut output_str: String = String::with_capacity(bytes.len() * 8);
    for byte in 0..bytes.len() {
        for pos in 0..8 {
            let bit = (bytes[byte] & (1 << pos) != 0) as i32; //if we get anything other than a 0, then set the bit to true
            output_str += bit.to_string().as_str();
        }
    }
    println!("{}", output_str);
}

// Main function
fn main() {
    let args: Args = Args::parse();
    let bytes: Vec<u8> = read_file(&args.target);

    if args.read_bits {
        print_bits(&bytes);
    }
}