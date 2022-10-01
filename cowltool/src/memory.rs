use std::{collections::{HashMap, HashSet}, rc::{Rc, Weak}, cell::RefCell};

/*

DESCRIPTION

This file creates the entire memory model. 
This layer is abstracted by the ALU.

*/



//
// GENERIC
//


// Allows types to implement either a strong or weak Rc
#[derive(Clone, Debug)]
pub enum SmartRc<T> {
    Strong(Rc<T>),
    Weak(Weak<T>),
}



//
// SCOPE
//


// Holds a list of Pointers
#[derive(Clone, Debug)]
pub struct Scope {
    pub parent: Option<Rc<RefCell<Scope>>>,     //all Scopes have a previous Scope except for the root
    pub pointers: Vec<Pointer>,                 //a Scope owns its set of Pointers, even though the Variables that the Pointers point to are shared
}

pub struct StackFrame {
    pub scope: Rc<RefCell<Scope>>,
}



//
// POINTERS
//


// Stored in a Scope, points to a Variable in the heap
#[derive(Clone, Debug)]
pub struct Pointer {
    pub var_type: PointerType,
    pub var_ref: SmartRc<RefCell<Variable>>,  //must change between strong and weak
}


// Utilized by the Pointer struct
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PointerType {
    
    //simple types
    Bool,
    Char,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,

    //complex types
    Array,
    Dict,
    List,
    Set,
    String,
    Struct,

    //weak types
    Dyn,    //will point to any variable type
    Var,    //once type is set, does not change

    //function types
    Void,   //does not point to anything
}



//
//  VARIABLES
//


// Used to store a value in the heap
#[derive(Clone, Debug)]
pub enum Variable {

    //simple types
    Bool(Box<bool>),
    Char(Box<char>),
    Float32(Box<f32>),
    Float64(Box<f64>),
    Int8(Box<i8>),
    Int16(Box<i16>),
    Int32(Box<i32>),
    Int64(Box<i64>),
    UInt8(Box<u8>),
    UInt16(Box<u16>),
    UInt32(Box<u32>),
    UInt64(Box<u64>),

    //complex types
    Dict(Box<Dictionary>),
    List(Box<List>),
    Set(Box<Set>),
    Struct(Box<Struct>),

    //special types
    Func(Box<Function>),
}

// Defines a Dictionary type
#[derive(Clone, Debug)]
pub struct Dictionary {
    pub contains0: PointerType,
    pub contains1: PointerType,
    pub data: HashMap<Rc<RefCell<Variable>>,Rc<RefCell<Variable>>>,
}

// Defines a List type, which also provides functionality for String and Array
#[derive(Clone, Debug)]
pub struct List {
    pub contains: PointerType,
    pub data: Vec<Rc<RefCell<Variable>>>,
}

// Defines a Set type
#[derive(Clone, Debug)]
pub struct Set {
    pub contains: PointerType,
    pub data: HashSet<Rc<RefCell<Variable>>>,
}

// Defines a Struct type
#[derive(Clone, Debug)]
pub struct Struct {
    pub contains: Vec<PointerType>,
    pub data: Vec<Rc<RefCell<Variable>>>,
}

// Defines a Function type
#[derive(Clone, Debug)]
pub struct Function {
    pub scope: SmartRc<RefCell<Scope>>, //must dynamically change between strong and weak
    pub instructions: Vec<String>,  //contains all the instructions
}



