use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::mul_div_rem::MulDiv;

pub trait FWHT {
    fn fwht(&mut self, inverse: bool);
}

impl<T: AddSub + MulDiv + AsIndex + Copy> FWHT for [T] {
    fn fwht(&mut self, inverse: bool) {
        assert!(!self.is_empty());
        let n = self.len();
        assert_eq!(n & (n - 1), 0);
        let mut len = 1;
        while len < n {
            for i in (0..n).step_by(2 * len) {
                let (head, tail) = self.split_at_mut(i + len);
                for (u, v) in head.iter_mut().skip(i).zip(tail.iter_mut().take(len)) {
                    let nu = *u + *v;
                    *v = *u - *v;
                    *u = nu;
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
