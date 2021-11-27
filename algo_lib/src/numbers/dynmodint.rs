use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::gcd::extended_gcd;
use crate::numbers::integer::{Integer, WeakInteger};
use crate::numbers::value::DynamicValue;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DynModInt<T: Integer, V: DynamicValue<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: Integer, V: DynamicValue<T>> DynModInt<T, V> {
    pub fn new(n: T) -> Self {
        let mut res = Self {
            n: n % (V::val()) + V::val(),
            phantom: Default::default(),
        };
        res.safe();
        res
    }

    pub fn new_from_long(n: <T as Integer>::W) -> Self {
        let mut res = Self {
            n: <T as Integer>::downcast(n % (V::val()).into()) + V::val(),
            phantom: Default::default(),
        };
        res.safe();
        res
    }

    pub fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n, V::val());
        if g != T::one() {
            None
        } else {
            Some(Self::new_from_long(x))
        }
    }

    pub fn log(&self, alpha: Self) -> T {
        let mut base = HashMap::new();
        let mut exp = T::zero();
        let mut pow = Self::one();
        let mut inv = *self;
        let alpha_inv = alpha.inv().unwrap();
        while exp * exp < V::val() {
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

    fn safe(&mut self) -> &mut Self {
        debug_assert!(self.n >= T::zero());
        debug_assert!(self.n < V::val() + V::val());
        if self.n >= V::val() {
            self.n -= V::val();
        }
        self
    }
}

impl<T: Integer, V: DynamicValue<T>> From<T> for DynModInt<T, V> {
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: Integer, V: DynamicValue<T>> AddAssign for DynModInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        self.n += rhs.n;
        self.safe();
    }
}

impl<T: Integer, V: DynamicValue<T>> Add for DynModInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Integer, V: DynamicValue<T>> SubAssign for DynModInt<T, V> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n += V::val() - rhs.n;
        self.safe();
    }
}

impl<T: Integer, V: DynamicValue<T>> Sub for DynModInt<T, V> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Integer, V: DynamicValue<T>> MulAssign for DynModInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n = <T as Integer>::downcast(
            <T as Integer>::W::from(self.n) * <T as Integer>::W::from(rhs.n)
                % <T as Integer>::W::from(V::val()),
        );
    }
}

impl<T: Integer, V: DynamicValue<T>> Mul for DynModInt<T, V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: Integer, V: DynamicValue<T>> DivAssign for DynModInt<T, V> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().unwrap();
    }
}

impl<T: Integer, V: DynamicValue<T>> Div for DynModInt<T, V> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: Integer, V: DynamicValue<T>> Neg for DynModInt<T, V> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n = V::val() - self.n;
        self.safe();
        self
    }
}

impl<T: Integer, V: DynamicValue<T>> Display for DynModInt<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.n.fmt(f)
    }
}

impl<T: Integer, V: DynamicValue<T>> Readable for DynModInt<T, V> {
    fn read(input: &mut Input) -> Self {
        Self::new(T::read(input))
    }
}

impl<T: Integer, V: DynamicValue<T>> Writable for DynModInt<T, V> {
    fn write(&self, output: &mut Output) {
        self.n.write(output);
    }
}

impl<T: Integer, V: DynamicValue<T>> WeakInteger for DynModInt<T, V> {
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

impl<T: Integer, V: DynamicValue<T>> std::fmt::Debug for DynModInt<T, V> {
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
                        return write!(f, "{}/{}", num, denum);
                    }
                    num += T::one();
                }
                denum += T::one();
            }
            write!(f, "(?? {} ??)", self.n)
        }
    }
}
