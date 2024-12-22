use crate::collections::fx_hash_map::FxHashMap;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::value::Value;
use crate::numbers::gcd::extended_gcd;
use crate::numbers::num_traits::algebra::{Field, One, Zero};
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::invertible::Invertible;
use crate::{value, when};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod convolution;
pub mod mod_utils;
pub mod prime_fft;

pub trait BaseModInt<T>: Field + Copy {
    fn from(v: T) -> Self;
    fn module() -> T;
    fn value(&self) -> T;
}

macro_rules! mod_int {
    ($name: ident, $t: ty, $s: ty, $w: ty) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
        pub struct $name<V: Value<$t>> {
            n: $t,
            phantom: PhantomData<V>,
        }

        impl<V: Value<$t>> $name<V> {
            unsafe fn unchecked_new(n: $t) -> Self {
                debug_assert!(n < V::val());
                Self {
                    n,
                    phantom: Default::default(),
                }
            }

            unsafe fn maybe_subtract_mod(mut n: $t) -> $t {
                debug_assert!(n < 2 * V::val());
                if n >= V::val() {
                    n -= V::val();
                }
                n
            }
        }

        impl<V: Value<$t>> $name<V> {
            pub fn new(n: $t) -> Self {
                unsafe { Self::unchecked_new(n % (V::val())) }
            }

            pub fn new_signed(n: $s) -> Self {
                unsafe {
                    Self::unchecked_new(Self::maybe_subtract_mod(
                        (n % (V::val() as $s) + V::val() as $s) as $t,
                    ))
                }
            }

            pub fn val(&self) -> $t {
                self.n
            }
        }

        impl<V: Value<$t>> $name<V> {
            pub fn log(&self, alpha: Self) -> $t {
                let mut base = FxHashMap::default();
                let mut exp = 0;
                let mut pow = Self::one();
                let mut inv = *self;
                let alpha_inv = alpha.inv().unwrap();
                while exp * exp < Self::module() {
                    if inv == Self::one() {
                        return exp;
                    }
                    base.insert(inv, exp);
                    exp += 1;
                    pow *= alpha;
                    inv *= alpha_inv;
                }
                let step = pow;
                let mut i = 1;
                loop {
                    if let Some(b) = base.get(&pow) {
                        break exp * i + *b;
                    }
                    pow *= step;
                    i += 1;
                }
            }
        }

        impl<V: Value<$t>> $name<V> {
            pub fn new_from_wide(n: $w) -> Self {
                unsafe {
                    Self::unchecked_new(Self::maybe_subtract_mod(
                        (n % V::val() as $w + V::val() as $w) as $t,
                    ))
                }
            }
        }

        impl<V: Value<$t>> Invertible for $name<V> {
            type Output = Self;

            fn inv(&self) -> Option<Self> {
                let (g, x, _) = extended_gcd(self.n as $w, V::val() as $w);
                if g != 1 {
                    None
                } else {
                    Some(Self::new_from_wide(x))
                }
            }
        }

        impl<V: Value<$t>> BaseModInt<$t> for $name<V> {
            fn from(v: $t) -> Self {
                Self::new(v)
            }

            fn module() -> $t {
                V::val()
            }

            fn value(&self) -> $t {
                self.n
            }
        }

        impl<V: Value<$t>> From<$t> for $name<V> {
            fn from(n: $t) -> Self {
                Self::new(n)
            }
        }

        impl<V: Value<$t>> AddAssign for $name<V> {
            fn add_assign(&mut self, rhs: Self) {
                self.n = unsafe { Self::maybe_subtract_mod(self.n + rhs.n) };
            }
        }

        impl<V: Value<$t>> Add for $name<V> {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }

        impl<V: Value<$t>> SubAssign for $name<V> {
            fn sub_assign(&mut self, rhs: Self) {
                self.n = unsafe { Self::maybe_subtract_mod(self.n + V::val() - rhs.n) };
            }
        }

        impl<V: Value<$t>> Sub for $name<V> {
            type Output = Self;

            fn sub(mut self, rhs: Self) -> Self::Output {
                self -= rhs;
                self
            }
        }

        impl<V: Value<$t>> MulAssign for $name<V> {
            fn mul_assign(&mut self, rhs: Self) {
                self.n = ((self.n as $w) * (rhs.n as $w) % (V::val() as $w)) as $t;
            }
        }

        impl<V: Value<$t>> Mul for $name<V> {
            type Output = Self;

            fn mul(mut self, rhs: Self) -> Self::Output {
                self *= rhs;
                self
            }
        }

        impl<V: Value<$t>> DivAssign for $name<V> {
            #[allow(clippy::suspicious_op_assign_impl)]
            fn div_assign(&mut self, rhs: Self) {
                *self *= rhs.inv().unwrap();
            }
        }

        impl<V: Value<$t>> Div for $name<V> {
            type Output = Self;

            fn div(mut self, rhs: Self) -> Self::Output {
                self /= rhs;
                self
            }
        }

        impl<V: Value<$t>> Neg for $name<V> {
            type Output = Self;

            fn neg(mut self) -> Self::Output {
                if self.n != 0 {
                    self.n = V::val() - self.n;
                }
                self
            }
        }

        impl<V: Value<$t>> Display for $name<V> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                <$t as Display>::fmt(&self.n, f)
            }
        }

        impl<V: Value<$t>> Readable for $name<V> {
            fn read(input: &mut Input) -> Self {
                Self::new(input.read())
            }
        }

        impl<V: Value<$t>> Writable for $name<V> {
            fn write(&self, output: &mut Output) {
                self.n.write(output);
            }
        }

        impl<V: Value<$t>> Zero for $name<V> {
            fn zero() -> Self {
                unsafe { Self::unchecked_new(0) }
            }
        }

        impl<V: Value<$t>> One for $name<V> {
            fn one() -> Self {
                Self::new(1)
            }
        }

        impl<V: Value<$t>> std::fmt::Debug for $name<V> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let max = 100;
                when! {
                    self.n <= max => write!(f, "{}", self.n),
                    self.n >= V::val() - max => write!(f, "-{}", V::val() - self.n),
                    else => {
                        for denominator in 1..max {
                            for num in 1..max {
                                if Self::new(num) / Self::new(denominator) == *self {
                                    return write!(f, "{}/{}", num, denominator);
                                }
                                if -Self::new(num) / Self::new(denominator) == *self {
                                    return write!(f, "-{}/{}", num, denominator);
                                }
                            }
                        }
                        write!(f, "(?? {} ??)", self.n)
                    },
                }
            }
        }

        impl<V: Value<$t>> AsIndex for $name<V> {
            fn from_index(idx: usize) -> Self {
                let v = idx as $w;
                if v >= V::val() as $w {
                    Self::new_from_wide(v)
                } else {
                    unsafe { Self::unchecked_new(v as $t) }
                }
            }

            fn to_index(self) -> usize {
                self.n.to_index()
            }
        }
    };
}

mod_int!(ModInt, u32, i32, i64);
mod_int!(ModInt64, u64, i64, i128);

value!(Val7: u32 = 1_000_000_007);
pub type ModInt7 = ModInt<Val7>;

value!(Val9: u32 = 1_000_000_009);
pub type ModInt9 = ModInt<Val9>;

value!(ValF: u32 = 998_244_353);
pub type ModIntF = ModInt<ValF>;
