use crate::numbers::mod_int::mod_utils::inverse_factorials;
use crate::numbers::mod_int::BaseModInt;
use crate::numbers::num_traits::algebra::IntegerSemiRing;
use crate::numbers::polynomial::{evaluate, PolynomialOps, ProductTree};
use std::marker::PhantomData;
use std::sync::OnceLock;

enum Samples<Mod> {
    Consecutive { values: Vec<Mod>, weights: Vec<Mod> },
    Arbitrary { len: usize },
}

/// A polynomial specified by its values at distinct modular coordinates.
///
/// Empty input represents the zero polynomial, but has no [`Self::degree`].
pub struct Interpolation<T, Mod: BaseModInt<T> + From<usize> + Into<usize>> {
    samples: Samples<Mod>,
    polynomial: OnceLock<Vec<Mod>>,
    phantom_data: PhantomData<T>,
}

impl<T: IntegerSemiRing + Copy, Mod: BaseModInt<T> + From<usize> + Into<usize>>
    Interpolation<T, Mod>
{
    /// Interpolates the values at x = 0, 1, ..., values.len() - 1 in O(n).
    pub fn new(values: Vec<Mod>) -> Self {
        let n = values.len();
        Self::with_inverse_factorials(values, inverse_factorials(n).as_slice())
    }

    pub fn with_inverse_factorials(values: Vec<Mod>, inv_fact: &[Mod]) -> Self {
        let n = values.len();
        let mut weights = Vec::with_capacity(n);
        for (i, &v) in values.iter().enumerate() {
            weights.push(
                v * inv_fact[i]
                    * inv_fact[n - i - 1]
                    * if (n - i - 1) & 1 == 1 {
                        -Mod::one()
                    } else {
                        Mod::one()
                    },
            );
        }
        Self {
            samples: Samples::Consecutive { values, weights },
            polynomial: OnceLock::new(),
            phantom_data: PhantomData,
        }
    }

    /// Interpolates arbitrary (x, y) pairs, without retaining a borrow of `points`.
    /// Coordinates may be unordered, but their pairwise differences must be
    /// invertible (equivalently, the coordinates must be distinct over a prime field).
    ///
    /// Construction takes O(n log^2 n) with FFT multiplication; see
    /// [`Self::calculate_many`] for supported moduli and transform limits.
    /// Wider moduli use quadratic polynomial arithmetic.
    ///
    /// ```
    /// use algo_lib::numbers::interpolation::Interpolation;
    /// use algo_lib::numbers::mod_int::ModIntF as M;
    ///
    /// let points = [(M::new(2), M::new(4)), (M::new(5), M::new(25)), (M::new(9), M::new(81))];
    /// let polynomial = Interpolation::from_points(&points);
    /// assert_eq!(polynomial.calculate_many(&[M::new(3), M::new(7)]), vec![M::new(9), M::new(49)]);
    /// ```
    ///
    /// # Panics
    /// Panics if a coordinate difference is not invertible, or an FFT transform
    /// exceeds the limits described in [`Self::calculate_many`].
    pub fn from_points(points: &[(Mod, Mod)]) -> Self
    where
        T: Into<u64>,
    {
        if points.is_empty() {
            return Self::new(Vec::new());
        }
        let xs: Vec<_> = points.iter().map(|&(x, _)| x).collect();
        let mut ops = PolynomialOps::new();
        let tree = ProductTree::new(&xs, &mut ops);
        // For P(x) = product(x - x_i), the Lagrange weight is y_i / P'(x_i).
        let derivative: Vec<_> = tree
            .product()
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| c * Mod::from(i))
            .collect();
        let mut denominators = tree.evaluate(&derivative, &mut ops);
        let mut prefixes = Vec::with_capacity(points.len());
        let mut product = Mod::one();
        for &denominator in &denominators {
            prefixes.push(product);
            product *= denominator;
        }
        // Invert all denominators with a single modular inverse.
        let mut inverse = product
            .inv()
            .expect("sample x coordinates must be distinct with invertible differences");
        for i in (0..points.len()).rev() {
            let denominator = denominators[i];
            denominators[i] = points[i].1 * prefixes[i] * inverse;
            inverse *= denominator;
        }
        Self {
            samples: Samples::Arbitrary { len: points.len() },
            polynomial: OnceLock::from(tree.interpolate(&denominators, &mut ops)),
            phantom_data: PhantomData,
        }
    }

    /// Evaluates one coordinate in O(n), with O(1) lookup of consecutive samples.
    pub fn calculate(&self, x: Mod) -> Mod {
        if let Samples::Consecutive { values, .. } = &self.samples {
            let i = x.into();
            if i < values.len() {
                return values[i];
            }
        }
        if let Some(polynomial) = self.polynomial.get() {
            return evaluate(polynomial, x);
        }
        let Samples::Consecutive { weights, .. } = &self.samples else {
            unreachable!()
        };
        // Prefix/suffix products evaluate Lagrange's formula without inverses.
        let mut prefixes = Vec::with_capacity(weights.len());
        let mut product = Mod::one();
        for i in 0..weights.len() {
            prefixes.push(product);
            product *= x - Mod::from(i);
        }
        let mut suffix = Mod::one();
        let mut result = Mod::zero();
        for i in (0..weights.len()).rev() {
            result += weights[i] * prefixes[i] * suffix;
            suffix *= x - Mod::from(i);
        }
        result
    }

    /// Evaluates a batch in input order. Query coordinates may repeat.
    ///
    /// Uses product/remainder trees in O((n + m) log^2(n + m)) time and
    /// O((n + m) log(n + m)) temporary space for n samples and m queries.
    /// Coefficients are cached, so subsequent calls skip reconstruction.
    /// Small batches use direct evaluation.
    ///
    /// See the [polynomial module](crate::numbers::polynomial) for supported
    /// moduli, FFT transform limits, and the quadratic fallback for wider moduli.
    pub fn calculate_many(&self, xs: &[Mod]) -> Vec<Mod>
    where
        T: Into<u64>,
    {
        if xs.len() <= 32 {
            return xs.iter().map(|&x| self.calculate(x)).collect();
        }
        let mut ops = PolynomialOps::new();
        let polynomial = self.polynomial.get_or_init(|| {
            let Samples::Consecutive { values, weights } = &self.samples else {
                unreachable!()
            };
            if values.is_empty() {
                return Vec::new();
            }
            let sample_xs: Vec<_> = (0..values.len()).map(Mod::from).collect();
            ProductTree::new(&sample_xs, &mut ops).interpolate(weights, &mut ops)
        });
        if polynomial.len() <= 32 {
            return xs.iter().map(|&x| evaluate(polynomial, x)).collect();
        }
        ProductTree::new(xs, &mut ops).evaluate(polynomial, &mut ops)
    }

    /// Returns the sample count minus one, an upper bound on polynomial degree.
    /// Panics for empty input.
    pub fn degree(&self) -> usize {
        let len = match &self.samples {
            Samples::Consecutive { values, .. } => values.len(),
            Samples::Arbitrary { len } => *len,
        };
        len.checked_sub(1)
            .expect("empty interpolation has no degree")
    }
}

