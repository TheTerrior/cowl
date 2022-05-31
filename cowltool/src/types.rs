use std::collections::HashMap;
use crate::smartlist::SmartList;

/*
TODO
boost the pointer type from i32 to i128 to allow for a much greater number of elements and variables
*/


// holds both the stack and the heap, where call frames can be popped once the frame is out of scope
pub struct Memory {
    pointer_stack: Vec<HashMap<u32, Pointer>>, //stack frames of pointers
    call_stack: Vec<HashMap<u32, Variable>>, //stack frames of variable values
    cur_pointer: u32, //decides what new name we can assign to a new pointer
    cur_call: u32, //decides what new name we can assign to a new variable
    //heap: Vec<Variable>, //may be deprecated later
}

impl Memory {
    
    // create a new memory instance (one per interpreter)
    fn new() -> Memory {
        return Memory {
            pointer_stack: Vec::new(),
            call_stack: Vec::new(),
            cur_pointer: 0,
            cur_call: 0,
        };
    }

    // returns mutable reference to top of call_stack
    fn cur_call_stack(&mut self) -> &mut HashMap<u32, Variable> {
        let len = self.call_stack.len();
        return &mut self.call_stack[len];
    }

    // returns mutable reference to top of variable_stack
    fn cur_variable_stack(&mut self) -> &mut HashMap<u32, Pointer> {
        let len = self.pointer_stack.len();
        return &mut self.pointer_stack[len];
    }

    fn pop_call_stack(&mut self) {

    }

    // initialize a new empty pointer
    fn create(&mut self) -> u32 {
        let new_var = Pointer {
            var_type: VarTypeRaw::Dyn,
            data: None,
        };
        let index: u32 = self.cur_pointer.clone();
        self.cur_variable_stack().insert(index, new_var);
        return index;
        
    }
}

// stored in virtual memory, used to hold information about a variable's information
struct Variable {
    data: VarType, //stores the type of variable and the data
    pointers: Vec<u32>, //keeps a list of all the pointers that refer to this variable
}

// utilized by the Variable struct
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
    Int128(i128),

    //complex types, generally contain pointers to other data in the memory
    Array(Vec<u32>),
    Dict(HashMap<u32,i32>),
    List(Vec<u32>),
    Struct, //tba
    String(Vec<u32>),
}

// utilized by the Pointer struct
pub enum VarTypeRaw {
    //simple types
    Bool,
    Char,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    //complex types
    Array,
    Dict,
    List,
    Struct,
    String,

    //weak types
    Dyn,
    Var,
}

struct Pointer {
    var_type: VarTypeRaw,
    data: Option<u32>, //simply points to a location in memory, which can either be a complex or simple variable
}


/* 
// main structure of a variable in Cowl, contains the different types a variable can be
pub enum Variable {
    Float32(Option<f32>),
    Float64(Option<f64>),
    Int8(Option<i8>),
    Int16(Option<i16>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    Int128(Option<i128>),
    List(Option<Box<List>>)
}

impl Variable {
    pub fn new_list() -> Variable {
        return Variable::List(Some(Box::new(List::new())));
    }
}

pub struct List {
    items: Vec<Variable>,
    list_type: Option<Variable>
}

impl List {
    fn new() -> List {
        return List {
            items: vec![],
            list_type: None
        }
    }
}

// the main structure of the variable in Cowl
//struct Variable<T> {
//    val: Option<Type<T>>, //for simple types like integers and chars
//}

// Variable<>
*/


/*
Simple Variables:
int
float
bool

Complex Variables:
function
struct
impl?
list
array
dictionary


*/