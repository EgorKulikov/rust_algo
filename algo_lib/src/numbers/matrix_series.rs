use crate::numbers::matrix::Matrix;
use crate::numbers::num_traits::algebra::SemiRing;
use crate::numbers::num_traits::bit_ops::BitOps;
use std::mem::swap;

pub struct MatrixPowers<T> {
    powers: Vec<Matrix<T>>,
}

impl<T: Copy + SemiRing> MatrixPowers<T> {
    pub fn new(mut base: Matrix<T>, max_power_log: usize) -> Self {
        let mut powers = Vec::with_capacity(max_power_log);
        for _ in 0..max_power_log {
            powers.push(base.clone());
            base = base.mult(&base);
        }
        Self { powers }
    }

    pub fn calculate(&self, mut matrix: Matrix<T>, power: usize) -> Matrix<T> {
        let mut temp = Matrix::zero(matrix.d1(), matrix.d2());
        for i in 0..self.powers.len() {
            if power.is_set(i) {
                Matrix::do_mult(&mut temp, &matrix, &self.powers[i]);
                swap(&mut matrix, &mut temp);
            }
        }
        matrix
    }
}
