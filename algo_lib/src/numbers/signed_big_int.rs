use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::numbers::unsigned_big_int::UBigInt;
use crate::string::str::StrReader;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Eq, PartialEq, Clone)]
pub struct BigInt {
    value: UBigInt,
    sign: i8,
}

impl From<&[u8]> for BigInt {
    fn from(value: &[u8]) -> Self {
        if value[0] == b'-' {
            Self {
                value: (&value[1..]).into(),
                sign: -1,
            }
        } else {
            Self {
                value: value.into(),
                sign: if value.len() == 1 && value[0] == b'0' {
                    0
                } else {
                    1
                },
            }
        }
    }
}

impl From<i32> for BigInt {
    fn from(v: i32) -> Self {
        if v >= 0 {
            Self {
                value: v.into(),
                sign: if v == 0 { 0 } else { 1 },
            }
        } else {
            Self {
                value: (-v).into(),
                sign: -1,
            }
        }
    }
}

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

impl MulAssign<i32> for BigInt {
    fn mul_assign(&mut self, rhs: i32) {
        if rhs >= 0 {
            self.value *= rhs;
            self.sign *= if rhs == 0 { 0 } else { 1 };
        } else {
            self.value *= -rhs;
            self.sign *= -1;
        }
    }
}

impl DivAssign<i32> for BigInt {
    fn div_assign(&mut self, rhs: i32) {
        if rhs >= 0 {
            self.value /= rhs;
        } else {
            self.value /= -rhs;
            self.sign *= -1;
        }
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            sign: self.sign * rhs.sign,
        }
    }
}

impl<'a> Mul<&'a Self> for BigInt {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        Self {
            value: self.value * &rhs.value,
            sign: self.sign * rhs.sign,
        }
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.replace_with(|s| s * rhs);
    }
}

impl Writable for BigInt {
    fn write(&self, output: &mut Output) {
        if self.sign == -1 {
            output.print(b'-');
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
