# The Cowl Programming Language

## General Information
Cowl aims to be a general-purpose, strongly-typed language with a C-like syntax that is easy to learn for new programmers. Cowl is compiled to bytecode and interpreted using the cowlc and cowl tools provided in this repository. Associated filetypes are .cowl and .cl for Cowl code, and .cowlc and .clc for Cowl bytecode.

Variable types can be dynamic, declared, or inferred. Internally, variables are implemented as pointers to variable objects in memory, and as such no variable truly owns its value. This form of data representation differs from most programming languages today, however I believe this improves consistency in how simple types (like integers) and complex types (like vectors/lists) are handled. All variables can be passed by value or by reference, no matter the type.

The interpreter and bytecode compiler will be written in Rust and Go respectively. These two languages will ensure high performance and cross-platform compatibility. Cowl will be available for Windows, MacOS, and Linux.

Notes:<br>
A machine code compiler may come in the future if I'm willing to take on the task, but currenly I'm the solo developer of this project and I have no knowledge on building compilers. A possible way to generate machine code would be via LLVM, but that's for future experimentation.

## Installation
Prerequesites: cargo, go

If you'd like to check for any errors, you may run this command.
```
make check
```

To build and install the components, simply run these commands:
```
make build
sudo make install
```
