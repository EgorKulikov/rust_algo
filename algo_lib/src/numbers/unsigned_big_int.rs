use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::numbers::mod_int::convolution::convolution;
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::sign::IsSigned;
use crate::string::str::StrReader;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

const DIGITS: usize = 9;
const BASE: i32 = 10i32.pow(DIGITS as u32);

/// Arbitrary-precision unsigned integer, little-endian limbs in base 10^9.
/// Invariant: no trailing zero limbs, so zero is the empty vector.
#[derive(Eq, PartialEq, Clone)]
pub struct UBigInt {
    z: Vec<i32>,
}

impl UBigInt {
    pub fn is_zero(&self) -> bool {
        self.z.is_empty()
    }

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

    /// Value as `u128`, or `None` if it does not fit.
    pub fn to_u128(&self) -> Option<u128> {
        let mut res = 0u128;
        for &i in self.z.iter().rev() {
            res = res.checked_mul(BASE as u128)?.checked_add(i as u128)?;
        }
        Some(res)
    }

    /// Quotient and remainder. Schoolbook long division, O(len(self) * len(rhs)).
    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert!(!rhs.is_zero(), "division by zero");
        if self < rhs {
            return (Self::zero(), self.clone());
        }
        if rhs.z.len() == 1 {
            let mut q = self.clone();
            q /= rhs.z[0];
            return (q, Self::from(self % rhs.z[0]));
        }
        // Knuth, TAOCP vol. 2, algorithm D: normalize so the top limb of the
        // divisor is at least BASE / 2, then each quotient digit estimated from
        // the top two limbs of the remainder is at most two too large.
        let norm = BASE / (rhs.z[rhs.z.len() - 1] + 1);
        let mut a = self.clone();
        a *= norm;
        let mut b = rhs.clone();
        b *= norm;
        let n = b.z.len();
        let top = b.z[n - 1] as i64;
        let mut q = vec![0; a.z.len()];
        let mut r = Self::zero();
        for i in (0..a.z.len()).rev() {
            r.z.insert(0, a.z[i]);
            trim_zeroes(&mut r.z);
            let limb = |at: usize| r.z.get(at).copied().unwrap_or(0) as i64;
            let mut d = ((limb(n) * BASE as i64 + limb(n - 1)) / top) as i32;
            let mut bd = b.clone();
            bd *= d;
            while r < bd {
                bd -= &b;
                d -= 1;
            }
            r -= &bd;
            q[i] = d;
        }
        trim_zeroes(&mut q);
        r /= norm;
        (Self { z: q }, r)
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
        trim_zeroes(&mut res);
        Self { z: res }
    }
}

/// Conversion from any primitive integer. Panics on a negative value.
impl<T: Primitive<u128> + Primitive<i128> + IsSigned> From<T> for UBigInt {
    fn from(v: T) -> Self {
        assert!(
            !T::SIGNED || <T as Primitive<i128>>::to(v) >= 0,
            "negative value in UBigInt"
        );
        let mut v = <T as Primitive<u128>>::to(v);
        let mut z = Vec::new();
        while v > 0 {
            z.push((v % BASE as u128) as i32);
            v /= BASE as u128;
        }
        Self { z }
    }
}

macro_rules! try_from {
    ($($t:ident)+) => {$(
        impl TryFrom<&UBigInt> for $t {
            type Error = ();

            fn try_from(v: &UBigInt) -> Result<Self, ()> {
                v.to_u128().and_then(|v| v.try_into().ok()).ok_or(())
            }
        }
    )+};
}

try_from!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl Zero for UBigInt {
    fn zero() -> Self {
        Self { z: Vec::new() }
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
        trim_zeroes(&mut self.z);
    }
}

fn trim_zeroes(z: &mut Vec<i32>) {
    while z.last() == Some(&0) {
        z.pop();
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

    /// FFT-based; the combined limb count of the operands is limited to about
    /// 2^21 (roughly 19 million decimal digits) by the convolution.
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
        trim_zeroes(&mut res);
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
        trim_zeroes(&mut self.z);
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

impl<'a> Div<&'a UBigInt> for &UBigInt {
    type Output = UBigInt;

    fn div(self, rhs: &'a UBigInt) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl Div for UBigInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl<'a> DivAssign<&'a Self> for UBigInt {
    fn div_assign(&mut self, rhs: &'a Self) {
        *self = &*self / rhs;
    }
}

impl DivAssign for UBigInt {
    fn div_assign(&mut self, rhs: Self) {
        *self /= &rhs;
    }
}

impl<'a> Rem<&'a UBigInt> for &UBigInt {
    type Output = UBigInt;

    fn rem(self, rhs: &'a UBigInt) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl Rem for UBigInt {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        &self % &rhs
    }
}

impl<'a> RemAssign<&'a Self> for UBigInt {
    fn rem_assign(&mut self, rhs: &'a Self) {
        *self = &*self % rhs;
    }
}

impl RemAssign for UBigInt {
    fn rem_assign(&mut self, rhs: Self) {
        *self %= &rhs;
    }
}

impl Writable for UBigInt {
    fn write(&self, output: &mut Output) {
        match self.z.split_last() {
            None => output.put(b'0'),
            Some((top, rest)) => {
                top.write(output);
                for &i in rest.iter().rev() {
                    let mut buf = [b'0'; DIGITS];
                    let mut v = i;
                    for b in buf.iter_mut().rev() {
                        *b += (v % 10) as u8;
                        v /= 10;
                    }
                    for b in buf {
                        output.put(b);
                    }
                }
            }
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
