use std::ops::Index;

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

pub struct Concat<'s, T> {
    first: &'s [T],
    second: &'s [T],
}

impl<T> Index<usize> for Concat<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.first.len() {
            &self.first[index]
        } else {
            &self.second[index - self.first.len()]
        }
    }
}

impl<T> Slicelike for Concat<'_, T> {
    fn len(&self) -> usize {
        self.first.len() + self.second.len()
    }
}

pub fn chain<'a, T>(first: &'a [T], second: &'a [T]) -> Concat<'a, T> {
    Concat { first, second }
}
