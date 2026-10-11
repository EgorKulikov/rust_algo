use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::numbers::num_traits::invertible::Invertible;
use crate::numbers::real::Real;
use crate::string::str::Str;
use std::cell::Cell;
use std::cmp::Ordering;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Double-double: a normalized sum `hi + lo` of two `f64`, about 106 bits of
/// mantissa (32 decimal digits) with the exponent range of `f64`.
///
/// Mirrors `Real`: arithmetic accepts anything convertible (`IntoWideReal`),
/// comparisons use a thread-local epsilon (`1e-18` by default, see
/// `set_epsilon`), and printing honours the output precision. Adapted from
/// bminaiev/rust-contests `double_double.rs`.
#[derive(Copy, Clone, Debug, Default)]
pub struct WideReal {
    hi: f64,
    lo: f64,
}

impl WideReal {
    pub const PI: Self = Self {
        hi: std::f64::consts::PI,
        lo: 1.2246467991473532e-16,
    };

    /// Normalizes `hi + lo` with an error-free TwoSum; neither argument needs
    /// to be the larger one. The fields stay private to keep this invariant.
    pub fn new(hi: f64, lo: f64) -> Self {
        let sum = hi + lo;
        if !sum.is_finite() {
            return Self { hi: sum, lo: 0.0 };
        }
        let v = sum - hi;
        let error = (hi - (sum - v)) + (lo - v);
        Self { hi: sum, lo: error }
    }

    pub fn hi(self) -> f64 {
        self.hi
    }

    pub fn lo(self) -> f64 {
        self.lo
    }

    /// Rounds back to double precision.
    pub fn to_f64(self) -> f64 {
        self.hi + self.lo
    }

    pub fn to_real(self) -> Real {
        Real(self.to_f64())
    }

    /// Truncates toward zero, saturating at the `i128` bounds (NaN gives 0).
    /// Unlike `to_f64() as i128` this keeps the integer bits of the low part.
    pub fn to_i128(self) -> i128 {
        if !self.hi.is_finite() {
            return self.hi as i128;
        }
        let limit = -(i128::MIN as f64); // 2^127
        if self.hi >= limit {
            if self.hi > limit || self.lo >= 0.0 {
                return i128::MAX;
            }
            return (i128::MAX + self.lo.floor() as i128) + 1;
        }
        if self.hi <= -limit {
            if self.hi < -limit || self.lo <= 0.0 {
                return i128::MIN;
            }
            return i128::MIN + self.lo.ceil() as i128;
        }
        let hi = self.hi as i128;
        let lo = self.lo as i128;
        let integer = hi + lo;
        let fraction = (self.hi - hi as f64) + (self.lo - lo as f64);
        if integer > 0 && fraction < 0.0 {
            integer - 1
        } else if integer < 0 && fraction > 0.0 {
            integer + 1
        } else {
            integer
        }
    }

    /// Largest integer not above `self`, as a `WideReal`.
    pub fn floor_wide(self) -> Self {
        let fh = self.hi.floor();
        if fh == self.hi {
            // `hi` is an integer, so the fractional part lives in `lo`.
            Self::new(fh, self.lo.floor())
        } else {
            Self::new(fh, 0.0)
        }
    }

    pub fn ceil_wide(self) -> Self {
        -(-self).floor_wide()
    }

    /// Rounds half away from zero, like `f64::round`.
    pub fn round_wide(self) -> Self {
        if self.hi < 0.0 {
            -(-self).round_wide()
        } else {
            (self + Self::new(0.5, 0.0)).floor_wide()
        }
    }

    pub fn floor(&self) -> i64 {
        saturate(self.floor_wide().to_i128())
    }

    pub fn ceil(&self) -> i64 {
        saturate(self.ceil_wide().to_i128())
    }

    pub fn round(&self) -> i64 {
        saturate(self.round_wide().to_i128())
    }

