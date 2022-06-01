use std::collections::{HashMap, HashSet};
use crate::smartlist::SmartList;

/*
TODO
boost the pointer type from i32 to i128 to allow for a much greater number of elements and variables
*/


/*
// holds both the stack and the heap, where call frames can be popped once the frame is out of scope
pub struct Memory {
    call_stack: Vec<HashMap<u32, Variable>>, //stack frames of variable values
    pointer_stack: Vec<HashMap<u32, Pointer>>, //stack frames of pointers
    cur_call: u32, //decides what new name we can assign to a new variable
    cur_pointer: u32, //decides what new name we can assign to a new pointer
    //heap: Vec<Variable>, //may be deprecated later
}
*/

pub struct Memory {
    pointer_stack: Vec<HashMap<u32, Pointer>>, //stack frames of pointers
    heap: SmartList<Variable>, //collection of all Variables (can be referenced directly via pointer or via a complex type)
}

impl Memory {
    
    // create a new memory instance (one per interpreter)
    fn new() -> Memory {
        return Memory {
            pointer_stack: Vec::with_capacity(1024),
            heap: SmartList::with_capacity(1024),
        };
    }

    // returns mutable reference to top of pointer_stack
    fn cur_pointer_stack(&mut self) -> &mut HashMap<u32, Pointer> {
        let len = self.pointer_stack.len();
        return &mut self.pointer_stack[len];
    }

    fn new_pointer_stack(&mut self) {
        
    }

    fn pop_pointer_stack(&mut self) {
        
    }

    /* 
    // creates a blank pointer, helper function for define_pointer
    fn declare_pointer(&mut self, pointer_id: u32, pointer_type: PointerType) -> &mut Pointer {
        self.cur_pointer_stack().insert(pointer_id, Pointer::new(pointer_type)); //insert a new pointer into the pointer stack
        return self.find_pointer_local(pointer_id).unwrap();
    }

    // define a pointer, whether or not it exists, and point it to the given data
    fn define_pointer(&mut self, pointer_id: u32, pointer_type: PointerType, data: VarType) {
        /*
        if no pointer exists, create it
        if pointer exists, then tell its variable that this pointer has been removed
        
        in both cases, tie the pointer to the newly created variable
        */

        if let PointerType::Dyn = pointer_type { // the pointer_type is dyn, so all types are valid and always will be
            //define a pointer of type dyn
        } else {
            //type of pointer_type and data must match, var is wildcard though
        }

        self.create_var(data)


        match pointer_option {
            None => { //create the new pointer
                pointer_ref = self.declare_pointer(pointer_id, pointer_type);
            }
            Some(var_ref) => { //pointer already exists
                pointer_ref = var_ref;
                self.find_var(pointer_ref.var_index.unwrap());
            }
        }
    }
    */

    // overwrites any previous pointer data if it existed, must receive an id to a Variable
    fn declare_pointer(&mut self, pointer_id: u32, pointer_type: PointerType, var_id: u32) -> Result<(), ()>{
        self.cur_pointer_stack().insert(pointer_id, Pointer::new(pointer_type)); //insert new pointer into the stack with the given id
        if vartype_to_pointertype(&self.find_var(var_id).unwrap().data) != pointer_type { //ensure the types match up
            return Err(());
        }
        let pointer_pointer: &mut Pointer = self.find_pointer_local(pointer_id).unwrap();
        pointer_pointer.var_index = Some(var_id);
        return Ok(());
    }

    /* 
    // like define, however requires the variable to already exist, modifies instead of redefines the pointer at id, may cause errors at many points
    fn assign_variable(&mut self, pointer_id: u32, data: VarType) {
        let var_id: u32 = self.cur_pointer_stack().get(&pointer_id).unwrap().data.unwrap(); //get id of the internal variable
        self.find_pointer(var_id).unwrap().data = data; //modify the variable
    }
    */

    // if a pointer with the given id exists, find it and return a reference to it
    fn find_pointer_global(&mut self, pointer_id: u32) -> Option<&mut Pointer> {
        for i in (0..self.pointer_stack.len()).rev() {
            if self.pointer_stack[i].contains_key(&pointer_id) {
                return self.pointer_stack[i].get_mut(&pointer_id);
            }
        }
        return None;
    }

    fn find_pointer_local(&mut self, pointer_id: u32) -> Option<&mut Pointer> {
        let cur = self.cur_pointer_stack();
        if cur.contains_key(&pointer_id) {
            return cur.get_mut(&pointer_id);
        }
        return None;
    }

    fn find_var(&mut self, var_id: u32) -> Option<&mut Variable> {
        return self.heap.retrieve(var_id.try_into().unwrap());
    }

    fn create_var(&mut self, data: Variable) -> u32 {
        return self.heap.add(data);
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
#[derive(Debug, PartialEq)]
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
    Struct(Option<i32>), //tba
}

// utilized by the Pointer struct
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
    Int128,

    //complex types
    Array,
    Dict,
    List,
    String,
    Struct,

    //weak types
    Dyn,
    Var,
}

// this structure is the front-end for a variable, generally immutable, replaced rather than modified
pub struct Pointer {
    var_type: PointerType,
    var_index: Option<u32>, //simply points to a location in memory, which can either be a complex or simple variable
}

impl Pointer {
    fn new(pointer_type: PointerType) -> Pointer {
        return Pointer {
            var_type: pointer_type,
            var_index: None,
        }
    }

    fn change_location(&mut self, var_index: Option<u32>) {
        self.var_index = var_index;
    }
}

// i hate myself, why did i do this *facepalm*
fn vartype_to_pointertype(var_type: &VarType) -> PointerType {
    match var_type {
        VarType::Bool(_) => {
            return PointerType::Bool;
        },
        VarType::Char(_) => {
            return PointerType::Char;
        },
        VarType::Float32(_) => {
            return PointerType::Float32;
        },
        VarType::Float64(_) => {
            return PointerType::Float64;
        },
        VarType::Int8(_) => {
            return PointerType::Int8;
        },
        VarType::Int16(_) => {
            return PointerType::Int16;
        },
        VarType::Int32(_) => {
            return PointerType::Int32;
        },
        VarType::Int64(_) => {
            return PointerType::Int64;
        },
        VarType::Int128(_) => {
            return PointerType::Int128;
        },
        VarType::Array(_) => {
            return PointerType::Array;
        },
        VarType::Dict(_) => {
            return PointerType::Dict;
        },
        VarType::List(_) => {
            return PointerType::List;
        },
        VarType::String(_) => {
            return PointerType::String;
        },
        VarType::Struct(_) => {
            return PointerType::Struct;
        },
    }
}

fn check_type_match(pointer_type: PointerType, var_type: VarType) -> bool {
    if let PointerType::Dyn = pointer_type {
        return true;
    }
    if vartype_to_pointertype(&var_type) == pointer_type {
        return true;
    }
    return false;
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