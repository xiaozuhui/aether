//! C-FFI interface for Aether language bindings
//!
//! This module provides C-compatible functions for use with other languages
//! through Foreign Function Interface (FFI).

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::panic;
use std::sync::Mutex;

use crate::{Aether, Value};
use serde_json::json;

/// Opaque handle for Aether engine
#[repr(C)]
pub struct AetherHandle {
    _opaque: [u8; 0],
}

/// Error codes returned by C-FFI functions
#[repr(C)]
pub enum AetherErrorCode {
    Success = 0,
    ParseError = 1,
    RuntimeError = 2,
    NullPointer = 3,
    Panic = 4,
    InvalidJSON = 5,
    VariableNotFound = 6,
}

/// Execution limits configuration
#[repr(C)]
pub struct AetherLimits {
    pub max_steps: c_int,
    pub max_recursion_depth: c_int,
    pub max_duration_ms: c_int,
}

/// Cache statistics
#[repr(C)]
pub struct AetherCacheStats {
    pub hits: c_int,
    pub misses: c_int,
    pub size: c_int,
}

/// Thread-safe wrapper for Aether engine
struct ThreadSafeEngine {
    #[allow(dead_code)]
    engine: Aether,
    #[allow(dead_code)]
    mutex: Mutex<()>,
}

impl ThreadSafeEngine {
    #[allow(dead_code)]
    fn new(engine: Aether) -> Self {
        Self {
            engine,
            mutex: Mutex::new(()),
        }
    }
}

/// Create a new Aether engine instance
///
/// Returns: Pointer to AetherHandle (must be freed with aether_free)
#[unsafe(no_mangle)]
pub extern "C" fn aether_new() -> *mut AetherHandle {
    let engine = Box::new(Aether::new());
    Box::into_raw(engine) as *mut AetherHandle
}

/// Create a new Aether engine with all IO permissions enabled
///
/// Returns: Pointer to AetherHandle (must be freed with aether_free)
#[unsafe(no_mangle)]
pub extern "C" fn aether_new_with_permissions() -> *mut AetherHandle {
    let engine = Box::new(Aether::with_all_permissions());
    Box::into_raw(engine) as *mut AetherHandle
}

/// Evaluate Aether code
///
/// # Parameters
/// - handle: Aether engine handle
/// - code: C string containing Aether code
/// - result: Output parameter for result (must be freed with aether_free_string)
/// - error: Output parameter for error message (must be freed with aether_free_string)
///
/// # Returns
/// - 0 (Success) if evaluation succeeded
/// - Non-zero error code if evaluation failed
#[unsafe(no_mangle)]
pub extern "C" fn aether_eval(
    handle: *mut AetherHandle,
    code: *const c_char,
    result: *mut *mut c_char,
    error: *mut *mut c_char,
) -> c_int {
    #![allow(clippy::not_unsafe_ptr_arg_deref)]
    if handle.is_null() || code.is_null() || result.is_null() || error.is_null() {
        return AetherErrorCode::NullPointer as c_int;
    }

    // Catch panics and convert them to errors
    let panic_result = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let code_str = match CStr::from_ptr(code).to_str() {
            Ok(s) => s,
            Err(_) => return AetherErrorCode::RuntimeError as c_int,
        };

        match engine.eval(code_str) {
            Ok(val) => {
                let result_str = value_to_string(&val);
                match CString::new(result_str) {
                    Ok(cstr) => {
                        *result = cstr.into_raw();
                        *error = std::ptr::null_mut();
                        AetherErrorCode::Success as c_int
                    }
                    Err(_) => AetherErrorCode::RuntimeError as c_int,
                }
            }
            Err(e) => {
                let error_str = e.to_string();
                match CString::new(error_str) {
                    Ok(cstr) => {
                        *error = cstr.into_raw();
                        *result = std::ptr::null_mut();
                        // Determine error type from message
                        if e.contains("Parse error") {
                            AetherErrorCode::ParseError as c_int
                        } else {
                            AetherErrorCode::RuntimeError as c_int
                        }
                    }
                    Err(_) => AetherErrorCode::RuntimeError as c_int,
                }
            }
        }
    });

    match panic_result {
        Ok(code) => code,
        Err(_) => {
            unsafe {
                let panic_msg = CString::new("Panic occurred during evaluation").unwrap();
                *error = panic_msg.into_raw();
                *result = std::ptr::null_mut();
            }
            AetherErrorCode::Panic as c_int
        }
    }
}

