pub trait Sorted {
    fn sorted(self) -> Self;
}

impl<T: Ord> Sorted for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
