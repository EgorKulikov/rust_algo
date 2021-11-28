use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::integer::{Integer, WeakInteger};
use crate::numbers::raw_mod_int::RawModInt;
use crate::numbers::value::Value;
use crate::value;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModInt<T: Integer, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: Integer, V: Value<T>> RawModInt<T> for ModInt<T, V> {
    type T = T;

    fn module() -> T {
        V::VAL
    }

    fn n(&self) -> T {
        self.n
    }

    fn n_mut(&mut self) -> &mut T {
        &mut self.n
    }

    fn safe_new(n: T) -> Self {
        assert!(n < V::VAL && n >= T::zero());
        Self {
            n,
            phantom: Default::default(),
        }
    }
}

impl<T: Integer, V: Value<T>> From<T> for ModInt<T, V> {
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: Integer, V: Value<T>> AddAssign for ModInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        self.n += rhs.n;
        self.make_safe();
    }
}

impl<T: Integer, V: Value<T>> Add for ModInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Integer, V: Value<T>> SubAssign for ModInt<T, V> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n += V::VAL - rhs.n;
        self.make_safe();
    }
}

impl<T: Integer, V: Value<T>> Sub for ModInt<T, V> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Integer, V: Value<T>> MulAssign for ModInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n = <T as Integer>::downcast(
            <T as Integer>::W::from(self.n) * <T as Integer>::W::from(rhs.n)
                % <T as Integer>::W::from(V::VAL),
        );
    }
}

impl<T: Integer, V: Value<T>> Mul for ModInt<T, V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: Integer, V: Value<T>> DivAssign for ModInt<T, V> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().unwrap();
    }
}

impl<T: Integer, V: Value<T>> Div for ModInt<T, V> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: Integer, V: Value<T>> Neg for ModInt<T, V> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n = V::VAL - self.n;
        self.make_safe();
        self
    }
}

impl<T: Integer, V: Value<T>> Display for ModInt<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <T as Display>::fmt(&self.n, f)
    }
}

impl<T: Integer, V: Value<T>> Readable for ModInt<T, V> {
    fn read(input: &mut Input) -> Self {
        Self::new(T::read(input))
    }
}

impl<T: Integer, V: Value<T>> Writable for ModInt<T, V> {
    fn write(&self, output: &mut Output) {
        self.n.write(output);
    }
}

impl<T: Integer, V: Value<T>> WeakInteger for ModInt<T, V> {
    type W = Self;
    fn zero() -> Self {
        Self::new(T::zero())
    }

    fn one() -> Self {
        Self::new(T::one())
    }

    fn from_u8(n: u8) -> Self {
        Self::new(T::from_u8(n))
    }

    fn downcast(w: Self::W) -> Self {
        w
    }
}

impl<T: Integer, V: Value<T>> std::fmt::Debug for ModInt<T, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max = T::from_u8(100);
        if self.n <= max {
            write!(f, "{}", self.n)
        } else if self.n >= V::VAL - max {
            write!(f, "-{}", V::VAL - self.n)
        } else {
            let mut denum = T::one();
            while denum < max {
                let mut num = T::one();
                while num < max {
                    if Self::new(num) / Self::new(denum) == *self {
                        return write!(f, "{}/{}", num, denum);
                    }
                    if -Self::new(num) / Self::new(denum) == *self {
                        return write!(f, "-{}/{}", num, denum);
                    }
                    num += T::one();
                }
                denum += T::one();
            }
            write!(f, "(?? {} ??)", self.n)
        }
    }
}

value!(Val7, u32, 1_000_000_007);
pub type ModInt7 = ModInt<u32, Val7>;

value!(Val9, u32, 1_000_000_009);
pub type ModInt9 = ModInt<u32, Val9>;

value!(ValF, u32, 998_244_353);
pub type ModIntF = ModInt<u32, ValF>;
