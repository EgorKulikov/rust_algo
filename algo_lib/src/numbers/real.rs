use crate::collections::iter_ext::iter_copied::ItersCopied;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::numbers::num_traits::invertible::Invertible;
use crate::string::str::Str;
use std::cell::Cell;
use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct Real(pub f64);

impl Real {
    pub fn round(&self) -> i64 {
        self.0.round() as i64
    }
    pub fn ceil(&self) -> i64 {
        self.0.ceil() as i64
    }
    pub fn floor(&self) -> i64 {
        self.0.floor() as i64
    }
    pub fn with_precision(&self, precision: usize) -> Str {
        let res = format!("{:.*}", precision, self.0).into_bytes();
        if res.starts_with(b"-") && res.copy_count(b'0') == precision + 1 {
            res[1..].into()
        } else {
            res.into()
        }
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<T: IntoReal + Copy> PartialOrd<T> for Real {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let other = other.into_real();
        if self == &other {
            Some(Ordering::Equal)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<T: IntoReal + Copy> PartialEq<T> for Real {
    fn eq(&self, other: &T) -> bool {
        (self.0 - other.into_real().0).abs() < Real::epsilon().0
    }
}

impl Eq for Real {}

impl Ord for Real {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

macro_rules! real_op_impl {
    ($($op: ident $method: ident $assign_op: ident $assign_method: ident $sign: tt),+) => {$(
        impl<T: IntoReal> $assign_op<T> for Real {
            fn $assign_method(&mut self, rhs: T) {
                self.0 $sign rhs.into_real().0;
            }
        }

        impl<T: IntoReal> $op<T> for Real {
            type Output = Self;

            fn $method(mut self, rhs: T) -> Self::Output {
                self.$assign_method(rhs);
                self
            }
        }
    )+};
}

real_op_impl!(
    Add add AddAssign add_assign +=,
    Sub sub SubAssign sub_assign -=,
    Mul mul MulAssign mul_assign *=,
    Div div DivAssign div_assign /=
);

impl Neg for Real {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Zero for Real {
    fn zero() -> Self {
        Self(0.0)
    }
}

impl One for Real {
    fn one() -> Self {
        Self(1.0)
    }
}

thread_local! {
    static EPSILON: Cell<Real> = const { Cell::new(Real(1e-9)) };
}

// Forwards unary methods to the underlying f64.
macro_rules! real_fn_impl {
    ($($f: ident)+) => {$(
        pub fn $f(self) -> Self {
            Self(self.0.$f())
        }
    )+};
}

impl Real {
    pub const PI: Self = Self(std::f64::consts::PI);

    real_fn_impl!(abs sqrt sin cos tan);

    pub fn hypot(x: Self, y: Self) -> Self {
        Self(f64::hypot(x.0, y.0))
    }

    pub fn atan2(y: Self, x: Self) -> Self {
        Self(f64::atan2(y.0, x.0))
    }

    pub fn epsilon() -> Self {
        EPSILON.with(|eps| eps.get())
    }

    pub fn set_epsilon(eps: impl Into<Self>) {
        EPSILON.with(|e| e.set(eps.into()));
    }
}

impl Deref for Real {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Real {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Invertible for Real {
    type Output = Self;

    fn inv(&self) -> Option<Self::Output> {
        if self == &Self::zero() {
            None
        } else {
            Some(Self(1.0 / self.0))
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
        match output.get_precision() {
            Some(precision) => {
                self.with_precision(precision).write(output);
            }
            None => {
                self.0.to_string().write(output);
            }
        }
    }
}

pub trait RealReader {
    fn read_real(&mut self) -> Real;
    fn read_real_vec(&mut self, n: usize) -> Vec<Real>;
}

impl RealReader for Input {
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

impl IntoReal for Real {
    fn into_real(self) -> Real {
        self
    }
}

macro_rules! into_real {
    ($($t: ident)+) => {$(
        impl IntoReal for $t {
            fn into_real(self) -> Real {
                Real(self as f64)
            }
        }

        impl From<$t> for Real {
            fn from(x: $t) -> Self {
                Real(x as f64)
            }
        }
    )+}
}

into_real!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
