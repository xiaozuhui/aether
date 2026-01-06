use aether::Aether;

#[test]
fn call_stack_includes_builtins_and_function_signature() {
    let mut engine = Aether::new();

    let err = engine
        .eval(
            r#"
Func BAD(X) {
    Return (X + Y)
}

Set A [1, 2]
MAP(A, BAD)
"#,
        )
        .unwrap_err();

    // Base error
    assert!(err.contains("Undefined variable"), "unexpected error: {err}");

    // Phase 1: call stack should be present.
    assert!(err.contains("Call stack:"), "unexpected error: {err}");

    // Built-in frame
    assert!(err.contains("MAP("), "unexpected error: {err}");

    // User function signature (name + params)
    assert!(err.contains("BAD(X)"), "unexpected error: {err}");
}
