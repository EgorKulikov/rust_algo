use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::num_traits::field::Field;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait RealTrait: Ord + Field {
    const PI: Self;

    fn abs(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn hypot(x: Self, y: Self) -> Self;
    fn atan2(y: Self, x: Self) -> Self;

    fn epsilon() -> Self;
    fn set_epsilon(eps: Self);
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Real(f64);

impl Real {
    pub fn round(&self) -> i64 {
        self.0.round() as i64
    }
}

impl Eq for Real {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Real {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl AddAssign for Real {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Add for Real {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Real {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Sub for Real {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign for Real {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Mul for Real {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0 *= rhs.0;
        self
    }
}

impl DivAssign for Real {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Div for Real {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Neg for Real {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl ZeroOne for Real {
    fn zero() -> Self {
        Self(0.0)
    }

    fn one() -> Self {
        Self(1.0)
    }
}

static mut EPSILON: Real = Real(1e-9);

impl RealTrait for Real {
    const PI: Self = Self(std::f64::consts::PI);

    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }

    fn sin(&self) -> Self {
        Self(self.0.sin())
    }

    fn cos(&self) -> Self {
        Self(self.0.cos())
    }

    fn tan(&self) -> Self {
        Self(self.0.tan())
    }

    fn hypot(x: Self, y: Self) -> Self {
        Self(f64::hypot(x.0, y.0))
    }

    fn atan2(y: Self, x: Self) -> Self {
        Self(f64::atan2(y.0, x.0))
    }

    fn epsilon() -> Self {
        unsafe { EPSILON }
    }

    fn set_epsilon(eps: Self) {
        unsafe {
            EPSILON = eps;
        }
    }
}

impl Readable for Real {
    fn read(input: &mut Input) -> Self {
        Self(
            String::from_utf8(input.next_token().unwrap())
                .unwrap()
                .parse()
                .unwrap(),
        )
    }
}

impl Writable for Real {
    fn write(&self, output: &mut Output) {
        self.0.to_string().write(output);
    }
}

pub trait RealReader {
    fn read_real(&mut self) -> Real;
    fn read_real_vec(&mut self, n: usize) -> Vec<Real>;
}

impl RealReader for Input<'_> {
    fn read_real(&mut self) -> Real {
        self.read()
    }

    fn read_real_vec(&mut self, n: usize) -> Vec<Real> {
        self.read_vec(n)
    }
}

pub trait IntoReal {
    fn into_real(self) -> Real;
}

macro_rules! into_real {
    ($($t: ident)+) => {$(
        impl IntoReal for $t {
            fn into_real(self) -> Real {
                Real(self as f64)
            }
        }
    )+};
}

into_real!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f64);
