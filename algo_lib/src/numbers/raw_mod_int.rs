use crate::numbers::gcd::extended_gcd;
use crate::numbers::integer::{Integer, WeakInteger};
use std::collections::HashMap;
use std::ops::Neg;

pub trait RawModInt<T: Integer>: WeakInteger + Neg {
    type T;

    fn module() -> T;
    fn n(&self) -> T;
    fn n_mut<'s>(&'s mut self) -> &'s mut T;
    fn safe_new(n: T) -> Self;

    fn new(n: T) -> Self {
        Self::safe_new(Self::safe(n % (Self::module()) + Self::module()))
    }

    fn new_from_long(n: <T as Integer>::W) -> Self {
        Self::safe_new(Self::safe(
            <T as Integer>::downcast(n % (Self::module()).into()) + Self::module(),
        ))
    }

    fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n(), Self::module());
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

    fn safe(mut n: T) -> T {
        assert!(n < Self::module() + Self::module() && n >= T::zero());
        if n >= Self::module() {
            n -= Self::module();
        }
        n
    }

    fn make_safe(&mut self) {
        *self.n_mut() = Self::safe(self.n());
    }
}
