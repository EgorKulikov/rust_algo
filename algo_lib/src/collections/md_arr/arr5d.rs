use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq)]
pub struct Arr5d<T> {
    d1: usize,
    d2: usize,
    d3: usize,
    d4: usize,
    d5: usize,
    data: Vec<T>,
}

impl<T: Clone> Arr5d<T> {
    pub fn new(d1: usize, d2: usize, d3: usize, d4: usize, d5: usize, value: T) -> Self {
        Self {
            d1,
            d2,
            d3,
            d4,
            d5,
            data: vec![value; d1 * d2 * d3 * d4 * d5],
        }
    }
}

impl<T> Arr5d<T> {
    pub fn generate<F>(d1: usize, d2: usize, d3: usize, d4: usize, d5: usize, mut gen: F) -> Self
    where
        F: FnMut(usize, usize, usize, usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(d1 * d2 * d3 * d4 * d5);
        for i in 0..d1 {
            for j in 0..d2 {
                for k in 0..d3 {
                    for l in 0..d4 {
                        for m in 0..d5 {
                            data.push(gen(i, j, k, l, m));
                        }
                    }
                }
            }
        }
        Self {
            d1,
            d2,
            d3,
            d4,
            d5,
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

    pub fn d5(&self) -> usize {
        self.d5
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

impl<T> Index<(usize, usize, usize, usize, usize)> for Arr5d<T> {
    type Output = T;

    fn index(&self, (a1, a2, a3, a4, a5): (usize, usize, usize, usize, usize)) -> &Self::Output {
        assert!(a1 < self.d1);
        assert!(a2 < self.d2);
        assert!(a3 < self.d3);
        assert!(a4 < self.d4);
        assert!(a5 < self.d5);
        &self.data[(((a1 * self.d2 + a2) * self.d3 + a3) * self.d4 + a4) * self.d5 + a5]
    }
}

impl<T> IndexMut<(usize, usize, usize, usize, usize)> for Arr5d<T> {
    fn index_mut(
        &mut self,
        (a1, a2, a3, a4, a5): (usize, usize, usize, usize, usize),
    ) -> &mut Self::Output {
        assert!(a1 < self.d1);
        assert!(a2 < self.d2);
        assert!(a3 < self.d3);
        assert!(a5 < self.d5);
        &mut self.data[(((a1 * self.d2 + a2) * self.d3 + a3) * self.d4 + a4) * self.d5 + a5]
    }
}

impl<T: Writable> Writable for Arr5d<T> {
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
                            output.put(b'\n');
                        }
                        for m in 0..self.d5 {
                            if m != 0 {
                                output.put(b' ');
                            }
                            self.data[at].write(output);
                            at += 1;
                        }
                    }
                }
            }
        }
    }
}

pub trait Arr5dRead {
    fn read_5d_table<T: Readable>(
        &mut self,
        d1: usize,
        d2: usize,
        d3: usize,
        d4: usize,
        d5: usize,
    ) -> Arr5d<T>;
}

impl Arr5dRead for Input<'_> {
    fn read_5d_table<T: Readable>(
        &mut self,
        d1: usize,
        d2: usize,
        d3: usize,
        d4: usize,
        d5: usize,
    ) -> Arr5d<T> {
        Arr5d::generate(d1, d2, d3, d4, d5, |_, _, _, _, _| self.read())
    }
}

impl<T: Readable> Readable for Arr5d<T> {
    fn read(input: &mut Input) -> Self {
        let d1 = input.read();
        let d2 = input.read();
        let d3 = input.read();
        let d4 = input.read();
        let d5 = input.read();
        input.read_5d_table(d1, d2, d3, d4, d5)
    }
}

impl<T: Clone> Arr5d<T> {
    pub fn fill(&mut self, elem: T) {
        self.data.legacy_fill(elem);
    }
}
