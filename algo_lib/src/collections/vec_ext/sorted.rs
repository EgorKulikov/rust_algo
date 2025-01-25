pub trait Sorted<T> {
    fn sorted(self) -> Self
    where
        T: Ord;
    fn reversed(self) -> Self;
    fn sorted_by_key<R: Ord>(self, f: impl FnMut(&T) -> R) -> Self;
}

impl<T> Sorted<T> for Vec<T> {
    fn sorted(mut self) -> Self
    where
        T: Ord,
    {
        self.sort();
        self
    }
    fn reversed(mut self) -> Self {
        self.reverse();
        self
    }
    fn sorted_by_key<R: Ord>(mut self, f: impl FnMut(&T) -> R) -> Self {
        self.sort_by_key(f);
        self
    }
}
