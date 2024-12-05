pub trait Split<T> {
    fn two_mut(&mut self, i: usize, j: usize) -> (&mut T, &mut T);
    fn three_mut(&mut self, i: usize, j: usize, k: usize) -> (&mut T, &mut T, &mut T);
}

impl<T> Split<T> for [T] {
    fn two_mut(&mut self, i: usize, j: usize) -> (&mut T, &mut T) {
        assert_ne!(i, j);
        if i < j {
            let (left, right) = self.split_at_mut(j);
            (&mut left[i], &mut right[0])
        } else {
            let (left, right) = self.split_at_mut(i);
            (&mut right[0], &mut left[j])
        }
    }

    fn three_mut(&mut self, i: usize, j: usize, k: usize) -> (&mut T, &mut T, &mut T) {
        assert_ne!(i, j);
        assert_ne!(j, k);
        assert_ne!(i, k);
        if i > j && i > k {
            let (left, right) = self.split_at_mut(i);
            let (r_j, r_k) = left.two_mut(j, k);
            (&mut right[0], r_j, r_k)
        } else if j > i && j > k {
            let (left, right) = self.split_at_mut(j);
            let (r_i, r_k) = left.two_mut(i, k);
            (r_i, &mut right[0], r_k)
        } else {
            let (left, right) = self.split_at_mut(k);
            let (r_i, r_j) = left.two_mut(i, j);
            (r_i, r_j, &mut right[0])
        }
    }
}
