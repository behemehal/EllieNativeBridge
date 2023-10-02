use std::ffi::{CStr, CString};

use items::EllieFunction;
use libc::c_char;

pub mod error;
pub mod items;
pub mod types;

#[repr(C)]
pub struct EllieModule {
    pub name: *const c_char,
    pub functions: *const EllieFunction,
}

impl EllieModule {
    pub fn new(name: &str, functions: *const EllieFunction) -> EllieModule {
        // Convert the Rust string to a C-compatible CString
        let c_name = CString::new(name).expect("Failed to create CString");

        EllieModule {
            name: c_name.into_raw(),
            functions,
        }
    }

    pub unsafe fn get_name(&self) -> String {
        CStr::from_ptr(self.name as *const c_char)
            .to_string_lossy()
            .into_owned()
    }

    // Add a method to free the memory when done
    pub fn free(self) {
        // Safety: Reclaim ownership and free the memory
        unsafe {
            CString::from_raw(self.name as *mut i8);
        }
    }
}
