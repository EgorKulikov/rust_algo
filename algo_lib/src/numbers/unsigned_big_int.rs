use crate::io::output::{Output, Writable};
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub, SubAssign};

const BASE: u32 = 1000000000;
// const FFT_MIN_SIZE: usize = 50000;

#[derive(Eq, PartialEq, Clone)]
pub struct UBigInt {
    z: Vec<u32>,
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
        let rhs = rhs.into_u64();
        let mut carry = 0;
        for i in self.z.iter_mut() {
            let val = i.into_u64() * rhs + carry;
            *i = (val % BASE.into_u64()).into_u32();
            carry = val / BASE.into_u64();
        }
        while carry > 0 {
            self.z.push((carry % BASE.into_u64()).into_u32());
            carry /= BASE.into_u64();
        }
    }
}

impl DivAssign<u32> for UBigInt {
    fn div_assign(&mut self, rhs: u32) {
        let mut carry = 0;
        for i in self.z.iter_mut().rev() {
            let val = carry + i.into_u64();
            *i = (val / rhs.into_u64()).into_u32();
            carry = (val % rhs.into_u64()) * BASE.into_u64();
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
        if self.z.is_empty() {
            0u32.write(output);
        } else {
            self.z.last().unwrap().write(output);
            for i in self.z[0..(self.z.len() - 1)].iter().rev() {
                format!("{:09}", *i).write(output);
            }
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
