use std::{collections::{HashMap, HashSet}, rc::{Rc, Weak}, cell::RefCell};

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
//  VARIABLES
//


// Used to store a variable in the heap
#[derive(Clone, Debug)]
pub struct Variable {
    pub value: VariableType,
    pub instructions: Option<Box<Function>>, //if this is Some, then "value"'s held value is disregarded in favor of "instruction"'s return value
}


// Utilized by the Variable struct
#[derive(Clone, Debug)]
pub enum VariableType {

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
    Dict(Box<HashMap<Rc<RefCell<Variable>>,Rc<RefCell<Variable>>>>),
    List(Box<List>),
    Set(Box<HashSet<Rc<RefCell<Variable>>>>),
    Struct(Box<Vec<Rc<RefCell<Variable>>>>),
}


// Utilized by the Variable struct
#[derive(Clone, Debug)]
pub struct Function {
    pub instructions: Vec<String>,  //contains all the instructions
    pub scope: SmartRc<RefCell<Scope>>, //must dynamically change between strong and weak
}

// Utilized by the Variable struct
#[derive(Clone, Debug)]
pub struct List {
    data: Vec<Rc<RefCell<Variable>>>,
    contains: PointerType,
}



//
// POINTERS
//


// Stored in a Scope, points to a Variable in the heap
#[derive(Clone, Debug)]
pub struct Pointer {
    pub var_type: PointerType,
    pub var_index: SmartRc<RefCell<Variable>>,  //must change between strong and weak
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
