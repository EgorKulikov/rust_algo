use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::numbers::mod_int::BaseModInt;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::MulDivRem;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::numbers::number_ext::Power;

pub struct PrimeFFT<M: BaseModInt> {
    root: M,
    reverse_root: M,
    root_power: M::T,
    aa: Vec<M>,
    bb: Vec<M>,
}

impl<M: BaseModInt> Default for PrimeFFT<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: BaseModInt> PrimeFFT<M> {
    pub fn new() -> Self {
        let mut exp = M::T::zero();
        let mut root_power = M::T::one();
        while (M::module() - M::T::one()) % (root_power + root_power) == M::T::zero() {
            exp = root_power;
            root_power += root_power;
        }
        let mut i = M::one() + M::one();
        loop {
            if i.power(exp) != M::one() && i.power(root_power) == M::one() {
                break Self {
                    root: i,
                    reverse_root: i.inv().unwrap(),
                    root_power,
                    aa: Vec::new(),
                    bb: Vec::new(),
                };
            }
            i += M::one();
        }
    }

    pub fn multiply_res(&mut self, a: &[M], b: &[M], res: &mut Vec<M>) {
        if a.is_empty() || b.is_empty() {
            res.legacy_fill(M::zero());
            return;
        }
        let res_len = a.len() + b.len() - 1;
        if res.len() < res_len {
            res.reserve(res_len - res.len());
            for _ in res.len()..res_len {
                res.push(M::zero());
            }
        }
        self.multiply_fix_len(a, b, res);
    }

    pub fn multiply_fix_len(&mut self, a: &[M], b: &[M], res: &mut [M]) {
        let res_len = res.len();
        if res_len <= Self::BORDER_LEN {
            res.legacy_fill(M::zero());
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
        let mut size = 1;
        let mut size_t = M::T::one();
        while size < res_len {
            size *= 2;
            size_t += size_t;
        }
        if self.root_power < size_t {
            panic!("unsuitable modulo");
        }
        if self.aa.len() < size {
            let was_len = self.aa.len();
            self.aa[..was_len.min(a.len())].copy_from_slice(&a[..was_len.min(a.len())]);
            self.aa[was_len.min(a.len())..].legacy_fill(M::zero());
            self.aa.extend_from_slice(&a[was_len.min(a.len())..]);
            self.aa.reserve(size - self.aa.len());
            for _ in self.aa.len()..size {
                self.aa.push(M::zero());
            }
        } else {
            self.aa[..a.len()].copy_from_slice(a);
            self.aa[a.len()..size].legacy_fill(M::zero());
        }
        Self::fft(
            &mut self.aa[..size],
            false,
            self.root,
            self.root_power,
            size_t,
        );
        if a == b {
            for i in self.aa[..size].iter_mut() {
                *i *= *i;
            }
        } else {
            if self.bb.len() < size {
                let was_len = self.bb.len();
                self.bb[..was_len.min(b.len())].copy_from_slice(&b[..was_len.min(b.len())]);
                self.bb[was_len.min(b.len())..].legacy_fill(M::zero());
                self.bb.extend_from_slice(&b[was_len.min(b.len())..]);
                self.bb.reserve(size - self.bb.len());
                for _ in self.bb.len()..size {
                    self.bb.push(M::zero());
                }
            } else {
                self.bb[..b.len()].copy_from_slice(b);
                self.bb[b.len()..size].legacy_fill(M::zero());
            }
            Self::fft(
                &mut self.bb[..size],
                false,
                self.root,
                self.root_power,
                size_t,
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
            size_t,
        );
        res.copy_from_slice(&self.aa[..res_len]);
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

    pub fn power<T: ZeroOne + PartialEq + MulDivRem + AddSub + Copy>(
        &mut self,
        a: &[M],
        exp: T,
    ) -> Vec<M> {
        let mut res = Vec::new();
        let mut temp = Vec::new();
        self.power_impl(a, exp, &mut res, &mut temp);
        res
    }

    fn power_impl<T: ZeroOne + PartialEq + MulDivRem + AddSub + Copy>(
        &mut self,
        a: &[M],
        exp: T,
        res: &mut Vec<M>,
        temp: &mut Vec<M>,
    ) {
        if exp == T::zero() {
            res.push(M::one());
            return;
        }
        let two = T::one() + T::one();
        if exp % two == T::zero() {
            self.power_impl(a, exp / two, temp, res);
            self.multiply_res(temp, temp, res);
        } else {
            self.power_impl(a, exp - T::one(), temp, res);
            self.multiply_res(temp, a, res);
        }
    }

    const BORDER_LEN: usize = 100;

    fn fft(a: &mut [M], invert: bool, root: M, root_power: M::T, size_t: M::T) {
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
        let mut len_t = M::T::one() + M::T::one();
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
