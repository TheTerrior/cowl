

// the main structure of the variable in Cowl
struct Variable {
    val: Option<Type>, //for simple types like integers and chars
}

// the different types a variable can be
enum Type {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}