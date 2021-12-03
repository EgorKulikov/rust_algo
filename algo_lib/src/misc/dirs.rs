use crate::misc::value::{ConstValue, Value};
use crate::value;
use std::marker::PhantomData;

pub struct Directions<V: Value<[(isize, isize); N]>, const N: usize> {
    phantom: PhantomData<V>,
}

impl<V: Value<[(isize, isize); N]>, const N: usize> Directions<V, N> {
    pub fn iter(row: usize, col: usize, n: usize, m: usize) -> DirectionsIter<V, N> {
        DirectionsIter {
            row,
            col,
            n,
            m,
            at: 0,
            phantom: Default::default(),
        }
    }
}

pub struct DirectionsIter<V: Value<[(isize, isize); N]>, const N: usize> {
    row: usize,
    col: usize,
    n: usize,
    m: usize,
    at: usize,
    phantom: PhantomData<V>,
}

impl<V: Value<[(isize, isize); N]>, const N: usize> Iterator for DirectionsIter<V, N> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.at < N {
            let nrow = (self.row as isize) + V::val()[self.at].0;
            let ncol = (self.col as isize) + V::val()[self.at].1;
            self.at += 1;
            if nrow >= 0 && (nrow as usize) < self.n && ncol >= 0 && (ncol as usize) < self.m {
                return Some((nrow as usize, ncol as usize));
            }
        }
        None
    }
}

value!(
    D4Dirs,
    [(isize, isize); 4],
    [
        (1isize, 0isize),
        (0isize, 1isize),
        (-1isize, 0isize),
        (0isize, -1isize)
    ]
);

pub type D4 = Directions<D4Dirs, 4>;
