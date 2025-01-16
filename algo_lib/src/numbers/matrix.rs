use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::num_traits::algebra::{One, SemiRing, Zero};
use crate::transparent_wrapper;
use std::ops::{Deref, DerefMut};

transparent_wrapper!(Matrix<T> = Arr2d<T>, derive Clone, Eq, PartialEq);

impl<T: Zero + One + Clone> Matrix<T> {
    pub fn zero(n: usize, m: usize) -> Self {
        Self(Arr2d::new(n, m, T::zero()))
    }

    pub fn ident(n: usize) -> Self {
        Self(Arr2d::with_gen(n, n, |i, j| {
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
        Self(Arr2d::with_gen(arr.len(), 1, |i, _| arr[i]))
    }

    pub fn row(arr: &[T]) -> Self {
        Self(Arr2d::with_gen(1, arr.len(), |_, i| arr[i]))
    }

    pub fn new(arr: &[&[T]]) -> Self {
        for a in arr {
            assert_eq!(a.len(), arr[0].len());
        }
        Self(Arr2d::with_gen(arr.len(), arr[0].len(), |i, j| arr[i][j]))
    }
}

impl<T: SemiRing + Copy> Matrix<T> {
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

    pub fn sum_power(&self, n: usize) -> Self {
        assert_eq!(self.d1(), self.d2());
        let mut res = Self::zero(self.d1(), self.d2());
        let mut temp = Self::zero(self.d1(), self.d2());
        let mut pw = Self::ident(self.d1());
        let mut temp_pw = Self::ident(self.d1());
        Self::do_sum_power(self, &mut res, &mut temp, &mut pw, &mut temp_pw, n);
        res
    }

    fn do_sum_power(
        a: &Matrix<T>,
        res: &mut Matrix<T>,
        temp: &mut Matrix<T>,
        pw: &mut Matrix<T>,
        temp_pw: &mut Matrix<T>,
        n: usize,
    ) {
        if n != 0 {
            if (n & 1) == 0 {
                Self::do_sum_power(a, temp, res, temp_pw, pw, n >> 1);
                pw.do_mult(temp_pw, temp_pw);
                for i in 0..pw.d1() {
                    temp_pw[(i, i)] += T::one();
                }
                res.do_mult(temp, temp_pw);
            } else {
                Self::do_sum_power(a, res, temp, temp_pw, pw, n - 1);
                pw.do_mult(temp_pw, a);
                res.add_to(temp_pw);
            }
        }
    }
}

impl<T> From<Arr2d<T>> for Matrix<T> {
    fn from(a: Arr2d<T>) -> Self {
        Self(a)
    }
}
