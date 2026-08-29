//! 数值核心：f64 ↔ Fraction 的**唯一权威转换器**与混合运算提升规则。
//!
//! 语义（0.6.0 冻结）：
//! - 纯 Number 运算（整数 / 小数 / 科学计数法字面量）保持 f64；
//! - 一旦表达式涉及 Fraction，Number 操作数**提升**为 Fraction 精确计算
//!   （见 [`lift`]），结果不再回退 f64；
//! - [`f64_to_fraction`] 将 f64 还原为「本意」分数：返回**最短的**满足
//!   浮点往返严格相等 `(p/q).to_f64() == x` 的最简分数。
//!   因此 `TO_FRACTION(1/3)` 得回 `1/3`、`TO_FRACTION(0.1)` 得 `1/10`、
//!   `TO_FRACTION(1e-7)` 得 `1/10000000`——舍入噪声被连分数重建剥掉。
//!
//! 需要容差的比较请显式书写 `ABS(A - B) < 0.000001`；
//! 相等运算符对 Number 是**严格位相等**（见 `Value::equals`）。

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// 分数值类型别名。
pub type Frac = Ratio<BigInt>;

/// 连分数展开的项数上限（f64 的连分数展开理论上可到千项，
/// 超过说明该值没有「短」分数表示，转十进制兜底）。
const MAX_CF_TERMS: usize = 200;

/// 渐近分数分母上限：10^18。超过后恢复出的分数已无认知价值。
fn denom_limit() -> BigInt {
    BigInt::from(10_u32).pow(18)
}

/// 将 f64 精确还原为最简分数。
///
/// - 整数值（含 0 与超过 i64 的大整数）走十进制字符串快路径，
///   避免 `as i64` 静默饱和；
/// - 非整数用**连分数渐近分数重建**：从 f64 的精确有理表示展开
///   连分数，逐个检查渐近分数，返回第一个浮点往返严格相等的
///   （即分子分母最短的「本意」分数）；
/// - 两个兜底：渐近分数超限（200 项或分母超 10^18）时改用 17 位
///   有效数字十进制展开；仍无法往返时返回 f64 的精确二进制有理值。
///   三条路径都保证往返恒等，永不返回「错误的有理数」。
/// - 非有限值（NaN / ±inf）返回 None。
pub fn f64_to_fraction(x: f64) -> Option<Frac> {
    if !x.is_finite() {
        return None;
    }
    // 快路径：整数值（含 |x| >= 2^52 的全部整数与 0）
    if x.fract() == 0.0 {
        return integer_ratio(x);
    }
    // |x| < 2^52 的非整数：连分数重建
    let negative = x < 0.0;
    let target = x.abs();
    let mut rem = rational_of_f64(target);

    // 渐近分数递推：h_{n} = a_n·h_{n-1} + h_{n-2}，k 同理
    let (mut h_prev, mut k_prev) = (BigInt::zero(), BigInt::one());
    let (mut h_cur, mut k_cur) = (BigInt::one(), BigInt::zero());

    for _ in 0..MAX_CF_TERMS {
        let a = rem.to_integer();
        let h = &a * &h_cur + &h_prev;
        let k = &a * &k_cur + &k_prev;
        h_prev = std::mem::replace(&mut h_cur, h);
        k_prev = std::mem::replace(&mut k_cur, k);

        // 渐近分数是否已与目标浮点严格相等（正确舍入的往返检验）
        if ratio_to_f64(&h_cur, &k_cur) == Some(target) {
            let frac = Ratio::new(h_cur, k_cur);
            return Some(if negative { -frac } else { frac });
        }

        rem -= Frac::from(a);
        if rem.is_zero() {
            // 精确有理值本身都不往返（理论不可达，防御性兜底）
            break;
        }
        rem = rem.recip();
        if k_cur > denom_limit() {
            break;
        }
    }
    decimal_fallback(x)
}

