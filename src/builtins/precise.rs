// src/builtins/precise.rs
use crate::evaluator::RuntimeError;
use crate::value::Value;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, ToPrimitive, Zero};

pub fn to_fraction(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Number(n) => {
            let s = format!("{}", n);
            if let Some(dot_pos) = s.find('.') {
                let decimal_places = s.len() - dot_pos - 1;
                let denominator = 10_i64.pow(decimal_places as u32);
                let numerator = (n * denominator as f64).round() as i64;
                let frac = Ratio::new(BigInt::from(numerator), BigInt::from(denominator));
                Ok(Value::Fraction(frac))
            } else {
                let frac = Ratio::new(BigInt::from(*n as i64), BigInt::one());
                Ok(Value::Fraction(frac))
            }
        }
        Value::Fraction(f) => Ok(Value::Fraction(f.clone())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Number or Fraction".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

pub fn to_float(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => {
            let num = f.numer().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert numerator".to_string())
            })?;
            let den = f.denom().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert denominator".to_string())
            })?;
            Ok(Value::Number(num / den))
        }
        Value::Number(n) => Ok(Value::Number(*n)),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction or Number".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

pub fn simplify(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => Ok(Value::Fraction(f.reduced())),
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

fn value_to_fraction(value: &Value) -> Result<Ratio<BigInt>, RuntimeError> {
    match value {
        Value::Fraction(f) => Ok(f.clone()),
        Value::Number(n) => {
            let s = format!("{}", n);
            if let Some(dot_pos) = s.find('.') {
                let decimal_places = s.len() - dot_pos - 1;
                let denominator = 10_i64.pow(decimal_places as u32);
                let numerator = (n * denominator as f64).round() as i64;
                Ok(Ratio::new(
                    BigInt::from(numerator),
                    BigInt::from(denominator),
                ))
            } else {
                Ok(Ratio::new(BigInt::from(*n as i64), BigInt::one()))
            }
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction or Number".to_string(),
            got: format!("{:?}", value),
        }),
    }
}

pub fn frac_add(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    Ok(Value::Fraction(frac1 + frac2))
}

pub fn frac_sub(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    Ok(Value::Fraction(frac1 - frac2))
}

pub fn frac_mul(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    Ok(Value::Fraction(frac1 * frac2))
}

pub fn frac_div(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let frac1 = value_to_fraction(&args[0])?;
    let frac2 = value_to_fraction(&args[1])?;
    if frac2.is_zero() {
        return Err(RuntimeError::InvalidOperation(
            "Division by zero".to_string(),
        ));
    }
    Ok(Value::Fraction(frac1 / frac2))
}

pub fn numerator(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => {
            let num = f.numer().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert numerator".to_string())
            })?;
            Ok(Value::Number(num))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

pub fn denominator(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Fraction(f) => {
            let den = f.denom().to_f64().ok_or_else(|| {
                RuntimeError::InvalidOperation("Failed to convert denominator".to_string())
            })?;
            Ok(Value::Number(den))
        }
        _ => Err(RuntimeError::TypeErrorDetailed {
            expected: "Fraction".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

pub fn gcd(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let a = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            })
        }
    };
    let b = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[1]),
            })
        }
    };
    fn gcd_impl(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }
    Ok(Value::Number(gcd_impl(a, b) as f64))
}

pub fn lcm(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
        });
    }
    let a = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[0]),
            })
        }
    };
    let b = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => {
            return Err(RuntimeError::TypeErrorDetailed {
                expected: "Number".to_string(),
                got: format!("{:?}", args[1]),
            })
        }
    };
    fn gcd_impl(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }
    let result = (a.abs() * b.abs()) / gcd_impl(a, b);
    Ok(Value::Number(result as f64))
}
