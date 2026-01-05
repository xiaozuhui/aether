use aether::pytranspile::{TranspileOptions, python_to_aether};

#[test]
fn list_and_dict_literals_transpile() {
    let src = r#"
# list
x = [1, 2, 3]
# dict
y = {"a": 1, "b": 2}
# subscript read
z = y["a"]
# subscript write
x[0] = 42
"#;

    let opts = TranspileOptions::default();
    let res = python_to_aether(src, &opts);

    assert!(!res.diagnostics.has_errors(), "{:?}", res.diagnostics);
    let code = res.aether.expect("expected aether output");

    // Basic shape checks (identifiers are converted to UPPER_SNAKE_CASE).
    assert!(code.contains("Set X ["), "{code}");
    assert!(code.contains("Set Y {"), "{code}");
    assert!(
        code.contains("Set Z Y[\"a\"]") || code.contains("Set Z Y[\"A\"]"),
        "{code}"
    );
    assert!(code.contains("Set X[0] 42"), "{code}");
}

#[test]
fn dict_unpack_is_rejected() {
    let src = r#"
x = {**y}
"#;

    let opts = TranspileOptions::default();
    let res = python_to_aether(src, &opts);

    assert!(res.diagnostics.has_errors());
    assert!(
        res.diagnostics
            .0
            .iter()
            .any(|d| d.code == "PY_UNSUPPORTED" && d.message.contains("dict unpack")),
        "{:?}",
        res.diagnostics
    );
}
