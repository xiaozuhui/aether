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
            Value::Function { params, .. } => {
                format!("<Function ({})>", params.join(", "))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_truthy() {
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
        assert!(!Value::Null.is_truthy());
        assert!(Value::Number(1.0).is_truthy());
        assert!(!Value::Number(0.0).is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
    }

    #[test]
    fn test_value_type_name() {
        assert_eq!(Value::Number(42.0).type_name(), "Number");
        assert_eq!(Value::String("test".to_string()).type_name(), "String");
        assert_eq!(Value::Boolean(true).type_name(), "Boolean");
        assert_eq!(Value::Null.type_name(), "Null");
        assert_eq!(Value::Array(vec![]).type_name(), "Array");
    }

    #[test]
    fn test_value_to_number() {
        assert_eq!(Value::Number(42.0).to_number(), Some(42.0));
        assert_eq!(Value::Boolean(true).to_number(), Some(1.0));
        assert_eq!(Value::Boolean(false).to_number(), Some(0.0));
        assert_eq!(Value::String("123".to_string()).to_number(), Some(123.0));
        assert_eq!(Value::String("abc".to_string()).to_number(), None);
        assert_eq!(Value::Null.to_number(), None);
    }

    #[test]
    fn test_value_to_string() {
        assert_eq!(Value::Number(42.0).to_string(), "42");
        assert_eq!(Value::Number(3.14).to_string(), "3.14");
        assert_eq!(Value::String("hello".to_string()).to_string(), "hello");
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Null.to_string(), "Null");
        assert_eq!(
            Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]).to_string(),
            "[1, 2]"
        );
    }

    #[test]
    fn test_value_equals() {
        assert!(Value::Number(42.0).equals(&Value::Number(42.0)));
        assert!(!Value::Number(42.0).equals(&Value::Number(43.0)));
        assert!(Value::String("test".to_string()).equals(&Value::String("test".to_string())));
        assert!(Value::Boolean(true).equals(&Value::Boolean(true)));
        assert!(Value::Null.equals(&Value::Null));
    }

    #[test]
    fn test_value_compare() {
        use std::cmp::Ordering;

        assert_eq!(
            Value::Number(42.0).compare(&Value::Number(43.0)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Value::String("a".to_string()).compare(&Value::String("b".to_string())),
            Some(Ordering::Less)
        );
        assert_eq!(
            Value::Boolean(false).compare(&Value::Boolean(true)),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn test_array_equality() {
        let arr1 = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        let arr2 = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        let arr3 = Value::Array(vec![Value::Number(1.0), Value::Number(3.0)]);

        assert!(arr1.equals(&arr2));
        assert!(!arr1.equals(&arr3));
    }
}
