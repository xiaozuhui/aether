//! WASM bindings for Aether language
//!
//! This module provides WebAssembly bindings for use with JavaScript/TypeScript

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::Value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Aether engine for WebAssembly
#[wasm_bindgen]
pub struct Aether {
    engine: crate::Aether,
}

#[wasm_bindgen]
impl Aether {
    /// Create a new Aether engine instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            engine: crate::Aether::new(),
        }
    }

    /// Create a new Aether engine with all IO permissions enabled
    #[wasm_bindgen(js_name = newWithPermissions)]
    pub fn new_with_permissions() -> Self {
        console_error_panic_hook::set_once();
        Self {
            engine: crate::Aether::with_all_permissions(),
        }
    }

    /// Evaluate Aether code and return the result
    ///
    /// Returns a JavaScript value (number, string, boolean, array, or object)
    #[wasm_bindgen]
    pub fn eval(&mut self, code: &str) -> Result<JsValue, JsValue> {
        match self.engine.eval(code) {
            Ok(value) => Ok(value_to_js(&value)),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }

    /// Get the version of the Aether engine
    #[wasm_bindgen]
    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

/// Convert Aether Value to JavaScript value
fn value_to_js(value: &Value) -> JsValue {
    match value {
        Value::Number(n) => JsValue::from_f64(*n),
        Value::String(s) => JsValue::from_str(s),
        Value::Boolean(b) => JsValue::from_bool(*b),
        Value::Array(arr) => {
            let js_arr = js_sys::Array::new();
            for v in arr {
                js_arr.push(&value_to_js(v));
            }
            js_arr.into()
        }
        Value::Dict(map) => {
            let obj = js_sys::Object::new();
            for (k, v) in map {
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(k), &value_to_js(v));
            }
            obj.into()
        }
        Value::Null => JsValue::NULL,
        Value::Function { .. } => JsValue::from_str("<function>"),
        Value::BuiltIn { name, .. } => JsValue::from_str(&format!("<builtin: {}>", name)),
        Value::Generator { .. } => JsValue::from_str("<generator>"),
        Value::Lazy { .. } => JsValue::from_str("<lazy>"),
        Value::Fraction(f) => JsValue::from_str(&f.to_string()),
    }
}

/// Helper function to convert JavaScript values to Aether values
#[allow(dead_code)]
fn js_to_value(js_val: JsValue) -> Result<Value, JsValue> {
    if js_val.is_null() || js_val.is_undefined() {
        return Ok(Value::Null);
    }

    if let Some(b) = js_val.as_bool() {
        return Ok(Value::Boolean(b));
    }

    if let Some(n) = js_val.as_f64() {
        return Ok(Value::Number(n));
    }

    if let Some(s) = js_val.as_string() {
        return Ok(Value::String(s));
    }

    if js_sys::Array::is_array(&js_val) {
        let arr = js_sys::Array::from(&js_val);
        let mut values = Vec::new();
        for i in 0..arr.length() {
            let item = arr.get(i);
            values.push(js_to_value(item)?);
        }
        return Ok(Value::Array(values));
    }

    if js_val.is_object() {
        let obj = js_sys::Object::from(js_val);
        let entries = js_sys::Object::entries(&obj);
        let mut map = HashMap::new();

        for i in 0..entries.length() {
            let entry = entries.get(i);
            let entry_arr = js_sys::Array::from(&entry);
            let key = entry_arr.get(0).as_string().unwrap_or_default();
            let value = js_to_value(entry_arr.get(1))?;
            map.insert(key, value);
        }

        return Ok(Value::Dict(map));
    }

    Err(JsValue::from_str("Unsupported JavaScript type"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_conversion() {
        // Number
        let num = Value::Number(42.0);
        let js_num = value_to_js(&num);
        assert_eq!(js_num.as_f64(), Some(42.0));

        // String
        let s = Value::String("hello".to_string());
        let js_s = value_to_js(&s);
        assert_eq!(js_s.as_string(), Some("hello".to_string()));

        // Boolean
        let b = Value::Boolean(true);
        let js_b = value_to_js(&b);
        assert_eq!(js_b.as_bool(), Some(true));

        // Null
        let n = Value::Null;
        let js_n = value_to_js(&n);
        assert!(js_n.is_null());
    }
}
