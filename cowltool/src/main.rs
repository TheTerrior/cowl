mod smartlist;
mod memory;
mod alu;
mod processor;

use smartlist::SmartList;
use std::fs;
use std::env;

fn process_arguments() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    let mut target: Option<String> = None; //target file we'll be interpreting
    
    let mode: i32 = 0;
    let mut help: bool = false; //has the user requested help?
    let mut version: bool = false; //has the user requested the version info?
    let mut too_many_args: bool = false; //have we received too many arguments?

    /*
        mode: represents what we're currently searching for in our arguments, like a directory or the second half of a flag
        0 -> default; flags or bytecode directory
    */

    //find any important flags and collect info
    for i in 0..args.len() { //iterate through all args
        if i == 0 { //skip the first arg for now, may change this behavior later
            continue;
        }

        if args[i].as_str() == "-h" || args[i].as_str() == "--help" { //no matter the mode or input, if user requests help then it overrides
            help = true;
            break;
        }
        if args[i].as_str() == "-V" || args[i].as_str() == "--version" { //no matter the mode or input, if user requests help then it overrides
            version = true;
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
    
    //handle all the individual cases
    if help {
        println!("{}",
            "\
            Cowl's interpeter\n\
            \n\
            USAGE:\n\
            \tcowl [OPTIONS] [TARGET FILE]\n\
            \n\
            OPTIONS:\n\
            \t-h, --help\tPrint help information (this message), and exit\n\
            \t-V, --version\tPrint version info and exit\n\
            "
        );
        return None;
    }
    if version {
        println!("cowl 1.0.0")
    }
    if too_many_args {
        println!("ERROR: Invalid arguments");
        return None;
    }

    return target;
}

// entry point for the interpreter, should receive some arguments
fn main() {
    process_arguments();

    test_bytecode_read();

    //check the target to see if it's valid

    //if nothing caused the interpreter to stop by now, then we're free to start interpreting the input file

}

fn test_bytecode_read() {

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