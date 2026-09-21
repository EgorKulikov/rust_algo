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
        if power(s, k) == Some(self.to()) {
            Some(Self::from(s))
        } else {
            None
        }
    }

    fn lower_root(self, k: usize) -> Self {
        let x = self.to();
        let mut s = (x as f64).powf(1. / (k as f64)).round() as u64;
        while power(s, k).map_or(true, |p| p > x) {
            s -= 1;
        }
        while s
            .checked_add(1)
            .and_then(|next| power(next, k))
            .map_or(false, |p| p <= x)
        {
            s += 1;
        }
        Self::from(s)
    }

    fn upper_root(self, k: usize) -> Self {
        let s = self.lower_root(k).to();
        if power(s, k) == Some(self.to()) {
            Self::from(s)
        } else {
            Self::from(s + 1)
        }
    }
}

/// `n^exp`, or `None` when it does not fit: an overflow has to stay
/// distinguishable from a value of exactly `u64::MAX`.
fn power(n: u64, exp: usize) -> Option<u64> {
    if exp == 0 {
        Some(1)
    } else {
        let half = power(n, exp / 2)?;
        let res = half.checked_mul(half)?;
        if exp % 2 == 1 {
            res.checked_mul(n)
        } else {
            Some(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntegerSqrt;

    /// Runs `f` on another thread so that an endless loop fails the test
    /// instead of hanging the suite.
    fn finishes<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> Option<T> {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(f());
        });
        rx.recv_timeout(std::time::Duration::from_secs(5)).ok()
    }

    #[test]
    fn roots_of_u64_max() {
        assert_eq!(finishes(|| u64::MAX.lower_sqrt()), Some(4294967295));
        assert_eq!(finishes(|| u64::MAX.upper_sqrt()), Some(4294967296));
        assert_eq!(finishes(|| u64::MAX.sqrt()), Some(None));
        assert_eq!(finishes(|| u64::MAX.lower_root(3)), Some(2642245));
        assert_eq!(finishes(|| u64::MAX.lower_root(64)), Some(1));
        assert_eq!(finishes(|| u64::MAX.lower_root(1)), Some(u64::MAX));
    }

    #[test]
    fn around_perfect_squares() {
        for r in [0u64, 1, 2, 3, 1000, 94906265, 3037000499, 4294967295] {
            let sq = r * r;
            assert_eq!(sq.sqrt(), Some(r));
            assert_eq!(sq.lower_sqrt(), r);
            assert_eq!(sq.upper_sqrt(), r);
            if r >= 2 {
                assert_eq!((sq - 1).lower_sqrt(), r - 1);
                assert_eq!((sq - 1).upper_sqrt(), r);
                assert_eq!((sq + 1).sqrt(), None);
                assert_eq!((sq + 1).lower_sqrt(), r);
            }
        }
        assert_eq!((i64::MAX).lower_sqrt(), 3037000499);
        assert_eq!(27i32.root(3), Some(3));
        assert_eq!(28i32.upper_root(3), 4);
    }
}
