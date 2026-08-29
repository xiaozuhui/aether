// src/value.rs
//! Runtime value types for the Aether language

use crate::ast::{Expr, Located};
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
        body: Vec<Located>,
        env: Rc<RefCell<Environment>>,
        /// 定义时所在的源文件（供调试器在函数体求值期间按定义文件命中行断点）
        file: Option<String>,
    },

    /// Generator（生成器实例）
    ///
    /// 语义（0.6.0 冻结，用户选择「首次触发急切收集」）：
    /// 第一次 `NEXT(G)` 时完整执行函数体**一次**，所有 Yield 值
    /// 按序收入内核缓冲；此后 NEXT 逐个弹出，耗尽返回 Null。
    /// 函数体内的副作用因此**恰好发生一次**。无限生成器会在收集
    /// 阶段触发步数上限报错，而不是挂起。
    Generator {
        /// 共享内核：克隆生成器值（如 `Set G2 G`）消费同一序列，
        /// 类似 Python 迭代器语义
        inner: Rc<RefCell<GeneratorInner>>,
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

/// 生成器内核：定义信息 + 已收集的值序列 + 消费位置。
#[derive(Clone)]
pub struct GeneratorInner {
    /// 参数名列表（用于调用时的参数绑定与显示）
    pub params: Vec<String>,
    /// 生成器函数体
    pub body: Vec<Located>,
    /// 定义（或实例化）时捕获的环境
    pub env: Rc<RefCell<Environment>>,
    /// 已收集的 Yield 值；None 表示尚未执行过函数体
    pub collected: Option<Vec<Value>>,
    /// 下一个待弹出的位置
    pub position: usize,
}

/// 手写 Debug：内核持有环境引用（可能成环），不递归打印内容。
impl fmt::Debug for GeneratorInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match &self.collected {
            None => "not-collected".to_string(),
            Some(values) => format!("{}/{} consumed", self.position, values.len()),
        };
        f.debug_struct("GeneratorInner")
            .field("params", &self.params)
            .field("state", &state)
            .finish()
    }
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
            Value::Generator { inner } => {
                let g = inner.borrow();
                format!("<Generator ({})>", g.params.join(", "))
            }
            Value::Lazy { .. } => "<Lazy>".to_string(),
            Value::BuiltIn { name, arity } => {
                format!("<BuiltIn {} ({} args)>", name, arity)
            }
        }
    }

    /// Compare values for equality
    ///
    /// 语义（0.6.0 冻结）：
    /// - Number 之间是**严格位相等**（不再使用绝对容差——那会让大数
    ///   恒等、小数恒不等）。需要容差请显式写 `ABS(A - B) < 0.000001`。
    /// - Dict 深相等：键数一致、逐键递归比较，键序无关。
    /// - 跨类型恒为 false：`5 == TO_FRACTION(5)` 为 false，
    ///   需显式转换为同一类型后再比较。
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Fraction(a), Value::Fraction(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.equals(y))
            }
            (Value::Dict(a), Value::Dict(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .all(|(k, v)| b.get(k).is_some_and(|bv| v.equals(bv)))
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
