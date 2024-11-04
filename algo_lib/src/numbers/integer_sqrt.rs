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
        if power(s, k) == self {
            Some(s)
        } else {
            None
        }
    }

    fn lower_root(self, k: usize) -> Self {
        assert!(self >= 0);
        let mut s = (self as f64).powf(1. / (k as f64)).round() as i64;
        while power(s, k) > self {
            s -= 1;
        }
        while power(s + 1, k) <= self {
            s += 1;
        }
        s
    }

    fn upper_root(self, k: usize) -> Self {
        let s = self.lower_root(k);
        if power(s, k) == self {
            s
        } else {
            s + 1
        }
    }
}

fn power(n: i64, exp: usize) -> i64 {
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
