use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut, Range};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct Arr2d<T> {
    d1: usize,
    d2: usize,
    data: Vec<T>,
}

impl<T: Clone> Arr2d<T> {
    pub fn new(d1: usize, d2: usize, value: T) -> Self {
        Self {
            d1,
            d2,
            data: vec![value; d1 * d2],
        }
    }
}

impl<T> Arr2d<T> {
    pub fn gen<F>(d1: usize, d2: usize, mut gen: F) -> Self
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

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item = &T> {
        assert!(row < self.d1);
        self.data.iter().skip(row * self.d2).take(self.d2)
    }

    pub fn row_mut(&mut self, row: usize) -> impl Iterator<Item = &mut T> {
        assert!(row < self.d1);
        self.data.iter_mut().skip(row * self.d2).take(self.d2)
    }

    pub fn column(&self, col: usize) -> impl Iterator<Item = &T> {
        assert!(col < self.d2);
        self.data.iter().skip(col).step_by(self.d2)
    }

    pub fn column_mut(&mut self, col: usize) -> impl Iterator<Item = &mut T> {
        assert!(col < self.d2);
        self.data.iter_mut().skip(col).step_by(self.d2)
    }

    pub fn swap(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
        assert!(r1 < self.d1);
        assert!(r2 < self.d1);
        assert!(c1 < self.d2);
        assert!(c2 < self.d2);
        self.data.swap(r1 * self.d2 + c1, r2 * self.d2 + c2);
    }

    pub fn rows(&self) -> Range<usize> {
        0..self.d1
    }

    pub fn cols(&self) -> Range<usize> {
        0..self.d2
    }

    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        assert!(r1 < self.d1);
        assert!(r2 < self.d1);
        if r1 == r2 {
            return;
        }
        let (r1, r2) = (r1.min(r2), r1.max(r2));
        let (head, tail) = self.data.split_at_mut(r2 * self.d2);
        head[r1 * self.d2..(r1 + 1) * self.d2].swap_with_slice(&mut tail[..self.d2]);
    }

    pub fn rotate_clockwise(self) -> Self {
        unsafe {
            let d1 = self.d1;
            let d2 = self.d2;
            let mut res = MaybeUninit::new(Vec::with_capacity(d1 * d2));
            (*res.as_mut_ptr()).set_len(d1 * d2);
            for (id, element) in self.into_iter().enumerate() {
                let (i, j) = (id / d2, id % d2);
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add(j * d1 + d1 - i - 1).write(element);
            }
            Self {
                d1: d2,
                d2: d1,
                data: res.assume_init(),
            }
        }
    }

    pub fn rotate_counterclockwise(self) -> Self {
        unsafe {
            let d1 = self.d1;
            let d2 = self.d2;
            let mut res = MaybeUninit::new(Vec::with_capacity(d1 * d2));
            (*res.as_mut_ptr()).set_len(d1 * d2);
            for (id, element) in self.into_iter().enumerate() {
                let (i, j) = (id / d2, id % d2);
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add((d2 - j - 1) * d1 + i).write(element);
            }
            Self {
                d1: d2,
                d2: d1,
                data: res.assume_init(),
            }
        }
    }
}

impl<T: Clone> Arr2d<T> {
    pub fn fill(&mut self, elem: T) {
        self.data.legacy_fill(elem);
    }

    pub fn transpose(&self) -> Self {
        Self::gen(self.d2, self.d1, |i, j| self[(j, i)].clone())
    }
}

impl<T> Index<(usize, usize)> for Arr2d<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(row < self.d1);
        assert!(col < self.d2);
        &self.data[self.d2 * row + col]
    }
}

impl<T> Index<usize> for Arr2d<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Arr2d<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        assert!(row < self.d1);
        assert!(col < self.d2);
        &mut self.data[self.d2 * row + col]
    }
}

impl<T> IndexMut<usize> for Arr2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

impl<T> AsRef<Vec<T>> for Arr2d<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T> AsMut<Vec<T>> for Arr2d<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
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

impl<T> IntoIterator for Arr2d<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Arr2d<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub trait Arr2dRead {
    fn read_table<T: Readable>(&mut self, d1: usize, d2: usize) -> Arr2d<T>;
    fn read_int_table(&mut self, d1: usize, d2: usize) -> Arr2d<i32>;
    fn read_long_table(&mut self, d1: usize, d2: usize) -> Arr2d<i64>;
    fn read_size_table(&mut self, d1: usize, d2: usize) -> Arr2d<usize>;
    fn read_char_table(&mut self, d1: usize, d2: usize) -> Arr2d<u8>;
}

impl Arr2dRead for Input<'_> {
    fn read_table<T: Readable>(&mut self, d1: usize, d2: usize) -> Arr2d<T> {
        Arr2d::gen(d1, d2, |_, _| self.read())
    }

    fn read_int_table(&mut self, d1: usize, d2: usize) -> Arr2d<i32> {
        self.read_table(d1, d2)
    }

    fn read_long_table(&mut self, d1: usize, d2: usize) -> Arr2d<i64> {
        self.read_table(d1, d2)
    }

    fn read_size_table(&mut self, d1: usize, d2: usize) -> Arr2d<usize> {
        self.read_table(d1, d2)
    }

    fn read_char_table(&mut self, d1: usize, d2: usize) -> Arr2d<u8> {
        self.read_table(d1, d2)
    }
}

pub trait Arr2dCharWrite {
    fn print_table(&mut self, table: &Arr2d<u8>);
}

impl Arr2dCharWrite for Output<'_> {
    fn print_table(&mut self, table: &Arr2d<u8>) {
        let mut at = 0usize;
        for _ in 0..table.d1 {
            for _ in 0..table.d2 {
                self.put(table.data[at]);
                at += 1;
            }
            self.put(b'\n');
        }
        self.maybe_flush();
    }
}

impl<T: Readable> Readable for Arr2d<T> {
    fn read(input: &mut Input) -> Self {
        let d1 = input.read();
        let d2 = input.read();
        input.read_table(d1, d2)
    }
}
