pub trait IterCollect<T>: Iterator<Item = T> + Sized {
    fn collect_vec(self) -> Vec<T> {
        self.collect()
    }
}

impl<T, I: Iterator<Item = T> + Sized> IterCollect<T> for I {}
