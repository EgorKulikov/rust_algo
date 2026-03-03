use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::num_traits::algebra::{AdditionMonoid, One};

pub fn combinations_table<T: AdditionMonoid + One + Copy>(n: usize) -> Arr2d<T> {
    let mut res = Arr2d::new(n + 1, n + 1, T::zero());
    for i in 0..=n {
        res[(i, 0)] = T::one();
        for j in 1..=i {
            res[(i, j)] = res[(i - 1, j - 1)] + res[(i - 1, j)];
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::combinations_table;

    #[test]
    fn pascal() {
        let c = combinations_table::<i64>(10);
        assert_eq!(c[(5, 2)], 10);
        assert_eq!(c[(10, 3)], 120);
        assert_eq!(c[(0, 0)], 1);
        assert_eq!(c[(7, 0)], 1);
    }
}
