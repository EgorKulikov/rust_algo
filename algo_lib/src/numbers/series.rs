use crate::numbers::num_traits::algebra::Ring;
use crate::numbers::number_ext::Power;
use std::ops::Div;

pub fn sum_arithmetic_series<T: Ring + Div<Output = T> + Copy>(first: T, step: T, len: T) -> T {
    (first + first + step * (len - T::one())) * len / (T::one() + T::one())
}

pub fn sum_geometric_series<T: Ring + Div<Output = T> + Copy + PartialEq>(
    first: T,
    ratio: T,
    len: usize,
) -> T {
    if ratio == T::one() {
        // The closed form divides by `1 - ratio`; here the sum is `first * len`,
        // computed by doubling since `T` need not convert from `usize`.
        let mut res = T::zero();
        let mut term = first;
        let mut rem = len;
        while rem > 0 {
            if rem & 1 == 1 {
                res = res + term;
            }
            term = term + term;
            rem >>= 1;
        }
        return res;
    }
    first * (T::one() - ratio.power(len)) / (T::one() - ratio)
}

#[cfg(test)]
mod tests {
    use super::sum_geometric_series;
    use crate::numbers::mod_int::ModInt7 as M;

    #[test]
    fn geometric_series_with_ratio_one() {
        assert_eq!(sum_geometric_series(M::new(3), M::new(1), 5), M::new(15));
        assert_eq!(sum_geometric_series(M::new(3), M::new(1), 0), M::new(0));
        assert_eq!(
            sum_geometric_series(M::new(3), M::new(1), 3_000_000_000_000),
            M::new(3) * M::from(3_000_000_000_000usize)
        );
    }

    #[test]
    fn geometric_series_with_other_ratios() {
        assert_eq!(sum_geometric_series(M::new(3), M::new(2), 5), M::new(93));
        assert_eq!(sum_geometric_series(M::new(3), M::new(0), 5), M::new(3));
    }
}