#[cfg(test)]
mod tests {
    use super::Interpolation;
    use crate::numbers::mod_int::mod_utils::inverse_factorials;
    use crate::numbers::mod_int::{BaseModInt, ModInt, ModInt64, ModInt7, ModInt9, ModIntF};
    use crate::numbers::num_traits::algebra::{IntegerSemiRing, One, Zero};
    use crate::numbers::number_ext::Power;
    use rand::seq::SliceRandom;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    use std::fmt::Debug;

    #[test]
    fn arbitrary_points_from_borrowed_slice() {
        type M = ModIntF;
        // y = x^2 + 2x + 3; unordered coordinates, including a negative one.
        let interpolation = {
            let points = [
                (M::from(5), M::from(38)),
                (M::from(-2), M::from(3)),
                (M::from(1), M::from(6)),
            ];
            Interpolation::from_points(&points[..])
        };
        assert_eq!(interpolation.degree(), 2);
        for (x, y) in [(-2, 3), (0, 3), (1, 6), (2, 11), (5, 38), (10, 123)] {
            assert_eq!(interpolation.calculate(M::from(x)), M::from(y));
        }
        let xs = [5, 0, -2, 10, 5, 2].map(M::from);
        assert_eq!(
            interpolation.calculate_many(&xs),
            [38, 3, 3, 123, 38, 11].map(M::from)
        );
        assert!(interpolation.calculate_many(&[]).is_empty());
    }

