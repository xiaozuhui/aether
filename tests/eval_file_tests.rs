use aether::{Aether, FileSystemModuleResolver, Value};
use std::path::PathBuf;

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let pid = std::process::id();

        let mut path = std::env::temp_dir();
        path.push(format!("{prefix}_{pid}_{nanos}"));
        std::fs::create_dir_all(&path).unwrap();

        Self { path }
    }

    fn write(&self, rel: &str, content: &str) -> PathBuf {
        let p = self.path.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&p, content).unwrap();
        p
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[test]
fn eval_file_resolves_relative_import_when_resolver_is_configured() {
    let dir = TempDir::new("aether_eval_file_ok");

    let _mod1 = dir.write(
        "math.aether",
        r#"
Func ADD(A, B) {
    Return (A + B)
}
Export ADD
"#,
    );

    let main = dir.write(
        "main.aether",
        r#"
Import ADD From "./math"
ADD(1, 2)
"#,
    );

    let mut engine = Aether::new();
    engine.set_module_resolver(Box::new(FileSystemModuleResolver::default()));

    let result = engine.eval_file(&main).unwrap();
    assert_eq!(result, Value::Number(3.0));

    // Import base should not leak; without base context, relative import should fail.
    let err = engine
        .eval("Import ADD From \"./math\"")
        .unwrap_err()
        .to_string();
    assert!(err.contains("No base directory"), "unexpected error: {err}");
}

#[test]
fn eval_file_does_not_enable_import_by_default() {
    let dir = TempDir::new("aether_eval_file_disabled");

    let _mod1 = dir.write(
        "math.aether",
        r#"
Func ADD(A, B) {
    Return (A + B)
}
Export ADD
"#,
    );

    let main = dir.write(
        "main.aether",
        r#"
Import ADD From "./math"
ADD(1, 2)
"#,
    );

    let mut engine = Aether::new();

    let err = engine.eval_file(&main).unwrap_err();
    assert!(
        err.contains("Import is disabled"),
        "unexpected error: {err}"
    );
}
