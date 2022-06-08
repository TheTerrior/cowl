use std::collections::{HashMap, HashSet};
use crate::smartlist::SmartList;


pub struct Memory {
    pointer_stack: Vec<HashMap<usize, Pointer>>, //stack frames of pointers
    heap: SmartList<Variable>, //collection of all Variables (can be referenced directly via pointer or via a complex type)
}

impl Memory {
    
    // create a new memory instance (the struct should be a singleton though)
    fn new() -> Memory {
        return Memory {
            pointer_stack: Vec::with_capacity(1024),
            heap: SmartList::with_capacity(1024),
        };
    }

    // returns mutable reference to top of pointer_stack
    fn cur_pointer_stack(&mut self) -> &mut HashMap<usize, Pointer> {
        let len = self.pointer_stack.len();
        return &mut self.pointer_stack[len];
    }

    // create a new stack frame on the pointer stack, should be called with every new scope, including the base scope
    fn new_pointer_stack(&mut self) {
        self.pointer_stack.push(HashMap::new());
    }

    // pop the current stack frame, used when leaving a scope
    fn pop_pointer_stack(&mut self) {
        self.pointer_stack.pop();
    }


    // overwrites any previous pointer data if it existed, must receive an id to a Variable
    fn declare_pointer(&mut self, pointer_id: usize, pointer_type: PointerType, var_id: usize) -> Result<(), ()>{
        let res: Option<&mut Pointer> = self.find_pointer_local(pointer_id); //try to get reference to the pointer
        if let Some(_) = res { //if the Pointer exists, then detach it from its Variable
            self.detach_pointer(pointer_id);
        }

        self.cur_pointer_stack().insert(pointer_id, Pointer::new(pointer_type)); //insert new pointer into the stack with the given id
        if vartype_to_pointertype(&self.find_var(var_id).unwrap().data) != pointer_type { //ensure the types match up
            return Err(());
        }
        let pointer_pointer: &mut Pointer = self.find_pointer_local(pointer_id).unwrap();
        pointer_pointer.var_index = Some(var_id);
        return Ok(());
    }

    // if a pointer with the given id exists, find it and return a reference to it
    fn find_pointer_global(&mut self, pointer_id: usize) -> Option<&mut Pointer> {
        for i in (0..self.pointer_stack.len()).rev() {
            if self.pointer_stack[i].contains_key(&pointer_id) {
                return self.pointer_stack[i].get_mut(&pointer_id);
            }
        }
        return None;
    }

    // if a pointer with the given id exists in the current stack frame, return reference
    fn find_pointer_local(&mut self, pointer_id: usize) -> Option<&mut Pointer> {
        let cur = self.cur_pointer_stack();
        if cur.contains_key(&pointer_id) {
            return cur.get_mut(&pointer_id);
        }
        return None;
    }

    // detach a pointer from its variable, which requires updating both the pointer and the variable
    fn detach_pointer(&mut self, pointer_id: usize) {
        let pos_pointer_ref: Option<&mut Pointer> = self.find_pointer_local(pointer_id);
        let pointer_ref: &mut Pointer;
        match pos_pointer_ref { //does the Pointer exist? If so, retrieve it
            None => return,
            Some(_) => pointer_ref = pos_pointer_ref.unwrap(),
        }
        let index_backup: Option<usize> = pointer_ref.var_index; //make a backup so we can delete var_index and use only one reference to self later
        pointer_ref.var_index = None; //remove reference to Variable from the Pointer

        if let Some(var_index) = index_backup { //if Pointer has an index at all
            let var_ref: &mut Variable = self.find_var(var_index).unwrap(); //then remove the reference from the Variable
            var_ref.pointers.remove(&pointer_id);
        }
    }

    // retrieves the variable at the given index
    fn find_var(&mut self, var_id: usize) -> Option<&mut Variable> {
        return self.heap.retrieve_mut(var_id.try_into().unwrap());
    }

    // creates a variable in the memory and returns its index
    fn create_var(&mut self, data: Variable) -> usize {
        return self.heap.add(data);
    }
    
}

// stored in virtual memory, used to hold information about a variable's information
#[derive(Clone)]
pub struct Variable {
    data: VarType, //stores the type of variable and the data
    pointers: HashSet<usize>, //keeps a list of all the pointers that refer to this variable
}

impl Variable {
    fn new(data: VarType) -> Variable {
        return Variable {
            data: data,
            pointers: HashSet::new(),
        };
    }

    fn add_pointer(&mut self, pointer: usize) {
        self.pointers.insert(pointer);
    }

    fn remove_pointer(&mut self, pointer: usize) {
        self.pointers.remove(&pointer);
    }
}

// this structure is the front-end for a variable, generally immutable, replaced rather than modified
#[derive(Clone)]
pub struct Pointer {
    var_type: PointerType,
    var_index: Option<usize>, //simply points to a location in memory, which can either be a complex or simple variable
}

impl Pointer {
    fn new(pointer_type: PointerType) -> Pointer {
        return Pointer {
            var_type: pointer_type,
            var_index: None,
        }
    }

    fn change_location(&mut self, var_index: Option<usize>) {
        self.var_index = var_index;
    }
}






// utilized by the Variable struct
#[derive(Debug, PartialEq, Clone)]
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
    Array(Vec<usize>),          //points to locations in the heap
    Dict(HashMap<usize,usize>), //points to pairs of locations in the heap
    List(Vec<usize>),           //points to locations in the heap
    Set(HashSet<usize>),        //points to locations in the heap
    String(Vec<usize>),         //points to locations in the heap

    //special types
    Func(Option<usize>),        //tba
    Struct(Option<usize>),      //tba
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

    //special types
    Func,
    Struct,

    //weak types
    Dyn,
    Var,
}

// i hate myself, why did i do this *facepalm*
fn vartype_to_pointertype(var_type: &VarType) -> PointerType {
    match var_type {
        VarType::Bool(_)    => return PointerType::Bool,
        VarType::Char(_)    => return PointerType::Char,
        VarType::Float32(_) => return PointerType::Float32,
        VarType::Float64(_) => return PointerType::Float64,
        VarType::Int8(_)    => return PointerType::Int8,
        VarType::Int16(_)   => return PointerType::Int16,
        VarType::Int32(_)   => return PointerType::Int32,
        VarType::Int64(_)   => return PointerType::Int64,
        VarType::UInt8(_)    => return PointerType::UInt8,
        VarType::UInt16(_)   => return PointerType::UInt16,
        VarType::UInt32(_)   => return PointerType::UInt32,
        VarType::UInt64(_)   => return PointerType::UInt64,

        VarType::Array(_)   => return PointerType::Array,
        VarType::Dict(_)    => return PointerType::Dict,
        VarType::List(_)    => return PointerType::List,
        VarType::Set(_)     => return PointerType::Set,
        VarType::String(_)  => return PointerType::String,

        VarType::Func(_)    => return PointerType::Func,
        VarType::Struct(_)  => return PointerType::Struct,
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



struct Instruction {
    opcode: u8,
    value: u32,
    value1: u32

}

struct Function {
    return_type: PointerType,
    parameters: Vec<PointerType>,
    instructions: Vec<Instruction>
}