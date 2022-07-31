# The Cowl Programming Language

## General Information
Cowl is an in-progress project. It is currently not ready to be built nor used in any capacity, however many parts have either been completed or planned out. This note will be updated when the language is ready.

Cowl aims to be a general-purpose, strongly-typed language with a C-like syntax that is easy to learn for beginners. Cowl is an interpreted language that utilizes bytecode interpreted by a VM, much like Java. Code is saved as a .cowl file, and compiled bytecode is saved as a .cowlc file.

Variable types can be declared or inferred at compile-time, or dynamic in nature. While many languages store simple types like integers in the stack and complex types like objects in the heap, Cowl stores everything in the heap. While this does sacrifice performance, it allows more flexibility in terms of passing variables by reference or by value, it simplifies how pointers work, and ultimately allows for memory safety via reference counting. No value is owned by any particular variable, but rather all variables are pointers to some value in memory. Once all pointers to a value are out of scope, the value is deallocated. This form of data representation differs from most programming languages today, however I believe this system is more intuitive and improves consistency. All variables can be passed by value or by reference, no matter the type.

Cowl is compiled using the cowlc tool. Code is compiled from its raw form into bytecode, which is consistent across platforms and can be copied to other systems. The compiler will be written using Go. This bytecode can then be interpreted using the cowl tool. This is a VM written in Rust which reads the instructions in the bytecode. The two implementation languages of choice will ensure high performance and cross-platform compatibility. Cowl itself will be available for Windows, MacOS, and Linux.

Notes:<br>
A machine code compiler may come in the future if I'm willing to take on the task. Currently, no developers working on this project have any knowledge on building compilers, however a possible way forward would be to utilize LLVM, however this is currently not in our plans and we're instead prioritizing a working product.

## Installation
Once Cowl is ready to deploy, build and install instructions will be provided here.

## Misc
The name for Cowl is inspired by a cowl neck, sometimes present on sweaters and other similar articles of clothing.