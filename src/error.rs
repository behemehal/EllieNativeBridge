use libc::c_char;

#[repr(C)]
pub struct CallbackError {
    pub message: *const c_char,
    pub code: usize,
}
