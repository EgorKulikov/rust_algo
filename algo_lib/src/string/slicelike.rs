use std::ops::{Index, IndexMut};

pub trait Slicelike: Index<usize>
where
    Self::Output: Sized,
{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Slicelike for [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

pub trait SlicelikeMut: Slicelike + IndexMut<usize>
where
    Self::Output: Sized,
{
}

impl<U> SlicelikeMut for U
where
    U: Slicelike + IndexMut<usize>,
    U::Output: Sized,
{
}