    pub fn abs(self) -> Self {
        if self.hi.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    pub fn sqrt(self) -> Self {
        if self.hi <= 0.0 || !self.hi.is_finite() {
            return Self::new(self.hi.sqrt(), 0.0);
        }
        // One Newton step from the double-precision root doubles its accuracy.
        let y = Self::new(self.hi.sqrt(), 0.0);
        y + (self - y * y) / (y + y)
    }

    pub fn hypot(x: Self, y: Self) -> Self {
        (x * x + y * y).sqrt()
    }

    pub fn is_finite(self) -> bool {
        self.hi.is_finite()
    }

    pub fn is_nan(self) -> bool {
        self.hi.is_nan()
    }

    /// `10^k`, exact up to about `10^31`.
    pub fn pow10(k: u32) -> Self {
        let mut res = Self::one();
        let mut base = Self::new(10.0, 0.0);
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                res *= base;
            }
            base *= base;
            k >>= 1;
        }
        res
    }

    /// Decimal representation with exactly `precision` fractional digits,
    /// correctly rounded (half away from zero). A result that is all zeros
    /// is printed without a sign, like `Real::with_precision`.
    pub fn with_precision(&self, precision: usize) -> Str {
        let negative = self.hi < 0.0;
        let scaled = self.abs() * Self::pow10(precision as u32) + Self::new(0.5, 0.0);
        if !scaled.is_finite() || scaled.hi >= 1e38 {
            // Beyond i128: fall back to double precision.
            let res = format!("{:.*}", precision, self.to_f64()).into_bytes();
            return if res.starts_with(b"-") && !res.iter().any(|c| (b'1'..=b'9').contains(c)) {
                res[1..].into()
            } else {
                res.into()
            };
        }
        let mut digits = scaled.floor_wide().to_i128().to_string().into_bytes();
        while digits.len() <= precision {
            digits.insert(0, b'0');
        }
        if precision > 0 {
            digits.insert(digits.len() - precision, b'.');
        }
        if negative && digits.iter().any(|c| (b'1'..=b'9').contains(c)) {
            digits.insert(0, b'-');
        }
        digits.into()
    }

    pub fn default_epsilon() -> Self {
        Self::new(1e-18, 0.0)
    }

    pub fn epsilon() -> Self {
        EPSILON.with(|eps| eps.get())
    }

    pub fn set_epsilon(eps: impl Into<Self>) {
        EPSILON.with(|e| e.set(eps.into()));
    }
}

fn saturate(x: i128) -> i64 {
    x.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

thread_local! {
    static EPSILON: Cell<WideReal> = const { Cell::new(WideReal { hi: 1e-18, lo: 0.0 }) };
}

impl Neg for WideReal {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
        }
    }
}

impl WideReal {
    fn add_wide(self, rhs: Self) -> Self {
        let high = Self::new(self.hi, rhs.hi);
        if !high.is_finite() {
            return high;
        }
        let low = Self::new(self.lo, rhs.lo);
        let sum = Self::new(high.hi, high.lo + low.hi);
        Self::new(sum.hi, sum.lo + low.lo)
    }

    fn sub_wide(self, rhs: Self) -> Self {
        self.add_wide(-rhs)
    }

    fn mul_wide(self, rhs: Self) -> Self {
        let hi = self.hi * rhs.hi;
        if !hi.is_finite() {
            return Self::new(hi, 0.0);
        }
        // Exact product error via fused multiply-add, then the cross terms.
        let lo = self.hi.mul_add(rhs.hi, -hi);
        let lo = self.hi.mul_add(rhs.lo, lo);
        let lo = self.lo.mul_add(rhs.hi, lo);
        let lo = self.lo.mul_add(rhs.lo, lo);
        Self::new(hi, lo)
    }

    fn div_wide(self, rhs: Self) -> Self {
        let q1 = Self::new(self.hi / rhs.hi, 0.0);
        if !q1.is_finite() || !rhs.is_finite() || self.hi == 0.0 {
            return q1;
        }
        let r1 = self - rhs * q1;
        let q2 = Self::new(r1.hi / rhs.hi, 0.0);
        let r2 = r1 - rhs * q2;
        let q3 = Self::new(r2.hi / rhs.hi, 0.0);
        q1 + q2 + q3
    }
}

