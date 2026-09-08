//! Polynomial arithmetic and multipoint evaluation over modular coefficients.
//!
//! Coefficients are in ascending order: `[a, b, c]` represents `a + b*x + c*x^2`.
//! An empty slice represents the zero polynomial; trailing zero coefficients
//! are also accepted.
//!
//! Fast multiplication supports moduli up to `u32::MAX`. Suitable prime moduli
//! use a direct FFT; other moduli or transform lengths use three FFTs with CRT
//! reconstruction through [`FFT`]. The CRT transform limit is 2^21; exceeding
//! it panics. Wider moduli continue to use quadratic multiplication.
//!
//! ```
//! use algo_lib::numbers::mod_int::ModIntF as M;
//! use algo_lib::numbers::polynomial::{evaluate, PolynomialOps, ProductTree};
//!
//! let mut ops = PolynomialOps::new();
//! let polynomial = ops.multiply(&[M::new(1), M::new(2)], &[M::new(3), M::new(4)]);
//! assert_eq!(polynomial, vec![M::new(3), M::new(10), M::new(8)]);
//! assert_eq!(evaluate(&polynomial, M::new(2)), M::new(55));
//!
//! let xs = [M::new(0), M::new(2), M::new(2)];
//! let tree = ProductTree::new(&xs, &mut ops);
//! assert_eq!(tree.evaluate(&polynomial, &mut ops), vec![M::new(3), M::new(55), M::new(55)]);
//! ```

use crate::numbers::mod_int::convolution::modulus;
use crate::numbers::mod_int::fft::FFT;
use crate::numbers::mod_int::BaseModInt;
use std::marker::PhantomData;

const DIRECT: usize = 60;

/// Polynomial arithmetic with reusable FFT buffers.
///
/// See the [module documentation](self) for coefficient order and FFT limits.
pub struct PolynomialOps<T, M: BaseModInt<T>> {
    fft: Option<FFT<M, T>>,
}

impl<T, M: BaseModInt<T>> Default for PolynomialOps<T, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, M: BaseModInt<T>> PolynomialOps<T, M> {
    /// Creates an arithmetic context; FFT buffers are allocated when needed.
    pub fn new() -> Self {
        Self { fft: None }
    }
}

impl<T: Into<u64>, M: BaseModInt<T>> PolynomialOps<T, M> {
    /// Multiplies two coefficient slices, returning an empty vector if either
    /// input is empty. Otherwise returns `a.len() + b.len() - 1` coefficients,
    /// including any trailing zeros.
    ///
    /// Takes O(k log k) time with FFT multiplication, where k is the output
    /// length. Small inputs and wide moduli use quadratic multiplication.
    pub fn multiply(&mut self, a: &[M], b: &[M]) -> Vec<M> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        if a.len().min(b.len()) <= DIRECT || modulus::<T, M>() > u32::MAX as u64 {
            let mut result = vec![M::zero(); a.len() + b.len() - 1];
            for (i, &x) in a.iter().enumerate() {
                for (j, &y) in b.iter().enumerate() {
                    result[i + j] += x * y;
                }
            }
            return result;
        }
        self.fft.get_or_insert_with(FFT::new).multiply(a, b)
    }

    // Newton iteration: g <- g * (2 - f*g) modulo x^len. Tree divisors
    // are monic, so their reversals have constant coefficient one.
    fn inverse(&mut self, f: &[M], len: usize) -> Vec<M> {
        debug_assert!(f[0] == M::one());
        let mut g = vec![M::one()];
        while g.len() < len {
            let size = (2 * g.len()).min(len);
            let mut correction = self.multiply(&f[..f.len().min(size)], &g);
            correction.resize(size, M::zero());
            for c in &mut correction {
                *c = -*c;
            }
            correction[0] += M::one() + M::one();
            g = self.multiply(&g, &correction);
            g.truncate(size);
        }
        g
    }

    fn remainder(&mut self, a: &[M], b: &[M]) -> Vec<M> {
        debug_assert!(b.last() == Some(&M::one()));
        if a.len() < b.len() {
            return a.to_vec();
        }
        let degree = b.len() - 1;
        if degree == 0 {
            return Vec::new();
        }
        let quotient_len = a.len() - degree;
        let mut result;
        if degree.min(quotient_len) <= DIRECT {
            result = a.to_vec();
            for i in (degree..a.len()).rev() {
                let c = result[i];
                for j in 0..degree {
                    result[i - degree + j] -= c * b[j];
                }
            }
            result.truncate(degree);
        } else {
            let reversed_b: Vec<_> = b.iter().rev().take(quotient_len).copied().collect();
            let inverse = self.inverse(&reversed_b, quotient_len);
            let reversed_a: Vec<_> = a.iter().rev().take(quotient_len).copied().collect();
            let mut quotient = self.multiply(&reversed_a, &inverse);
            quotient.truncate(quotient_len);
            quotient.reverse();
            let product = self.multiply(&quotient, b);
            result = (0..degree).map(|i| a[i] - product[i]).collect();
        }
        trim(&mut result);
        result
    }
}

