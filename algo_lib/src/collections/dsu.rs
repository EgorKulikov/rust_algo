use crate::collections::iter_ext::IterExt;
use std::cell::Cell;

pub struct DSU {
    id: Vec<Cell<usize>>,
    size: Vec<usize>,
    count: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            id: (0..n).map(|i| Cell::new(i)).collect_vec(),
            size: vec![1; n],
            count: n,
        }
    }

    pub fn size(&self, i: usize) -> usize {
        self.size[self.get(i)]
    }

    pub fn len(&self) -> usize {
        self.id.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.id
            .iter()
            .enumerate()
            .filter_map(|(i, id)| if i == id.get() { Some(i) } else { None })
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn join(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.get(a);
        b = self.get(b);
        if a == b {
            false
        } else {
            self.size[a] += self.size[b];
            self.id[b].replace(a);
            self.count -= 1;
            true
        }
    }

    pub fn get(&self, i: usize) -> usize {
        if self.id[i].get() != i {
            let res = self.get(self.id[i].get());
            self.id[i].replace(res);
        }
        self.id[i].get()
    }

    pub fn clear(&mut self) {
        self.count = self.id.len();
        self.size.fill(1);
        self.id.iter().enumerate().for_each(|(i, id)| {
            id.replace(i);
        });
    }
}
