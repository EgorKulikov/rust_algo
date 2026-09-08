use crate::numbers::mod_int::convolution::{modulus, Convolution};
use crate::numbers::mod_int::prime_fft::PrimeFFT;
use crate::numbers::mod_int::BaseModInt;
use crate::numbers::primes::prime::is_prime;

/// Reusable polynomial multiplication for arbitrary 32-bit moduli.
///
/// Uses one `PrimeFFT` when the modulus is a suitable prime, or a three-prime
/// [`Convolution`] otherwise. If a later operation outgrows the direct FFT's
/// capacity, the context switches to CRT and keeps those buffers for reuse.
/// The CRT transform limit is 2^21. Small operands use direct multiplication.
/// See [`Convolution`] for coefficient storage requirements above `i32::MAX`.
///
/// ```
/// use algo_lib::numbers::mod_int::{fft::FFT, ModInt7 as M};
/// let mut fft = FFT::<M>::new();
/// assert_eq!(fft.multiply(&[M::new(1), M::new(2)], &[M::new(3), M::new(4)]),
///            vec![M::new(3), M::new(10), M::new(8)]);
/// ```
pub enum FFT<M: BaseModInt<T>, T = u32> {
    Prime(PrimeFFT<M, T>),
    Convolution(Convolution<M, T>),
}

impl<T: Into<u64>, M: BaseModInt<T>> Default for FFT<M, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Into<u64>, M: BaseModInt<T>> FFT<M, T> {
    /// Selects a backend. Panics if the modulus does not fit in `u32`.
    pub fn new() -> Self {
        let p = modulus::<T, M>();
        assert!(
            (1..=u32::MAX as u64).contains(&p),
            "FFT requires a 32-bit modulus"
        );
        // The smallest non-schoolbook product needs a transform of length 128.
        if p > 2 && (p - 1) % 128 == 0 && is_prime(p) {
            Self::Prime(PrimeFFT::new())
        } else {
            Self::Convolution(Convolution::new())
        }
    }

    /// Returns the full product, or an empty vector if either input is empty.
    pub fn multiply(&mut self, a: &[M], b: &[M]) -> Vec<M> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut result = vec![M::zero(); a.len() + b.len() - 1];
        self.multiply_fix_len(a, b, &mut result);
        result
    }

    /// Writes the full product, growing `res` if needed and zeroing its tail.
    pub fn multiply_res(&mut self, a: &[M], b: &[M], res: &mut Vec<M>) {
        if a.is_empty() || b.is_empty() {
            res.fill(M::zero());
            return;
        }
        let len = a.len() + b.len() - 1;
        if res.len() < len {
            res.resize(len, M::zero());
        }
        self.multiply_fix_len(a, b, res);
    }

    /// Writes a prefix of the linear product, padding with zeros if necessary.
    pub fn multiply_fix_len(&mut self, a: &[M], b: &[M], res: &mut [M]) {
        let a = &a[..a.len().min(res.len())];
        let b = &b[..b.len().min(res.len())];
        if let Self::Prime(fft) = self {
            if a.len().min(b.len()) > PrimeFFT::<M, T>::BORDER_LEN
                && a.len() + b.len() - 1 > fft.max_len()
            {
                *self = Self::Convolution(Convolution::new());
            }
        }
        match self {
            Self::Prime(fft) => fft.multiply_fix_len(a, b, res),
            Self::Convolution(fft) => fft.multiply_fix_len(a, b, res),
        }
    }

    /// Raises a polynomial to a nonnegative integer power, switching to CRT
    /// if intermediate products outgrow the direct transform.
    pub fn power(&mut self, a: &[M], exp: usize) -> Vec<M> {
        if exp == 0 {
            return vec![M::one()];
        }
        let half = self.power(a, exp / 2);
        let square = self.multiply(&half, &half);
        if exp % 2 == 0 {
            square
        } else {
            self.multiply(&square, a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FFT;
    use crate::numbers::mod_int::{BaseModInt, ModInt, ModInt64, ModInt7, ModIntF};
    use crate::numbers::num_traits::algebra::{One, Zero};

    fn check_product<M: BaseModInt<u32>>(fft: &mut FFT<M>, n: usize, m: usize) {
        let a: Vec<_> = (0..n).map(|i| -M::from(i * 117 + 1)).collect();
        let b: Vec<_> = (0..m).map(|i| M::from(i * 79 + 5)).collect();
        let mut expected = vec![M::zero(); n + m - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                expected[i + j] += x * y;
            }
        }
        assert!(fft.multiply(&a, &b) == expected);
        for len in [0, 61, n, expected.len() + 17] {
            let mut want = expected.clone();
            want.resize(len, M::zero());
            let mut result = vec![M::one(); len];
            fft.multiply_fix_len(&a, &b, &mut result);
            assert!(result == want);
        }
        let mut result = vec![];
        fft.multiply_res(&a, &b, &mut result);
        assert!(result == expected);
        fft.multiply_res(&[], &b, &mut result);
        assert!(result.iter().all(|&x| x == M::zero()));
    }

    #[test]
    fn dispatches_by_modulus_and_transform_capacity() {
        crate::value!(SmallFftPrime: u32 = 257);
        type M = ModInt<SmallFftPrime>;
        let mut fft = FFT::<M>::new();
        assert!(matches!(fft, FFT::Prime(_)));
        check_product(&mut fft, 65, 65);
        assert!(matches!(fft, FFT::Prime(_)));
        check_product(&mut fft, 200, 200);
        assert!(matches!(fft, FFT::Convolution(_)));
        check_product(&mut fft, 65, 65);
    }

    #[test]
    fn recognizes_other_fft_primes_and_composite_moduli() {
        crate::value!(OtherPrime: u32 = 167772161);
        let mut fft = FFT::<ModInt<OtherPrime>>::new();
        assert!(matches!(fft, FFT::Prime(_)));
        check_product(&mut fft, 129, 257);
        let mut fft = FFT::<ModInt7>::new();
        assert!(matches!(fft, FFT::Convolution(_)));
        check_product(&mut fft, 129, 257);
        // 2^7 divides p-1, but p is composite and unsuitable for a direct NTT.
        crate::value!(Composite: u32 = 385);
        let mut fft = FFT::<ModInt<Composite>>::new();
        assert!(matches!(fft, FFT::Convolution(_)));
        check_product(&mut fft, 129, 257);
    }

    #[test]
    fn power_can_outgrow_the_direct_transform() {
        crate::value!(SmallFftPrime: u32 = 257);
        type M = ModInt<SmallFftPrime>;
        let mut fft = FFT::<M>::new();
        let mut a = vec![M::zero(); 65];
        a[0] = M::one();
        a[64] = M::one();
        let mut expected = vec![M::zero(); 321];
        for (i, c) in [1usize, 5, 10, 10, 5, 1].into_iter().enumerate() {
            expected[i * 64] = M::from(c);
        }
        assert_eq!(fft.power(&a, 5), expected);
        assert!(matches!(fft, FFT::Convolution(_)));
        assert_eq!(fft.power(&[], 0), vec![M::one()]);
        assert!(fft.power(&[], 5).is_empty());
        let mut fft = FFT::<ModIntF>::default();
        check_product(&mut fft, 61, 129);
    }

    #[test]
    #[should_panic(expected = "32-bit")]
    fn rejects_wide_moduli() {
        crate::value!(Wide: u64 = (1u64 << 61) - 1);
        FFT::<ModInt64<Wide>, u64>::new();
    }
}
