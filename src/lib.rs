use items::EllieFunction;
use libc::c_char;

pub mod error;
pub mod items;
pub mod types;

#[repr(C)]
pub struct EllieModule {
    pub name: *mut c_char,
    pub version: *mut c_char,
    pub functions: *mut EllieFunction,
}
