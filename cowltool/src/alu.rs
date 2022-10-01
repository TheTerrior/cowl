use std::{rc::Rc, cell::RefCell, collections::HashMap};
use core::mem::size_of as sizeof;

use crate::memory::{Variable, Variable::*, self};

/*

DESCRIPTION

This file serves as the user's built-in interface/functions. 
Users will be able to directly interact with these functions.
All type-specific behavior is defined here too, for the time being.

*/



//
// GENERICS
//

//
fn size_of(var: &mut Variable) -> usize {
    let base = match var {
        Bool(_) => sizeof::<bool>(),
        Char(_) => sizeof::<char>(),
        Float32(_) => sizeof::<f32>(),
        Float64(_) => sizeof::<f64>(),
        Int8(_) => sizeof::<i8>(),
        Int16(_) => sizeof::<i16>(),
        Int32(_) => sizeof::<i32>(),
        Int64(_) => sizeof::<i64>(),
        UInt8(_) => sizeof::<u8>(),
        UInt16(_) => sizeof::<u16>(),
        UInt32(_) => sizeof::<u32>(),
        UInt64(_) => sizeof::<u64>(),

        Dict(x) => sizeof::<memory::Dictionary>() + x.data.capacity() / 8,
        List(x) => sizeof::<memory::List>() + x.data.capacity() / 8,
        Set(x) => sizeof::<memory::Set>() + x.data.capacity() / 8,
        Struct(x) => sizeof::<memory::Struct>() + x.data.capacity() / 8,

        //Func(x) => sizeof::<memory::Function>() + x.data.capacity() / 8,
        _ => 0,
    };

    return sizeof::<Variable>() + base;
}

fn clone<T>() {

}


//
// STRINGS
//
