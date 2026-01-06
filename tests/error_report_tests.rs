use aether::Aether;

#[test]
fn error_report_json_includes_call_stack_and_phase() {
    let mut engine = Aether::new();

    let report = engine
        .eval_report(
            r#"
Func BAD(X) {
    Return (X + Y)
}

Set A [1, 2]
MAP(A, BAD)
"#,
        )
        .unwrap_err();

    assert_eq!(report.phase, "runtime");
    assert!(!report.call_stack.is_empty());

    let json = report.to_json_pretty();
    let v: serde_json::Value = serde_json::from_str(&json).expect("valid json");

    assert_eq!(v["phase"], "runtime");
    assert_eq!(v["kind"], "UndefinedVariable");

    let frames = v["call_stack"].as_array().expect("call_stack array");
    let signatures: Vec<String> = frames
        .iter()
        .filter_map(|f| f["signature"].as_str().map(|s| s.to_string()))
        .collect();

    assert!(
        signatures.iter().any(|s| s.contains("MAP(")),
        "missing MAP frame: {signatures:?}"
    );
    assert!(
        signatures.iter().any(|s| s.contains("BAD(X)")),
        "missing BAD(X) frame: {signatures:?}"
    );
}
