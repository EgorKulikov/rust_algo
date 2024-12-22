use crate::numbers::num_traits::primitive::Primitive;

pub trait IntegerSqrt: Sized {
    fn sqrt(self) -> Option<u64> {
        self.root(2)
    }
    fn lower_sqrt(self) -> u64 {
        self.lower_root(2)
    }
    fn upper_sqrt(self) -> u64 {
        self.upper_root(2)
    }

    fn root(self, k: usize) -> Option<u64>;
    fn lower_root(self, k: usize) -> u64;
    fn upper_root(self, k: usize) -> u64;
}

impl<T: Primitive<u64>> IntegerSqrt for T
where
    u64: Primitive<T>,
{
    fn root(self, k: usize) -> Option<u64> {
        let this = self.to();
        let s = this.lower_root(k);
        if power(s, k) == this {
            Some(s)
        } else {
            None
        }
    }

    fn lower_root(self, k: usize) -> u64 {
        let mut s = (self.to() as f64).powf(1. / (k as f64)).round() as u64;
        while power(s, k) > self.to() {
            s -= 1;
        }
        while power(s + 1, k) <= self.to() {
            s += 1;
        }
        s
    }

    fn upper_root(self, k: usize) -> u64 {
        let s = self.lower_root(k);
        if power(s, k) == self.to() {
            s
        } else {
            s + 1
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
