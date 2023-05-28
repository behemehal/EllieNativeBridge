
fn main() {
    unsafe {
        let lib = libloading::Library::new("../libellie.so").unwrap();
        let func: libloading::Symbol<unsafe extern "C" fn() -> ellie_native_bridge::EllieModule> = lib.get(b"getLib").unwrap();
    }
}
