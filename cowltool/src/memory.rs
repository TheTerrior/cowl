use std::{collections::{HashMap, HashSet}, rc::Rc, cell::RefCell};





//
// SCOPE
//


// Holds a list of Pointers
#[derive(Clone, Debug)]
pub struct Scope {
    pub parent: Option<Rc<RefCell<Scope>>>,     //all Scopes have a previous Scope except for the root
    pub pointers: Vec<Pointer>,                 //a Scope owns its set of Pointers, even though the Variables that the Pointers point to are shared
}



//
//  VARIABLES
//


// Used to store a variable in the heap
#[derive(Clone, Debug)]
pub struct Variable {
    pub value: VarType,
    pub instructions: Option<Function>, //if this is Some, then "value"'s held value is disregarded in favor of "instruction"'s return value
}


// Utilized by the Variable struct
#[derive(Clone, Debug)]
pub enum VarType {

    //simple types
    Bool(bool),
    Char(char),
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),

    //complex types, generally contain pointers to other data in the memory
    Array(Vec<Rc<RefCell<Variable>>>),
    Dict(HashMap<Rc<RefCell<Variable>>,Rc<RefCell<Variable>>>),
    List(Vec<Rc<RefCell<Variable>>>),
    Set(HashSet<Rc<RefCell<Variable>>>),
    String(Vec<char>),                      //a vector of chars acts the same as runes, allows better flexibility
    Struct(Vec<Rc<RefCell<Variable>>>),
}


// Utilized by the Variable struct
#[derive(Clone, Debug)]
pub struct Function {
    pub instructions: Vec<String>,  //contains all the instructions
    pub scope: Rc<RefCell<Scope>>,
}



//
// POINTERS
//


// Stored in a Scope, points to a Variable in the heap
#[derive(Clone, Debug)]
pub struct Pointer {
    pub var_type: PointerType,
    pub var_index: Rc<RefCell<Variable>>,
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
    Array,  //internally equivalent to list
    Dict,
    List,
    Set,
    String, //internally equivalent to list

    //special types
    Func,
    Struct, //internally equivalent to list

    //weak types
    Dyn,    //will point to any variable type
    Var,    //once type is set, does not change

    //function types
    Void,   //does not point to anything
}
