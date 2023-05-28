#[repr(C)]
pub struct CallbackError {
    pub message: *mut char,
    pub code: usize,
}
