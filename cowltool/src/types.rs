use std::collections::{HashMap, HashSet};
use crate::smartlist::SmartList;

/*
TODO
boost the pointer type from i32 to i128 to allow for a much greater number of elements and variables
*/


// holds both the stack and the heap, where call frames can be popped once the frame is out of scope
pub struct Memory {
    call_stack: Vec<HashMap<u32, Variable>>, //stack frames of variable values
    pointer_stack: Vec<HashMap<u32, Pointer>>, //stack frames of pointers
    cur_call: u32, //decides what new name we can assign to a new variable
    cur_pointer: u32, //decides what new name we can assign to a new pointer
    //heap: Vec<Variable>, //may be deprecated later
}

impl Memory {
    
    // create a new memory instance (one per interpreter)
    fn new() -> Memory {
        return Memory {
            call_stack: Vec::with_capacity(1024),
            pointer_stack: Vec::with_capacity(1024),
            cur_call: 0,
            cur_pointer: 0,
        };
    }

    // returns mutable reference to top of call_stack
    fn cur_call_stack(&mut self) -> &mut HashMap<u32, Variable> {
        let len = self.call_stack.len();
        return &mut self.call_stack[len];
    }

    // returns mutable reference to top of pointer_stack
    fn cur_pointer_stack(&mut self) -> &mut HashMap<u32, Pointer> {
        let len = self.pointer_stack.len();
        return &mut self.pointer_stack[len];
    }

    fn new_call_stack(&mut self) {

    }

    fn new_pointer_stack(&mut self) {
        
    }

    fn pop_call_stack(&mut self) {

    }

    fn pop_pointer_stack(&mut self) {
        
    }

    // creates a blank pointer, helper function for define_pointer
    fn declare_pointer(&mut self, pointer_id: u32, pointer_type: PointerType) {
        self.cur_pointer_stack().insert(pointer_id, Pointer::new(pointer_type)); //insert a new pointer into the pointer stack
    }

    // define a pointer, whether or not it exists, and point it to the given data
    fn define_pointer(&mut self, pointer_id: u32, pointer_type: PointerType, data: VarType) {
        match self.find_var(pointer_id) {
            None => { //create the new pointer
                self.declare_pointer(pointer_id, pointer_type);
            }
            Some(var_ref) => { //pointer already exists

            }
        }
    }

    // like define, however requires the variable to already exist, modifies instead of redefines the pointer at id, may cause errors at many points
    fn assign_variable(&mut self, pointer_id: u32, data: VarType) {
        let var_id: u32 = self.cur_pointer_stack().get(&pointer_id).unwrap().data.unwrap(); //get id of the internal variable
        self.find_var(var_id).unwrap().data = data; //modify the variable
    }

    // if a variable with the given id exists, find it and return a reference to it
    fn find_var(&mut self, id: u32) -> Option<&mut Variable> {
        for i in (0..self.call_stack.len()).rev() {
            if self.call_stack[i].contains_key(&id) {
                return self.call_stack[i].get_mut(&id);
            }
        }
        return None;
    }
}

// stored in virtual memory, used to hold information about a variable's information
struct Variable {
    data: VarType, //stores the type of variable and the data
    pointers: HashSet<u32>, //keeps a list of all the pointers that refer to this variable
}

impl Variable {
    fn new(data: VarType) -> Variable {
        return Variable {
            data: data,
            pointers: HashSet::new(),
        };
    }

    fn add_pointer(&mut self, pointer: u32) {
        self.pointers.insert(pointer);
    }

    fn remove_pointer(&mut self, pointer: u32) {
        self.pointers.remove(&pointer);
    }
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
    String(Vec<u32>),
    Struct, //tba
}

// utilized by the Pointer struct
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

// this structure is the front-end for a variable, generally immutable, replaced rather than modified
pub struct Pointer {
    var_type: PointerType,
    data: Option<u32>, //simply points to a location in memory, which can either be a complex or simple variable
}

impl Pointer {
    fn new(pointer_type: PointerType) -> Pointer {
        return Pointer {
            var_type: pointer_type,
            data: None,
        }
    }

    fn change_location(&mut self, location: Option<u32>) {
        self.data = location;
    }
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