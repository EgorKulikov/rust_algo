use crate::numbers::num_traits::algebra::{
    IntegerMultiplicationMonoid, IntegerSemiRingWithSub, Zero,
};
use std::mem::swap;

pub fn extended_gcd<T: IntegerSemiRingWithSub + Copy>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= b / a * y;
        (d, x, y)
    }
}

pub fn gcd<T: Copy + Zero + IntegerMultiplicationMonoid>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Copy + Zero + IntegerMultiplicationMonoid>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}

pub fn remainder<T: IntegerSemiRingWithSub + Copy>(a1: T, n1: T, a2: T, n2: T) -> Option<T> {
    let (d, m1, _) = extended_gcd(n1, n2);
    if (a2 - a1) % d != T::zero() {
        return None;
    }
    let m = lcm(n1, n2);
    let n2d = n2 / d;
    let t = (a2 - a1) / d % n2d * (m1 % n2d) % n2d;
    Some((a1 + t * n1) % m)
}

#[cfg(test)]
mod test {
    use super::*;

    fn verify_remainder(a1: i64, n1: i64, a2: i64, n2: i64) {
        match remainder(a1, n1, a2, n2) {
            Some(x) => {
                let m = lcm(n1, n2);
                // Normalize x to [0, m)
                let x = ((x % m) + m) % m;
                assert_eq!(
                    x % n1, ((a1 % n1) + n1) % n1,
                    "remainder({},{},{},{})={} but {} mod {} = {} != {}",
                    a1, n1, a2, n2, x, x, n1, x % n1, ((a1 % n1) + n1) % n1,
                );
                assert_eq!(
                    x % n2, ((a2 % n2) + n2) % n2,
                    "remainder({},{},{},{})={} but {} mod {} = {} != {}",
                    a1, n1, a2, n2, x, x, n2, x % n2, ((a2 % n2) + n2) % n2,
                );
            }
            None => {
                let d = gcd(n1, n2);
                assert_ne!(
                    (a2 - a1) % d, 0,
                    "remainder({},{},{},{}) returned None but should be solvable (gcd={})",
                    a1, n1, a2, n2, d,
                );
            }
        }
    }

    #[test]
    fn remainder_coprime() {
        // gcd(3, 5) = 1
        verify_remainder(2, 3, 3, 5);
        verify_remainder(0, 3, 0, 5);
        verify_remainder(1, 7, 3, 11);
    }

    #[test]
    fn remainder_non_coprime() {
        // gcd(4, 6) = 2 -- the case that was broken before the fix
        verify_remainder(1, 4, 3, 6);
        verify_remainder(0, 4, 2, 6);
        verify_remainder(3, 4, 1, 6);
    }

    #[test]
    fn remainder_one_divides_other() {
        // gcd(3, 6) = 3
        verify_remainder(1, 3, 1, 6);
        verify_remainder(2, 3, 5, 6);
    }

    #[test]
    fn remainder_no_solution() {
        // x ≡ 0 (mod 4), x ≡ 1 (mod 6): (1-0) % gcd(4,6) = 1 % 2 = 1 ≠ 0
        assert!(remainder(0i64, 4, 1, 6).is_none());
        // x ≡ 1 (mod 4), x ≡ 0 (mod 6): (0-1) % 2 = -1 % 2 = -1 ≠ 0
        assert!(remainder(1i64, 4, 0, 6).is_none());
    }

    #[test]
    fn remainder_same_modulus() {
        verify_remainder(3, 5, 3, 5);
    }

    #[test]
    fn remainder_larger_values() {
        verify_remainder(7, 12, 3, 8);
        verify_remainder(13, 18, 7, 12);
    }
}
