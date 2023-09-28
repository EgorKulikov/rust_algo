use std::ops::{Index, IndexMut};

pub struct Reversed<'a, T>(&'a [T]);

impl<T> Index<usize> for Reversed<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(self.0.len() - index - 1)
    }
}

pub struct ReversedMut<'a, T>(&'a mut [T]);

impl<T> Index<usize> for ReversedMut<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(self.0.len() - index - 1)
    }
}

impl<T> IndexMut<usize> for ReversedMut<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(self.0.len() - index - 1)
    }
}

pub trait ReversedSlice<T> {
    fn rev(&self) -> Reversed<T>;
    fn rev_mut(&mut self) -> ReversedMut<T>;
}

impl<T> ReversedSlice<T> for [T] {
    fn rev(&self) -> Reversed<T> {
        Reversed(self)
    }

    fn rev_mut(&mut self) -> ReversedMut<T> {
        ReversedMut(self)
    }
}
