use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::num_traits::add_sub::Addable;
use crate::numbers::num_traits::mul_div_rem::Multable;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Matrix<T>(Arr2d<T>);

impl<T: ZeroOne + Clone> Matrix<T> {
    pub fn zero(n: usize, m: usize) -> Self {
        Self(Arr2d::new(n, m, T::zero()))
    }

    pub fn ident(n: usize) -> Self {
        Self(Arr2d::generate(n, n, |i, j| {
            if i == j {
                T::one()
            } else {
                T::zero()
            }
        }))
    }
}

impl<T: Copy> Matrix<T> {
    pub fn column(arr: &[T]) -> Self {
        Self(Arr2d::generate(arr.len(), 1, |i, _| arr[i]))
    }

    pub fn row(arr: &[T]) -> Self {
        Self(Arr2d::generate(1, arr.len(), |_, i| arr[i]))
    }

    pub fn new(arr: &[&[T]]) -> Self {
        for a in arr {
            assert_eq!(a.len(), arr[0].len());
        }
        Self(Arr2d::generate(arr.len(), arr[0].len(), |i, j| arr[i][j]))
    }
}

impl<T: ZeroOne + Addable + Multable + Copy> Matrix<T> {
    pub fn mult(&self, a: &Matrix<T>) -> Self {
        let mut res = Self::zero(self.d1(), a.d2());
        Self::do_mult(&mut res, self, a);
        res
    }

    pub fn do_mult(&mut self, a: &Matrix<T>, b: &Matrix<T>) {
        assert_eq!(self.d1(), a.d1());
        assert_eq!(a.d2(), b.d1());
        assert_eq!(b.d2(), self.d2());
        self.fill(T::zero());
        for i in 0..self.d1() {
            for j in 0..a.d2() {
                for k in 0..b.d2() {
                    self[(i, k)] += a[(i, j)] * b[(j, k)];
                }
            }
        }
    }

    pub fn add(&mut self, a: &Matrix<T>, b: &Matrix<T>) {
        assert_eq!(self.d1(), a.d1());
        assert_eq!(self.d2(), a.d2());
        assert_eq!(self.d1(), b.d1());
        assert_eq!(self.d2(), b.d2());
        for i in 0..self.d1() {
            for j in 0..self.d2() {
                self[(i, j)] = a[(i, j)] + b[(i, j)];
            }
        }
    }

    pub fn add_to(&mut self, a: &Matrix<T>) {
        assert_eq!(self.d1(), a.d1());
        assert_eq!(self.d2(), a.d2());
        for i in 0..self.d1() {
            for j in 0..self.d2() {
                self[(i, j)] += a[(i, j)];
            }
        }
    }

    pub fn power(&self, n: usize) -> Matrix<T> {
        assert_eq!(self.d1(), self.d2());
        let mut res = Self::ident(self.d1());
        let mut temp = Self::ident(self.d1());
        Self::do_power(self, &mut res, &mut temp, n);
        res
    }

    fn do_power(a: &Matrix<T>, res: &mut Matrix<T>, temp: &mut Matrix<T>, n: usize) {
        if n != 0 {
            if (n & 1) == 0 {
                Self::do_power(a, temp, res, n >> 1);
                res.do_mult(temp, temp);
            } else {
                Self::do_power(a, temp, res, n - 1);
                res.do_mult(temp, a);
            }
        }
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Arr2d<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Arr2d<T>> for Matrix<T> {
    fn from(a: Arr2d<T>) -> Self {
        Self(a)
    }
}
