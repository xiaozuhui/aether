use aether::{Aether, Value};

#[test]
fn trace_collects_and_drains() {
    let mut engine = Aether::new();

    let result = engine
        .eval(
            r#"
TRACE("hello")
TRACE(123)
TRACE([1, 2, 3])
42
"#,
        )
        .unwrap();

    assert_eq!(result, Value::Number(42.0));

    let trace = engine.take_trace();
    assert_eq!(
        trace,
        vec![
            "#1 hello".to_string(),
            "#2 123".to_string(),
            "#3 [1, 2, 3]".to_string(),
        ]
    );

    // Drained
    assert!(engine.take_trace().is_empty());
}

#[test]
fn trace_supports_optional_label() {
    let mut engine = Aether::new();

    engine
        .eval(
            r#"
TRACE("dbg", 1, 2)
TRACE("note", "hello")
"#,
        )
        .unwrap();

    let trace = engine.take_trace();
    assert_eq!(
        trace,
        vec!["#1 [dbg] 1 2".to_string(), "#2 [note] hello".to_string(),]
    );
}

#[test]
fn clear_trace_resets_sequence() {
    let mut engine = Aether::new();

    engine.eval("TRACE(1)").unwrap();
    engine.clear_trace();

    engine.eval("TRACE(2)").unwrap();
    let trace = engine.take_trace();
    assert_eq!(trace, vec!["#1 2".to_string()]);
}

#[test]
fn trace_buffer_has_max_capacity_and_drops_oldest() {
    let mut engine = Aether::new();

    // Get the current buffer size from trace stats
    let max_entries = engine.trace_stats().buffer_size;

    let mut code = String::new();
    for i in 1..=(max_entries + 2) {
        code.push_str(&format!("TRACE({})\n", i));
    }
    engine.eval(&code).unwrap();

    let trace = engine.take_trace();
    assert_eq!(trace.len(), max_entries);

    // We emitted max_entries + 2 entries, so the oldest 2 were dropped.
    // The remaining entries should start at sequence 3.
    assert_eq!(trace.first().unwrap(), "#3 3");
    assert_eq!(
        trace.last().unwrap(),
        &format!("#{} {}", max_entries + 2, max_entries + 2)
    );
}

#[test]
fn trace_multi_args_without_string_label_is_not_labeled() {
    let mut engine = Aether::new();

    engine.eval("TRACE(1, 2, 3)").unwrap();

    let trace = engine.take_trace();
    assert_eq!(trace, vec!["#1 1 2 3".to_string()]);
}

#[test]
fn set_trace_buffer_size_works() {
    let mut engine = Aether::new();

    // Set a small buffer size
    engine.set_trace_buffer_size(3);

    // Add more entries than the buffer size
    for i in 1..=5 {
        engine.eval(&format!("TRACE({})", i)).unwrap();
    }

    let trace = engine.take_trace();
    assert_eq!(trace.len(), 3);

    // Should only keep the last 3 entries
    assert_eq!(trace[0], "#3 3");
    assert_eq!(trace[1], "#4 4");
    assert_eq!(trace[2], "#5 5");

    // Verify stats reflect the new buffer size
    let stats = engine.trace_stats();
    assert_eq!(stats.buffer_size, 3);
    assert_eq!(stats.total_entries, 0); // Buffer was cleared by take_trace
}
