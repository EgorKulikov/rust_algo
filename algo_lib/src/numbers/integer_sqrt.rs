use crate::numbers::num_traits::primitive::Primitive;

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

impl<T: Primitive<u64>> IntegerSqrt for T {
    fn root(self, k: usize) -> Option<Self> {
        let s = self.lower_root(k).to();
        if power(s, k) == self.to() {
            Some(Self::from(s))
        } else {
            None
        }
    }

    fn lower_root(self, k: usize) -> Self {
        let mut s = (self.to() as f64).powf(1. / (k as f64)).round() as u64;
        while power(s, k) > self.to() {
            s -= 1;
        }
        while power(s + 1, k) <= self.to() {
            s += 1;
        }
        Self::from(s)
    }

    fn upper_root(self, k: usize) -> Self {
        let s = self.lower_root(k).to();
        if power(s, k) == self.to() {
            Self::from(s)
        } else {
            Self::from(s + 1)
        }
    }
}

fn power(n: u64, exp: usize) -> u64 {
    if exp == 0 {
        1
    } else {
        let mut res = power(n, exp / 2);
        res = res.saturating_mul(res);
        if exp % 2 == 1 {
            res = res.saturating_mul(n);
        }
        res
    }
}