fn trim<T, M: BaseModInt<T>>(polynomial: &mut Vec<M>) {
    while polynomial.last() == Some(&M::zero()) {
        polynomial.pop();
    }
}

/// Evaluates a coefficient slice at `x` in O(n) time and O(1) extra space.
pub fn evaluate<T, M: BaseModInt<T>>(polynomial: &[M], x: M) -> M {
    polynomial.iter().rev().fold(M::zero(), |y, &c| y * x + c)
}

/// A reusable product tree for evaluating polynomials at a fixed set of points.
///
/// Borrows the query coordinates, which may repeat. Construction takes
/// O(m log^2 m) time and O(m log m) space with FFT multiplication.
pub struct ProductTree<'a, T, M: BaseModInt<T>> {
    xs: &'a [M],
    size: usize,
    products: Vec<Vec<M>>,
    phantom: PhantomData<T>,
}

impl<'a, T: Into<u64>, M: BaseModInt<T>> ProductTree<'a, T, M> {
    /// Builds the tree for `xs` using the supplied arithmetic context.
    ///
    /// # Panics
    /// Panics if `xs` is empty or an FFT transform exceeds the module's limits.
    pub fn new(xs: &'a [M], ops: &mut PolynomialOps<T, M>) -> Self {
        assert!(!xs.is_empty());
        let size = xs.len().next_power_of_two();
        let mut products = vec![Vec::new(); size * 2];
        for i in 0..size {
            products[size + i] = if i < xs.len() {
                vec![-xs[i], M::one()]
            } else {
                vec![M::one()]
            };
        }
        for i in (1..size).rev() {
            products[i] = ops.multiply(&products[2 * i], &products[2 * i + 1]);
        }
        Self {
            xs,
            size,
            products,
            phantom: PhantomData,
        }
    }

    /// Returns the coefficients of `product(x - xs[i])` in ascending order.
    pub fn product(&self) -> &[M] {
        &self.products[1]
    }

    /// Evaluates a coefficient slice at every query point in input order.
    ///
    /// Takes O((n + m) log^2(n + m)) time with FFT multiplication for n
    /// coefficients and m query points. The same tree can evaluate multiple
    /// polynomials without rebuilding its products.
    pub fn evaluate(&self, polynomial: &[M], ops: &mut PolynomialOps<T, M>) -> Vec<M> {
        let mut result = vec![M::zero(); self.xs.len()];
        let remainder = ops.remainder(polynomial, self.product());
        self.evaluate_node(1, 0, self.size, &remainder, ops, &mut result);
        result
    }

    fn evaluate_node(
        &self,
        node: usize,
        left: usize,
        right: usize,
        polynomial: &[M],
        ops: &mut PolynomialOps<T, M>,
        result: &mut [M],
    ) {
        if left >= self.xs.len() {
            return;
        }
        if polynomial.len() <= 32 || right - left <= 32 {
            for i in left..right.min(self.xs.len()) {
                result[i] = evaluate(polynomial, self.xs[i]);
            }
            return;
        }
        let mid = (left + right) / 2;
        let remainder = ops.remainder(polynomial, &self.products[node * 2]);
        self.evaluate_node(node * 2, left, mid, &remainder, ops, result);
        if mid < self.xs.len() {
            let remainder = ops.remainder(polynomial, &self.products[node * 2 + 1]);
            self.evaluate_node(node * 2 + 1, mid, right, &remainder, ops, result);
        }
    }

    pub(crate) fn interpolate(&self, weights: &[M], ops: &mut PolynomialOps<T, M>) -> Vec<M> {
        self.interpolate_node(1, weights, ops)
    }

    fn interpolate_node(
        &self,
        node: usize,
        weights: &[M],
        ops: &mut PolynomialOps<T, M>,
    ) -> Vec<M> {
        if node >= self.size {
            let i = node - self.size;
            return if i < weights.len() && weights[i] != M::zero() {
                vec![weights[i]]
            } else {
                Vec::new()
            };
        }
        let left = self.interpolate_node(node * 2, weights, ops);
        let right = self.interpolate_node(node * 2 + 1, weights, ops);
        let mut result = ops.multiply(&left, &self.products[node * 2 + 1]);
        let other = ops.multiply(&right, &self.products[node * 2]);
        result.resize(result.len().max(other.len()), M::zero());
        for (x, y) in result.iter_mut().zip(other) {
            *x += y;
        }
        trim(&mut result);
        result
    }
}
