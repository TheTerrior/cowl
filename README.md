# The Cowl Programming Language
See General Information below for more info about this language.

## General Information
This language is being written in my free time, just for fun and to gain programming experience.

Cowl aims to be a general-purpose, statically-typed language that is easy to learn for new programmers. The language is compiled to bytecode, which is then interpreted. Syntax is reminiscent of C. Cowl files are stored as either .cowl or .cl, and Cowl bytecode is saved as a .cowlc or .clc file.

The interpreter will use Rust, whereas the bytecode compiler will use Haskell. These compiled languages will ensure high performance.

Notes:
A true compiler may come in the future if I'm willing to take on the task, but I have no knowledge on how to make a compiler and I'm currently the only one working on this language.

## Installation
Prerequesites: cargo, ghc 

If you'd like to check for any errors, you may run this command:
```
    make check
```
This will run unoptimized build scripts for both components, just to ensure everything builds as it should.

To build the components for release, simply run this command:
```
    make build
```
To install the components, run this command:
```
    sudo make install
```