/// 有理数 h/k → f64 的**正确舍入**转换。
///
/// num-rational 未给 `Ratio<BigInt>` 实现 `ToPrimitive`，而
/// `(h.to_f64()) / (k.to_f64())` 存在双重舍入（分子分母各自先舍入），
/// 在大数与次正规数上会出错。这里用 BigInt 缩放整数除法：
/// 取 t = round(h·2^s / k) ∈ [2^53, 2^54)，再乘 2^(-s)——
/// 两步都无精度损失（t 精确、2 的幂缩放精确），整除边界即正确舍入。
pub fn ratio_to_f64(h: &BigInt, k: &BigInt) -> Option<f64> {
    use num_traits::Zero;
    if k.is_zero() {
        return None;
    }
    if h.is_zero() {
        return Some(0.0);
    }
    let negative = h.is_negative();
    let (h, k) = (h.magnitude(), k.magnitude()); // 绝对值（BigUint 视图）
    let (bits_h, bits_k) = (h.bits() as i64, k.bits() as i64);

    // 估计 s 使 t ≈ 2^53：h/k ≈ 2^(bits_h - bits_k)
    let mut s = 53 - (bits_h - bits_k);
    let (mut t, mut twice_r) = scale_divide(h, k, s);
    // 规格化到 [2^53, 2^54)，最多调整两次
    let tb = t.bits() as i64;
    if tb < 53 {
        s -= 53 - tb;
        let (t2, r2) = scale_divide(h, k, s);
        t = t2;
        twice_r = r2;
    } else if tb > 54 {
        s += tb - 54;
        let (t2, r2) = scale_divide(h, k, s);
        t = t2;
        twice_r = r2;
    }

    // 舍入：twice_r 与 k 比较决定进位
    if &twice_r >= k {
        t += 1u32;
    }
    let mantissa = t.to_f64()?; // ≤ 2^54，精确
    let value = mul_by_pow2(mantissa, -s); // 2 的幂缩放（分块避免下溢）
    Some(if negative { -value } else { value })
}

/// 精确乘以 2^e（e 可正可负）。直接 `powi` 在 |e| > 1074 时
/// 会得到 0/inf（超出 f64 指数范围），分块缩放保证次正规数可达。
fn mul_by_pow2(mut v: f64, mut e: i64) -> f64 {
    // 每块 ≤ 512，中间值始终落在 f64 动态范围内
    while e > 512 {
        v *= 2f64.powi(512);
        e -= 512;
    }
    while e < -512 {
        v *= 2f64.powi(-512);
        e += 512;
    }
    v * 2f64.powi(e as i32)
}

/// 计算 (h·2^s / k, 2·(h·2^s mod k))，s 可正可负。
fn scale_divide(
    h: &num_bigint::BigUint,
    k: &num_bigint::BigUint,
    s: i64,
) -> (num_bigint::BigUint, num_bigint::BigUint) {
    if s >= 0 {
        let num = h << s as usize;
        (&num / k, (&num % k) << 1)
    } else {
        let den = k << (-s) as usize;
        (h / &den, (h % &den) << 1)
    }
}

/// 整数值 f64 → Ratio，经十进制字符串中转避免浮点截断。
fn integer_ratio(n: f64) -> Option<Frac> {
    let s = format!("{:.0}", n);
    let big = BigInt::parse_bytes(s.as_bytes(), 10)?;
    Some(Ratio::new(big, BigInt::one()))
}

/// 提取 f64 的精确有理值 m/2^e（非整数、有限值）。
fn rational_of_f64(x: f64) -> Frac {
    debug_assert!(x.is_finite() && x > 0.0);
    let bits = x.to_bits();
    let raw_exp = ((bits >> 52) & 0x7ff) as i64;
    let mantissa_field = bits & 0xf_ffff_ffff_ffff; // 低 52 位尾数
    let (m, e) = if raw_exp == 0 {
        // 次正规数：无隐含位
        (mantissa_field, -1074_i64)
    } else {
        (mantissa_field | (1_u64 << 52), raw_exp - 1075)
    };
    let num = BigInt::from(m);
    if e >= 0 {
        Ratio::new(num << e as usize, BigInt::one())
    } else {
        Ratio::new(num, BigInt::one() << (-e) as usize)
    }
}

/// 17 位有效数字十进制展开兜底（f64 的 17 位有效数字表示唯一确定值）。
fn decimal_fallback(x: f64) -> Option<Frac> {
    let s = format!("{:.16e}", x.abs()); // 形如 "3.1415926535897931e0"
    let (mant, exp) = s.split_once('e')?;
    let digits: String = mant.chars().filter(|c| c.is_ascii_digit()).collect();
    let exp: i64 = exp.parse().ok()?;
    // digits 共 17 位（1 位整数 + 16 位小数）
    let num = BigInt::parse_bytes(digits.as_bytes(), 10)?;
    let frac = if exp >= 16 {
        Ratio::new(
            num * BigInt::from(10_u32).pow((exp - 16) as u32),
            BigInt::one(),
        )
    } else {
        Ratio::new(num, BigInt::from(10_u32).pow((16 - exp) as u32))
    };
    if ratio_to_f64(frac.numer(), frac.denom()) == Some(x) {
        Some(frac)
    } else {
        // 十进制展开也不往返（极端次正规数等）：返回精确二进制有理值
        let exact = rational_of_f64(x.abs());
        Some(if x < 0.0 { -exact } else { exact })
    }
}

