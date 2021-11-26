use crate::io::output::{Output, Writable};
use std::ops::{Index, IndexMut};

pub struct Arr2d<T> {
    d1: usize,
    d2: usize,
    data: Vec<T>,
}

impl<T: Copy> Arr2d<T> {
    pub fn new(d1: usize, d2: usize, value: T) -> Self {
        Self {
            d1,
            d2,
            data: vec![value; d1 * d2],
        }
    }
}

impl<T> Arr2d<T> {
    pub fn generate<F>(d1: usize, d2: usize, mut gen: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(d1 * d2);
        for i in 0usize..d1 {
            for j in 0usize..d2 {
                data.push(gen(i, j));
            }
        }
        Self { d1, d2, data }
    }

    pub fn d1(&self) -> usize {
        self.d1
    }

    pub fn d2(&self) -> usize {
        self.d2
    }
}

impl<T> Index<(usize, usize)> for Arr2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.d2 * index.0 + index.1]
    }
}

impl<T> Index<usize> for Arr2d<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Arr2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.data[self.d2 * index.0 + index.1]
    }
}

impl<T> IndexMut<usize> for Arr2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

impl<T: Writable> Writable for Arr2d<T> {
    fn write(&self, output: &mut Output) {
        let mut at = 0usize;
        for i in 0usize..self.d1 {
            if i != 0 {
                output.put(b'\n');
            }
            for j in 0usize..self.d2 {
                if j != 0 {
                    output.put(b' ');
                }
                self.data[at].write(output);
                at += 1;
            }
        }
    }
}