/// Get the version string of Aether
///
/// Returns: C string with version (must NOT be freed)
#[unsafe(no_mangle)]
pub extern "C" fn aether_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const c_char
}

/// Free an Aether engine handle
#[unsafe(no_mangle)]
pub extern "C" fn aether_free(handle: *mut AetherHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle as *mut Aether);
        }
    }
}

/// Free a string allocated by Aether
#[unsafe(no_mangle)]
pub extern "C" fn aether_free_string(s: *mut c_char) {
    #![allow(clippy::not_unsafe_ptr_arg_deref)]
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Helper function to convert Value to string representation
fn value_to_string(value: &Value) -> String {
    match value {
        Value::Number(n) => {
            // Format number nicely - remove trailing zeros
            if n.fract() == 0.0 {
                format!("{:.0}", n)
            } else {
                n.to_string()
            }
        }
        Value::String(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Dict(map) => {
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
        Value::Null => "null".to_string(),
        Value::Function { .. } => "<function>".to_string(),
        Value::BuiltIn { name, .. } => format!("<builtin: {}>", name),
        Value::Generator { .. } => "<generator>".to_string(),
        Value::Lazy { .. } => "<lazy>".to_string(),
        Value::Fraction(f) => f.to_string(),
    }
}

/// Helper function to convert Value to JSON string
fn value_to_json(value: &Value) -> String {
    match value {
        Value::Number(n) => {
            if n.fract() == 0.0 {
                json!(n.to_string()).to_string()
            } else {
                json!(n).to_string()
            }
        }
        Value::String(s) => json!(s).to_string(),
        Value::Boolean(b) => json!(b).to_string(),
        Value::Array(arr) => {
            let items: Vec<serde_json::Value> = arr.iter().map(|v| json_from_value(v)).collect();
            json!(items).to_string()
        }
        Value::Dict(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                obj.insert(k.clone(), json_from_value(v));
            }
            json!(obj).to_string()
        }
        Value::Null => "null".to_string(),
        Value::Function { .. } => json!("<function>").to_string(),
        Value::BuiltIn { name, .. } => json!(format!("<builtin: {}>", name)).to_string(),
        Value::Generator { .. } => json!("<generator>").to_string(),
        Value::Lazy { .. } => json!("<lazy>").to_string(),
        Value::Fraction(f) => json!(f.to_string()).to_string(),
    }
}

/// Helper function to convert Value to serde_json::Value
fn json_from_value(value: &Value) -> serde_json::Value {
    match value {
        Value::Number(n) => {
            if n.fract() == 0.0 {
                json!(n.to_string())
            } else {
                json!(n)
            }
        }
        Value::String(s) => json!(s),
        Value::Boolean(b) => json!(b),
        Value::Array(arr) => {
            let items: Vec<serde_json::Value> = arr.iter().map(json_from_value).collect();
            json!(items)
        }
        Value::Dict(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                obj.insert(k.clone(), json_from_value(v));
            }
            json!(obj)
        }
        Value::Null => json!(null),
        Value::Function { .. } => json!("<function>"),
        Value::BuiltIn { name, .. } => json!(format!("<builtin: {}>", name)),
        Value::Generator { .. } => json!("<generator>"),
        Value::Lazy { .. } => json!("<lazy>"),
        Value::Fraction(f) => json!(f.to_string()),
    }
}

/// Helper function to parse JSON to Value
fn json_to_value(json_str: &str) -> Result<Value, String> {
    let v: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| format!("Invalid JSON: {}", e))?;

    Ok(match v {
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                Value::Number(n.as_i64().unwrap() as f64)
            } else {
                Value::Number(n.as_f64().unwrap())
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Array(arr) => {
            let items: Result<Vec<_>, _> =
                arr.iter().map(|v| json_to_value(&v.to_string())).collect();
            Value::Array(items?)
        }
        serde_json::Value::Object(obj) => {
            let mut map = std::collections::HashMap::new();
            for (k, v) in obj {
                map.insert(k, json_to_value(&v.to_string())?);
            }
            Value::Dict(map)
        }
        serde_json::Value::Null => Value::Null,
    })
}

