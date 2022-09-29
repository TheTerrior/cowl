//mod memory;
//mod alu;
mod args;
mod memory;

use args::Args;
use clap::Parser;
use std::{rc::Rc, cell::{RefCell, Cell}};

#[derive(Clone, Copy, Debug)]
pub struct TestStruct {
    pub a: i32,
    pub b: usize,
}

// Main function
fn main() {
    //let args: Args = Args::parse();
    let mut z = TestStruct{
        a: 3,
        b: 3,
    };
    
    let a = Rc::new(RefCell::new(vec![1, 2, 3, 4]));
    let b = Rc::clone(&a);
    let c = Rc::clone(&b);

    a.borrow_mut()[0] = 10;
    a.borrow_mut()[2] = 12;

    println!("{:?}", a.borrow());
    


}