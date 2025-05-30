use std::iter::{Copied, Skip, Zip};
use std::slice::Iter;

pub trait ConsecutiveIter<T> {
    fn consecutive_iter(&self) -> Zip<Iter<T>, Skip<Iter<T>>>;
}

impl<T> ConsecutiveIter<T> for [T] {
    fn consecutive_iter(&self) -> Zip<Iter<T>, Skip<Iter<T>>> {
        self.iter().zip(self.iter().skip(1))
    }
}

pub trait ConsecutiveIterCopy<T: Copy> {
    fn consecutive_iter_copy(&self) -> Zip<Copied<Iter<T>>, Copied<Skip<Iter<T>>>>;
}

impl<T: Copy> ConsecutiveIterCopy<T> for [T] {
    fn consecutive_iter_copy(&self) -> Zip<Copied<Iter<T>>, Copied<Skip<Iter<T>>>> {
        self.iter().copied().zip(self.iter().skip(1).copied())
    }
}
