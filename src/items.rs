use libc::c_char;

use crate::{error::CallbackError, types::Types};

#[repr(C)]
pub struct ThreadInfo {
    pub id: usize,
    pub stack_id: usize,
    pub stack_caller: usize,
}

#[repr(C)]
pub struct TypeId {
    pub id: u8,
    pub size: usize,
}

impl TypeId {
    pub fn new(id: u8, size: usize) -> TypeId {
        TypeId { id, size }
    }
}

#[repr(C)]
pub struct StaticRawType {
    pub type_id: TypeId,
    pub data: [u8; 8],
}

impl StaticRawType {
    pub fn new(type_id: TypeId, data: [u8; 8]) -> StaticRawType {
        StaticRawType { type_id, data }
    }
}

#[repr(C)]
pub struct RawType {
    pub type_id: TypeId,
    pub data: *mut u8, //This is platfform dependent
}

impl RawType {
    pub fn new(type_id: TypeId, data: *mut u8) -> RawType {
        RawType { type_id, data }
    }
}

#[repr(C)]
pub enum ReturnType {
    Value(Types),
    Void,
    Error(CallbackError),
}

pub type FunctionElementCallback =
    extern "C" fn(thread_info: ThreadInfo, params: *mut Types) -> ReturnType;

#[repr(C)]
pub struct EllieFunction {
    pub name: *const c_char,
    pub on_call: FunctionElementCallback,
}
