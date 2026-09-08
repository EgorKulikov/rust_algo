use crate::numbers::mod_int::prime_fft::PrimeFFT;
use crate::numbers::mod_int::{BaseModInt, ModInt, ModIntF};
use crate::numbers::num_traits::invertible::Invertible;
use std::marker::PhantomData;

crate::value!(Modulus2: u32 = 985661441);
crate::value!(Modulus3: u32 = 975175681);
type Mod2 = ModInt<Modulus2>;
type Mod3 = ModInt<Modulus3>;
const P1: u64 = 998244353;
const P12: u64 = P1 * 985661441;
const P123: i128 = P12 as i128 * 975175681;

/// Modular convolution using three FFT primes and exact CRT reconstruction.
///
/// The target modulus must fit in `u32`; it need not be prime. Coefficients
/// are stored in ascending order. FFT buffers are reused between operations.
/// Required transform lengths must not exceed 2^21. Small operands use direct
/// multiplication and do not have this transform limit.
///
/// The library's `ModInt` supports moduli up to `i32::MAX`; use `ModInt64`
/// storage for larger 32-bit moduli so coefficient additions do not overflow.
pub struct Convolution<M: BaseModInt<T>, T = u32> {
    first: PrimeFFT<ModIntF>,
    second: PrimeFFT<Mod2>,
    third: PrimeFFT<Mod3>,
    inverse_p1: Mod2,
    inverse_p12: Mod3,
    phantom: PhantomData<(T, M)>,
}

impl<T: Into<u64>, M: BaseModInt<T>> Default for Convolution<M, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Into<u64>, M: BaseModInt<T>> Convolution<M, T> {
    /// Creates a reusable context. Panics if the modulus does not fit in `u32`.
    pub fn new() -> Self {
        assert!(
            (1..=u32::MAX as u64).contains(&modulus::<T, M>()),
            "convolution requires a 32-bit modulus"
        );
        Self {
            first: PrimeFFT::new(),
            second: PrimeFFT::new(),
            third: PrimeFFT::new(),
            inverse_p1: Mod2::from(P1 as usize).inv().unwrap(),
            inverse_p12: Mod3::new_wide(P12 as i64).inv().unwrap(),
            phantom: PhantomData,
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

    /// Writes the full product, growing `res` if necessary. An existing longer
    /// buffer keeps its length and its unused coefficients are set to zero.
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

    /// Writes the first `res.len()` coefficients of the linear product,
    /// truncating or padding with zeros as needed.
    pub fn multiply_fix_len(&mut self, a: &[M], b: &[M], res: &mut [M]) {
        res.fill(M::zero());
        if res.is_empty() || a.is_empty() || b.is_empty() {
            return;
        }
        let a = &a[..a.len().min(res.len())];
        let b = &b[..b.len().min(res.len())];
        if a.len().min(b.len()) <= PrimeFFT::<ModIntF>::BORDER_LEN {
            for (i, &x) in a.iter().enumerate() {
                for (j, &y) in b.iter().take(res.len() - i).enumerate() {
                    res[i + j] += x * y;
                }
            }
            return;
        }
        let a: Vec<_> = a.iter().map(|&x| Into::<usize>::into(x) as i64).collect();
        let b: Vec<_> = b.iter().map(|&x| Into::<usize>::into(x) as i64).collect();
        let modulo = modulus::<T, M>() as i128;
        for (out, coefficient) in res.iter_mut().zip(self.multiply_integer(&a, &b)) {
            *out = M::from((coefficient % modulo) as usize);
        }
    }

    /// Raises a polynomial to a nonnegative integer power.
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

    // Inputs are signed i32 coefficients, or nonnegative u32 residues. Their
    // exact products fit in the centered CRT interval at supported FFT lengths.
    fn multiply_integer(&mut self, a: &[i64], b: &[i64]) -> Vec<i128> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        if a.len().min(b.len()) <= PrimeFFT::<ModIntF>::BORDER_LEN {
            let mut result = vec![0; a.len() + b.len() - 1];
            for (i, &x) in a.iter().enumerate() {
                for (j, &y) in b.iter().enumerate() {
                    result[i + j] += x as i128 * y as i128;
                }
            }
            return result;
        }
        assert!(
            a.len() + b.len() - 1 <= 1 << 21,
            "convolution transform exceeds 2^21"
        );
        let c1 = multiply_as(&mut self.first, a, b);
        let c2 = multiply_as(&mut self.second, a, b);
        let c3 = multiply_as(&mut self.third, a, b);
        (0..c1.len())
            .map(|i| {
                let digit2 = ((c2[i] - Mod2::from(c1[i].val())) * self.inverse_p1).val();
                let x12 = c1[i].val() as u64 + P1 * digit2 as u64;
                let digit3 = ((c3[i] - Mod3::new_wide(x12 as i64)) * self.inverse_p12).val();
                let x = x12 as i128 + P12 as i128 * digit3 as i128;
                if x >= P123 / 2 {
                    x - P123
                } else {
                    x
                }
            })
            .collect()
    }
}

pub(crate) fn modulus<T: Into<u64>, M: BaseModInt<T>>() -> u64 {
    M::module().into()
}

fn multiply_as<M: BaseModInt<u32> + From<i64>>(
    fft: &mut PrimeFFT<M>,
    a: &[i64],
    b: &[i64],
) -> Vec<M> {
    let a: Vec<_> = a.iter().map(|&x| M::from(x)).collect();
    let b: Vec<_> = b.iter().map(|&x| M::from(x)).collect();
    fft.multiply(&a, &b)
}

/// Exact signed-integer convolution, retaining the original convenience API.
pub fn convolution(a: &[i32], b: &[i32]) -> Vec<i128> {
    let a: Vec<_> = a.iter().map(|&x| x as i64).collect();
    let b: Vec<_> = b.iter().map(|&x| x as i64).collect();
    Convolution::<ModIntF>::new().multiply_integer(&a, &b)
}

#[cfg(test)]
mod tests {
    use super::{convolution, Convolution};
    use crate::numbers::mod_int::{BaseModInt, ModInt, ModInt7, ModIntF};

