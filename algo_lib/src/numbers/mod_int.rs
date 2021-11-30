use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::gcd::extended_gcd;
use crate::numbers::integer::{Integer, WeakInteger};
use crate::types::value::{ConstValue, Value};
use crate::value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait BaseModInt: WeakInteger + Neg {
    type T: Integer;

    fn new(n: Self::T) -> Self;
    fn new_from_long(n: <Self::T as Integer>::W) -> Self;
    fn inv(&self) -> Option<Self>;
    fn log(&self, alpha: Self) -> Self::T;
    fn module() -> Self::T;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModInt<T: Integer, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: Integer, V: Value<T>> ModInt<T, V> {
    fn safe_new(n: T) -> Self {
        debug_assert!(n >= T::zero() && n < V::val());
        Self {
            n,
            phantom: Default::default(),
        }
    }

    fn safe(mut n: T) -> T {
        assert!(n < V::val() + V::val() && n >= T::zero());
        if n >= V::val() {
            n -= V::val();
        }
        n
    }

    fn make_safe(&mut self) {
        self.n = Self::safe(self.n);
    }
}

impl<T: Integer, V: Value<T>> BaseModInt for ModInt<T, V> {
    type T = T;

    fn new(n: T) -> Self {
        Self::safe_new(Self::safe(n % (V::val()) + V::val()))
    }

    fn new_from_long(n: <T as Integer>::W) -> Self {
        Self::safe_new(Self::safe(
            <T as Integer>::downcast(n % (V::val()).into()) + V::val(),
        ))
    }

    fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n, V::val());
        if g != T::one() {
            None
        } else {
            Some(Self::new_from_long(x))
        }
    }

    fn log(&self, alpha: Self) -> T {
        let mut base = HashMap::new();
        let mut exp = T::zero();
        let mut pow = Self::one();
        let mut inv = *self;
        let alpha_inv = alpha.inv().unwrap();
        while exp * exp < Self::module() {
            if inv == Self::one() {
                return exp;
            }
            base.insert(inv, exp);
            exp += T::one();
            pow *= alpha;
            inv *= alpha_inv;
        }
        let step = pow;
        let mut i = T::one();
        loop {
            if let Some(b) = base.get(&pow) {
                break exp * i + *b;
            }
            pow *= step;
            i += T::one();
        }
    }

    fn module() -> T {
        V::val()
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
        self.n += V::val() - rhs.n;
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
                % <T as Integer>::W::from(V::val()),
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
        self.n = V::val() - self.n;
        self.make_safe();
        self
    }
}

impl<T: Integer + Display, V: Value<T>> Display for ModInt<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <T as Display>::fmt(&self.n, f)
    }
}

impl<T: Integer + Readable, V: Value<T>> Readable for ModInt<T, V> {
    fn read(input: &mut Input) -> Self {
        Self::new(T::read(input))
    }
}

impl<T: Integer + Writable, V: Value<T>> Writable for ModInt<T, V> {
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

impl<T: Integer + Display, V: Value<T>> std::fmt::Debug for ModInt<T, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max = T::from_u8(100);
        if self.n <= max {
            write!(f, "{}", self.n)
        } else if self.n >= V::val() - max {
            write!(f, "-{}", V::val() - self.n)
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
