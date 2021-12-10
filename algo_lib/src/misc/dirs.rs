use crate::const_value_ref;
use crate::misc::value_ref::ValueRef;
use std::marker::PhantomData;

pub struct Directions<V: ValueRef<[(isize, isize)]>> {
    phantom: PhantomData<V>,
}

impl<V: ValueRef<[(isize, isize)]>> Directions<V> {
    pub fn iter(
        row: usize,
        col: usize,
        n: usize,
        m: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        DirectionsIter::<V> {
            row,
            col,
            n,
            m,
            at: 0,
            phantom: Default::default(),
        }
    }
}

struct DirectionsIter<V: ValueRef<[(isize, isize)]>> {
    row: usize,
    col: usize,
    n: usize,
    m: usize,
    at: usize,
    phantom: PhantomData<V>,
}

impl<V: ValueRef<[(isize, isize)]>> Iterator for DirectionsIter<V> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.at < V::val().len() {
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

const_value_ref!(
    D4Dirs,
    D4_DIRS_INNER,
    [(isize, isize); 4],
    [(isize, isize)],
    [
        (1isize, 0isize),
        (0isize, 1isize),
        (-1isize, 0isize),
        (0isize, -1isize)
    ]
);

pub type D4 = Directions<D4Dirs>;
