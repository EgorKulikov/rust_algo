//! Criterion benches for the performance-sensitive kernels of algo_lib.
//!
//! Run all:        cargo bench --bench kernels
//! Save baseline:  cargo bench --bench kernels -- --save-baseline pre
//! Compare:        cargo bench --bench kernels -- --baseline pre

use algo_lib::collections::int_set::IntSet;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::seg_tree::{LazyMonoid, LazySegTree, Monoid, SegTree};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::prime_fft::PrimeFFT;
use algo_lib::numbers::mod_int::ModIntF as M;
use algo_lib::numbers::mod_linear;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::polynomial::PolynomialOps;
use algo_lib::numbers::primes::prime::is_prime;
use algo_lib::numbers::primes::sieve::primes;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn random_mods(rng: &mut ChaCha20Rng, n: usize) -> Vec<M> {
    (0..n).map(|_| M::new(rng.gen())).collect()
}

fn bench_io(c: &mut Criterion) {
    let mut rng = ChaCha20Rng::seed_from_u64(1);
    let n = 1_000_000;
    let values: Vec<i64> = (0..n)
        .map(|_| rng.gen::<i64>() >> rng.gen_range(0..64))
        .collect();
    let mut text = String::new();
    for (i, v) in values.iter().enumerate() {
        text += &v.to_string();
        text.push(if i % 8 == 7 { '\n' } else { ' ' });
    }
    c.bench_function("io/parse_1e6_i64", |b| {
        b.iter_batched(
            || Input::slice(text.as_bytes()),
            |mut input| {
                let mut sum = 0i64;
                for _ in 0..n {
                    sum = sum.wrapping_add(input.read_long());
                }
                black_box(sum)
            },
            BatchSize::LargeInput,
        );
    });
    c.bench_function("io/format_1e6_i64", |b| {
        let mut buf = Vec::with_capacity(text.len() + 64);
        b.iter(|| {
            buf.clear();
            let mut out = Output::buf(&mut buf);
            for &v in &values {
                out.print(v);
                out.put(b' ');
            }
            out.flush();
        });
    });
}

fn bench_ntt(c: &mut Criterion) {
    let mut rng = ChaCha20Rng::seed_from_u64(2);
    for &log in &[12usize, 17, 19] {
        let a = random_mods(&mut rng, 1 << log);
        let b = random_mods(&mut rng, 1 << log);
        let mut fft = PrimeFFT::<M>::new();
        c.bench_function(&format!("ntt/multiply_2^{log}"), |bench| {
            bench.iter(|| black_box(fft.multiply(&a, &b)));
        });
    }
}

fn bench_fps(c: &mut Criterion) {
    let mut rng = ChaCha20Rng::seed_from_u64(3);
    let n = 100_000;
    let mut f = random_mods(&mut rng, n);
    f[0] = M::one();
    let mut g = f.clone();
    g[0] = M::zero();
    let mut ops = PolynomialOps::new();
    c.bench_function("fps/inverse_1e5", |b| {
        b.iter(|| black_box(ops.inverse_series(&f, n)))
    });
    c.bench_function("fps/log_1e5", |b| b.iter(|| black_box(ops.log(&f, n))));
    c.bench_function("fps/exp_1e5", |b| b.iter(|| black_box(ops.exp(&g, n))));
}

struct Affine;
impl Monoid for Affine {
    type V = (M, M);
    fn e() -> Self::V {
        (M::one(), M::zero())
    }
    fn op(a: Self::V, b: Self::V) -> Self::V {
        (a.0 * b.0, a.1 * b.0 + b.1)
    }
}

struct SumAffine;
impl Monoid for SumAffine {
    type V = (M, u32);
    fn e() -> Self::V {
        (M::zero(), 0)
    }
    fn op(a: Self::V, b: Self::V) -> Self::V {
        (a.0 + b.0, a.1 + b.1)
    }
}
impl LazyMonoid for SumAffine {
    type F = (M, M);
    fn id() -> Self::F {
        (M::one(), M::zero())
    }
    fn mapping(f: Self::F, v: Self::V) -> Self::V {
        (f.0 * v.0 + f.1 * M::from(v.1 as usize), v.1)
    }
    fn composition(f: Self::F, g: Self::F) -> Self::F {
        (f.0 * g.0, f.0 * g.1 + f.1)
    }
}

