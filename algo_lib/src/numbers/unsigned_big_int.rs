use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::string::str::{Str, StrReader};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub, SubAssign};

const DIGITS: usize = 9;
const BASE: u32 = 10u32.pow(DIGITS as u32);
// const FFT_MIN_SIZE: usize = 50000;

#[derive(Eq, PartialEq, Clone)]
pub struct UBigInt {
    z: Vec<u32>,
}

impl From<Str<'_>> for UBigInt {
    fn from(value: Str) -> Self {
        let mut at = value.len();
        let mut res = Vec::with_capacity((at + DIGITS - 1) / DIGITS);
        while at > 0 {
            let mut cur = 0;
            let start = at.saturating_sub(DIGITS);
            for &c in &value[start..at] {
                cur *= 10;
                cur += (c - b'0') as u32;
            }
            res.push(cur);
            at = start;
        }
        Self { z: res }
    }
}

impl From<u32> for UBigInt {
    fn from(mut v: u32) -> Self {
        let mut z = Vec::new();
        while v > 0 {
            z.push(v % BASE);
            v /= BASE;
        }
        Self { z }
    }
}

impl ZeroOne for UBigInt {
    fn zero() -> Self {
        Self::from(0)
    }

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

impl MulAssign<u32> for UBigInt {
    fn mul_assign(&mut self, rhs: u32) {
        if rhs == 0 {
            *self = Self::zero();
            return;
        }
        let rhs = rhs as u64;
        let mut carry = 0;
        let base = BASE as u64;
        for i in self.z.iter_mut() {
            let val: u64 = (*i as u64) * rhs + carry;
            *i = (val % base) as u32;
            carry = val / base;
        }
        while carry > 0 {
            self.z.push((carry % base) as u32);
            carry /= base;
        }
    }
}

impl DivAssign<u32> for UBigInt {
    fn div_assign(&mut self, rhs: u32) {
        let rhs = rhs as u64;
        let mut carry = 0;
        let base = BASE as u64;
        for i in self.z.iter_mut().rev() {
            let val = carry + *i as u64;
            *i = (val / rhs) as u32;
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

impl ToString for UBigInt {
    fn to_string(&self) -> String {
        if let Some(tail) = self.z.last() {
            let mut ans = tail.to_string();
            for &i in self.z.iter().rev().skip(1) {
                ans += format!("{:09}", i).as_str();
            }
            ans
        } else {
            "0".to_string()
        }
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
        input.read_str().into()
    }
}
