use crate::collections::slice_ext::next_permutation::NextPermutation;

pub trait Permutation: NextPermutation {
    fn inv(&self) -> Vec<usize>;
    fn mul(&self, other: &Self) -> Vec<usize>;
}

impl Permutation for [usize] {
    fn inv(&self) -> Vec<usize> {
        let mut inv = vec![self.len(); self.len()];
        for i in 0..self.len() {
            inv[self[i]] = i;
        }
        debug_assert!(inv.iter().all(|&x| x < self.len()));
        inv
    }

    fn mul(&self, other: &Self) -> Vec<usize> {
        assert_eq!(self.len(), other.len());
        let mut res = vec![0; self.len()];
        for i in 0..self.len() {
            res[i] = self[other[i]];
        }
        res
    }
}
