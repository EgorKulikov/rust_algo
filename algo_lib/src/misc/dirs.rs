use crate::const_value_ref;
use crate::misc::value_ref::ConstValueRef;
use std::marker::PhantomData;

pub struct Directions<V: ConstValueRef<[(isize, isize)]>> {
    phantom: PhantomData<V>,
}

impl<V: ConstValueRef<[(isize, isize)]>> Directions<V> {
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

    pub fn go(row: usize, col: usize, dir: usize, n: usize, m: usize) -> (usize, usize) {
        let (dr, dc) = V::val()[dir];
        let mut row = row as isize + dr;
        let mut col = col as isize + dc;
        if row < 0 {
            row = 0;
        }
        if row >= n as isize {
            row = n as isize - 1;
        }
        if col < 0 {
            col = 0;
        }
        if col >= m as isize {
            col = m as isize - 1;
        }
        (row as usize, col as usize)
    }
}

struct DirectionsIter<V: ConstValueRef<[(isize, isize)]>> {
    row: usize,
    col: usize,
    n: usize,
    m: usize,
    at: usize,
    phantom: PhantomData<V>,
}

impl<V: ConstValueRef<[(isize, isize)]>> Iterator for DirectionsIter<V> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.at < V::val().len() {
            let n_row = (self.row as isize) + V::val()[self.at].0;
            let n_col = (self.col as isize) + V::val()[self.at].1;
            self.at += 1;
            if n_row >= 0 && (n_row as usize) < self.n && n_col >= 0 && (n_col as usize) < self.m {
                return Some((n_row as usize, n_col as usize));
            }
        }
        None
    }
}

const_value_ref!(
    D4Dirs D4_DIRS_INNER: [(isize, isize); 4] as [(isize, isize)] = [
        (0isize, 1isize),
        (1isize, 0isize),
        (0isize, -1isize),
        (-1isize, 0isize),
    ]
);

pub type D4 = Directions<D4Dirs>;

const_value_ref!(
    D8Dirs D8_DIRS_INNER: [(isize, isize); 8] as [(isize, isize)] = [
        (0isize, 1isize),
        (1isize, 1isize),
        (1isize, 0isize),
        (1isize, -1isize),
        (0isize, -1isize),
        (-1isize, -1isize),
        (-1isize, 0isize),
        (-1isize, 1isize),
    ]
);

pub type D8 = Directions<D8Dirs>;
