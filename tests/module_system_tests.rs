use aether::{Aether, FileSystemModuleResolver, Value};
use std::path::{Path, PathBuf};

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

fn engine_with_fs_import(base_file: &Path) -> Aether {
    let mut engine = Aether::new();
    engine.set_module_resolver(Box::new(FileSystemModuleResolver::default()));

    let canon = base_file
        .canonicalize()
        .unwrap_or_else(|_| base_file.to_path_buf());
    let base_dir = canon.parent().map(|p| p.to_path_buf());
    engine.push_import_base(canon.display().to_string(), base_dir);

    engine
}

#[test]
fn module_import_export_basic_works() {
    let dir = TempDir::new("aether_module_basic");

    let _math = dir.write(
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
    Import {ADD} From "./math"
    ADD(1, 2)
    "#,
    );

    let code = std::fs::read_to_string(&main).unwrap();
    let mut engine = engine_with_fs_import(&main);

    let result = engine.eval(&code).unwrap();
    engine.pop_import_base();

    assert_eq!(result, Value::Number(3.0));

    // Namespace import binds module exports as a Dict.
    let mut engine = engine_with_fs_import(&main);
    let result = engine
        .eval(
            r#"
Import M From "./math"
Set F M["ADD"]
F(1, 2)
"#,
        )
        .unwrap();
    engine.pop_import_base();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn module_import_supports_alias() {
    let dir = TempDir::new("aether_module_alias");

    let _math = dir.write(
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
    Import ADD As PLUS From "./math"
    PLUS(10, 20)
    "#,
    );

    let code = std::fs::read_to_string(&main).unwrap();
    let mut engine = engine_with_fs_import(&main);

    let result = engine.eval(&code).unwrap();
    engine.pop_import_base();

    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn import_fails_if_symbol_not_exported() {
    let dir = TempDir::new("aether_module_missing_export");

    let _math = dir.write(
        "math.aether",
        r#"
Func ADD(A, B) {
    Return (A + B)
}
"#,
    );

    let main = dir.write(
        "main.aether",
        r#"
    Import {ADD} From "./math"
    ADD(1, 2)
    "#,
    );

    let code = std::fs::read_to_string(&main).unwrap();
    let mut engine = engine_with_fs_import(&main);

    let err = engine.eval(&code).unwrap_err().to_string();
    engine.pop_import_base();

    assert!(err.contains("is not exported"), "unexpected error: {err}");
}

#[test]
fn circular_import_is_detected() {
    let dir = TempDir::new("aether_module_cycle");

    let _a = dir.write(
        "a.aether",
        r#"
Import {BVAL} From "./b"
Set AVAL 1
Export AVAL
"#,
    );

    let _b = dir.write(
        "b.aether",
        r#"
Import {AVAL} From "./a"
Set BVAL 2
Export BVAL
"#,
    );

    let main = dir.write(
        "main.aether",
        r#"
    Import {AVAL} From "./a"
    AVAL
    "#,
    );

    let code = std::fs::read_to_string(&main).unwrap();
    let mut engine = engine_with_fs_import(&main);

    let err = engine.eval(&code).unwrap_err().to_string();
    engine.pop_import_base();

    assert!(
        err.contains("circular import detected"),
        "unexpected error: {err}"
    );
}

#[test]
fn dsl_default_disables_import() {
    let mut engine = Aether::new();

    let err = engine
        .eval("Import X From \"./nope\"")
        .unwrap_err()
        .to_string();

    assert!(
        err.contains("Import is disabled"),
        "unexpected error: {err}"
    );
}