// ============================================================
// Variable Operations
// ============================================================

/// Set a global variable from host application
///
/// # Parameters
/// - handle: Aether engine handle
/// - name: Variable name
/// - value_json: Variable value as JSON string
///
/// # Returns
/// - 0 (Success) if variable was set
/// - Non-zero error code if failed
#[unsafe(no_mangle)]
pub extern "C" fn aether_set_global(
    handle: *mut AetherHandle,
    name: *const c_char,
    value_json: *const c_char,
) -> c_int {
    if handle.is_null() || name.is_null() || value_json.is_null() {
        return AetherErrorCode::NullPointer as c_int;
    }

    let panic_result = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let name_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return AetherErrorCode::RuntimeError as c_int,
        };
        let json_str = match CStr::from_ptr(value_json).to_str() {
            Ok(s) => s,
            Err(_) => return AetherErrorCode::RuntimeError as c_int,
        };

        // Parse JSON to Value
        let value = match json_to_value(json_str) {
            Ok(v) => v,
            Err(_) => return AetherErrorCode::InvalidJSON as c_int,
        };

        engine.set_global(name_str, value);
        AetherErrorCode::Success as c_int
    });

    match panic_result {
        Ok(code) => code,
        Err(_) => AetherErrorCode::Panic as c_int,
    }
}

/// Get a variable's value as JSON
///
/// # Parameters
/// - handle: Aether engine handle
/// - name: Variable name
/// - value_json: Output parameter (must be freed with aether_free_string)
///
/// # Returns
/// - 0 (Success) if variable was found
/// - VariableNotFound (6) if variable doesn't exist
/// - Non-zero error code for other failures
#[unsafe(no_mangle)]
pub extern "C" fn aether_get_global(
    handle: *mut AetherHandle,
    name: *const c_char,
    value_json: *mut *mut c_char,
) -> c_int {
    if handle.is_null() || name.is_null() || value_json.is_null() {
        return AetherErrorCode::NullPointer as c_int;
    }

    let panic_result = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let name_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return AetherErrorCode::RuntimeError as c_int,
        };

        // Try to evaluate the variable name
        match engine.eval(name_str) {
            Ok(val) => {
                let json_str = value_to_json(&val);
                match CString::new(json_str) {
                    Ok(cstr) => {
                        *value_json = cstr.into_raw();
                        AetherErrorCode::Success as c_int
                    }
                    Err(_) => AetherErrorCode::RuntimeError as c_int,
                }
            }
            Err(_) => AetherErrorCode::VariableNotFound as c_int,
        }
    });

    match panic_result {
        Ok(code) => code,
        Err(_) => AetherErrorCode::Panic as c_int,
    }
}

/// Reset the runtime environment (clears all variables)
///
/// # Parameters
/// - handle: Aether engine handle
#[unsafe(no_mangle)]
pub extern "C" fn aether_reset_env(handle: *mut AetherHandle) {
    if handle.is_null() {
        return;
    }

    let _ = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        engine.reset_env();
    });
}

// ============================================================
// Trace Operations
// ============================================================

/// Get all trace entries as JSON array
///
/// # Parameters
/// - handle: Aether engine handle
/// - trace_json: Output parameter (must be freed with aether_free_string)
///
/// # Returns
/// - 0 (Success) if trace was retrieved
/// - Non-zero error code if failed
#[unsafe(no_mangle)]
pub extern "C" fn aether_take_trace(
    handle: *mut AetherHandle,
    trace_json: *mut *mut c_char,
) -> c_int {
    if handle.is_null() || trace_json.is_null() {
        return AetherErrorCode::NullPointer as c_int;
    }

    let panic_result = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let traces = engine.take_trace();

        let json_array = json!(traces).to_string();
        match CString::new(json_array) {
            Ok(cstr) => {
                *trace_json = cstr.into_raw();
                AetherErrorCode::Success as c_int
            }
            Err(_) => AetherErrorCode::RuntimeError as c_int,
        }
    });

    match panic_result {
        Ok(code) => code,
        Err(_) => AetherErrorCode::Panic as c_int,
    }
}