fn bench_seg_tree(c: &mut Criterion) {
    let mut rng = ChaCha20Rng::seed_from_u64(4);
    let n = 200_000usize;
    let init: Vec<(M, M)> = (0..n)
        .map(|_| (M::new(rng.gen()), M::new(rng.gen())))
        .collect();
    let ops: Vec<(bool, usize, usize, M, M)> = (0..n)
        .map(|_| {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l + 1..=n);
            (rng.gen(), l, r, M::new(rng.gen()), M::new(rng.gen()))
        })
        .collect();
    c.bench_function("seg_tree/point_set_range_composite_2e5", |b| {
        b.iter_batched(
            || SegTree::<Affine>::from_slice(&init),
            |mut st| {
                let mut acc = M::zero();
                for &(set, l, r, x, y) in &ops {
                    if set {
                        st.set(l, (x, y));
                    } else {
                        acc += st.query(l..r).1;
                    }
                }
                black_box(acc)
            },
            BatchSize::LargeInput,
        );
    });
    c.bench_function("seg_tree/range_affine_range_sum_2e5", |b| {
        b.iter_batched(
            || LazySegTree::<SumAffine>::with_gen(n, |i| (init[i].0, 1)),
            |mut st| {
                let mut acc = M::zero();
                for &(apply, l, r, x, y) in &ops {
                    if apply {
                        st.apply(l..r, (x, y));
                    } else {
                        acc += st.query(l..r).0;
                    }
                }
                black_box(acc)
            },
            BatchSize::LargeInput,
        );
    });
}

fn bench_linear_algebra(c: &mut Criterion) {
    let mut rng = ChaCha20Rng::seed_from_u64(5);
    let n = 200;
    let a = Arr2d::with_gen(n, n, |_, _| M::new(rng.gen()));
    let b = Arr2d::with_gen(n, n, |_, _| M::new(rng.gen()));
    c.bench_function("mod_linear/mat_mul_200", |bench| {
        bench.iter(|| black_box(mod_linear::mat_mul(&a, &b)))
    });
    c.bench_function("mod_linear/det_200", |bench| {
        bench.iter(|| black_box(mod_linear::det(&a)))
    });
    c.bench_function("mod_linear/invert_200", |bench| {
        bench.iter(|| black_box(mod_linear::invert(&a)))
    });
}

fn bench_int_set(c: &mut Criterion) {
    let mut rng = ChaCha20Rng::seed_from_u64(6);
    let n = 1_000_000usize;
    let ops: Vec<(u8, usize)> = (0..n)
        .map(|_| (rng.gen_range(0..4), rng.gen_range(0..n)))
        .collect();
    c.bench_function("int_set/mixed_1e6", |b| {
        b.iter_batched(
            || IntSet::new(n),
            |mut set| {
                let mut acc = 0usize;
                for &(t, x) in &ops {
                    match t {
                        0 => {
                            set.insert(x);
                        }
                        1 => {
                            set.remove(x);
                        }
                        2 => acc = acc.wrapping_add(set.next(x).unwrap_or(n)),
                        _ => acc = acc.wrapping_add(set.prev(x).unwrap_or(n)),
                    }
                }
                black_box(acc)
            },
            BatchSize::LargeInput,
        );
    });
}

fn bench_primes(c: &mut Criterion) {
    let mut rng = ChaCha20Rng::seed_from_u64(7);
    let queries: Vec<u64> = (0..10_000)
        .map(|_| rng.gen_range(1..=1_000_000_000_000_000_000u64))
        .collect();
    c.bench_function("primes/is_prime_1e4_x_1e18", |b| {
        b.iter(|| black_box(queries.iter().filter(|&&x| is_prime(x)).count()));
    });
    c.bench_function("primes/sieve_1e7", |b| {
        b.iter(|| black_box(primes(10_000_000).len()))
    });
}

criterion_group!(
    benches,
    bench_io,
    bench_ntt,
    bench_fps,
    bench_seg_tree,
    bench_linear_algebra,
    bench_int_set,
    bench_primes
);
criterion_main!(benches);
