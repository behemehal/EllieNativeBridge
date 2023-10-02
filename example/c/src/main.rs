use std::{ffi::CStr, mem};

use ellie_native_bridge::items::EllieFunction;
use libloading::Symbol;

fn main() {
    unsafe {
        let lib = libloading::Library::new("./example_library.so").unwrap();

        if let Ok(init_fn) = lib.get::<Symbol<unsafe extern "C" fn()>>(b"onInit") {
            init_fn();
        }

        if let Ok(module_ptr) = lib.get::<*const ellie_native_bridge::EllieModule>(b"module") {
            let module = *module_ptr;

            println!(
                "Module.name = {:?}",
                CStr::from_ptr((*module).name).to_string_lossy()
            );

            let functions_ptr = (*module).functions;

            let mut i = 0;
            while !(*functions_ptr.offset(i)).name.is_null() {
                let function = &*functions_ptr.offset(i);
                println!("Size of function.name = {}", mem::size_of_val(function.));
                let function_name = CStr::from_ptr(function.name).to_string_lossy();
                println!("Function {} at address {:?}", function_name, function.name);

                i += 1;
            }
        } else {
            panic!("Failed to find module export in library");
        }
    }
}