/// Clear the trace buffer
///
/// # Parameters
/// - handle: Aether engine handle
#[unsafe(no_mangle)]
pub extern "C" fn aether_clear_trace(handle: *mut AetherHandle) {
    if handle.is_null() {
        return;
    }

    let _ = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        engine.clear_trace();
    });
}

/// Get structured trace entries as JSON
///
/// # Parameters
/// - handle: Aether engine handle
/// - trace_json: Output parameter (must be freed with aether_free_string)
///
/// # Returns
/// - 0 (Success) if trace was retrieved
/// - Non-zero error code if failed
#[unsafe(no_mangle)]
pub extern "C" fn aether_trace_records(
    handle: *mut AetherHandle,
    trace_json: *mut *mut c_char,
) -> c_int {
    if handle.is_null() || trace_json.is_null() {
        return AetherErrorCode::NullPointer as c_int;
    }

    let panic_result = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let records = engine.trace_records();

        // Convert TraceEntry to JSON
        let json_array: Vec<serde_json::Value> = records
            .iter()
            .map(|entry| {
                json!({
                    "level": format!("{:?}", entry.level),
                    "category": entry.category,
                    "timestamp": entry.timestamp.elapsed().as_secs(),
                    "values": entry.values.iter().map(|v| value_to_json(v)).collect::<Vec<_>>(),
                    "label": entry.label,
                })
            })
            .collect();

        match CString::new(json!(json_array).to_string()) {
            Ok(cstr) => {
                *trace_json = cstr.into_raw();
                AetherErrorCode::Success as c_int
            }
            Err(_) => AetherErrorCode::RuntimeError as c_int,
        }
    });

    match panic_result {
        Ok(code) => code,
        Err(_) => AetherErrorCode::Panic as c_int,
    }
}

/// Get trace statistics as JSON
///
/// # Parameters
/// - handle: Aether engine handle
/// - stats_json: Output parameter (must be freed with aether_free_string)
///
/// # Returns
/// - 0 (Success) if stats were retrieved
/// - Non-zero error code if failed
#[unsafe(no_mangle)]
pub extern "C" fn aether_trace_stats(
    handle: *mut AetherHandle,
    stats_json: *mut *mut c_char,
) -> c_int {
    if handle.is_null() || stats_json.is_null() {
        return AetherErrorCode::NullPointer as c_int;
    }

    let panic_result = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let stats = engine.trace_stats();

        let json_stats = json!({
            "total_entries": stats.total_entries,
            "by_level": stats.by_level,
            "by_category": stats.by_category,
            "buffer_size": stats.buffer_size,
            "buffer_full": stats.buffer_full,
        })
        .to_string();

        match CString::new(json_stats) {
            Ok(cstr) => {
                *stats_json = cstr.into_raw();
                AetherErrorCode::Success as c_int
            }
            Err(_) => AetherErrorCode::RuntimeError as c_int,
        }
    });

    match panic_result {
        Ok(code) => code,
        Err(_) => AetherErrorCode::Panic as c_int,
    }
}

// ============================================================
// Execution Limits
// ============================================================

/// Set execution limits
///
/// # Parameters
/// - handle: Aether engine handle
/// - limits: Limits configuration
#[unsafe(no_mangle)]
pub extern "C" fn aether_set_limits(handle: *mut AetherHandle, limits: *const AetherLimits) {
    if handle.is_null() || limits.is_null() {
        return;
    }

    let _ = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let limits_ref = &*limits;

        let rust_limits = crate::runtime::ExecutionLimits {
            max_steps: if limits_ref.max_steps < 0 {
                None
            } else {
                Some(limits_ref.max_steps as usize)
            },
            max_recursion_depth: if limits_ref.max_recursion_depth < 0 {
                None
            } else {
                Some(limits_ref.max_recursion_depth as usize)
            },
            max_duration_ms: if limits_ref.max_duration_ms < 0 {
                None
            } else {
                Some(limits_ref.max_duration_ms as u64)
            },
            max_memory_bytes: None,
        };

        engine.set_limits(rust_limits);
    });
}

