use libc::c_char;

#[repr(C)]
pub struct EllieInteger {
    pub as_isize: isize,
    pub as_usize: usize,
}

#[repr(C)]
pub enum EllieData {
    Integer(EllieInteger),
    Float(f32),
    Double(f64),
    Byte(u8),
    Bool(bool),
    String(*const c_char),
    Char(c_char),
    Void,
    Null,
    Array(*const EllieData),
    Class(*const EllieData),
}

#[repr(C)]
pub enum FunctionAnswer {
    RuntimeError(*const c_char),
    Ok(EllieData),
}

#[repr(C)]
pub struct FunctionCallParameter {
    pub data: EllieData,
    pub memory_location: usize,
}

type FunctionCallback = unsafe extern "C" fn(arguments: *const FunctionCallParameter) -> FunctionAnswer;

#[repr(C)]
pub struct EllieFunction {
    pub name: *const c_char,
    pub on_call: FunctionCallback,
}

#[repr(C)]
pub struct EllieModule {
    pub name: *const c_char,
    pub version: *const c_char,
    pub functions: *const EllieFunction,
}

