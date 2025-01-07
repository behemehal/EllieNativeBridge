use ellie_native_bridge::rust::{EllieData, EllieInteger, FunctionCallParameter};
use std::path::PathBuf;

// Helper to log test step results
fn log_status<T>(description: &str, result: Result<T, String>) -> T
where
    T: std::fmt::Debug,
{
    match result {
        Ok(value) => {
            println!("[PASS] {}: {:?}", description, value);
            value
        }
        Err(error) => panic!("[FAIL] {}: {}", description, error),
    }
}

// Helper function to get current directory
fn current_dir() -> PathBuf {
    std::env::current_dir().expect("Failed to get current directory")
}

#[test]
#[cfg(feature = "rust_bridge")]
fn compile_rust_test() {
    // Build the path to the Rust test directory
    let test_dir = current_dir().join("tests/rust");

    // 1. Compile the Rust module with `cargo build --release`
    let cargo_build = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&test_dir)
        .output();

    // Log the result of the build step
    log_status(
        "Compiling Rust test module",
        match cargo_build {
            Ok(output) if output.status.success() => Ok("Build successful"),
            Ok(output) => Err(format!(
                "Build failed with status: {:?}\nStderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            )),
            Err(_) => Err("Failed to execute `cargo build`".to_owned()),
        },
    );

    // 2. Check if the DLL file exists
    let release_dir = test_dir.join("target/release");

    let dynamic_library_extension = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    let dylib_file = release_dir.join(format!(
        "rust_integration_test.{}",
        dynamic_library_extension
    ));
    log_status(
        "Checking if Dynamic Library File file exists",
        if dylib_file.clone().exists() {
            Ok("Dynamic Library File file found")
        } else {
            Err(format!(
                "Dynamic Library File not found at: {}",
                dylib_file.clone().display()
            ))
        },
    );

    //
    unsafe {
        let ellie_module_lib = match libloading::Library::new(dylib_file.clone()) {
            Ok(lib) => {
                println!("[PASS] Loading DLL: {:?}", dylib_file.clone());
                lib
            }
            Err(error) => panic!("[FAIL] Loading Dynamic Library File: {}", error),
        };

        let load_module: libloading::Symbol<
            unsafe extern "C" fn() -> ellie_native_bridge::rust::EllieModule,
        > = match ellie_module_lib.get(b"load_module") {
            Ok(func) => {
                println!("[PASS] Getting symbol: load_module");
                func
            }
            Err(error) => panic!("[FAIL] Getting symbol: load_module: {}", error),
        };

        let module = load_module();

        assert_eq!(
            module.name, "RustMath",
            "[FAIL] Module name does not match expected value"
        );
        println!("[PASS] Module name: {}", module.name);

        assert_eq!(
            module.version, "0.1.0",
            "[FAIL] Module version does not match expected value"
        );
        println!("[PASS] Module version: {}", module.version);

        let functions = module.functions;

        assert_eq!(
            functions.len(),
            1,
            "[FAIL] Expected 1 function in the module, found {}",
            functions.len()
        );
        println!("[PASS] Found {} functions in the module", functions.len());

        let add_function = &functions[0];

        assert_eq!(
            add_function.name, "add",
            "[FAIL] Function name does not match expected value"
        );
        println!("[PASS] Function name: {}", add_function.name);

        let add_function_pointer = add_function.on_call;

        let response = add_function_pointer(vec![
            FunctionCallParameter {
                data: EllieData::Integer(1_isize.try_into().unwrap()),
                memory_location: 0,
            },
            FunctionCallParameter {
                data: EllieData::Integer(2_isize.try_into().unwrap()),
                memory_location: 0,
            },
        ]);

        assert!(
            match response {
                ellie_native_bridge::rust::FunctionAnswer::Ok(_) => true,
                ellie_native_bridge::rust::FunctionAnswer::RuntimeError(_) => false,
            },
            "[FAIL] Function response is not Ok",
        );
        println!("[PASS] Function response is Ok");

        match response {
            ellie_native_bridge::rust::FunctionAnswer::Ok(ellie_data) => match ellie_data {
                EllieData::Integer(ellie_integer) => {
                    assert_eq!(
                        ellie_integer.as_isize, 3,
                        "[FAIL] Expected 3, got {}",
                        ellie_integer.as_isize
                    );
                    println!("[PASS] Function result: {}", ellie_integer.as_isize);
                }
                _ => panic!("[FAIL] Expected EllieData::Integer, got something else"),
            },
            ellie_native_bridge::rust::FunctionAnswer::RuntimeError(_) => {
                panic!("[FAIL] Function response is not Ok")
            }
        }
    }
}

//#[test]
#[cfg(feature = "c_bridge")]
fn compile_c_test() {
    // Build the path to the Rust test directory
    let test_dir = current_dir().join("tests/c");

    //check if make command is available
    let make_check = std::process::Command::new("make")
        .arg("--version")
        .output()
        .expect("Failed to execute `make --version`");

    if !make_check.status.success() {
        panic!("[FAIL] make command not found");
    }

    // 1. Compile the C module with `make`
    let make_build = std::process::Command::new("make")
        .current_dir(&test_dir)
        .output();

    // Log the result of the build step
    log_status(
        "Compiling C test module",
        match make_build {
            Ok(output) if output.status.success() => Ok("Build successful"),
            Ok(output) => Err(format!(
                "Build failed with status: {:?}\nStderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            )),
            Err(_) => Err("Failed to execute `make`".to_owned()),
        },
    );

    // 2. Check if the SO file exists
    let so_file = test_dir.join("libellie_native_bridge.so");

    log_status(
        "Checking if Shared Object file exists",
        if so_file.clone().exists() {
            Ok("Shared Object file found")
        } else {
            Err(format!(
                "Shared Object file not found at: {}",
                so_file.clone().display()
            ))
        },
    );

    unsafe {
        let ellie_module_lib = match libloading::Library::new(so_file.clone()) {
            Ok(lib) => {
                println!("[PASS] Loading SO: {:?}", so_file.clone());
                lib
            }
            Err(error) => panic!("[FAIL] Loading Shared Object file: {}", error),
        };

        let load_module: libloading::Symbol<
            unsafe extern "C" fn() -> ellie_native_bridge::c::EllieModule,
        > = match ellie_module_lib.get(b"load_module") {
            Ok(func) => {
                println!("[PASS] Getting symbol: load_module");
                func
            }
            Err(error) => panic!("[FAIL] Getting symbol: load_module: {}", error),
        };

        let module = load_module();

        //convert *const c_char to &str
        let module_name = std::ffi::CStr::from_ptr(module.name).to_str().unwrap();

        assert_eq!(
            module_name, "RustMath",
            "[FAIL] Module name does not match expected value"
        );
        println!("[PASS] Module name: {}", module_name);
    }
}
