pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T: Ord> NextPermutation for [T] {
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 1 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        true
    }
}
