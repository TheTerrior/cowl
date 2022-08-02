use crate::memory::{VarType, Variable};

// The data type that instructions are represented as, results in a tree
// Note: the type of instruction dictates whether the body is a single instruction or a stream of instructions (a block)
struct Instruction {
    opcode: i8,
    body: Instruction_Body,

}

// Used by the Instruction struct, more complex instructions will use the Block type rather than the Stream type
enum Instruction_Body {
    Single(Vec<bool>),
    Stream(Vec<Instruction>),
}

// TODO
// Will receive the bit stream and pop from it as needed to construct instructions; this function acts as this file's main function
// Produces an Instruction tree that we then pass to execute()
// Note: bit stream parameter is reversed (first bit is at vector end) so that we can pop efficiently
pub fn process_stream(mut bits: Vec<bool>) {
    let mut instr: Vec<bool> = Vec::new();  //contains the newly popped instruction
    let mut instr_tree: Vec<Instruction> = Vec::new();  //contains the tree of instructions (organizes itself as instructions are popped)
    loop { //loop to collect all the instructions, filling up the Instruction tree as we go
        
        instr = bits.split_off(bits.len() - 8); //collect the next opcode
    }
}

// TODO
// Take in a Memory struct and a target function to run
fn execute(program: Instruction_Body, function: i32) -> Option<Variable> {
    return None;
}