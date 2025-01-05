fn main() {
    //check c feature enabled
    #[cfg(feature = "c_bridge")]
    {
        use std::env;
        use std::path::PathBuf;

        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let package_name = env::var("CARGO_PKG_NAME").unwrap();
        let output_file = if let Ok(target) = env::var("CARGO_TARGET_DIR") {
            PathBuf::from(target)
        } else {
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
        };

        let output_file = output_file
            .join(format!("{}.h", package_name))
            .display()
            .to_string();

        cbindgen::generate(&crate_dir)
            .unwrap()
            .write_to_file(&output_file);
    }
}
