# Ellie Naive Bridge
Ellie native bridge is a bridge between Ellie and native code. It allows you to define functions in native code and call them from Ellie.

```rust
use ellie_native_bridge::rust::{
    EllieFunction, EllieModule, FunctionAnswer, FunctionCallParameter,
};

#[no_mangle]
pub extern "Rust" fn add(args: Vec<FunctionCallParameter>) -> FunctionAnswer {
    if args.len() != 2 {
        return FunctionAnswer::RuntimeError("Expected 2 arguments".to_string());
    }

    let left = match args[0].data.clone().into_integer() {
        Ok(eint) => eint.as_isize,
        _ => return FunctionAnswer::RuntimeError("Expected integer".to_string()),
    };

    let right = match args[1].data.clone().into_integer() {
        Ok(eint) => eint.as_isize,
        _ => return FunctionAnswer::RuntimeError("Expected integer".to_string()),
    };

    FunctionAnswer::Ok((left + right).into())
}

//Required for the module to be loaded
#[no_mangle]
pub extern "Rust" fn load_module() -> EllieModule {
    EllieModule {
        name: "RustMath",
        version: "0.1.0",
        functions: vec![EllieFunction {
            name: "add",
            on_call: add,
        }],
    }
}
```

## How to use
Check rust and c examples in `examples` directory, also you can take a look at the `tests` directory for how things work.