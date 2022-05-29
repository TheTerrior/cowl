
// holds both the stack and the heap, where call frames can be popped once the frame is out of scope
struct Memory {
    call_stack: Vec<Vec<Variable>>,
    heap: Vec<Variable>,
}

// stored in virtual memory, used to hold information about a variable's information
struct Variable {
    data: Var_Type, //stores the type of variable and the data
    pointers: Vec<i32>, //keeps a list of all the pointers that refer to this variable
}

// utilized by the Variable struct
pub enum Var_Type {
    Array,
    Bool(bool),
    Char(char),
    Dict,
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    List,
    Struct,
    String,
}

struct Pointer {
    data: Option<i32>,
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