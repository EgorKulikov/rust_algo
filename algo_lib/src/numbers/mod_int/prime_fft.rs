use crate::numbers::mod_int::BaseModInt;
use crate::numbers::number_ext::Power;

pub struct PrimeFFT<M: BaseModInt<u32>> {
    root: M,
    reverse_root: M,
    root_power: u32,
    aa: Vec<M>,
    bb: Vec<M>,
}

impl<M: BaseModInt<u32>> Default for PrimeFFT<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: BaseModInt<u32>> PrimeFFT<M> {
    pub fn new() -> Self {
        let mut exp = 0;
        let mut root_power = 1;
        while (M::module() - 1) % (2 * root_power) == 0 {
            exp = root_power;
            root_power += root_power;
        }
        let mut i = M::from(2u32);
        let rem = (M::module() - 1) / root_power;
        loop {
            let j = i.power(rem);
            if j.power(exp) != M::one() && j.power(root_power) == M::one() {
                break Self {
                    root: j,
                    reverse_root: j.inv().unwrap(),
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
            res.fill(M::zero());
            return;
        }
        let res_len = a.len() + b.len() - 1;
        if res.len() < res_len {
            res.resize(res_len, M::zero());
        }
        self.multiply_fix_len(a, b, res);
    }

    pub fn multiply_fix_len(&mut self, a: &[M], b: &[M], res: &mut [M]) {
        let res_len = res.len();
        if a.len().min(b.len()) <= Self::BORDER_LEN {
            res.fill(M::zero());
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
        while size < res_len {
            size *= 2;
        }
        if self.root_power < size as u32 {
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

    const BORDER_LEN: usize = 60;

    fn fft(a: &mut [M], invert: bool, root: M, root_power: u32, size_t: usize) {
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
            let inv_size = M::from(size_t as u32).inv().unwrap();
            for i in a {
                *i *= inv_size;
            }
        }
    }
}

fn copy<M: BaseModInt<u32>>(aa: &mut Vec<M>, a: &[M], size: usize) {
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