macro_rules! wide_op_impl {
    ($($op: ident $method: ident $assign_op: ident $assign_method: ident $wide: ident),+) => {$(
        impl<T: IntoWideReal> $assign_op<T> for WideReal {
            fn $assign_method(&mut self, rhs: T) {
                *self = self.$wide(rhs.into_wide_real());
            }
        }

        impl<T: IntoWideReal> $op<T> for WideReal {
            type Output = Self;

            fn $method(self, rhs: T) -> Self {
                self.$wide(rhs.into_wide_real())
            }
        }
    )+};
}

wide_op_impl!(
    Add add AddAssign add_assign add_wide,
    Sub sub SubAssign sub_assign sub_wide,
    Mul mul MulAssign mul_assign mul_wide,
    Div div DivAssign div_assign div_wide
);

impl<T: IntoWideReal + Copy> PartialEq<T> for WideReal {
    fn eq(&self, other: &T) -> bool {
        (*self - other.into_wide_real())
            .abs()
            .exact_lt(Self::epsilon())
    }
}

impl Eq for WideReal {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<T: IntoWideReal + Copy> PartialOrd<T> for WideReal {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let other = other.into_wide_real();
        if self == &other {
            Some(Ordering::Equal)
        } else if self.exact_lt(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for WideReal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl WideReal {
    /// Epsilon-free comparison; NaN is never less.
    fn exact_lt(self, other: Self) -> bool {
        self.hi < other.hi || (self.hi == other.hi && self.lo < other.lo)
    }

    /// Epsilon-free comparison, for when the default tolerance is too coarse.
    pub fn cmp_exact(self, other: Self) -> Ordering {
        if self.exact_lt(other) {
            Ordering::Less
        } else if other.exact_lt(self) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Zero for WideReal {
    fn zero() -> Self {
        Self { hi: 0.0, lo: 0.0 }
    }
}

impl One for WideReal {
    fn one() -> Self {
        Self { hi: 1.0, lo: 0.0 }
    }
}

impl Invertible for WideReal {
    type Output = Self;

    fn inv(&self) -> Option<Self> {
        if self == &Self::zero() {
            None
        } else {
            Some(Self::one() / *self)
        }
    }
}

pub trait IntoWideReal {
    fn into_wide_real(self) -> WideReal;
}

impl IntoWideReal for WideReal {
    fn into_wide_real(self) -> WideReal {
        self
    }
}

impl From<f64> for WideReal {
    fn from(value: f64) -> Self {
        Self { hi: value, lo: 0.0 }
    }
}

impl From<f32> for WideReal {
    fn from(value: f32) -> Self {
        Self::from(value as f64)
    }
}

impl From<Real> for WideReal {
    fn from(value: Real) -> Self {
        Self::from(value.0)
    }
}

impl From<i128> for WideReal {
    fn from(value: i128) -> Self {
        let hi = value as f64;
        // A positive value may round up to 2^127, outside the i128 range.
        let remainder = if hi == -(i128::MIN as f64) {
            value - i128::MAX - 1
        } else {
            value - hi as i128
        };
        Self::new(hi, remainder as f64)
    }
}

impl From<u128> for WideReal {
    fn from(value: u128) -> Self {
        let high = Self::from((value >> 64) as i128);
        let scale = 18446744073709551616.0; // 2^64, exact in f64
        Self {
            hi: high.hi * scale,
            lo: high.lo * scale,
        } + Self::from(value as u64)
    }
}

macro_rules! from_integer {
    ($($t: ty)+) => {$(
        impl From<$t> for WideReal {
            fn from(value: $t) -> Self {
                Self::from(value as i128)
            }
        }
    )+};
}

from_integer!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

macro_rules! into_wide_real {
    ($($t: ty)+) => {$(
        impl IntoWideReal for $t {
            fn into_wide_real(self) -> WideReal {
                WideReal::from(self)
            }
        }
    )+};
}

into_wide_real!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 Real);

impl From<WideReal> for f64 {
    fn from(value: WideReal) -> Self {
        value.to_f64()
    }
}

impl Sum for WideReal {
    /// Pairwise summation: keeps O(log n) partial sums of power-of-two sizes.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut partials: Vec<Option<Self>> = Vec::new();
        for mut value in iter {
            let mut level = 0;
            loop {
                if level == partials.len() {
                    partials.push(Some(value));
                    break;
                }
                if let Some(previous) = partials[level].take() {
                    value = previous + value;
                    level += 1;
                } else {
                    partials[level] = Some(value);
                    break;
                }
            }
        }
        partials
            .into_iter()
            .flatten()
            .fold(Self::zero(), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for WideReal {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for WideReal {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}

impl<'a> Product<&'a Self> for WideReal {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl Readable for WideReal {
    /// Parses `[+-]digits[.digits][(e|E)[+-]digits]` with full precision.
    fn read(input: &mut Input) -> Self {
        let token = input.next_token().unwrap();
        let mut at = 0;
        let negative = token[0] == b'-';
        if token[0] == b'-' || token[0] == b'+' {
            at = 1;
        }
        let mut mantissa = Self::zero();
        let mut fraction_digits = 0i32;
        let mut seen_dot = false;
        while at < token.len() && token[at] != b'e' && token[at] != b'E' {
            if token[at] == b'.' {
                seen_dot = true;
            } else {
                mantissa = mantissa * 10 + (token[at] - b'0') as i64;
                if seen_dot {
                    fraction_digits += 1;
                }
            }
            at += 1;
        }
        let mut exponent = 0i32;
        if at < token.len() {
            let rest = std::str::from_utf8(&token[at + 1..]).unwrap();
            exponent = rest.parse().unwrap();
        }
        let shift = exponent - fraction_digits;
        let res = if shift >= 0 {
            mantissa * Self::pow10(shift as u32)
        } else {
            mantissa / Self::pow10((-shift) as u32)
        };
        if negative {
            -res
        } else {
            res
        }
    }
}

impl Writable for WideReal {
    /// With an output precision prints that many digits; without one prints
    /// all 32 significant digits with trailing zeros removed.
    fn write(&self, output: &mut Output) {
        match output.get_precision() {
            Some(precision) => self.with_precision(precision).write(output),
            None => {
                let mut s = self.with_precision(32);
                if s.contains(&b'.') {
                    while s.last() == Some(&b'0') {
                        s.pop();
                    }
                    if s.last() == Some(&b'.') {
                        s.pop();
                    }
                }
                s.write(output);
            }
        }
    }
}

pub trait WideRealReader {
    fn read_wide_real(&mut self) -> WideReal;
    fn read_wide_real_vec(&mut self, n: usize) -> Vec<WideReal>;
}

impl WideRealReader for Input {
    fn read_wide_real(&mut self) -> WideReal {
        self.read()
    }

    fn read_wide_real_vec(&mut self, n: usize) -> Vec<WideReal> {
        self.read_vec(n)
    }
}

#[cfg(test)]
mod tests {
    use super::WideReal;
    use crate::io::input::Input;
    use crate::io::output::Output;
    use crate::numbers::num_traits::algebra::{One, Zero};
    use crate::numbers::real::Real;
    use std::cmp::Ordering;

    fn fmt(x: WideReal, precision: usize) -> String {
        String::from_utf8(x.with_precision(precision).unwrap()).unwrap()
    }

    fn parse(s: &str) -> WideReal {
        Input::slice(s.as_bytes()).read()
    }

    #[test]
    fn keeps_bits_that_f64_loses() {
        let big = WideReal::from(1u64 << 60);
        let x = big + 1 - big;
        assert_eq!(x.to_f64(), 1.0);
        assert_eq!((big + 1).to_i128(), (1i128 << 60) + 1);
        let third = WideReal::one() / 3;
        assert_eq!(fmt(third, 30), "0.333333333333333333333333333333");
        assert_eq!(fmt(third * 3, 30), "1.000000000000000000000000000000");
        let r = WideReal::from(2).sqrt();
        assert!((r * r - 2).abs().to_f64() < 1e-30);
        assert_eq!((r * r).cmp_exact(WideReal::from(2)), Ordering::Greater);
    }

    #[test]
    fn mixed_operands_and_comparisons() {
        let mut x = WideReal::from(Real(1.5));
        x += 2;
        x *= 2.0;
        x -= WideReal::from(7u8);
        x /= 1i128;
        assert_eq!(x.to_f64(), 0.0);
        assert!(x == 0);
        assert!(x == WideReal::zero());
        assert!(WideReal::from(1e-25) == 0.0, "within the default epsilon");
        assert!(WideReal::from(1e-15) > 0);
        assert!(WideReal::from(1) < 2 && WideReal::from(3) > 2.5);
        WideReal::set_epsilon(1e-30);
        assert!(WideReal::from(1e-25) != 0.0);
        WideReal::set_epsilon(WideReal::default_epsilon());
    }

    #[test]
    fn rounding_to_integers() {
        let x = parse("1234567890123456789.75");
        assert_eq!(x.floor(), 1234567890123456789);
        assert_eq!(x.ceil(), 1234567890123456790);
        assert_eq!(x.round(), 1234567890123456790);
        let y = -x;
        assert_eq!(y.floor(), -1234567890123456790);
        assert_eq!(y.ceil(), -1234567890123456789);
        assert_eq!(y.round(), -1234567890123456790);
        assert_eq!(WideReal::from(2.5).round(), 3);
        assert_eq!(WideReal::from(-2.5).round(), -3);
        assert_eq!(WideReal::from(-0.3).ceil(), 0);
        assert_eq!(WideReal::from(i128::MAX).to_i128(), i128::MAX);
        assert_eq!(WideReal::from(u128::MAX).to_f64(), u128::MAX as f64);
        assert_eq!(
            (WideReal::from(u128::MAX) - WideReal::from(u128::MAX - 1)).to_f64(),
            1.0
        );
    }

    #[test]
    fn parsing_is_more_precise_than_f64() {
        let x = parse("0.1");
        assert_eq!(fmt(x * 10, 30), "1.000000000000000000000000000000");
        assert_eq!(fmt(parse("-12.5e-3"), 5), "-0.01250");
        assert_eq!(fmt(parse("+3.25E2"), 1), "325.0");
        assert_eq!(parse("1e30").to_f64(), 1e30);
        assert_eq!(
            fmt(parse("12345678901234567890123456789"), 0),
            "12345678901234567890123456789"
        );
        assert_eq!(fmt(parse(".5"), 2), "0.50");
    }

    #[test]
    fn formatting_like_real() {
        assert_eq!(fmt(WideReal::from(-10), 6), "-10.000000");
        assert_eq!(fmt(WideReal::from(-0.0004), 3), "0.000");
        assert_eq!(fmt(WideReal::from(-0.0006), 3), "-0.001");
        assert_eq!(fmt(WideReal::from(2.5), 0), "3");
        assert_eq!(fmt(WideReal::zero(), 2), "0.00");
        assert_eq!(fmt(WideReal::PI, 31), "3.1415926535897932384626433832795");
        let mut buf = Vec::new();
        {
            let mut out = Output::buf(&mut buf);
            out.print_line(WideReal::one() / 4);
            out.set_precision(3);
            out.print_line(WideReal::one() / 4);
            out.flush();
        }
        assert_eq!(String::from_utf8(buf).unwrap(), "0.25\n0.250\n");
    }

    #[test]
    fn sums_and_products() {
        // The decimal 0.1, not the f64 one: that one sums to 100.0000000000000055...
        let xs: Vec<WideReal> = (0..1000).map(|_| parse("0.1")).collect();
        let s: WideReal = xs.iter().sum();
        assert!((s - 100).abs().to_f64() < 1e-28, "{}", fmt(s, 32));
        let p: WideReal = (1..=30).map(WideReal::from).product();
        assert_eq!(p.to_i128(), (1..=30i128).product::<i128>());
    }
}
