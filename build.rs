extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir).join("bindings").join("aether.h");

    // Create bindings directory if it doesn't exist
    std::fs::create_dir_all(PathBuf::from(&crate_dir).join("bindings")).ok();

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_cpp_compat(true)
        .with_include_guard("AETHER_H")
        .with_documentation(true)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(output_file);

    println!("cargo:rerun-if-changed=src/ffi.rs");

    // 告诉 Cargo 如果标准库文件改变了就重新编译
    println!("cargo:rerun-if-changed=stdlib/string_utils.aether");
    println!("cargo:rerun-if-changed=stdlib/array_utils.aether");
    println!("cargo:rerun-if-changed=stdlib/validation.aether");
    println!("cargo:rerun-if-changed=stdlib/datetime.aether");
    println!("cargo:rerun-if-changed=stdlib/testing.aether");
}
