use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::sign::IsSigned;
use crate::numbers::unsigned_big_int::UBigInt;
use crate::string::str::StrReader;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// Arbitrary-precision signed integer as sign and magnitude.
/// Invariant: `sign` is 0 exactly when the magnitude is zero.
#[derive(Eq, PartialEq, Clone)]
pub struct BigInt {
    value: UBigInt,
    sign: i8,
}

impl BigInt {
    fn new(value: UBigInt, sign: i8) -> Self {
        let sign = if value.is_zero() { 0 } else { sign };
        Self { value, sign }
    }

    pub fn abs(self) -> UBigInt {
        self.value
    }

    /// Value as `i128`, or `None` if it does not fit.
    pub fn to_i128(&self) -> Option<i128> {
        let v = self.value.to_u128()?;
        if self.sign < 0 {
            (v <= i128::MIN.unsigned_abs()).then(|| (v as i128).wrapping_neg())
        } else {
            v.try_into().ok()
        }
    }

    /// Quotient and remainder, truncating toward zero like the primitive types.
    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        let (q, r) = self.value.div_rem(&rhs.value);
        (Self::new(q, self.sign * rhs.sign), Self::new(r, self.sign))
    }
}

impl From<&[u8]> for BigInt {
    fn from(value: &[u8]) -> Self {
        if value[0] == b'-' {
            Self::new((&value[1..]).into(), -1)
        } else {
            Self::new(value.into(), 1)
        }
    }
}

impl From<UBigInt> for BigInt {
    fn from(value: UBigInt) -> Self {
        Self::new(value, 1)
    }
}

impl<T: Primitive<u128> + Primitive<i128> + IsSigned> From<T> for BigInt {
    fn from(v: T) -> Self {
        if T::SIGNED {
            let v = <T as Primitive<i128>>::to(v);
            Self::new(v.unsigned_abs().into(), v.signum() as i8)
        } else {
            Self::new(<T as Primitive<u128>>::to(v).into(), 1)
        }
    }
}

macro_rules! try_from_signed {
    ($($t:ident)+) => {$(
        impl TryFrom<&BigInt> for $t {
            type Error = ();

            fn try_from(v: &BigInt) -> Result<Self, ()> {
                v.to_i128().and_then(|v| v.try_into().ok()).ok_or(())
            }
        }
    )+};
}

macro_rules! try_from_unsigned {
    ($($t:ident)+) => {$(
        impl TryFrom<&BigInt> for $t {
            type Error = ();

            fn try_from(v: &BigInt) -> Result<Self, ()> {
                if v.sign < 0 {
                    Err(())
                } else {
                    (&v.value).try_into()
                }
            }
        }
    )+};
}

try_from_signed!(i8 i16 i32 i64 i128 isize);
try_from_unsigned!(u8 u16 u32 u64 u128 usize);

impl Zero for BigInt {
    fn zero() -> Self {
        Self::from(0)
    }
}

impl One for BigInt {
    fn one() -> Self {
        Self::from(1)
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl<'a> AddAssign<&'a Self> for BigInt {
    fn add_assign(&mut self, rhs: &'a Self) {
        match (self.sign, rhs.sign) {
            (0, _) => {
                *self = rhs.clone();
            }
            (_, 0) => {}
            (1, 1) | (-1, -1) => {
                self.value += &rhs.value;
            }
            (1, -1) | (-1, 1) => {
                if self.value >= rhs.value {
                    self.value -= &rhs.value;
                    if self.value.is_zero() {
                        self.sign = 0;
                    }
                } else {
                    self.value = rhs.value.clone() - &self.value;
                    self.sign *= -1;
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a> Add<&'a Self> for BigInt {
    type Output = Self;

    fn add(mut self, rhs: &'a Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, mut rhs: Self) {
        rhs.sign *= -1;
        *self += rhs;
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Neg for BigInt {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.sign *= -1;
        self
    }
}

impl MulAssign<i32> for BigInt {
    fn mul_assign(&mut self, rhs: i32) {
        // `u32`: the magnitude of `i32::MIN` does not fit in `i32`.
        self.value *= rhs.unsigned_abs();
        self.sign *= rhs.signum() as i8;
    }
}

impl DivAssign<i32> for BigInt {
    fn div_assign(&mut self, rhs: i32) {
        self.value /= rhs.unsigned_abs();
        self.sign *= rhs.signum() as i8;
        if self.value.is_zero() {
            self.sign = 0;
        }
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<'a> Mul<&'a Self> for BigInt {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        &self * rhs
    }
}

impl<'a> Mul<&'a BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &'a BigInt) -> Self::Output {
        BigInt {
            value: &self.value * &rhs.value,
            sign: self.sign * rhs.sign,
        }
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.replace_with(|s| s * rhs);
    }
}

impl<'a> Div<&'a BigInt> for &BigInt {
    type Output = BigInt;

    fn div(self, rhs: &'a BigInt) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl Div for BigInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl<'a> DivAssign<&'a Self> for BigInt {
    fn div_assign(&mut self, rhs: &'a Self) {
        *self = &*self / rhs;
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, rhs: Self) {
        *self /= &rhs;
    }
}

impl<'a> Rem<&'a BigInt> for &BigInt {
    type Output = BigInt;

    fn rem(self, rhs: &'a BigInt) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl Rem for BigInt {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        &self % &rhs
    }
}

impl<'a> RemAssign<&'a Self> for BigInt {
    fn rem_assign(&mut self, rhs: &'a Self) {
        *self = &*self % rhs;
    }
}

impl RemAssign for BigInt {
    fn rem_assign(&mut self, rhs: Self) {
        *self %= &rhs;
    }
}

impl Writable for BigInt {
    fn write(&self, output: &mut Output) {
        if self.sign == -1 {
            output.put(b'-');
        }
        self.value.write(output);
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign == -1 {
            write!(f, "-")?;
        }
        write!(f, "{}", self.value)
    }
}

impl Debug for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialOrd<Self> for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sign != other.sign {
            return self.sign.cmp(&other.sign);
        }
        if self.sign == 0 {
            return Ordering::Equal;
        }
        if self.sign == 1 {
            self.value.cmp(&other.value)
        } else {
            other.value.cmp(&self.value)
        }
    }
}

impl Readable for BigInt {
    fn read(input: &mut Input) -> Self {
        input.read_str().as_slice().into()
    }
}

#[cfg(test)]
mod tests {
    use super::BigInt;

    #[test]
    fn multiply_and_divide_by_i32_min() {
        let mut z = BigInt::from(5);
        z *= i32::MIN;
        assert_eq!(z, BigInt::from(5i128 * i32::MIN as i128));
        assert_eq!(z.to_string(), "-10737418240");
        let mut z = BigInt::from(-(1i128 << 40));
        z /= i32::MIN;
        assert_eq!(z, BigInt::from(512));
        let mut z = BigInt::from(1i128 << 100);
        z *= i32::MIN;
        z /= i32::MIN;
        assert_eq!(z, BigInt::from(1i128 << 100));
    }

    #[test]
    fn multiply_and_divide_by_ordinary_i32() {
        let mut z = BigInt::from(123456789012345678i128);
        z *= -1000;
        assert_eq!(z, BigInt::from(-123456789012345678000i128));
        z /= 7;
        assert_eq!(z, BigInt::from(-123456789012345678000i128 / 7));
    }
}
