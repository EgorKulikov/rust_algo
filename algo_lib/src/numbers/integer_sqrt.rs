pub trait IntegerSqrt: Sized {
    fn sqrt(self) -> Option<Self>;
    fn lower_sqrt(self) -> Self;
    fn upper_sqrt(self) -> Self;
}

impl IntegerSqrt for i64 {
    fn sqrt(self) -> Option<Self> {
        let s = self.lower_sqrt();
        if s * s == self {
            Some(s)
        } else {
            None
        }
    }

    fn lower_sqrt(self) -> Self {
        assert!(self >= 0);
        let mut s = (self as f64).sqrt().round() as i64;
        while s * s > self {
            s -= 1;
        }
        while (s + 1) * (s + 1) <= self {
            s += 1;
        }
        s
    }

    fn upper_sqrt(self) -> Self {
        let s = self.lower_sqrt();
        if s * s == self {
            s
        } else {
            s + 1
        }
    }
}
