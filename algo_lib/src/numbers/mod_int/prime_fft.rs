use crate::numbers::mod_int::BaseModInt;
use crate::numbers::number_ext::Power;
use std::marker::PhantomData;

/// Montgomery arithmetic for a modulus below 2^30 with R = 2^32. Values are
/// kept "lazy" in `[0, 2p)`; `reduce` maps any `t < 4p^2` into that range.
#[derive(Clone, Copy)]
struct Montgomery {
    p: u32,
    p2: u32,
    /// `-p^{-1} mod 2^32`
    n_inv: u32,
    /// `R^2 mod p`
    r2: u32,
}

impl Montgomery {
    fn new(p: u32) -> Self {
        assert!(
            p % 2 == 1 && p < (1 << 30),
            "Montgomery NTT needs an odd modulus below 2^30"
        );
        let mut inv = p;
        for _ in 0..5 {
            inv = inv.wrapping_mul(2u32.wrapping_sub(p.wrapping_mul(inv)));
        }
        Self {
            p,
            p2: 2 * p,
            n_inv: inv.wrapping_neg(),
            r2: ((1u128 << 64) % p as u128) as u32,
        }
    }

    #[inline(always)]
    fn reduce(self, t: u64) -> u32 {
        let m = (t as u32).wrapping_mul(self.n_inv);
        ((t + m as u64 * self.p as u64) >> 32) as u32
    }

    #[inline(always)]
    fn mul(self, a: u32, b: u32) -> u32 {
        self.reduce(a as u64 * b as u64)
    }

    #[inline(always)]
    fn strict(self, a: u32) -> u32 {
        if a >= self.p {
            a - self.p
        } else {
            a
        }
    }

    #[inline(always)]
    fn add(self, a: u32, b: u32) -> u32 {
        let s = a + b;
        if s >= self.p2 {
            s - self.p2
        } else {
            s
        }
    }

    fn to_mont(self, a: u32) -> u32 {
        self.strict(self.mul(a, self.r2))
    }
}

