// src/value.rs
//! Runtime value types for the Aether language

use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Zero;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// Runtime value types
#[derive(Debug, Clone)]
pub enum Value {
    /// Numeric value (f64)
    Number(f64),

    /// Rational number (exact fraction)
    Fraction(Ratio<BigInt>),

    /// String value
    String(String),

    /// Boolean value
    Boolean(bool),

    /// Null value
    Null,

    /// Array of values
    Array(Vec<Value>),

    /// Dictionary (key-value map)
    Dict(HashMap<String, Value>),

    /// Function (closure)
    Function {
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
    },

    /// Generator (lazy iterator)
    Generator {
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
        state: GeneratorState,
    },

    /// Lazy value (computed on demand)
    Lazy {
        expr: Expr,
        env: Rc<RefCell<Environment>>,
        cached: Option<Box<Value>>,
    },

    /// Built-in function
    BuiltIn { name: String, arity: usize },
}

/// Generator execution state
#[derive(Debug, Clone)]
pub enum GeneratorState {
    /// Not started yet
    NotStarted,

    /// Running with current position
    Running { position: usize },

    /// Completed
    Done,
}

impl Value {
    /// Check if value is truthy (for conditional evaluation)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::Fraction(f) => !f.is_zero(),
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Dict(dict) => !dict.is_empty(),
            _ => true,
        }
    }

    /// Get type name as string
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::Fraction(_) => "Fraction",
            Value::String(_) => "String",
            Value::Boolean(_) => "Boolean",
            Value::Null => "Null",
            Value::Array(_) => "Array",
            Value::Dict(_) => "Dict",
            Value::Function { .. } => "Function",
            Value::Generator { .. } => "Generator",
            Value::Lazy { .. } => "Lazy",
            Value::BuiltIn { .. } => "BuiltIn",
        }
    }

    /// Convert to number if possible
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Fraction(f) => Some(
                f.numer().to_string().parse::<f64>().ok()?
                    / f.denom().to_string().parse::<f64>().ok()?,
            ),
            Value::Boolean(true) => Some(1.0),
            Value::Boolean(false) => Some(0.0),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert to string
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                // Format number nicely (remove .0 for integers)
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    format!("{}", n)
                }
            }
            Value::Fraction(f) => {
                if f.is_integer() {
                    format!("{}", f.numer())
                } else {
                    format!("{}/{}", f.numer(), f.denom())
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "Null".to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Dict(dict) => {
                let pairs: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            Value::Function { name, params, .. } => {
                if let Some(n) = name {
                    format!("<Function {} ({})>", n, params.join(", "))
                } else {
                    format!("<Function ({})>", params.join(", "))
                }
            }
            Value::Generator { params, .. } => {
                format!("<Generator ({})>", params.join(", "))
            }
            Value::Lazy { .. } => "<Lazy>".to_string(),
            Value::BuiltIn { name, arity } => {
                format!("<BuiltIn {} ({} args)>", name, arity)
            }
        }
    }

    /// Compare values for equality
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Fraction(a), Value::Fraction(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.equals(y))
            }
            _ => false,
        }
    }

    /// Compare values for ordering
    pub fn compare(&self, other: &Value) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::Fraction(a), Value::Fraction(b)) => Some(a.cmp(b)),
            (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
            (Value::Boolean(a), Value::Boolean(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
