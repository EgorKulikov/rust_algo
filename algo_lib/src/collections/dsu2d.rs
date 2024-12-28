use crate::collections::dsu::DSU;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct DSU2d {
    inner: DSU,
    cols: usize,
}

impl Deref for DSU2d {
    type Target = DSU;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DSU2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl DSU2d {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            inner: DSU::new(rows * cols),
            cols,
        }
    }

    pub fn size(&self, row: usize, col: usize) -> usize {
        self.inner.size(row * self.cols + col)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let cols = self.cols;
        self.inner.iter().map(move |i| (i / cols, i % cols))
    }

    pub fn union(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) -> bool {
        self.inner.union(r1 * self.cols + c1, r2 * self.cols + c2)
    }

    pub fn find(&self, row: usize, col: usize) -> (usize, usize) {
        let res = self.inner.find(row * self.cols + col);
        (res / self.cols, res % self.cols)
    }

    pub fn parts(&self) -> Vec<Vec<(usize, usize)>> {
        self.inner
            .parts()
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|i| (i / self.cols, i % self.cols))
                    .collect()
            })
            .collect()
    }
}
