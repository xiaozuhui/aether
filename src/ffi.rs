//! C-FFI interface for Aether language bindings
//!
//! This module provides C-compatible functions for use with other languages
//! through Foreign Function Interface (FFI).

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::panic;

use crate::{Aether, Value};

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
}

/// Create a new Aether engine instance
///
/// Returns: Pointer to AetherHandle (must be freed with aether_free)
#[no_mangle]
pub extern "C" fn aether_new() -> *mut AetherHandle {
    let engine = Box::new(Aether::new());
    Box::into_raw(engine) as *mut AetherHandle
}

/// Create a new Aether engine with all IO permissions enabled
///
/// Returns: Pointer to AetherHandle (must be freed with aether_free)
#[no_mangle]
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
#[no_mangle]
pub extern "C" fn aether_eval(
    handle: *mut AetherHandle,
    code: *const c_char,
    result: *mut *mut c_char,
    error: *mut *mut c_char,
) -> c_int {
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
                let error_str = format!("{}", e);
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
#[no_mangle]
pub extern "C" fn aether_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const c_char
}

/// Free an Aether engine handle
#[no_mangle]
pub extern "C" fn aether_free(handle: *mut AetherHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle as *mut Aether);
        }
    }
}

/// Free a string allocated by Aether
#[no_mangle]
pub extern "C" fn aether_free_string(s: *mut c_char) {
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

        unsafe {
            aether_free_string(error);
        }

        aether_free(handle);
    }
}
