use std::ffi::{CStr, CString, c_char, c_int};

use aether::ffi::{AetherErrorCode, aether_eval, aether_free, aether_free_string, aether_new};

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
