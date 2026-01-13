use aether::stdlib::{
    ARRAY_UTILS, DATETIME, HEAP, QUEUE, SET, SORTING, STACK, STRING_UTILS, TESTING, VALIDATION,
    get_module,
};

#[test]
fn test_module_exists() {
    #[allow(clippy::const_is_empty)]
    {
        assert!(!STRING_UTILS.is_empty());
        assert!(!ARRAY_UTILS.is_empty());
        assert!(!VALIDATION.is_empty());
        assert!(!DATETIME.is_empty());
        assert!(!TESTING.is_empty());
        assert!(!SET.is_empty());
        assert!(!QUEUE.is_empty());
        assert!(!STACK.is_empty());
        assert!(!HEAP.is_empty());
        assert!(!SORTING.is_empty());
    }
}

#[test]
fn test_get_module() {
    assert!(get_module("string_utils").is_some());
    assert!(get_module("array_utils").is_some());
    assert!(get_module("set").is_some());
    assert!(get_module("queue").is_some());
    assert!(get_module("stack").is_some());
    assert!(get_module("heap").is_some());
    assert!(get_module("sorting").is_some());
    assert!(get_module("unknown").is_none());
}
