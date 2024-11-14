use crate::collections::fx_hash_map::FxHashMap;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::value::Value;
use crate::numbers::gcd::extended_gcd;
use crate::numbers::num_traits::algebra::{Field, IntegerRing, One, Ring, Zero};
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::invertible::Invertible;
use crate::numbers::num_traits::wideable::Wideable;
use crate::{value, when};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait BaseModInt: Field + Copy {
    type W: IntegerRing + Copy + From<Self::T>;
    type T: IntegerRing + Ord + Copy + Wideable<W = Self::W>;

    fn from(v: Self::T) -> Self;
    fn module() -> Self::T;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ModInt<T, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: Copy, V: Value<T>> ModInt<T, V> {
    pub fn val(&self) -> T {
        self.n
    }
}

impl<T: Ring + Ord + Copy, V: Value<T>> ModInt<T, V> {
    unsafe fn unchecked_new(n: T) -> Self {
        debug_assert!(n >= T::zero() && n < V::val());
        Self {
            n,
            phantom: Default::default(),
        }
    }

    unsafe fn maybe_subtract_mod(mut n: T) -> T {
        debug_assert!(n < V::val() + V::val() && n >= T::zero());
        if n >= V::val() {
            n -= V::val();
        }
        n
    }
}

impl<T: IntegerRing + Ord + Copy, V: Value<T>> ModInt<T, V> {
    pub fn new(n: T) -> Self {
        unsafe { Self::unchecked_new(Self::maybe_subtract_mod(n % (V::val()) + V::val())) }
    }
}

impl<T: Copy + IntegerRing + Ord + Wideable + Hash, V: Value<T>> ModInt<T, V>
where
    T::W: Copy + IntegerRing,
{
    pub fn log(&self, alpha: Self) -> T {
        let mut base = FxHashMap::default();
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

impl<T: Wideable + Ring + Ord + Copy, V: Value<T>> ModInt<T, V>
where
    T::W: IntegerRing,
{
    pub fn new_from_wide(n: T::W) -> Self {
        unsafe {
            Self::unchecked_new(Self::maybe_subtract_mod(
                T::downcast(n % V::val().into()) + V::val(),
            ))
        }
    }
}

impl<T: Copy + IntegerRing + Ord + Wideable, V: Value<T>> Invertible for ModInt<T, V>
where
    T::W: Copy + IntegerRing,
{
    type Output = Self;

    fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n, V::val());
        if g != T::one() {
            None
        } else {
            Some(Self::new_from_wide(x))
        }
    }
}

impl<T: IntegerRing + Ord + Copy + Wideable, V: Value<T>> BaseModInt for ModInt<T, V>
where
    T::W: IntegerRing + Copy,
{
    type W = T::W;
    type T = T;

    fn from(v: Self::T) -> Self {
        Self::new(v)
    }

    fn module() -> T {
        V::val()
    }
}

impl<T: IntegerRing + Ord + Copy, V: Value<T>> From<T> for ModInt<T, V> {
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: Ring + Ord + Copy, V: Value<T>> AddAssign for ModInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        self.n = unsafe { Self::maybe_subtract_mod(self.n + rhs.n) };
    }
}

impl<T: Ring + Ord + Copy, V: Value<T>> Add for ModInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Ring + Ord + Copy, V: Value<T>> SubAssign for ModInt<T, V> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n = unsafe { Self::maybe_subtract_mod(self.n + V::val() - rhs.n) };
    }
}

impl<T: Ring + Ord + Copy, V: Value<T>> Sub for ModInt<T, V> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: IntegerRing + Ord + Copy + Wideable, V: Value<T>> MulAssign for ModInt<T, V>
where
    T::W: IntegerRing + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.n = T::downcast(T::W::from(self.n) * T::W::from(rhs.n) % T::W::from(V::val()));
    }
}

impl<T: IntegerRing + Ord + Copy + Wideable, V: Value<T>> Mul for ModInt<T, V>
where
    T::W: IntegerRing + Copy,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: IntegerRing + Ord + Copy + Wideable, V: Value<T>> DivAssign for ModInt<T, V>
where
    T::W: IntegerRing + Copy,
{
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().unwrap();
    }
}

impl<T: IntegerRing + Ord + Copy + Wideable, V: Value<T>> Div for ModInt<T, V>
where
    T::W: IntegerRing + Copy,
{
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: Ring + Ord + Copy, V: Value<T>> Neg for ModInt<T, V> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n = unsafe { Self::maybe_subtract_mod(V::val() - self.n) };
        self
    }
}

impl<T: Display, V: Value<T>> Display for ModInt<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <T as Display>::fmt(&self.n, f)
    }
}

impl<T: IntegerRing + Ord + Copy + Readable, V: Value<T>> Readable for ModInt<T, V> {
    fn read(input: &mut Input) -> Self {
        Self::new(T::read(input))
    }
}

impl<T: Writable, V: Value<T>> Writable for ModInt<T, V> {
    fn write(&self, output: &mut Output) {
        self.n.write(output);
    }
}

impl<T: Ring + Ord + Copy, V: Value<T>> Zero for ModInt<T, V> {
    fn zero() -> Self {
        unsafe { Self::unchecked_new(T::zero()) }
    }
}

impl<T: IntegerRing + Ord + Copy, V: Value<T>> One for ModInt<T, V> {
    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T, V: Value<T>> Wideable for ModInt<T, V> {
    type W = Self;

    fn downcast(w: Self::W) -> Self {
        w
    }
}

impl<T: IntegerRing + Ord + Copy + Wideable + Display + AsIndex, V: Value<T>> std::fmt::Debug
    for ModInt<T, V>
where
    T::W: IntegerRing + Copy,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let max = T::from_index(100);
        when! {
            self.n <= max => write!(f, "{}", self.n),
            self.n >= V::val() - max => write!(f, "{}", self.n - V::val()),
            else => {
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
            },
        }
    }
}

impl<T: IntegerRing + Ord + Copy + AsIndex + Wideable, V: Value<T>> AsIndex for ModInt<T, V>
where
    T::W: AsIndex + IntegerRing + Ord,
{
    fn from_index(idx: usize) -> Self {
        let t = T::W::from_index(idx);
        if t >= T::W::from(V::val()) {
            Self::new_from_wide(t)
        } else {
            unsafe { Self::unchecked_new(T::downcast(t)) }
        }
    }

    fn to_index(self) -> usize {
        self.n.to_index()
    }
}

value!(Val7: i32 = 1_000_000_007);
pub type ModInt7 = ModInt<i32, Val7>;

value!(Val9: i32 = 1_000_000_009);
pub type ModInt9 = ModInt<i32, Val9>;

value!(ValF: i32 = 998_244_353);
pub type ModIntF = ModInt<i32, ValF>;
