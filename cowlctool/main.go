package main

import (
	"fmt"
	"os"
	"strconv"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

// Convert the string of 1's and 0's into bytes to save into the bytecode file
func string_to_bytes(input string) []byte {
	var ret []byte                      //Return value
	var retCrawler int = 0              //Current byte position in the return array
	var isDangling bool = false         //Boolean to determine if values are dangling
	var danglingAmount = len(input) % 8 //The amount of dangling values to parse

	//Allocate a byte array based on the size of string, create one extra byte for any dangling values
	if danglingAmount == 0 {
		ret = make([]byte, len(input)/8)
	} else {
		ret = make([]byte, len(input)/8+1)
		isDangling = true
	}

	//Iterate through the entire length of the input string
	for inputCrawler := 0; inputCrawler < len(input); {
		var splicedInput string = ""
		var amountToParse int = 8 //Tell the crawler how many bits to parse, defaulting to 8

		//Only when reaching the last byte check to see if the amount to parse needs to be reduced
		if inputCrawler/8 == len(input)/8 && isDangling {
			//If there is a dangling value parse only that amount instead
			///fmt.Println("Excess: ", danglingAmount)
			amountToParse = danglingAmount
		}

		//Parse a given amount of values from the input string into a temporary splicedInput value
		for i := 0; i < amountToParse; i++ {
			if input[inputCrawler] != byte(0) && input[inputCrawler] != byte(1) { //Check if the current character is a 1 or 0 byte
				///fmt.Print(input[inputCrawler], " ")
				splicedInput += string(input[inputCrawler]) //Append the valid values to a spliced input
			} else {
				fmt.Println("ERROR: Compiler received values other than 0 and 1") //Otherwise throw an error and terminate the program
				panic("Compiler error")
			}
			inputCrawler++
		}
		///fmt.Println("\nSPLICED INPUT:", splicedInput)

		// if not a normal sized byte (8 bits) pad with zeros to fill remaining space
		if amountToParse < 8 {
			for i := 0; i < 8-amountToParse; i++ {
				splicedInput += "0"
			}
			///fmt.Println("PADDED INPUT:", splicedInput)
		}

		//Convert a size 8 splicedInput into a Uint8 value using strconv.ParseUint
		if i, err := strconv.ParseUint(splicedInput, 2, 8); err != nil { //strconv.ParseInt(string, base, bitsize)
			check(err)
		} else {
			//fmt.Printf("%T, %v\n", uint8(i), uint8(i))
			ret[retCrawler] = byte(uint8(i)) //i value is int64 by default from ParseUint
			retCrawler++
		}
	}
	///fmt.Println(ret)
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
	//Testing code for ARGS
	///var args []string = os.Args[1:]
	///fmt.Println(os.Args[0])
	///fmt.Println(args)

	file, err := os.Create("../cowltool/src/test.cowlc")
	check(err)
	defer file.Close()

	var bytes_string string = "011000010110001001100011"
	var bytes []byte = string_to_bytes(bytes_string)

	file.Write(bytes)
	file.Sync()
	///fmt.Println(len(bytes))

	//Testing concurrent writes:
	//var bytes_string_secondary string = "011000010110001001100011001100010011001000110011010000010100001001000011001000010100000000100011"
	//bytes = string_to_bytes(bytes_string_secondary)
	//file.Write(bytes)
	//file.Sync()

	/*TESTING OPTIONS:
	01100001 = a
	011000010110001001100011 = abc
	011000010110001001100011001100010011001000110011010000010100001001000011001000010100000000100011 = abc123ABC!@#
	010101011 = Initial Test String (Garbage)
	011000010110001001100011111 = abc + [111] excess bytes
	*/
}
