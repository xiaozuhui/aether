use aether::{Aether, Value};
use std::collections::HashMap;

#[test]
fn isolated_scope_drops_injected_bindings() {
    let mut engine = Aether::new();

    // Outer env persists
    engine.eval("Set BASE 10").unwrap();

    let result = engine
        .with_isolated_scope(|engine| {
            // Inject Rust-side data without eval
            engine.set_global("X", Value::Number(2.0));

            // Load per-request functions (e.g. from DB)
            engine.eval(
                r#"
Func ADDX (Y) {
    Return (Y + X + BASE)
}
"#,
            )?;

            engine.eval("ADDX(1)")
        })
        .unwrap();

    assert_eq!(result, Value::Number(13.0));

    // Scope is gone: injected data and functions should not leak
    assert!(engine.eval("X").is_err());
    assert!(engine.eval("ADDX(1)").is_err());

    // Outer env still exists
    assert_eq!(engine.eval("BASE").unwrap(), Value::Number(10.0));
}

#[test]
fn can_inject_rust_dict_as_global() {
    let mut engine = Aether::new();

    let mut dict = HashMap::new();
    dict.insert("a".to_string(), Value::Number(1.0));
    dict.insert("b".to_string(), Value::Number(2.0));

    let result = engine
        .with_isolated_scope(|engine| {
            engine.set_global("DATA", Value::Dict(dict));
            engine.eval("(DATA[\"a\"] + DATA[\"b\"])")
        })
        .unwrap();

    assert_eq!(result, Value::Number(3.0));

    // Not leaked
    assert!(engine.eval("DATA").is_err());
}
