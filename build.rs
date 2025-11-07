extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir).join("bindings").join("aether.h");

    // Create bindings directory if it doesn't exist
    std::fs::create_dir_all(PathBuf::from(&crate_dir).join("bindings")).ok();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_cpp_compat(true)
        .with_include_guard("AETHER_H")
        .with_documentation(true)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(output_file);

    println!("cargo:rerun-if-changed=src/ffi.rs");
}
