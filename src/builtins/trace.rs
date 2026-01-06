// src/builtins/trace.rs
//
// Trace built-in.
//
// Note: TRACE is implemented as a special-case inside the evaluator so it can
// write into the engine's in-memory trace buffer without performing any IO.
// The function here is only a placeholder to satisfy registry wiring.

use crate::evaluator::RuntimeError;
use crate::value::Value;

pub fn trace(_args: &[Value]) -> Result<Value, RuntimeError> {
    Ok(Value::Null)
}