    #[test]
    fn consecutive_constructors_and_repeated_batches() {
        type M = ModInt7;
        fn generic_scalar<T: IntegerSemiRing + Copy, M: BaseModInt<T>>(values: Vec<M>, x: M) -> M {
            Interpolation::new(values).calculate(x)
        }
        let values = [3, 6, 11].map(M::from).to_vec();
        assert_eq!(generic_scalar(values.clone(), M::from(10)), M::from(123));
        let inv_fact = inverse_factorials::<u32, M>(10);
        for interpolation in [
            Interpolation::new(values.clone()),
            Interpolation::with_inverse_factorials(values, &inv_fact),
        ] {
            assert_eq!(interpolation.calculate(M::from(10)), M::from(123));
            for xs in [vec![10, 1, 0, -2, 10], vec![4, 2, 3, 0]] {
                let xs: Vec<_> = xs.into_iter().map(M::from).collect();
                let expected: Vec<_> = xs
                    .iter()
                    .map(|&x| x * x + M::from(2) * x + M::from(3))
                    .collect();
                assert_eq!(interpolation.calculate_many(&xs), expected);
            }
            assert_eq!(interpolation.calculate(M::from(10)), M::from(123));
        }
    }

    #[test]
    fn empty_and_constant_polynomials() {
        type M = ModIntF;
        let xs = [M::zero(), M::from(7), M::from(-1)];
        for interpolation in [Interpolation::new(vec![]), Interpolation::from_points(&[])] {
            assert_eq!(interpolation.calculate(M::from(123)), M::zero());
            assert_eq!(interpolation.calculate_many(&xs), vec![M::zero(); 3]);
            assert!(interpolation.calculate_many(&[]).is_empty());
        }
        for n in [1usize, 3, 129] {
            for c in [M::zero(), M::from(19)] {
                let points: Vec<_> = (0..n).map(|i| (M::from(i + 100), c)).collect();
                for interpolation in [
                    Interpolation::from_points(&points),
                    Interpolation::new(vec![c; n]),
                ] {
                    assert_eq!(interpolation.degree(), n - 1);
                    assert_eq!(interpolation.calculate(M::from(55)), c);
                    assert_eq!(interpolation.calculate_many(&xs), vec![c; 3]);
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "distinct")]
    fn duplicate_sample_coordinates_are_rejected() {
        type M = ModIntF;
        Interpolation::from_points(&[
            (M::one(), M::from(2)),
            (M::new(M::module() + 1), M::from(2)),
        ]);
    }

    fn check_polynomials<T: IntegerSemiRing + Copy + Into<u64>, M: BaseModInt<T> + Debug>() {
        let mut rng = ChaCha8Rng::seed_from_u64(713);
        for n in [1usize, 2, 3, 31, 60, 61, 64, 65, 127, 128, 129, 255, 257] {
            let coefficients: Vec<M> = (0..n)
                .map(|_| M::from(rng.gen_range(0..1_000_000usize)))
                .collect();
            let evaluate = |x: M| coefficients.iter().rev().fold(M::zero(), |y, &c| y * x + c);
            let mut points: Vec<_> = (0..n)
                .map(|i| {
                    let x = M::from(i * 137 + 23);
                    (x, evaluate(x))
                })
                .collect();
            points.shuffle(&mut rng);
            let arbitrary = Interpolation::from_points(&points);
            let consecutive = Interpolation::new((0..n).map(|i| evaluate(M::from(i))).collect());
            for interpolation in [&arbitrary, &consecutive] {
                assert_eq!(
                    interpolation.calculate(M::from(200_000usize)),
                    evaluate(M::from(200_000usize))
                );
                for m in [0usize, 1, 17, 33, 65, 131, n, n + 1, 513] {
                    let xs: Vec<_> = (0..m)
                        .map(|i| {
                            if i % 3 == 0 {
                                points[i % n].0
                            } else {
                                M::from(rng.gen_range(0..1_000_000usize))
                            }
                        })
                        .collect();
                    let expected: Vec<_> = xs.iter().copied().map(evaluate).collect();
                    assert_eq!(interpolation.calculate_many(&xs), expected, "n={n}, m={m}");
                }
            }
        }
    }

    #[test]
    fn random_polynomials_fft_modulus() {
        check_polynomials::<u32, ModIntF>();
    }

    #[test]
    fn random_polynomials_other_moduli() {
        check_polynomials::<u32, ModInt7>();
        check_polynomials::<u32, ModInt9>();
    }

    #[test]
    fn wide_modulus() {
        crate::value!(Wide: u64 = (1u64 << 61) - 1);
        type M = ModInt64<Wide>;
        let evaluate = |x: M| x.power(64usize) + M::from(3usize) * x + M::from(9usize);
        let points: Vec<_> = (0..65usize)
            .map(|i| {
                let x = M::from(i) - M::from(1000usize);
                (x, evaluate(x))
            })
            .collect();
        let interpolation = Interpolation::from_points(&points);
        let xs: Vec<_> = (0..129usize).map(|i| -M::from(i)).collect();
        assert_eq!(
            interpolation.calculate_many(&xs),
            xs.iter().copied().map(evaluate).collect::<Vec<_>>()
        );
    }

    #[test]
    fn points_cover_entire_small_field() {
        crate::value!(Small: u32 = 17);
        type M = ModInt<Small>;
        let evaluate = |x: M| x.power(16usize) + x * M::from(3usize);
        let xs: Vec<_> = (0..17usize).map(M::from).collect();
        let ys: Vec<_> = xs.iter().copied().map(evaluate).collect();
        let points: Vec<_> = xs.iter().copied().zip(ys.iter().copied()).collect();
        for interpolation in [
            Interpolation::from_points(&points),
            Interpolation::new(ys.clone()),
        ] {
            assert_eq!(interpolation.calculate_many(&xs), ys);
        }
    }

    fn large_batch<M: BaseModInt<u32> + Debug>() {
        use std::time::Instant;
        let n = 100_000usize;
        let evaluate = |x: M| x.power(n - 1) + M::from(3usize) * x.power(17usize) + M::from(7usize);
        let points: Vec<_> = (0..n)
            .map(|i| {
                let x = M::from(i * 2 + 1);
                (x, evaluate(x))
            })
            .collect();
        let xs: Vec<_> = (0..n).map(|i| M::from(i * 3 + 7)).collect();
        let expected: Vec<_> = xs.iter().copied().map(evaluate).collect();
        let start = Instant::now();
        let interpolation = Interpolation::from_points(&points);
        let construction = start.elapsed();
        let start = Instant::now();
        assert_eq!(interpolation.calculate_many(&xs), expected);
        let batch = start.elapsed();
        eprintln!(
            "modulus={}, samples={n}, queries={n}: construction={construction:?}, batch={batch:?}",
            M::module()
        );
        let values: Vec<_> = (0..n).map(|i| evaluate(M::from(i))).collect();
        let consecutive = Interpolation::new(values.clone());
        let small_xs = [M::from(n + 1), M::from(n + 2)];
        let start = Instant::now();
        assert_eq!(
            consecutive.calculate_many(&small_xs),
            small_xs.map(evaluate)
        );
        let small_batch = start.elapsed();

        // A fresh interpolator measures reconstruction on the first large batch.
        let consecutive = Interpolation::new(values);
        let start = Instant::now();
        assert_eq!(consecutive.calculate_many(&xs), expected);
        let first_batch = start.elapsed();
        let mut reversed_xs = xs;
        reversed_xs.reverse();
        let mut reversed_expected = expected;
        reversed_expected.reverse();
        let start = Instant::now();
        assert_eq!(consecutive.calculate_many(&reversed_xs), reversed_expected);
        let repeated_batch = start.elapsed();
        eprintln!("consecutive: two_queries={small_batch:?}, first_batch={first_batch:?}, repeated_batch={repeated_batch:?}");
    }

    #[test]
    #[ignore = "large performance check; run in release mode"]
    fn large_batch_fft_modulus() {
        large_batch::<ModIntF>();
    }

    #[test]
    #[ignore = "large performance check; run in release mode"]
    fn large_batch_other_modulus() {
        large_batch::<ModInt7>();
    }
}
