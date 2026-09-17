//! Linear algebra modulo a 32-bit prime with delayed reduction: rows are
//! kept as `u64` and reduced only every `batch` accumulated products, so the
//! inner loops are plain multiply-adds that vectorize.

use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::mod_int::BaseModInt;

/// Products that fit in a `u64` accumulator on top of a reduced value.
fn batch_size(p: u64) -> usize {
    (((u64::MAX - p) / (p * p)) as usize).clamp(1, 1 << 20)
}

fn modulus<M: BaseModInt<u32>>() -> u64 {
    M::module() as u64
}

fn to_m<M: BaseModInt<u32>>(v: u64) -> M {
    M::from(v as usize)
}

/// `acc[j] += x * row[j]` for every column.
#[inline]
fn axpy(acc: &mut [u64], x: u32, row: &[u32], avx2: bool) {
    #[cfg(target_arch = "x86_64")]
    if avx2 {
        // SAFETY: `avx2` is only set when the CPU reports AVX2.
        unsafe { axpy_avx2(acc, x, row) };
        return;
    }
    let _ = avx2;
    for (s, &y) in acc.iter_mut().zip(row.iter()) {
        *s += x as u64 * y as u64;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn axpy_avx2(acc: &mut [u64], x: u32, row: &[u32]) {
    use std::arch::x86_64::*;
    let xv = _mm256_set1_epi64x(x as i64);
    for (a, r) in acc.chunks_exact_mut(4).zip(row.chunks_exact(4)) {
        let rv = _mm256_cvtepu32_epi64(_mm_loadu_si128(r.as_ptr() as *const __m128i));
        let av = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
        _mm256_storeu_si256(
            a.as_mut_ptr() as *mut __m256i,
            _mm256_add_epi64(av, _mm256_mul_epu32(xv, rv)),
        );
    }
    let tail = acc.len() / 4 * 4;
    for (s, &y) in acc[tail..].iter_mut().zip(row[tail..].iter()) {
        *s += x as u64 * y as u64;
    }
}

fn avx2_available() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        is_x86_feature_detected!("avx2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// `a * b` for an `n x k` and a `k x m` matrix.
pub fn mat_mul<M: BaseModInt<u32>>(a: &Arr2d<M>, b: &Arr2d<M>) -> Arr2d<M> {
    assert_eq!(a.d2(), b.d1());
    let (n, k, m) = (a.d1(), a.d2(), b.d2());
    let p = modulus::<M>();
    let batch = batch_size(p);
    let avx2 = avx2_available();
    let b_rows: Vec<Vec<u32>> = (0..k)
        .map(|i| b.row(i).map(|x| x.value()).collect())
        .collect();
    let mut acc = vec![0u64; m];
    Arr2d::with_gen(n, m, |i, j| {
        if j == 0 {
            acc.fill(0);
            for (start, chunk) in b_rows.chunks(batch).enumerate() {
                for (t, row) in chunk.iter().enumerate() {
                    let x = a[(i, start * batch + t)].value();
                    if x != 0 {
                        axpy(&mut acc, x, row, avx2);
                    }
                }
                for s in acc.iter_mut() {
                    *s %= p;
                }
            }
        }
        to_m(acc[j])
    })
}

/// Row-reduced working copy with delayed reduction.
struct Rows {
    p: u64,
    batch: usize,
    cols: usize,
    rows: Vec<Vec<u64>>,
    dirty: Vec<usize>,
    pivot: Vec<u32>,
}

impl Rows {
    fn new<M: BaseModInt<u32>>(
        mat: &Arr2d<M>,
        extra: impl Fn(usize, usize) -> M,
        extra_cols: usize,
    ) -> Self {
        let p = modulus::<M>();
        let cols = mat.d2() + extra_cols;
        let rows = (0..mat.d1())
            .map(|i| {
                let mut row: Vec<u64> = mat.row(i).map(|x| x.value() as u64).collect();
                row.extend((0..extra_cols).map(|j| extra(i, j).value() as u64));
                row
            })
            .collect();
        Self {
            p,
            batch: batch_size(p),
            cols,
            rows,
            dirty: vec![0; mat.d1()],
            pivot: vec![0; cols],
        }
    }

    fn reduce_row(&mut self, j: usize) {
        let p = self.p;
        for x in self.rows[j].iter_mut() {
            *x %= p;
        }
        self.dirty[j] = 0;
    }

    /// Eliminates over the first `pivot_cols` columns. Returns the pivot
    /// columns in order and the determinant factor (sign times pivots) of the
    /// eliminated block, with rows below (or all rows if `full`) cleared.
    fn eliminate<M: BaseModInt<u32>>(&mut self, pivot_cols: usize, full: bool) -> (Vec<usize>, M) {
        let n = self.rows.len();
        let p = self.p;
        let mut det = M::one();
        let mut pivots = Vec::new();
        for col in 0..pivot_cols {
            let r = pivots.len();
            if r == n {
                break;
            }
            let Some(j) = (r..n).find(|&j| self.rows[j][col] % p != 0) else {
                det = M::zero();
                continue;
            };
            if j != r {
                self.rows.swap(j, r);
                self.dirty.swap(j, r);
                det = -det;
            }
            self.reduce_row(r);
            let pv = self.rows[r][col];
            det *= to_m::<M>(pv);
            let inv = to_m::<M>(pv).inv().unwrap().value() as u64;
            for x in self.rows[r][col..].iter_mut() {
                *x = *x * inv % p;
            }
            for (k, &x) in self.rows[r].iter().enumerate() {
                self.pivot[k] = x as u32;
            }
            let from = if full { 0 } else { r + 1 };
            for j in from..n {
                if j == r {
                    continue;
                }
                let f = self.rows[j][col] % p;
                if f == 0 {
                    continue;
                }
                let c = p - f;
                let row = &mut self.rows[j];
                for (x, &y) in row[col..].iter_mut().zip(self.pivot[col..].iter()) {
                    *x += c * y as u64;
                }
                self.dirty[j] += 1;
                if self.dirty[j] >= self.batch {
                    self.reduce_row(j);
                }
            }
            pivots.push(col);
        }
        for j in 0..n {
            self.reduce_row(j);
        }
        (pivots, det)
    }
}

pub fn det<M: BaseModInt<u32>>(mat: &Arr2d<M>) -> M {
    assert_eq!(mat.d1(), mat.d2());
    if mat.d1() == 0 {
        return M::one();
    }
    let mut rows = Rows::new(mat, |_, _| M::zero(), 0);
    let (pivots, det) = rows.eliminate::<M>(mat.d2(), false);
    if pivots.len() < mat.d1() {
        M::zero()
    } else {
        det
    }
}

pub fn rank<M: BaseModInt<u32>>(mat: &Arr2d<M>) -> usize {
    if mat.d1() == 0 || mat.d2() == 0 {
        return 0;
    }
    let mut rows = Rows::new(mat, |_, _| M::zero(), 0);
    rows.eliminate::<M>(mat.d2(), false).0.len()
}

pub fn invert<M: BaseModInt<u32>>(mat: &Arr2d<M>) -> Option<Arr2d<M>> {
    let n = mat.d1();
    assert_eq!(mat.d2(), n);
    let mut rows = Rows::new(mat, |i, j| if i == j { M::one() } else { M::zero() }, n);
    let (pivots, _) = rows.eliminate::<M>(n, true);
    if pivots.len() < n {
        return None;
    }
    Some(Arr2d::with_gen(n, n, |i, j| to_m(rows.rows[i][n + j])))
}

/// One solution of `a x = b` together with a basis of the kernel of `a`, or
/// `None` if the system is inconsistent.
pub fn solve<M: BaseModInt<u32>>(a: &Arr2d<M>, b: &[M]) -> Option<(Vec<M>, Vec<Vec<M>>)> {
    let (n, m) = (a.d1(), a.d2());
    assert_eq!(b.len(), n);
    let mut rows = Rows::new(a, |i, _| b[i], 1);
    let (pivots, _) = rows.eliminate::<M>(m, true);
    let r = pivots.len();
    if rows.rows[r..].iter().any(|row| row[m] != 0) {
        return None;
    }
    let mut x = vec![M::zero(); m];
    for (i, &col) in pivots.iter().enumerate() {
        x[col] = to_m(rows.rows[i][m]);
    }
    let mut is_pivot = vec![false; m];
    for &col in &pivots {
        is_pivot[col] = true;
    }
    let mut kernel = Vec::with_capacity(m - r);
    for free in 0..m {
        if is_pivot[free] {
            continue;
        }
        let mut v = vec![M::zero(); m];
        v[free] = M::one();
        for (i, &col) in pivots.iter().enumerate() {
            v[col] = -to_m::<M>(rows.rows[i][free]);
        }
        kernel.push(v);
    }
    let _ = rows.cols;
    Some((x, kernel))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::gauss;
    use crate::numbers::matrix::Matrix;
    use crate::numbers::mod_int::{ModInt7, ModIntF};

    fn random_matrix<M: BaseModInt<u32>>(
        rng: &mut Random,
        n: usize,
        m: usize,
        small: bool,
    ) -> Arr2d<M> {
        Arr2d::with_gen(n, m, |_, _| {
            M::from(if small {
                rng.gen_range(0..3usize)
            } else {
                rng.gen_u128() as u32 as usize
            })
        })
    }

    fn check<M: BaseModInt<u32> + std::fmt::Debug>(rng: &mut Random) {
        for _ in 0..150 {
            let n = rng.gen_range(1..=9usize);
            let m = rng.gen_range(1..=9usize);
            let small = rng.gen_bool();
            let a: Arr2d<M> = random_matrix(rng, n, m, small);
            let b: Arr2d<M> = random_matrix(rng, m, n, small);
            assert!(
                Matrix::from(mat_mul(&a, &b))
                    == Matrix::from(a.clone()).mult(&Matrix::from(b.clone()))
            );
            if n == m {
                let mut copy = a.clone();
                assert_eq!(det(&a), gauss::det(&mut copy));
                let inv = invert(&a);
                let expected = gauss::invert(&a);
                assert_eq!(inv.is_some(), expected.is_some());
                if let Some(inv) = inv {
                    assert!(Matrix::from(mat_mul(&a, &inv)) == Matrix::ident(n));
                }
            }
            let mut copy = a.clone();
            gauss::gauss(&mut copy);
            let expected_rank = (0..n)
                .filter(|&i| copy.row(i).any(|&x| x != M::zero()))
                .count();
            assert_eq!(rank(&a), expected_rank);
            let x_true: Vec<M> = (0..m).map(|_| M::from(rng.gen_range(0..5usize))).collect();
            let rhs: Vec<M> = (0..n)
                .map(|i| (0..m).fold(M::zero(), |s, j| s + a[(i, j)] * x_true[j]))
                .collect();
            let (x, kernel) = solve(&a, &rhs).expect("consistent system");
            for i in 0..n {
                let got = (0..m).fold(M::zero(), |s, j| s + a[(i, j)] * x[j]);
                assert_eq!(got, rhs[i]);
            }
            assert_eq!(kernel.len(), m - expected_rank);
            for v in &kernel {
                for i in 0..n {
                    let got = (0..m).fold(M::zero(), |s, j| s + a[(i, j)] * v[j]);
                    assert_eq!(got, M::zero());
                }
            }
            if expected_rank < n {
                // a right-hand side outside the column space is inconsistent
                let mut bad = rhs.clone();
                let mut any = false;
                for _ in 0..20 {
                    for x in bad.iter_mut() {
                        *x = M::from(rng.gen_u128() as u32 as usize);
                    }
                    if solve(&a, &bad).is_none() {
                        any = true;
                        break;
                    }
                }
                assert!(any);
            }
        }
    }

    #[test]
    fn matches_generic_gauss() {
        let mut rng = Random::new_with_seed(61);
        check::<ModIntF>(&mut rng);
        check::<ModInt7>(&mut rng);
    }

    #[test]
    fn larger_sizes() {
        let mut rng = Random::new_with_seed(62);
        let n = 120;
        let a: Arr2d<ModIntF> = random_matrix(&mut rng, n, n, false);
        let inv = invert(&a).unwrap();
        assert!(Matrix::from(mat_mul(&a, &inv)) == Matrix::ident(n));
        let mut copy = a.clone();
        assert_eq!(det(&a), gauss::det(&mut copy));
        assert_eq!(rank(&a), n);
        let b: Arr2d<ModIntF> = random_matrix(&mut rng, n, 77, false);
        assert!(Matrix::from(mat_mul(&a, &b)) == Matrix::from(a).mult(&Matrix::from(b)));
    }
}
