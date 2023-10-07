pub trait IterFindCount<T: PartialEq>: Iterator<Item = T> + Sized {
    fn find_eq(mut self, item: T) -> Option<usize> {
        self.position(|r| r == item)
    }
    fn count_eq(self, item: &T) -> usize {
        self.filter(|r| r == item).count()
    }
}

impl<T: PartialEq, I: Iterator<Item = T>> IterFindCount<T> for I {}
