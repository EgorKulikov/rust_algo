use std::ops::{Index, IndexMut};

pub struct Backward<'a, T>(&'a [T]);

impl<T> Index<usize> for Backward<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(self.0.len() - index - 1)
    }
}

pub struct BackwardMut<'a, T>(&'a mut [T]);

impl<T> Index<usize> for BackwardMut<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(self.0.len() - index - 1)
    }
}

impl<T> IndexMut<usize> for BackwardMut<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(self.0.len() - index - 1)
    }
}

pub trait BackwardSlice<T> {
    fn backward<'a, 'b: 'a>(&'b self) -> Backward<'a, T>;
    fn backward_mut<'a, 'b: 'a>(&'b mut self) -> BackwardMut<'a, T>;
}

impl<T> BackwardSlice<T> for [T] {
    fn backward<'a, 'b: 'a>(&'b self) -> Backward<'a, T> {
        Backward(self)
    }

    fn backward_mut<'a, 'b: 'a>(&'b mut self) -> BackwardMut<'a, T> {
        BackwardMut(self)
    }
}
