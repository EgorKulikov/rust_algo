use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

#[derive(Clone, Eq, PartialEq, Default, Debug, Hash)]
pub struct Arr4d<T> {
    d1: usize,
    d2: usize,
    d3: usize,
    d4: usize,
    data: Vec<T>,
}

impl<T: Clone> Arr4d<T> {
    pub fn new(d1: usize, d2: usize, d3: usize, d4: usize, value: T) -> Self {
        Self {
            d1,
            d2,
            d3,
            d4,
            data: vec![value; d1 * d2 * d3 * d4],
        }
    }
}

impl<T> Arr4d<T> {
    pub fn with_gen<F>(d1: usize, d2: usize, d3: usize, d4: usize, mut g: F) -> Self
    where
        F: FnMut(usize, usize, usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(d1 * d2 * d3 * d4);
        for i in 0..d1 {
            for j in 0..d2 {
                for k in 0..d3 {
                    for l in 0..d4 {
                        data.push(g(i, j, k, l));
                    }
                }
            }
        }
        Self {
            d1,
            d2,
            d3,
            d4,
            data,
        }
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

    pub fn d4(&self) -> usize {
        self.d4
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T> Index<(usize, usize, usize, usize)> for Arr4d<T> {
    type Output = T;

    fn index(&self, (a1, a2, a3, a4): (usize, usize, usize, usize)) -> &Self::Output {
        assert!(a2 < self.d2);
        assert!(a3 < self.d3);
        assert!(a4 < self.d4);
        &self.data[((a1 * self.d2 + a2) * self.d3 + a3) * self.d4 + a4]
    }
}

impl<T> IndexMut<(usize, usize, usize, usize)> for Arr4d<T> {
    fn index_mut(&mut self, (a1, a2, a3, a4): (usize, usize, usize, usize)) -> &mut Self::Output {
        assert!(a2 < self.d2);
        assert!(a3 < self.d3);
        assert!(a4 < self.d4);
        &mut self.data[((a1 * self.d2 + a2) * self.d3 + a3) * self.d4 + a4]
    }
}

impl<T: Writable> Writable for Arr4d<T> {
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
                        output.put(b'\n');
                    }
                    for l in 0..self.d4 {
                        if l != 0 {
                            output.put(output.separator());
                        }
                        self.data[at].write(output);
                        at += 1;
                    }
                }
            }
        }
    }
}

pub trait Arr4dRead {
    fn read_4d_table<T: Readable>(
        &mut self,
        d1: usize,
        d2: usize,
        d3: usize,
        d4: usize,
    ) -> Arr4d<T>;
}

impl Arr4dRead for Input {
    fn read_4d_table<T: Readable>(
        &mut self,
        d1: usize,
        d2: usize,
        d3: usize,
        d4: usize,
    ) -> Arr4d<T> {
        Arr4d::with_gen(d1, d2, d3, d4, |_, _, _, _| self.read())
    }
}

impl<T: Readable> Readable for Arr4d<T> {
    fn read(input: &mut Input) -> Self {
        let d1 = input.read();
        let d2 = input.read();
        let d3 = input.read();
        let d4 = input.read();
        input.read_4d_table(d1, d2, d3, d4)
    }
}

impl<T: Clone> Arr4d<T> {
    pub fn fill(&mut self, elem: T) {
        self.data.fill(elem);
    }
}
