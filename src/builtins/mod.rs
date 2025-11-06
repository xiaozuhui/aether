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
pub mod precise;
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
        registry.register("PRINT", io::print, 1);
        registry.register("PRINTLN", io::println, 1);
        registry.register("INPUT", io::input, 1);

        // Array functions
        registry.register("RANGE", array::range, 1); // Variadic: 1-3 args
        registry.register("LEN", types::len, 1);
        registry.register("PUSH", array::push, 2);
        registry.register("POP", array::pop, 1);
        registry.register("MAP", array::map, 2);
        registry.register("FILTER", array::filter, 2);
        registry.register("REDUCE", array::reduce, 3);
        registry.register("JOIN", array::join, 2);
        registry.register("REVERSE", array::reverse, 1);
        registry.register("SORT", array::sort, 1);
        registry.register("SUM", array::sum, 1);
        registry.register("MAX", array::max, 1);
        registry.register("MIN", array::min, 1);

        // Dict functions
        registry.register("KEYS", dict::keys, 1);
        registry.register("VALUES", dict::values, 1);
        registry.register("HAS", dict::has, 2);
        registry.register("MERGE", dict::merge, 2);

        // String functions
        registry.register("SPLIT", string::split, 2);
        registry.register("UPPER", string::upper, 1);
        registry.register("LOWER", string::lower, 1);
        registry.register("TRIM", string::trim, 1);
        registry.register("CONTAINS", string::contains, 2);
        registry.register("STARTS_WITH", string::starts_with, 2);
        registry.register("ENDS_WITH", string::ends_with, 2);
        registry.register("REPLACE", string::replace, 3);
        registry.register("REPEAT", string::repeat, 2);

        // Math functions - Basic
        registry.register("ABS", math::abs, 1);
        registry.register("FLOOR", math::floor, 1);
        registry.register("CEIL", math::ceil, 1);
        registry.register("ROUND", math::round, 1);
        registry.register("SQRT", math::sqrt, 1);
        registry.register("POW", math::pow, 2);

        // Math functions - Trigonometry
        registry.register("SIN", math::sin, 1);
        registry.register("COS", math::cos, 1);
        registry.register("TAN", math::tan, 1);
        registry.register("ASIN", math::asin, 1);
        registry.register("ACOS", math::acos, 1);
        registry.register("ATAN", math::atan, 1);
        registry.register("ATAN2", math::atan2, 2);
        registry.register("SINH", math::sinh, 1);
        registry.register("COSH", math::cosh, 1);
        registry.register("TANH", math::tanh, 1);

        // Math functions - Logarithms & Exponentials
        registry.register("LOG", math::log, 1);
        registry.register("LN", math::ln, 1);
        registry.register("LOG2", math::log2, 1);
        registry.register("EXP", math::exp, 1);
        registry.register("EXP2", math::exp2, 1);
        registry.register("EXPM1", math::expm1, 1);
        registry.register("LOG1P", math::log1p, 1);

        // Math functions - Special
        registry.register("FACTORIAL", math::factorial, 1);
        registry.register("GAMMA", math::gamma, 1);
        registry.register("ERF", math::erf, 1);
        registry.register("HYPOT", math::hypot, 2);
        registry.register("SIGN", math::sign, 1);
        registry.register("CLAMP", math::clamp, 3);

        // Math functions - Statistics
        registry.register("MEAN", math::mean, 1);
        registry.register("MEDIAN", math::median, 1);
        registry.register("VARIANCE", math::variance, 1);
        registry.register("STD", math::std, 1);
        registry.register("QUANTILE", math::quantile, 2);

        // Math functions - Vector Operations
        registry.register("DOT", math::dot, 2);
        registry.register("NORM", math::norm, 1);
        registry.register("CROSS", math::cross, 2);
        registry.register("DISTANCE", math::distance, 2);
        registry.register("NORMALIZE", math::normalize, 1);

        // Math functions - Matrix Operations
        registry.register("MATMUL", math::matmul, 2);
        registry.register("TRANSPOSE", math::transpose, 1);
        registry.register("DETERMINANT", math::determinant, 1);
        registry.register("INVERSE", math::matrix_inverse, 1);

        // Math functions - Statistics & Regression
        registry.register("LINEAR_REGRESSION", math::linear_regression, 2);

        // Math functions - Probability Distributions
        registry.register("NORMAL_PDF", math::normal_pdf, 1); // Variadic: 1 or 3
        registry.register("NORMAL_CDF", math::normal_cdf, 1); // Variadic: 1 or 3
        registry.register("POISSON_PMF", math::poisson_pmf, 2);

        // Math constants
        registry.register("PI", math::pi, 0);
        registry.register("E", math::e, 0);
        registry.register("TAU", math::tau, 0);
        registry.register("PHI", math::phi, 0);

        // Precision arithmetic functions
        registry.register("ROUND_TO", math::round_to, 2);
        registry.register("ADD_WITH_PRECISION", math::add_with_precision, 3);
        registry.register("SUB_WITH_PRECISION", math::sub_with_precision, 3);
        registry.register("MUL_WITH_PRECISION", math::mul_with_precision, 3);
        registry.register("DIV_WITH_PRECISION", math::div_with_precision, 3);
        registry.register("SET_PRECISION", math::set_precision, 2);

        // Precise (Fraction) arithmetic functions
        registry.register("TO_FRACTION", precise::to_fraction, 1);
        registry.register("TO_FLOAT", precise::to_float, 1);
        registry.register("SIMPLIFY", precise::simplify, 1);
        registry.register("FRAC_ADD", precise::frac_add, 2);
        registry.register("FRAC_SUB", precise::frac_sub, 2);
        registry.register("FRAC_MUL", precise::frac_mul, 2);
        registry.register("FRAC_DIV", precise::frac_div, 2);
        registry.register("NUMERATOR", precise::numerator, 1);
        registry.register("DENOMINATOR", precise::denominator, 1);
        registry.register("GCD", precise::gcd, 2);
        registry.register("LCM", precise::lcm, 2);

        // Type functions
        registry.register("TYPE", types::type_of, 1);
        registry.register("TO_STRING", types::to_string, 1);
        registry.register("TO_NUMBER", types::to_number, 1);

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
