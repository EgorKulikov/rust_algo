use crate::numbers::number_ext::Power;

pub trait IntegerSqrt: Sized {
    fn sqrt(self) -> Option<Self> {
        self.root(2)
    }
    fn lower_sqrt(self) -> Self {
        self.lower_root(2)
    }
    fn upper_sqrt(self) -> Self {
        self.upper_root(2)
    }

    fn root(self, k: usize) -> Option<Self>;
    fn lower_root(self, k: usize) -> Self;
    fn upper_root(self, k: usize) -> Self;
}

impl IntegerSqrt for i64 {
    fn root(self, k: usize) -> Option<Self> {
        let s = self.lower_root(k);
        if self.power(k) == self {
            Some(s)
        } else {
            None
        }
    }

    fn lower_root(self, k: usize) -> Self {
        assert!(self >= 0);
        let mut s = (self as f64).powf(1. / (k as f64)).round() as i64;
        while s.power(k) > self {
            s -= 1;
        }
        while (s + 1).power(k) <= self {
            s += 1;
        }
        s
    }

    fn upper_root(self, k: usize) -> Self {
        let s = self.lower_root(k);
        if s.power(k) == self {
            s
        } else {
            s + 1
        }
    }
}
