use crate::numbers::signed_integers::SignedInteger;

pub fn extended_gcd<T: SignedInteger>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (d, y, mut x) = extended_gcd(b.clone() % a.clone(), a.clone());
        x -= (b / a) * y.clone();
        (d, x, y)
    }
}
