use crate::numbers::num_traits::algebra::IntegerSemiRingWithSub;
use crate::numbers::num_traits::as_index::AsIndex;

pub trait FWHT {
    fn fwht(&mut self, inverse: bool);
}

impl<T: IntegerSemiRingWithSub + AsIndex + Copy> FWHT for [T] {
    fn fwht(&mut self, inverse: bool) {
        assert!(!self.is_empty());
        let n = self.len();
        assert_eq!(n & (n - 1), 0);
        let mut len = 1;
        while len < n {
            for i in (0..n).step_by(2 * len) {
                let (head, tail) = self.split_at_mut(i + len);
                for (u, v) in head.iter_mut().skip(i).zip(tail.iter_mut()) {
                    (*u, *v) = (*u + *v, *u - *v);
                }
            }
            len *= 2;
        }

        if inverse {
            for p in self.iter_mut() {
                *p /= T::from_index(n);
            }
        }
    }
}