/// Reusable number-theoretic transform for a prime modulus below 2^30 with
/// enough powers of two dividing `p - 1`. Forward transforms leave the
/// spectrum in bit-reversed order; the inverse transform undoes that, so no
/// permutation pass is needed. For other moduli or longer transforms, use
/// [`FFT`](crate::numbers::mod_int::fft::FFT).
pub struct PrimeFFT<M: BaseModInt<T>, T = u32> {
    mont: Montgomery,
    avx2: bool,
    rank: usize,
    rate2: [u32; 32],
    irate2: [u32; 32],
    /// `tw[s]` is the twiddle of block `s` at every level (strict Montgomery
    /// form); the table for a smaller transform is a prefix of a larger one.
    tw: Vec<u32>,
    itw: Vec<u32>,
    aa: Vec<u32>,
    bb: Vec<u32>,
    phantom: PhantomData<(T, M)>,
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
        let mont = Montgomery::new(module as u32);
        let rank = (module - 1).trailing_zeros() as usize;
        let rem = ((module - 1) >> rank) as usize;
        // Primitive 2^rank-th root of unity.
        let mut root = M::one();
        if rank > 0 {
            let mut g = M::from(2usize);
            loop {
                let j = g.power(rem);
                if j.power(1 << (rank - 1)) != M::one() {
                    root = j;
                    break;
                }
                g += M::one();
            }
        }
        // roots[i] is a primitive 2^i-th root; rate2[i] = roots[i+2] / prod(roots[k+2], k<i).
        let mut roots = vec![M::one(); rank + 1];
        let mut iroots = vec![M::one(); rank + 1];
        if rank > 0 {
            roots[rank] = root;
            iroots[rank] = root.inv().unwrap();
            for i in (0..rank).rev() {
                roots[i] = roots[i + 1] * roots[i + 1];
                iroots[i] = iroots[i + 1] * iroots[i + 1];
            }
        }
        let mut rate2 = [0u32; 32];
        let mut irate2 = [0u32; 32];
        let (mut prod, mut iprod) = (M::one(), M::one());
        for i in 0..rank.saturating_sub(1) {
            rate2[i] = mont.to_mont(Into::<u64>::into((roots[i + 2] * prod).value()) as u32);
            irate2[i] = mont.to_mont(Into::<u64>::into((iroots[i + 2] * iprod).value()) as u32);
            prod *= iroots[i + 2];
            iprod *= roots[i + 2];
        }
        Self {
            mont,
            avx2: avx2_available(),
            rank,
            rate2,
            irate2,
            tw: Vec::new(),
            itw: Vec::new(),
            aa: Vec::new(),
            bb: Vec::new(),
            phantom: PhantomData,
        }
    }

    /// Forces the portable scalar butterflies (used by tests).
    #[cfg(test)]
    pub(crate) fn scalar_only(mut self) -> Self {
        self.avx2 = false;
        self
    }

    pub(crate) fn max_len(&self) -> usize {
        1 << self.rank
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
        if self.max_len() < size {
            panic!("unsuitable modulo");
        }
        let mont = self.mont;
        self.ensure_twiddles(size);
        load(&mut self.aa, a, size, mont);
        let avx2 = self.avx2;
        Self::dif(&mut self.aa[..size], mont, &self.tw, avx2);
        if a == b {
            for x in self.aa[..size].iter_mut() {
                *x = mont.mul(*x, *x);
            }
        } else {
            load(&mut self.bb, b, size, mont);
            Self::dif(&mut self.bb[..size], mont, &self.tw, avx2);
            if avx2 && size >= 8 {
                // SAFETY: `avx2` is only set when the CPU reports AVX2.
                unsafe { simd::pointwise(&mut self.aa[..size], &self.bb[..size], mont) };
            } else {
                for (x, y) in self.aa[..size].iter_mut().zip(self.bb[..size].iter()) {
                    *x = mont.mul(*x, *y);
                }
            }
        }
        Self::dit(&mut self.aa[..size], mont, &self.itw, avx2);
        // One Montgomery multiply by the plain 1/size both scales and converts
        // out of Montgomery form.
        let inv_size = Into::<u64>::into(M::from(size).inv().unwrap().value()) as u32;
        let len = res_len.min(product_len);
        for (out, &x) in res[..len].iter_mut().zip(self.aa.iter()) {
            *out = M::from(mont.strict(mont.mul(x, inv_size)) as usize);
        }
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

    /// Grows the twiddle tables to cover transforms of length `size`.
    fn ensure_twiddles(&mut self, size: usize) {
        let need = size / 2;
        if self.tw.len() >= need.max(1) {
            return;
        }
        let mont = self.mont;
        if self.tw.is_empty() {
            let one = mont.to_mont(1);
            self.tw.push(one);
            self.itw.push(one);
        }
        while self.tw.len() < need {
            let t = self.tw.len() - 1;
            let k = (!t).trailing_zeros() as usize;
            self.tw
                .push(mont.strict(mont.mul(self.tw[t], self.rate2[k])));
            self.itw
                .push(mont.strict(mont.mul(self.itw[t], self.irate2[k])));
        }
    }

    /// Decimation in frequency: natural order in, bit-reversed order out.
    /// Values stay in `[0, 2p)`.
    fn dif(a: &mut [u32], mont: Montgomery, tw: &[u32], avx2: bool) {
        let n = a.len();
        let h = n.trailing_zeros() as usize;
        let leaf = if avx2 && h >= 3 { 3 } else { 0 };
        for len in 0..h - leaf {
            let half = 1 << (h - len - 1);
            for (s, block) in a.chunks_exact_mut(2 * half).enumerate() {
                let rot = tw[s];
                let (lo, hi) = block.split_at_mut(half);
                if avx2 && half >= 8 {
                    // SAFETY: `avx2` is only set when the CPU reports AVX2.
                    unsafe { simd::dif_block(lo, hi, rot, mont) };
                } else if s == 0 {
                    for (l, r) in lo.iter_mut().zip(hi.iter_mut()) {
                        let (x, y) = (*l, *r);
                        *l = mont.add(x, y);
                        *r = mont.add(x + mont.p2 - y, 0);
                    }
                } else {
                    for (l, r) in lo.iter_mut().zip(hi.iter_mut()) {
                        let (x, y) = (*l, mont.mul(*r, rot));
                        *l = mont.add(x, y);
                        *r = mont.add(x + mont.p2 - y, 0);
                    }
                }
            }
        }
        if leaf == 3 {
            // SAFETY: `avx2` is only set when the CPU reports AVX2.
            unsafe { simd::dif_leaf(a, tw, mont) };
        }
    }

    /// Decimation in time: bit-reversed order in, natural order out (unscaled).
    fn dit(a: &mut [u32], mont: Montgomery, itw: &[u32], avx2: bool) {
        let n = a.len();
        let h = n.trailing_zeros() as usize;
        let leaf = if avx2 && h >= 3 { 3 } else { 0 };
        if leaf == 3 {
            // SAFETY: `avx2` is only set when the CPU reports AVX2.
            unsafe { simd::dit_leaf(a, itw, mont) };
        }
        for len in (1..=h - leaf).rev() {
            let half = 1 << (h - len);
            for (s, block) in a.chunks_exact_mut(2 * half).enumerate() {
                let irot = itw[s];
                let (lo, hi) = block.split_at_mut(half);
                if avx2 && half >= 8 {
                    // SAFETY: `avx2` is only set when the CPU reports AVX2.
                    unsafe { simd::dit_block(lo, hi, irot, mont) };
                } else if s == 0 {
                    for (l, r) in lo.iter_mut().zip(hi.iter_mut()) {
                        let (x, y) = (*l, *r);
                        *l = mont.add(x, y);
                        *r = mont.add(x + mont.p2 - y, 0);
                    }
                } else {
                    for (l, r) in lo.iter_mut().zip(hi.iter_mut()) {
                        let (x, y) = (*l, *r);
                        *l = mont.add(x, y);
                        *r = mont.mul(x + mont.p2 - y, irot);
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
fn avx2_available() -> bool {
    is_x86_feature_detected!("avx2")
}

#[cfg(not(target_arch = "x86_64"))]
fn avx2_available() -> bool {
    false
}

/// AVX2 versions of the block butterflies: eight lanes of lazy Montgomery
/// arithmetic. Only called with slices whose length is a multiple of 8.
#[cfg(target_arch = "x86_64")]
mod simd {
    use super::Montgomery;
    use std::arch::x86_64::*;

    /// Lazy Montgomery product of eight u32 lanes (inputs `< 2p`, twiddle
    /// lanes `< p`), result `< 2p`.
    #[inline(always)]
    unsafe fn mul(a: __m256i, b: __m256i, p: __m256i, n_inv: __m256i) -> __m256i {
        let even = _mm256_mul_epu32(a, b);
        let odd = _mm256_mul_epu32(_mm256_srli_epi64::<32>(a), _mm256_srli_epi64::<32>(b));
        let m_even = _mm256_mul_epu32(even, n_inv);
        let m_odd = _mm256_mul_epu32(odd, n_inv);
        let r_even = _mm256_add_epi64(even, _mm256_mul_epu32(m_even, p));
        let r_odd = _mm256_add_epi64(odd, _mm256_mul_epu32(m_odd, p));
        _mm256_blend_epi32::<0b1010_1010>(_mm256_srli_epi64::<32>(r_even), r_odd)
    }

    /// `x + y` reduced into `[0, 2p)` for `x, y < 2p`.
    #[inline(always)]
    unsafe fn add(x: __m256i, y: __m256i, p2: __m256i) -> __m256i {
        let s = _mm256_add_epi32(x, y);
        _mm256_min_epu32(s, _mm256_sub_epi32(s, p2))
    }

    /// `x - y` reduced into `[0, 2p)` for `x, y < 2p`.
    #[inline(always)]
    unsafe fn sub(x: __m256i, y: __m256i, p2: __m256i) -> __m256i {
        let d = _mm256_sub_epi32(x, y);
        _mm256_min_epu32(d, _mm256_add_epi32(d, p2))
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn dif_block(lo: &mut [u32], hi: &mut [u32], rot: u32, mont: Montgomery) {
        let p = _mm256_set1_epi32(mont.p as i32);
        let p2 = _mm256_set1_epi32(mont.p2 as i32);
        let n_inv = _mm256_set1_epi32(mont.n_inv as i32);
        let w = _mm256_set1_epi32(rot as i32);
        for (l, r) in lo.chunks_exact_mut(8).zip(hi.chunks_exact_mut(8)) {
            let x = _mm256_loadu_si256(l.as_ptr() as *const __m256i);
            let y = mul(
                _mm256_loadu_si256(r.as_ptr() as *const __m256i),
                w,
                p,
                n_inv,
            );
            _mm256_storeu_si256(l.as_mut_ptr() as *mut __m256i, add(x, y, p2));
            _mm256_storeu_si256(r.as_mut_ptr() as *mut __m256i, sub(x, y, p2));
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn dit_block(lo: &mut [u32], hi: &mut [u32], irot: u32, mont: Montgomery) {
        let p = _mm256_set1_epi32(mont.p as i32);
        let p2 = _mm256_set1_epi32(mont.p2 as i32);
        let n_inv = _mm256_set1_epi32(mont.n_inv as i32);
        let w = _mm256_set1_epi32(irot as i32);
        for (l, r) in lo.chunks_exact_mut(8).zip(hi.chunks_exact_mut(8)) {
            let x = _mm256_loadu_si256(l.as_ptr() as *const __m256i);
            let y = _mm256_loadu_si256(r.as_ptr() as *const __m256i);
            _mm256_storeu_si256(l.as_mut_ptr() as *mut __m256i, add(x, y, p2));
            let d = sub(x, y, p2);
            _mm256_storeu_si256(r.as_mut_ptr() as *mut __m256i, mul(d, w, p, n_inv));
        }
    }

    /// Twiddle vector `[t[0] x4, t[1] x4]` from two consecutive table entries.
    #[inline(always)]
    unsafe fn dup4(t: *const u32) -> __m256i {
        let v = _mm256_castsi128_si256(_mm_loadl_epi64(t as *const __m128i));
        _mm256_permutevar8x32_epi32(v, _mm256_setr_epi32(0, 0, 0, 0, 1, 1, 1, 1))
    }

    /// Twiddle vector `[t[0] x2, t[1] x2, t[2] x2, t[3] x2]`.
    #[inline(always)]
    unsafe fn dup2(t: *const u32) -> __m256i {
        let v = _mm256_castsi128_si256(_mm_loadu_si128(t as *const __m128i));
        _mm256_permutevar8x32_epi32(v, _mm256_setr_epi32(0, 0, 1, 1, 2, 2, 3, 3))
    }

    /// The last three forward levels for every block of eight elements:
    /// block `s` uses `tw[s]`, then `tw[2s..2s+2]`, then `tw[4s..4s+4]`.
    #[target_feature(enable = "avx2")]
    pub unsafe fn dif_leaf(a: &mut [u32], tw: &[u32], mont: Montgomery) {
        let p = _mm256_set1_epi32(mont.p as i32);
        let p2 = _mm256_set1_epi32(mont.p2 as i32);
        let n_inv = _mm256_set1_epi32(mont.n_inv as i32);
        for (s, block) in a.chunks_exact_mut(8).enumerate() {
            let ptr = block.as_mut_ptr() as *mut __m256i;
            let mut v = _mm256_loadu_si256(ptr);
            // half = 4: x = lanes 0..4, y = lanes 4..8
            let x = _mm256_permute2x128_si256::<0x00>(v, v);
            let y = mul(
                _mm256_permute2x128_si256::<0x11>(v, v),
                _mm256_set1_epi32(tw[s] as i32),
                p,
                n_inv,
            );
            v = _mm256_blend_epi32::<0b1111_0000>(add(x, y, p2), sub(x, y, p2));
            // half = 2: x = lanes {0,1,4,5}, y = lanes {2,3,6,7}
            let x = _mm256_shuffle_epi32::<0b0100_0100>(v);
            let y = mul(
                _mm256_shuffle_epi32::<0b1110_1110>(v),
                dup4(tw.as_ptr().add(2 * s)),
                p,
                n_inv,
            );
            v = _mm256_blend_epi32::<0b1100_1100>(add(x, y, p2), sub(x, y, p2));
            // half = 1: x = even lanes, y = odd lanes
            let x = _mm256_shuffle_epi32::<0b1010_0000>(v);
            let y = mul(
                _mm256_shuffle_epi32::<0b1111_0101>(v),
                dup2(tw.as_ptr().add(4 * s)),
                p,
                n_inv,
            );
            v = _mm256_blend_epi32::<0b1010_1010>(add(x, y, p2), sub(x, y, p2));
            _mm256_storeu_si256(ptr, v);
        }
    }

    /// The first three inverse levels (mirror of `dif_leaf`).
    #[target_feature(enable = "avx2")]
    pub unsafe fn dit_leaf(a: &mut [u32], itw: &[u32], mont: Montgomery) {
        let p = _mm256_set1_epi32(mont.p as i32);
        let p2 = _mm256_set1_epi32(mont.p2 as i32);
        let n_inv = _mm256_set1_epi32(mont.n_inv as i32);
        for (s, block) in a.chunks_exact_mut(8).enumerate() {
            let ptr = block.as_mut_ptr() as *mut __m256i;
            let mut v = _mm256_loadu_si256(ptr);
            // half = 1
            let x = _mm256_shuffle_epi32::<0b1010_0000>(v);
            let y = _mm256_shuffle_epi32::<0b1111_0101>(v);
            let d = mul(sub(x, y, p2), dup2(itw.as_ptr().add(4 * s)), p, n_inv);
            v = _mm256_blend_epi32::<0b1010_1010>(add(x, y, p2), d);
            // half = 2
            let x = _mm256_shuffle_epi32::<0b0100_0100>(v);
            let y = _mm256_shuffle_epi32::<0b1110_1110>(v);
            let d = mul(sub(x, y, p2), dup4(itw.as_ptr().add(2 * s)), p, n_inv);
            v = _mm256_blend_epi32::<0b1100_1100>(add(x, y, p2), d);
            // half = 4
            let x = _mm256_permute2x128_si256::<0x00>(v, v);
            let y = _mm256_permute2x128_si256::<0x11>(v, v);
            let d = mul(sub(x, y, p2), _mm256_set1_epi32(itw[s] as i32), p, n_inv);
            v = _mm256_blend_epi32::<0b1111_0000>(add(x, y, p2), d);
            _mm256_storeu_si256(ptr, v);
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn pointwise(a: &mut [u32], b: &[u32], mont: Montgomery) {
        let p = _mm256_set1_epi32(mont.p as i32);
        let n_inv = _mm256_set1_epi32(mont.n_inv as i32);
        for (x, y) in a.chunks_exact_mut(8).zip(b.chunks_exact(8)) {
            let vx = _mm256_loadu_si256(x.as_ptr() as *const __m256i);
            let vy = _mm256_loadu_si256(y.as_ptr() as *const __m256i);
            _mm256_storeu_si256(x.as_mut_ptr() as *mut __m256i, mul(vx, vy, p, n_inv));
        }
    }
}

/// Copies `a` into `buf` in Montgomery form, zero-padded to `size`.
fn load<T: Into<u64>, M: BaseModInt<T>>(
    buf: &mut Vec<u32>,
    a: &[M],
    size: usize,
    mont: Montgomery,
) {
    buf.clear();
    buf.reserve(size);
    buf.extend(
        a.iter()
            .map(|x| mont.to_mont(Into::<u64>::into(x.value()) as u32)),
    );
    buf.resize(size, 0);
}

#[cfg(test)]
mod tests {
    use super::PrimeFFT;
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::mod_int::ModIntF as M;
    use crate::numbers::num_traits::algebra::{One, Zero};

    fn naive(a: &[M], b: &[M]) -> Vec<M> {
        let mut res = vec![M::zero(); a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                res[i + j] += x * y;
            }
        }
        res
    }

    fn eval(p: &[M], x: M) -> M {
        p.iter().rev().fold(M::zero(), |acc, &c| acc * x + c)
    }

    #[test]
    fn fixed_length_is_a_linear_product_prefix() {
        let mut fft = PrimeFFT::<M>::new();
        let a: Vec<_> = (0..100usize).map(|i| M::from(i + 1)).collect();
        let b: Vec<_> = (0..85usize).map(|i| M::from(i * 13 + 5)).collect();
        let expected = naive(&a, &b);
        for len in [0, 61, 75, 100, 184, 300] {
            let mut want = expected.clone();
            want.resize(len, M::zero());
            let mut result = vec![M::one(); len];
            fft.multiply_fix_len(&a, &b, &mut result);
            assert_eq!(result, want);
        }
    }

    #[test]
    fn random_sizes_match_naive() {
        let mut rng = Random::new_with_seed(17);
        let mut fft = PrimeFFT::<M>::new();
        for _ in 0..200 {
            let n = rng.gen_range(1..400usize);
            let m = rng.gen_range(1..400usize);
            let a: Vec<M> = (0..n).map(|_| M::new(rng.gen_u128() as u32)).collect();
            let b: Vec<M> = (0..m).map(|_| M::new(rng.gen_u128() as u32)).collect();
            assert_eq!(fft.multiply(&a, &b), naive(&a, &b), "sizes {n} x {m}");
            assert_eq!(fft.multiply(&a, &a), naive(&a, &a), "square {n}");
        }
    }

    #[test]
    fn large_product_evaluates_correctly() {
        let mut rng = Random::new_with_seed(18);
        let mut fft = PrimeFFT::<M>::new();
        for &(n, m) in &[
            (1usize << 16, 1usize << 16),
            (300_000, 123_457),
            (1 << 19, 1 << 19),
        ] {
            let a: Vec<M> = (0..n).map(|_| M::new(rng.gen_u128() as u32)).collect();
            let b: Vec<M> = (0..m).map(|_| M::new(rng.gen_u128() as u32)).collect();
            let c = fft.multiply(&a, &b);
            assert_eq!(c.len(), n + m - 1);
            for _ in 0..3 {
                let x = M::new(rng.gen_u128() as u32);
                assert_eq!(eval(&c, x), eval(&a, x) * eval(&b, x));
            }
            assert_eq!(c[0], a[0] * b[0]);
            assert_eq!(c[n + m - 2], a[n - 1] * b[m - 1]);
        }
    }

    #[test]
    fn scalar_and_simd_paths_agree() {
        let mut rng = Random::new_with_seed(19);
        let mut fast = PrimeFFT::<M>::new();
        let mut slow = PrimeFFT::<M>::new().scalar_only();
        for _ in 0..60 {
            let n = rng.gen_range(1..700usize);
            let m = rng.gen_range(1..700usize);
            let a: Vec<M> = (0..n).map(|_| M::new(rng.gen_u128() as u32)).collect();
            let b: Vec<M> = (0..m).map(|_| M::new(rng.gen_u128() as u32)).collect();
            let expected = naive(&a, &b);
            assert_eq!(slow.multiply(&a, &b), expected, "scalar {n} x {m}");
            assert_eq!(fast.multiply(&a, &b), expected, "simd {n} x {m}");
        }
        for &(n, m) in &[(1usize << 15, 1usize << 15), (200_000, 70_001)] {
            let a: Vec<M> = (0..n).map(|_| M::new(rng.gen_u128() as u32)).collect();
            let b: Vec<M> = (0..m).map(|_| M::new(rng.gen_u128() as u32)).collect();
            let c = slow.multiply(&a, &b);
            assert_eq!(c, fast.multiply(&a, &b));
            let x = M::new(rng.gen_u128() as u32);
            assert_eq!(eval(&c, x), eval(&a, x) * eval(&b, x));
        }
    }

    #[test]
    fn power_matches_repeated_multiply() {
        let mut fft = PrimeFFT::<M>::new();
        let a: Vec<M> = (0..70usize).map(|i| M::from(i * i + 3)).collect();
        let mut expected = vec![M::one()];
        for _ in 0..7 {
            expected = naive(&expected, &a);
        }
        assert_eq!(fft.power(&a, 7), expected);
    }
}
