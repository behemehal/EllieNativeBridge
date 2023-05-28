use libc::c_char;

use crate::{error::CallbackError, types::Types};

#[repr(C)]
pub struct Argument {
    pub name: *mut c_char,
    pub argument_type: Types,
}

#[repr(C)]
pub enum ReturnType {
    Value(Types),
    Void,
    Error(CallbackError),
}

#[must_use]
#[repr(C)]
pub struct EllieFunction {
    pub name: *mut c_char,
    pub on_call: extern "C" fn(arguments: *mut Argument) -> ReturnType,
}
