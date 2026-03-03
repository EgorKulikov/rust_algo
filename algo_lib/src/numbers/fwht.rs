use crate::numbers::num_traits::algebra::SemiRingWithSub;
use std::ops::DivAssign;

pub trait FWHT {
    fn fwht(&mut self, inverse: bool);
}

impl<T: SemiRingWithSub + DivAssign + From<usize> + Copy> FWHT for [T] {
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
                *p /= T::from(n);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::FWHT;
    use crate::numbers::mod_int::ModIntF;

    #[test]
    fn roundtrip() {
        let original = vec![
            ModIntF::from(3usize),
            ModIntF::from(1usize),
            ModIntF::from(4usize),
            ModIntF::from(1usize),
        ];
        let mut a = original.clone();
        a.fwht(false);
        a.fwht(true);
        assert_eq!(a, original);
    }

    #[test]
    fn xor_convolution() {
        let mut a = vec![
            ModIntF::from(1usize),
            ModIntF::from(2usize),
            ModIntF::from(0usize),
            ModIntF::from(0usize),
        ];
        let mut b = vec![
            ModIntF::from(3usize),
            ModIntF::from(0usize),
            ModIntF::from(1usize),
            ModIntF::from(0usize),
        ];
        a.fwht(false);
        b.fwht(false);
        let mut c: Vec<_> = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).collect();
        c.fwht(true);
        assert_eq!(
            c,
            vec![
                ModIntF::from(3usize),
                ModIntF::from(6usize),
                ModIntF::from(1usize),
                ModIntF::from(2usize),
            ]
        );
    }
}
