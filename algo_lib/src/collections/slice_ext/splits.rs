pub trait Split<T> {
    fn two_mut(&mut self, i: usize, j: usize) -> (&mut T, &mut T);
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
}
