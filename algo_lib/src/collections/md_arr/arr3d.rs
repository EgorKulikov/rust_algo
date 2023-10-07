use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::ops::{Index, IndexMut};
use std::vec::IntoIter;

#[derive(Clone, Eq, PartialEq)]
pub struct Arr3d<T> {
    d1: usize,
    d2: usize,
    d3: usize,
    data: Vec<T>,
}

impl<T: Clone> Arr3d<T> {
    pub fn new(d1: usize, d2: usize, d3: usize, value: T) -> Self {
        Self {
            d1,
            d2,
            d3,
            data: vec![value; d1 * d2 * d3],
        }
    }
}

impl<T> Arr3d<T> {
    pub fn generate<F>(d1: usize, d2: usize, d3: usize, mut gen: F) -> Self
    where
        F: FnMut(usize, usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(d1 * d2 * d3);
        for i in 0usize..d1 {
            for j in 0usize..d2 {
                for k in 0..d3 {
                    data.push(gen(i, j, k));
                }
            }
        }
        Self { d1, d2, d3, data }
    }

    pub fn d1(&self) -> usize {
        self.d1
    }

    pub fn d2(&self) -> usize {
        self.d2
    }

    pub fn d3(&self) -> usize {
        self.d3
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

impl<T> IntoIterator for Arr3d<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Index<(usize, usize, usize)> for Arr3d<T> {
    type Output = T;

    fn index(&self, (a1, a2, a3): (usize, usize, usize)) -> &Self::Output {
        assert!(a1 < self.d1);
        assert!(a2 < self.d2);
        assert!(a3 < self.d3);
        &self.data[(a1 * self.d2 + a2) * self.d3 + a3]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for Arr3d<T> {
    fn index_mut(&mut self, (a1, a2, a3): (usize, usize, usize)) -> &mut Self::Output {
        assert!(a1 < self.d1);
        assert!(a2 < self.d2);
        assert!(a3 < self.d3);
        &mut self.data[(a1 * self.d2 + a2) * self.d3 + a3]
    }
}

impl<T: Writable> Writable for Arr3d<T> {
    fn write(&self, output: &mut Output) {
        let mut at = 0usize;
        for i in 0..self.d1 {
            if i != 0 {
                output.put(b'\n');
            }
            for j in 0..self.d2 {
                if j != 0 {
                    output.put(b'\n');
                }
                for k in 0..self.d3 {
                    if k != 0 {
                        output.put(b' ');
                    }
                    self.data[at].write(output);
                    at += 1;
                }
            }
        }
    }
}

pub trait Arr3dRead {
    fn read_3d_table<T: Readable>(&mut self, d1: usize, d2: usize, d3: usize) -> Arr3d<T>;
}

impl Arr3dRead for Input<'_> {
    fn read_3d_table<T: Readable>(&mut self, d1: usize, d2: usize, d3: usize) -> Arr3d<T> {
        Arr3d::generate(d1, d2, d3, |_, _, _| self.read())
    }
}

impl<T: Readable> Readable for Arr3d<T> {
    fn read(input: &mut Input) -> Self {
        let d1 = input.read();
        let d2 = input.read();
        let d3 = input.read();
        input.read_3d_table(d1, d2, d3)
    }
}

impl<T: Clone> Arr3d<T> {
    pub fn fill(&mut self, elem: T) {
        self.data.legacy_fill(elem);
    }
}
