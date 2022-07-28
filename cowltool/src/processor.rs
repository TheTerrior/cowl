use std::net::TcpStream;

use crate::memory::{VarType, Variable};

// The form an instruction will be saved as in the processor, variable type will influence the body type
struct Instruction {
    opcode: i8,
    body: Box<Instruction_Body>,

}

// Used by the Instruction struct, more complex instructions will use the Block type rather than the Stream type
enum Instruction_Body {
    Stream(Vec<bool>),
    Block(Vec<Instruction>),
}

// Will receive the bit stream, and will pop from it as needed to construct instructions (acts as this file's main function)
// Note: bit stream is reversed (first bit is at vector end) so that we can pop efficiently
pub fn process_stream(bits: Vec<bool>) {
    
}

// Take in a list of Instructions and execute them
fn execute(program: Instruction_Body) {

    //filter out an invalid case
    if let Instruction_Body::Stream(_) = program {
        //we should not be here, something did not go to plan
        return;
    }
    
    //get the instruction list
    if let Instruction_Body::Block(instructions) = program {
        //execute the code here
    }
}