/// Get current execution limits
///
/// # Parameters
/// - handle: Aether engine handle
/// - limits: Output parameter
#[unsafe(no_mangle)]
pub extern "C" fn aether_get_limits(handle: *mut AetherHandle, limits: *mut AetherLimits) {
    if handle.is_null() || limits.is_null() {
        return;
    }

    let _ = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let rust_limits = engine.limits();

        (*limits).max_steps = match rust_limits.max_steps {
            Some(v) => v as c_int,
            None => -1,
        };
        (*limits).max_recursion_depth = match rust_limits.max_recursion_depth {
            Some(v) => v as c_int,
            None => -1,
        };
        (*limits).max_duration_ms = match rust_limits.max_duration_ms {
            Some(v) => v as c_int,
            None => -1,
        };
    });
}

// ============================================================
// Cache Control
// ============================================================

/// Clear the AST cache
///
/// # Parameters
/// - handle: Aether engine handle
#[unsafe(no_mangle)]
pub extern "C" fn aether_clear_cache(handle: *mut AetherHandle) {
    if handle.is_null() {
        return;
    }

    let _ = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        engine.clear_cache();
    });
}

/// Get cache statistics
///
/// # Parameters
/// - handle: Aether engine handle
/// - stats: Output parameter
#[unsafe(no_mangle)]
pub extern "C" fn aether_cache_stats(handle: *mut AetherHandle, stats: *mut AetherCacheStats) {
    if handle.is_null() || stats.is_null() {
        return;
    }

    let _ = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        let rust_stats = engine.cache_stats();

        (*stats).hits = rust_stats.hits as c_int;
        (*stats).misses = rust_stats.misses as c_int;
        (*stats).size = rust_stats.size as c_int;
    });
}

// ============================================================
// Optimization Control
// ============================================================

/// Set optimization options
///
/// # Parameters
/// - handle: Aether engine handle
/// - constant_folding: Enable constant folding (1 = yes, 0 = no)
/// - dead_code_elimination: Enable dead code elimination (1 = yes, 0 = no)
/// - tail_recursion: Enable tail recursion optimization (1 = yes, 0 = no)
#[unsafe(no_mangle)]
pub extern "C" fn aether_set_optimization(
    handle: *mut AetherHandle,
    constant_folding: c_int,
    dead_code_elimination: c_int,
    tail_recursion: c_int,
) {
    if handle.is_null() {
        return;
    }

    let _ = panic::catch_unwind(|| unsafe {
        let engine = &mut *(handle as *mut Aether);
        engine.set_optimization(
            constant_folding != 0,
            dead_code_elimination != 0,
            tail_recursion != 0,
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_basic_eval() {
        let handle = aether_new();
        assert!(!handle.is_null());

        let code = CString::new("Set X 10\n(X + 20)").unwrap();
        let mut result: *mut c_char = std::ptr::null_mut();
        let mut error: *mut c_char = std::ptr::null_mut();

        let status = aether_eval(handle, code.as_ptr(), &mut result, &mut error);

        assert_eq!(status, AetherErrorCode::Success as c_int);
        assert!(!result.is_null());
        assert!(error.is_null());

        unsafe {
            let result_str = CStr::from_ptr(result).to_str().unwrap();
            assert_eq!(result_str, "30");
            aether_free_string(result);
        }

        aether_free(handle);
    }

    #[test]
    fn test_ffi_error_handling() {
        let handle = aether_new();
        let code = CString::new("UNDEFINED_VAR").unwrap();
        let mut result: *mut c_char = std::ptr::null_mut();
        let mut error: *mut c_char = std::ptr::null_mut();

        let status = aether_eval(handle, code.as_ptr(), &mut result, &mut error);

        assert_ne!(status, AetherErrorCode::Success as c_int);
        assert!(result.is_null());
        assert!(!error.is_null());

        #[allow(unused_unsafe)]
        unsafe {
            aether_free_string(error);
        }

        aether_free(handle);
    }
}
