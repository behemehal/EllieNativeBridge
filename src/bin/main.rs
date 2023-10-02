use ellie_native_bridge::{EllieModule, items::{EllieFunction, ReturnType, ThreadInfo, Argument}, error::CallbackError};

extern "C" fn func1() -> &'static str {
    "Hello, world!"
}


fn main() {

    

    

    println!("Hello, world!");
}