use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::numbers::mod_int::convolution::convolution;
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::string::str::StrReader;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

const DIGITS: usize = 9;
const BASE: i32 = 10i32.pow(DIGITS as u32);

#[derive(Eq, PartialEq, Clone)]
pub struct UBigInt {
    z: Vec<i32>,
}

impl UBigInt {
    pub fn power(&self, exp: usize) -> Self {
        if exp == 0 {
            Self::one()
        } else if exp % 2 == 0 {
            let half = self.power(exp / 2);
            &half * &half
        } else {
            &self.power(exp - 1) * self
        }
    }
}

impl From<&[u8]> for UBigInt {
    fn from(value: &[u8]) -> Self {
        let mut at = value.len();
        // 1.73
        #[allow(clippy::manual_div_ceil)]
        let mut res = Vec::with_capacity((at + DIGITS - 1) / DIGITS);
        while at > 0 {
            let mut cur = 0;
            let start = at.saturating_sub(DIGITS);
            for &c in &value[start..at] {
                assert!(c.is_ascii_digit());
                cur *= 10;
                cur += (c - b'0') as i32;
            }
            res.push(cur);
            at = start;
        }
        Self { z: res }
    }
}

impl From<i32> for UBigInt {
    fn from(mut v: i32) -> Self {
        let mut z = Vec::new();
        while v > 0 {
            z.push(v % BASE);
            v /= BASE;
        }
        Self { z }
    }
}

impl Zero for UBigInt {
    fn zero() -> Self {
        Self::from(0)
    }
}

impl One for UBigInt {
    fn one() -> Self {
        Self::from(1)
    }
}

impl AddAssign for UBigInt {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl<'a> AddAssign<&'a Self> for UBigInt {
    fn add_assign(&mut self, rhs: &'a Self) {
        let mut carry = 0;
        let mut at = 0;
        if rhs.z.len() > self.z.len() {
            self.z.reserve(rhs.z.len() - self.z.len() + 1);
        }
        while at < rhs.z.len() || carry != 0 {
            if at == self.z.len() {
                self.z.push(0);
            }
            self.z[at] += if at < rhs.z.len() { rhs.z[at] } else { 0 } + carry;
            if self.z[at] >= BASE {
                self.z[at] -= BASE;
                carry = 1;
            } else {
                carry = 0;
            }
            at += 1;
        }
    }
}

impl Add for UBigInt {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a> Add<&'a Self> for UBigInt {
    type Output = Self;

    fn add(mut self, rhs: &'a Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a> SubAssign<&'a Self> for UBigInt {
    fn sub_assign(&mut self, rhs: &'a Self) {
        assert!(self.z.len() >= rhs.z.len());
        let mut carry = 0;
        for (i, j) in self.z.iter_mut().zip(rhs.z.iter()) {
            if *i >= j + carry {
                *i -= j + carry;
                carry = 0;
            } else {
                *i += BASE - (j + carry);
                carry = 1;
            }
        }
        if carry == 1 {
            let mut at = rhs.z.len();
            loop {
                if self.z[at] == 0 {
                    self.z[at] = BASE - 1;
                } else {
                    self.z[at] -= 1;
                    break;
                }
                at += 1;
            }
        }
        while !self.z.is_empty() && *self.z.last().unwrap() == 0 {
            self.z.pop();
        }
    }
}

impl SubAssign for UBigInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs);
    }
}

impl<'a> Sub<&'a Self> for UBigInt {
    type Output = Self;

    fn sub(mut self, rhs: &'a Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Sub for UBigInt {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign<i32> for UBigInt {
    fn mul_assign(&mut self, rhs: i32) {
        if rhs == 0 {
            *self = Self::zero();
            return;
        }
        let rhs = rhs as i64;
        let mut carry = 0;
        let base = BASE as i64;
        for i in self.z.iter_mut() {
            let val: i64 = (*i as i64) * rhs + carry;
            *i = (val % base) as i32;
            carry = val / base;
        }
        while carry > 0 {
            self.z.push((carry % base) as i32);
            carry /= base;
        }
    }
}

impl<'a> Mul<&'a UBigInt> for &UBigInt {
    type Output = UBigInt;

    fn mul(self, rhs: &'a UBigInt) -> Self::Output {
        let c = convolution(&self.z, &rhs.z);
        let mut carry = 0;
        let mut res = Vec::new();
        for i in c {
            carry += i;
            let last = carry % BASE as i128;
            res.push(last as i32);
            carry /= BASE as i128;
        }
        while carry > 0 {
            res.push((carry % BASE as i128) as i32);
            carry /= BASE as i128;
        }
        while let Some(d) = res.last() {
            if *d == 0 {
                res.pop();
            } else {
                break;
            }
        }
        UBigInt { z: res }
    }
}

impl Mul for UBigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl MulAssign for UBigInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.replace_with(|s| s * rhs);
    }
}

impl DivAssign<i32> for UBigInt {
    fn div_assign(&mut self, rhs: i32) {
        let rhs = rhs as i64;
        let mut carry = 0;
        let base = BASE as i64;
        for i in self.z.iter_mut().rev() {
            let val = carry + *i as i64;
            *i = (val / rhs) as i32;
            carry = (val % rhs) * base;
        }
        while let Some(d) = self.z.last() {
            if *d == 0 {
                self.z.pop();
            } else {
                break;
            }
        }
    }
}

impl Rem<i32> for &UBigInt {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        let mut res = 0i64;
        for &i in self.z.iter().rev() {
            res *= BASE as i64;
            res += i as i64;
            res %= rhs as i64;
        }
        res as i32
    }
}

impl Writable for UBigInt {
    fn write(&self, output: &mut Output) {
        if let Some(tail) = self.z.last() {
            tail.write(output);
            for &i in self.z.iter().rev().skip(1) {
                format!("{:09}", i).write(output);
            }
        } else {
            0u32.write(output);
        }
    }
}

impl Display for UBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(tail) = self.z.last() {
            write!(f, "{}", tail)?;
            for &i in self.z.iter().rev().skip(1) {
                write!(f, "{:09}", i)?;
            }
        } else {
            write!(f, "0")?;
        }
        Ok(())
    }
}

impl Debug for UBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialOrd<Self> for UBigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UBigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.z.len() != other.z.len() {
            return self.z.len().cmp(&other.z.len());
        }
        for (i, j) in self.z.iter().rev().zip(other.z.iter().rev()) {
            if i != j {
                return i.cmp(j);
            }
        }
        Ordering::Equal
    }
}

impl Readable for UBigInt {
    fn read(input: &mut Input) -> Self {
        input.read_str().as_slice().into()
    }
}
