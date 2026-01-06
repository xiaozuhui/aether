use aether::{Aether, FileSystemModuleResolver};
use std::path::Path;

fn main() {
    let mut engine = Aether::new();

    // 方案1：eval_file 只管理 base_dir 上下文；resolver 由调用方显式启用。
    engine.set_module_resolver(Box::new(FileSystemModuleResolver::default()));

    let script = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("module_import")
        .join("main.aether");

    let result = engine.eval_file(&script).unwrap();
    println!("{}", result);
}