    fn naive<M: BaseModInt<u32>>(a: &[M], b: &[M]) -> Vec<M> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut result = vec![M::zero(); a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                result[i + j] += x * y;
            }
        }
        result
    }

    fn check_context<M: BaseModInt<u32>>() {
        let mut fft = Convolution::<M>::new();
        for (n, m) in [
            (0usize, 65usize),
            (1, 129),
            (60, 73),
            (61, 65),
            (129, 257),
            (65, 61),
        ] {
            let a: Vec<_> = (0..n).map(|i| -M::from(i * 123457 + 1)).collect();
            let b: Vec<_> = (0..m).map(|i| -M::from(i * 76543 + 3)).collect();
            let expected = naive(&a, &b);
            assert!(fft.multiply(&a, &b) == expected);
            for len in [0, 1, 61, 80, expected.len(), expected.len() + 31] {
                let mut want = expected.clone();
                want.resize(len, M::zero());
                let mut result = vec![M::one(); len];
                fft.multiply_fix_len(&a, &b, &mut result);
                assert!(result == want, "n={n}, m={m}, output={len}");
            }
            let mut result = vec![M::one(); expected.len() + 13];
            let mut want = expected.clone();
            want.resize(result.len(), M::zero());
            fft.multiply_res(&a, &b, &mut result);
            assert!(result == want);
            let mut result = vec![];
            fft.multiply_res(&a, &b, &mut result);
            assert!(result == expected);
        }
        let polynomial: Vec<_> = (0..65usize).map(|i| M::from(i + 1)).collect();
        let expected = naive(&naive(&polynomial, &polynomial), &polynomial);
        assert!(fft.power(&polynomial, 3) == expected);
        assert!(fft.power(&[], 0) == vec![M::one()]);
        assert!(fft.power(&[], 3).is_empty());
    }

    #[test]
    fn modular_context_methods() {
        check_context::<ModInt7>();
        check_context::<ModIntF>();
        crate::value!(Composite: u32 = 12);
        check_context::<ModInt<Composite>>();
        crate::value!(Two: u32 = 2);
        check_context::<ModInt<Two>>();
        crate::value!(One: u32 = 1);
        check_context::<ModInt<One>>();
    }

    #[test]
    fn full_u32_residues() {
        use crate::numbers::mod_int::ModInt64;
        use crate::numbers::num_traits::algebra::One;
        crate::value!(Large: u64 = 4294967291);
        // Use wider storage to avoid ModInt's u32 addition limit, while the
        // modulus and all residues still fit in 32 bits.
        type M = ModInt64<Large>;
        let a = vec![-M::one(); 73];
        let b = vec![-M::from(2usize); 85];
        let expected: Vec<_> = (0..157usize)
            .map(|i| M::from(2 * (i + 1).min(73).min(85).min(157 - i)))
            .collect();
        let mut fft = Convolution::<M, u64>::new();
        assert_eq!(fft.multiply(&a, &b), expected);
    }

    #[test]
    fn signed_integer_convolution() {
        let a: Vec<_> = (0..73)
            .map(|i| [i32::MIN, i32::MAX, -7, 0][i % 4])
            .collect();
        let b: Vec<_> = (0..89).map(|i| [i32::MAX, -1, i32::MIN][i % 3]).collect();
        let mut expected = vec![0i128; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                expected[i + j] += x as i128 * y as i128;
            }
        }
        assert_eq!(convolution(&a, &b), expected);
        assert!(convolution(&[], &b).is_empty());
    }
}
