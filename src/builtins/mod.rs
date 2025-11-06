// src/builtins/mod.rs
//! Built-in functions standard library

use crate::evaluator::RuntimeError;
use crate::value::Value;
use std::collections::HashMap;

// Module declarations
pub mod array;
pub mod dict;
pub mod io;
pub mod math;
pub mod string;
pub mod types;

/// Type alias for built-in function implementations
pub type BuiltInFn = fn(&[Value]) -> Result<Value, RuntimeError>;

/// Registry of all built-in functions
pub struct BuiltInRegistry {
    functions: HashMap<String, (BuiltInFn, usize)>, // (function, arity)
}

impl BuiltInRegistry {
    /// Create a new registry with all built-in functions
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };

        // IO functions
        registry.register("Print", io::print, 1);
        registry.register("Println", io::println, 1);
        registry.register("Input", io::input, 1);

        // Array functions
        registry.register("Range", array::range, 1); // Variadic: 1-3 args
        registry.register("Len", types::len, 1);
        registry.register("Push", array::push, 2);
        registry.register("Pop", array::pop, 1);
        registry.register("Map", array::map, 2);
        registry.register("Filter", array::filter, 2);
        registry.register("Reduce", array::reduce, 3);
        registry.register("Join", array::join, 2);
        registry.register("Reverse", array::reverse, 1);
        registry.register("Sort", array::sort, 1);
        registry.register("Sum", array::sum, 1);
        registry.register("Max", array::max, 1);
        registry.register("Min", array::min, 1);

        // Dict functions
        registry.register("Keys", dict::keys, 1);
        registry.register("Values", dict::values, 1);
        registry.register("Has", dict::has, 2);
        registry.register("Merge", dict::merge, 2);

        // String functions
        registry.register("Split", string::split, 2);
        registry.register("Upper", string::upper, 1);
        registry.register("Lower", string::lower, 1);
        registry.register("Trim", string::trim, 1);
        registry.register("Contains", string::contains, 2);
        registry.register("StartsWith", string::starts_with, 2);
        registry.register("EndsWith", string::ends_with, 2);
        registry.register("Replace", string::replace, 3);
        registry.register("Repeat", string::repeat, 2);

        // Math functions - Basic
        registry.register("Abs", math::abs, 1);
        registry.register("Floor", math::floor, 1);
        registry.register("Ceil", math::ceil, 1);
        registry.register("Round", math::round, 1);
        registry.register("Sqrt", math::sqrt, 1);
        registry.register("Pow", math::pow, 2);

        // Math functions - Trigonometry
        registry.register("Sin", math::sin, 1);
        registry.register("Cos", math::cos, 1);
        registry.register("Tan", math::tan, 1);
        registry.register("Asin", math::asin, 1);
        registry.register("Acos", math::acos, 1);
        registry.register("Atan", math::atan, 1);
        registry.register("Atan2", math::atan2, 2);
        registry.register("Sinh", math::sinh, 1);
        registry.register("Cosh", math::cosh, 1);
        registry.register("Tanh", math::tanh, 1);

        // Math functions - Logarithms & Exponentials
        registry.register("Log", math::log, 1);
        registry.register("Ln", math::ln, 1);
        registry.register("Log2", math::log2, 1);
        registry.register("Exp", math::exp, 1);
        registry.register("Exp2", math::exp2, 1);
        registry.register("Expm1", math::expm1, 1);
        registry.register("Log1p", math::log1p, 1);

        // Math functions - Special
        registry.register("Factorial", math::factorial, 1);
        registry.register("Gamma", math::gamma, 1);
        registry.register("Erf", math::erf, 1);
        registry.register("Hypot", math::hypot, 2);
        registry.register("Sign", math::sign, 1);
        registry.register("Clamp", math::clamp, 3);

        // Math functions - Statistics
        registry.register("Mean", math::mean, 1);
        registry.register("Median", math::median, 1);
        registry.register("Variance", math::variance, 1);
        registry.register("Std", math::std, 1);
        registry.register("Quantile", math::quantile, 2);

        // Math functions - Vector Operations
        registry.register("Dot", math::dot, 2);
        registry.register("Norm", math::norm, 1);
        registry.register("Cross", math::cross, 2);
        registry.register("Distance", math::distance, 2);
        registry.register("Normalize", math::normalize, 1);

        // Math functions - Matrix Operations
        registry.register("Matmul", math::matmul, 2);
        registry.register("Transpose", math::transpose, 1);
        registry.register("Determinant", math::determinant, 1);
        registry.register("Inverse", math::matrix_inverse, 1);

        // Math functions - Statistics & Regression
        registry.register("LinearRegression", math::linear_regression, 2);

        // Math functions - Probability Distributions
        registry.register("NormalPDF", math::normal_pdf, 1); // Variadic: 1 or 3
        registry.register("NormalCDF", math::normal_cdf, 1); // Variadic: 1 or 3
        registry.register("PoissonPMF", math::poisson_pmf, 2);

        // Math constants
        registry.register("PI", math::pi, 0);
        registry.register("E", math::e, 0);
        registry.register("TAU", math::tau, 0);
        registry.register("PHI", math::phi, 0);

        // Type functions
        registry.register("Type", types::type_of, 1);
        registry.register("ToString", types::to_string, 1);
        registry.register("ToNumber", types::to_number, 1);

        registry
    }

    /// Register a built-in function
    fn register(&mut self, name: &str, func: BuiltInFn, arity: usize) {
        self.functions.insert(name.to_string(), (func, arity));
    }

    /// Get a built-in function by name
    pub fn get(&self, name: &str) -> Option<(BuiltInFn, usize)> {
        self.functions.get(name).copied()
    }

    /// Check if a function exists
    pub fn has(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get all function names
    pub fn names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

impl Default for BuiltInRegistry {
    fn default() -> Self {
        Self::new()
    }
}