/// 混合运算提升：Number → Fraction。
///
/// 涉及 Fraction 的算术一律提升 Number 操作数精确计算：
/// - 整数直接经十进制字符串转换（无精度损失）；
/// - 非整数经 [`f64_to_fraction`] 重建（`0.5 + TO_FRACTION(1/3) == 5/6`）。
pub fn lift(n: f64) -> Option<Frac> {
    f64_to_fraction(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frac(num: i64, den: i64) -> Frac {
        Ratio::new(BigInt::from(num), BigInt::from(den))
    }

    #[test]
    fn integer_fast_path() {
        assert_eq!(f64_to_fraction(2.5e3), Some(frac(2500, 1)));
        assert_eq!(f64_to_fraction(0.0), Some(frac(0, 1)));
        // 超过 i64 的整数字符串转换
        assert_eq!(
            f64_to_fraction(1e19),
            Some(Ratio::new(
                BigInt::parse_bytes(b"10000000000000000000", 10).unwrap(),
                BigInt::one()
            ))
        );
    }

    #[test]
    fn continued_fraction_recovers_intent() {
        // 舍入噪声被剥掉：1/3 的 f64 → 1/3
        assert_eq!(f64_to_fraction(1.0_f64 / 3.0), Some(frac(1, 3)));
        assert_eq!(f64_to_fraction(0.1), Some(frac(1, 10)));
        assert_eq!(f64_to_fraction(1e-7), Some(frac(1, 10_000_000)));
        assert_eq!(f64_to_fraction(2.5), Some(frac(5, 2)));
        assert_eq!(f64_to_fraction(-0.25), Some(frac(-1, 4)));
    }

    #[test]
    fn round_trip_always_exact() {
        for x in [
            1.0 / 3.0,
            0.1,
            2.5,
            std::f64::consts::PI,
            1e-7,
            1e15,
            999_999_999_999_999.0,
            0.1 + 0.2,
            123.456,
            5e-324, // 最小次正规数
        ] {
            let f = f64_to_fraction(x).expect("有限值必须可转换");
            assert_eq!(ratio_to_f64(f.numer(), f.denom()), Some(x), "往返失配: {x}");
        }
    }

    #[test]
    fn ratio_to_f64_is_correctly_rounded() {
        use std::str::FromStr;
        // 分子分母都 ≤ 2^53 时，普通 IEEE 除法即正确舍入——交叉验证
        for (h, k) in [(1i64, 3i64), (7, 11), (123456789, 987654321)] {
            let ours = ratio_to_f64(&BigInt::from(h), &BigInt::from(k)).unwrap();
            assert_eq!(ours, h as f64 / k as f64, "{h}/{k}");
        }
        // 大分子大分母：结果应与 128 位近似一致（比值 ≈ 0.125）
        let h = BigInt::from_str("123456789012345678901234567890").unwrap();
        let k = BigInt::from_str("987654321098765432109876543210").unwrap();
        let v = ratio_to_f64(&h, &k).unwrap();
        assert!((v - 0.125).abs() < 1e-8, "大数比值应接近 0.125，实际 {v}");
        // 次正规数区间：3/2^1074 是 2^-1074 的整数倍，可精确表示
        let sub = Ratio::new(BigInt::from(3u32), BigInt::one() << 1074usize);
        assert_eq!(
            ratio_to_f64(sub.numer(), sub.denom()),
            Some(3.0 * f64::from_bits(1)), // 1.0·2^-1074 即最小次正规数
            "次正规数转换"
        );
        // 零与除零
        assert_eq!(ratio_to_f64(&BigInt::zero(), &BigInt::one()), Some(0.0));
        assert_eq!(ratio_to_f64(&BigInt::one(), &BigInt::zero()), None);
    }

    #[test]
    fn non_finite_is_none() {
        assert_eq!(f64_to_fraction(f64::NAN), None);
        assert_eq!(f64_to_fraction(f64::INFINITY), None);
    }
}
