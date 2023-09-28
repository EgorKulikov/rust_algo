use std::ops::Range;

pub trait Indices {
    fn indices(&self) -> Range<usize>;
}

impl<T> Indices for [T] {
    fn indices(&self) -> Range<usize> {
        0..self.len()
    }
}
