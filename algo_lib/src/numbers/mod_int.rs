use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::value::Value;
use crate::numbers::gcd::extended_gcd;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::from_u8::FromU8;
use crate::numbers::num_traits::mul_div_rem::{MulDiv, MulDivRem};
use crate::numbers::num_traits::wideable::Wideable;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait BaseModInt: AddSub + MulDiv + Neg<Output = Self> + Copy + ZeroOne + PartialEq {
    type W: AddSub + MulDivRem + Copy + ZeroOne + From<Self::T>;
    type T: AddSub + MulDivRem + Copy + PartialEq + ZeroOne + Wideable<W = Self::W> + Ord;

    fn new(n: Self::T) -> Self;
    fn new_from_long(n: <Self::T as Wideable>::W) -> Self;
    fn inv(&self) -> Option<Self>;
    fn module() -> Self::T;
}

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct ModInt<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    n: T,
    phantom: PhantomData<V>,
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
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

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Hash, V: Value<T>>
    ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    pub fn log(&self, alpha: Self) -> T {
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
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> BaseModInt
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type W = T::W;
    type T = T;

    fn new(n: T) -> Self {
        Self::safe_new(Self::safe(n % (V::val()) + V::val()))
    }

    fn new_from_long(n: T::W) -> Self {
        Self::safe_new(Self::safe(T::downcast(n % (V::val()).into()) + V::val()))
    }

    fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n, V::val());
        if g != T::one() {
            None
        } else {
            Some(Self::new_from_long(x))
        }
    }

    fn module() -> T {
        V::val()
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> From<T>
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> AddAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn add_assign(&mut self, rhs: Self) {
        self.n += rhs.n;
        self.make_safe();
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Add
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> SubAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.n += V::val() - rhs.n;
        self.make_safe();
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Sub
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> MulAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.n = T::downcast(T::W::from(self.n) * T::W::from(rhs.n) % T::W::from(V::val()));
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Mul
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> DivAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().unwrap();
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Div
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Neg
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n = V::val() - self.n;
        self.make_safe();
        self
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Display,
        V: Value<T>,
    > Display for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <T as Display>::fmt(&self.n, f)
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Readable,
        V: Value<T>,
    > Readable for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn read(input: &mut Input) -> Self {
        Self::new(T::read(input))
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Writable,
        V: Value<T>,
    > Writable for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn write(&self, output: &mut Output) {
        self.n.write(output);
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> ZeroOne
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn zero() -> Self {
        Self::new(T::zero())
    }

    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Wideable
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type W = Self;

    fn downcast(w: Self::W) -> Self {
        w
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + FromU8, V: Value<T>>
    FromU8 for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn from_u8(n: u8) -> Self {
        Self::new(T::from_u8(n))
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Display + FromU8,
        V: Value<T>,
    > std::fmt::Debug for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max = T::from_u8(100);
        if self.n <= max {
            write!(f, "{}", self.n)
        } else if self.n >= V::val() - max {
            write!(f, "-{}", V::val() - self.n)
        } else {
            let mut denominator = T::one();
            while denominator < max {
                let mut num = T::one();
                while num < max {
                    if Self::new(num) / Self::new(denominator) == *self {
                        return write!(f, "{}/{}", num, denominator);
                    }
                    if -Self::new(num) / Self::new(denominator) == *self {
                        return write!(f, "-{}/{}", num, denominator);
                    }
                    num += T::one();
                }
                denominator += T::one();
            }
            write!(f, "(?? {} ??)", self.n)
        }
    }
}

impl<T: AddSub + MulDivRem + Copy + Eq + Wideable + Hash + ZeroOne + Ord, V: Value<T>> Eq
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + AsIndex,
        V: Value<T>,
    > AsIndex for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn from_index(idx: usize) -> Self {
        Self::new(T::from_index(idx))
    }

    fn to_index(&self) -> usize {
        self.n.to_index()
    }
}

value!(Val7, i32, 1_000_000_007);
pub type ModInt7 = ModInt<i32, Val7>;

value!(Val9, i32, 1_000_000_009);
pub type ModInt9 = ModInt<i32, Val9>;

value!(ValF, i32, 998_244_353);
pub type ModIntF = ModInt<i32, ValF>;
