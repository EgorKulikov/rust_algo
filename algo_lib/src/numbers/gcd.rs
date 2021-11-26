use crate::numbers::integer::{Integer, WeakInteger};

pub fn extended_gcd<T: Integer>(a: T, b: T) -> (T, <T as Integer>::W, <T as Integer>::W) {
    if a == T::zero() {
        (b, <T as Integer>::W::zero(), <T as Integer>::W::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= <T as Integer>::W::from(b / a) * y;
        (d, x, y)
    }
}
