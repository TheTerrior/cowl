mod smartlist;
mod memory;
mod alu;
mod processor;

use smartlist::SmartList;
use std::fs;
use std::env;

// entry point for the interpreter, should receive some arguments
fn main() {
    let args: Vec<String> = env::args().collect();
    //let contents = fs::read();
    let mut target: Option<String> = None;
    let mut help: bool = false;
    let mut mode: i32 = 0; //represents what we're looking for in our arguments, like a directory or something
    let mut too_many_args: bool = false;
    /*
        mode:
        0 -> default, flags or bytecode directory
        1 -> 
    */

    for i in 0..args.len() { //iterate through all args
        if i == 0 { //skip the first arg for now, may change this behavior later
            continue;
        }

        if args[i].as_str() == "-h" || args[i].as_str() == "--help" { //no matter the mode or input, if user requests help then it overrides
            help = true;
            break;
        }

        match mode { //for futureproofing
            _ => {
                match target {
                    None => {
                        target = Some(args[i].clone()); //target directory has not been set
                    },
                    _ => {
                        too_many_args = true;
                        break;
                    },
                }
            }
        }
    }
    
    if help {
        println!("{}",
            "\
            Cowl's bytecode interpeter\n\
            \n\
            USAGE:\n\
            \tcowl [OPTIONS] [TARGET FILE]\n\
            \n\
            OPTIONS:\n\
            \t-h, --help\tPrint help information (this message), and exit\n
            \t-V, --version\tPrint version info and exit\n\
            "
        );
        return;
    }
    if too_many_args {
        println!("Invalid arguments");
        return;
    }

    //check the target to see if it's valid

    //if nothing caused the interpreter to stop by now, then we're free to start interpreting the input file

}

/* 

fn test_smartlist() {
    let mut sl: SmartList<i32> = SmartList::new();
    println!("{:?}", sl.add(4));
    println!("{:?}", sl.add(7));
    println!("{:?}", sl.add(8));
    let mut status = sl.remove(1);
    println!("{:?}", status);
    println!("{:?}", &sl.data);
    println!("{:?}", sl.add(10));
    println!("{:?}", sl.add(17));
    println!("{:?}", &sl.data);
    status = sl.remove(1);
    println!("{:?}", status);
    status = sl.remove(0);
    println!("{:?}", status);
    status = sl.remove(2);
    println!("{:?}", status);
    status = sl.remove(3);
    println!("{:?}", status);
    status = sl.remove(3);
    println!("{:?}", status);
    status = sl.remove(4);
    println!("{:?}", status);
    println!("{:?}", &sl.data);
}
*/