mod smartlist;
mod memory;
mod alu;
mod processor;

use std::fs;
use std::env;
use processor::execute;

// given a u8, return all 8 bits as a vector of bools
fn read_bits(input: u8) -> Vec<bool> {
    //let mut bits: Vec<bool> = Vec::from([false, false, false, false, false, false, false, false]); //kind of verbose, but with_capacity wasn't working
    let mut bits: Vec<bool> = vec![false; 8]; //initialize a vector of length 8 with all falses
    for i in 0..8 {
        bits[i] = input & (1 << i) != 0;
    }
    return bits;
}

// read the given file with the given flags
fn read_file(target: String, read: bool, _: bool) {
    println!("{:?}", target);
    let contents: Vec<u8> = fs::read(target).expect("Something went wrong reading the file");
    let mut bits: Vec<bool> = Vec::with_capacity(contents.len() * 8);

    for i in (0..contents.len()).rev() {
        bits.append(&mut read_bits(contents[i]));
    }

    //if user requested to read the bits, then output the bits and end
    if read {
        println!("{:?}", bits);
        return;
    } else {    //if user did not request a read, feed the bits to processor::execute
        execute(bits);
    }
}

// called by main, collects the arguments and processes them
fn process_arguments() {
    let args: Vec<String> = env::args().collect();

    let mut target: Option<String> = None; //target file we'll be interpreting
    
    let mode: i32 = 0;
    let mut too_many_args: bool = false; //have we received too many arguments?
    let mut help: bool = false; //has the user requested help?
    let mut version: bool = false; //has the user requested the version info?
    let mut read: bool = false; //has the user requested to read the instructions?
    let mut memconv: bool = false; //will conserve memory at the cost of performance, currently on by default until (if) implemented

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
        if args[i].as_str() == "-r" || args[i].as_str() == "--read" { //only takes action if a file is provided
            read = true;
            continue;
        }
        if args[i].as_str() == "-r" || args[i].as_str() == "--read" { //only takes action if a file is provided
            read = true;
            continue;
        }
        if args[i].as_str() == "-m" || args[i].as_str() == "--low-memory" { //only takes action if a file is provided
            memconv = true;
            continue;
        }

        match mode { //for futureproofing
            _ => {
                match target {
                    None => {
                        target = Some(args[i].clone()); //target directory has not been set
                    },
                    _ => {
                        too_many_args = true;
                        continue;
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
            \t-h, --help\tPrint help information (this message) and exit\n\
            \t-V, --version\tPrint version info and exit\n\
            \t-r, --read\tPrint the encoded instructions in the given file as text and exit\n\
            "
            // Im going to break your code
            // damn, i already backed it up to github sadge
            // fuck
            //alright so this works, im glad, i fixed stuff on my linux
            // macos >> linux
            /*operator >> {
                redefine <<
            }*/
            // oh you think you're clever
            //yes actually
            // frick this, i'm leaving
            // o/
            // lmao

            //I see people have talked here so I'm leaving my mark too
            //- Mr Freeze
        );
        return;
    }
    if version {
        println!("cowl 1.0.0");
        return;
    }
    if too_many_args {
        println!("ERROR: Invalid arguments");
        return;
    }

    //haven't run into any weird cases, run the target
    match target {
        None => {
            println!("ERROR: Requested to read instructions but no file was given");
        },
        _ => {
            read_file(target.unwrap(), read, memconv);
        }
    }
    return;
}

// entry point for the interpreter, immediately calls process_arguments
fn main() {
    process_arguments();
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
