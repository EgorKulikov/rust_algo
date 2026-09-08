use crate::numbers::mod_int::BaseModInt;
use crate::numbers::number_ext::Power;
use std::marker::PhantomData;

/// Reusable radix-two FFT for a prime modulus that fits in `u32`.
/// For automatic fallback with other moduli or longer transforms, use
/// [`FFT`](crate::numbers::mod_int::fft::FFT).
pub struct PrimeFFT<M: BaseModInt<T>, T = u32> {
    root: M,
    reverse_root: M,
    root_power: usize,
    aa: Vec<M>,
    bb: Vec<M>,
    phantom: PhantomData<T>,
}

impl<T: Into<u64>, M: BaseModInt<T>> Default for PrimeFFT<M, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Into<u64>, M: BaseModInt<T>> PrimeFFT<M, T> {
    pub fn new() -> Self {
        let module = M::module().into();
        assert!(
            (2..=u32::MAX as u64).contains(&module),
            "PrimeFFT requires a 32-bit prime modulus"
        );
        let mut root_power = 1usize;
        let mut rem = (module - 1) as usize;
        while rem % 2 == 0 {
            rem /= 2;
            root_power *= 2;
        }
        let root = if root_power == 1 {
            M::one()
        } else {
            let mut i = M::from(2usize);
            loop {
                let j = i.power(rem);
                if j.power(root_power / 2) != M::one() && j.power(root_power) == M::one() {
                    break j;
                }
                i += M::one();
            }
        };
        Self {
            root,
            reverse_root: root.inv().unwrap(),
            root_power,
            aa: Vec::new(),
            bb: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub(crate) fn max_len(&self) -> usize {
        self.root_power
    }

    pub fn multiply_res(&mut self, a: &[M], b: &[M], res: &mut Vec<M>) {
        if a.is_empty() || b.is_empty() {
            res.fill(M::zero());
            return;
        }
        let res_len = a.len() + b.len() - 1;
        if res.len() < res_len {
            res.resize(res_len, M::zero());
        }
        self.multiply_fix_len(a, b, res);
    }

    /// Writes a prefix of the linear product, padding with zeros if necessary.
    pub fn multiply_fix_len(&mut self, a: &[M], b: &[M], res: &mut [M]) {
        let res_len = res.len();
        res.fill(M::zero());
        if res_len == 0 || a.is_empty() || b.is_empty() {
            return;
        }
        let a = &a[..a.len().min(res_len)];
        let b = &b[..b.len().min(res_len)];
        if a.len().min(b.len()) <= Self::BORDER_LEN {
            for (i, f) in a.iter().enumerate() {
                for (j, s) in b.iter().enumerate() {
                    if i + j < res.len() {
                        res[i + j] += (*f) * (*s);
                    } else {
                        break;
                    }
                }
            }
            return;
        }
        // Truncating the output still needs a full linear convolution of the
        // relevant input prefixes, otherwise high coefficients wrap around.
        let product_len = a.len() + b.len() - 1;
        let size = product_len.next_power_of_two();
        if self.root_power < size {
            panic!("unsuitable modulo");
        }
        copy(&mut self.aa, a, size);
        Self::fft(
            &mut self.aa[..size],
            false,
            self.root,
            self.root_power,
            size,
        );
        if a == b {
            for i in self.aa[..size].iter_mut() {
                *i *= *i;
            }
        } else {
            copy(&mut self.bb, b, size);
            Self::fft(
                &mut self.bb[..size],
                false,
                self.root,
                self.root_power,
                size,
            );
            for (i, j) in self.aa[..size].iter_mut().zip(self.bb[..size].iter()) {
                *i *= *j;
            }
        }
        Self::fft(
            &mut self.aa[..size],
            true,
            self.reverse_root,
            self.root_power,
            size,
        );
        let len = res_len.min(product_len);
        res[..len].copy_from_slice(&self.aa[..len]);
    }

    pub fn multiply(&mut self, a: &[M], b: &[M]) -> Vec<M> {
        if a.is_empty() || b.is_empty() {
            Vec::new()
        } else {
            let mut res = vec![M::zero(); a.len() + b.len() - 1];
            self.multiply_res(a, b, &mut res);
            res
        }
    }

    pub fn power(&mut self, a: &[M], exp: usize) -> Vec<M> {
        let mut res = Vec::new();
        let mut temp = Vec::new();
        self.power_impl(a, exp, &mut res, &mut temp);
        res
    }

    fn power_impl(&mut self, a: &[M], exp: usize, res: &mut Vec<M>, temp: &mut Vec<M>) {
        if exp == 0 {
            res.push(M::one());
            return;
        }
        if exp % 2 == 0 {
            self.power_impl(a, exp / 2, temp, res);
            self.multiply_res(temp, temp, res);
        } else {
            self.power_impl(a, exp - 1, temp, res);
            self.multiply_res(temp, a, res);
        }
    }

    pub(crate) const BORDER_LEN: usize = 60;

    fn fft(a: &mut [M], invert: bool, root: M, root_power: usize, size_t: usize) {
        let mut j = 0usize;
        for i in 1..a.len() {
            let mut bit = a.len() >> 1;
            while j >= bit {
                j -= bit;
                bit >>= 1;
            }
            j += bit;
            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;
        let mut len_t = 2;
        while len <= a.len() {
            let mut w_len = root;
            let mut i = len_t;
            while i < root_power {
                w_len *= w_len;
                i += i;
            }
            let half = len >> 1;
            for i in (0..a.len()).step_by(len) {
                let mut w = M::one();
                for j in 0..half {
                    let u = a[i + j];
                    let v = a[i + j + half] * w;
                    a[i + j] = u + v;
                    a[i + j + half] = u - v;
                    w *= w_len;
                }
            }
            len <<= 1;
            len_t += len_t;
        }
        if invert {
            let inv_size = M::from(size_t).inv().unwrap();
            for i in a {
                *i *= inv_size;
            }
        }
    }
}

fn copy<T, M: BaseModInt<T>>(aa: &mut Vec<M>, a: &[M], size: usize) {
    if aa.len() < size {
        let was_len = aa.len();
        aa[..was_len.min(a.len())].copy_from_slice(&a[..was_len.min(a.len())]);
        aa[was_len.min(a.len())..].fill(M::zero());
        aa.reserve(size - aa.len());
        aa.extend_from_slice(&a[was_len.min(a.len())..]);
        aa.resize(size, M::zero());
    } else {
        aa[..a.len()].copy_from_slice(a);
        aa[a.len()..size].fill(M::zero());
    }
}

#[cfg(test)]
mod tests {
    use super::PrimeFFT;
    use crate::numbers::mod_int::ModIntF as M;
    use crate::numbers::num_traits::algebra::{One, Zero};

    #[test]
    fn fixed_length_is_a_linear_product_prefix() {
        let mut fft = PrimeFFT::<M>::new();
        let a: Vec<_> = (0..100usize).map(|i| M::from(i + 1)).collect();
        let b: Vec<_> = (0..85usize).map(|i| M::from(i * 13 + 5)).collect();
        let mut expected = vec![M::zero(); a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                expected[i + j] += x * y;
            }
        }
        for len in [0, 61, 75, 100, 184, 300] {
            let mut want = expected.clone();
            want.resize(len, M::zero());
            let mut result = vec![M::one(); len];
            fft.multiply_fix_len(&a, &b, &mut result);
            assert_eq!(result, want);
        }
    }
}
