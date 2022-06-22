package main

import (
	"fmt"
	"os"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

// convert the string of 1's and 0's into bytes to save into the bytecode file
func string_to_bytes(input string) []byte {
	var result int = len(input) % 8
	var ret []byte
	if result == 0 {
		ret = make([]byte, len(input) / 8)
	} else {
		ret = make([]byte, len(input) / 8 + 1)
	}

	/*
	for i := 0; i < len(input); i++ {
		if (input[i] != byte() && input[i]) != "1" {
			fmt.Println("ERROR: Compiler received values other than 0 and 1")
			panic("Compiler error")
		} 
	}
	*/

	var bytes []byte = []byte(input) //each byte here represents a single character, either "0" or "1"
	fmt.Println(bytes)

	return ret
}

// see main.rs:process_arguments() for more details, these two functions overlap greatly
func process_arguments() {
	var args []string = os.Args
	
	//var target string //target file we'll be compiling
	//var output string //output file of our compiled bytecode
	
	//var mode int32 = 0
	var help bool = false
	var version bool = false
	//var too_many_args bool = true

	/*
        mode:
        0 -> default; flags or code directory
    */

	for i := 0; i < len(args); i++ {
		if i == 0 {
			continue
		}

		if args[i] == "-h" || args[i] == "--help" {
			help = true
		}
		if args[i] == "-V" || args[i] == "--version" {
			version = true
		}
	}

	// I have no idea why Go acts this weird with multiline strings
	if help {
        fmt.Println(
`Cowl's compiler

USAGE:
	cowlc [OPTIONS] [TARGET FILE]
            
OPTIONS:
	-o [FILE]	Output the compiled bytecode into [FILE]
    	-h, --help	Print help information (this message), and exit
    	-O, --optimize	Compile into a larger bytecode file that runs faster
    	-V, --version	Print version info and exit`)
        return
    }
	if version {
		fmt.Println("cowlc 1.0.0")
		return
	}

}

func main() {
	process_arguments()

	testing()
	

}

func testing() {
	var args []string = os.Args[1:]
	fmt.Println(os.Args[0])
	fmt.Println(args)

	fmt.Println("wassup")
	file, err := os.Create("../cowltool/src/test.cowlc")
	check(err)
	defer file.Close()

	//var text string = "a"
	//var arr []byte = []byte{115, 111, 109, 101, 10, 255}
	var bytes_string string = "010101011"
	var bytes []byte = string_to_bytes(bytes_string)
	var thing []byte = []byte{1};

	//file.WriteString(text)
	//file.Write(arr)
	file.Write(thing)
	file.Write(bytes)
	file.Sync()
	fmt.Println(len(bytes))
}