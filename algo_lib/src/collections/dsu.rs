use crate::collections::slice_ext::indices::Indices;
use crate::numbers::num_traits::primitive::Primitive;
use std::cell::Cell;

#[derive(Clone)]
pub struct DSU {
    id: Vec<Cell<i32>>,
    count: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            id: vec![Cell::new(-1); n],
            count: n,
        }
    }

    pub fn size(&self, i: usize) -> usize {
        (-self.id[self.find(i)].get()) as usize
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.id.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.id
            .iter()
            .enumerate()
            .filter_map(|(i, id)| if id.get() < 0 { Some(i) } else { None })
    }

    pub fn set_count(&self) -> usize {
        self.count
    }

    pub fn union(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            false
        } else {
            self.id[a].set(self.id[a].get() + self.id[b].get());
            self.id[b].set(a as i32);
            self.count -= 1;
            true
        }
    }

    pub fn find(&self, i: usize) -> usize {
        let mut root = i;
        while self.id[root].get() >= 0 {
            root = self.id[root].get() as usize;
        }
        let mut cur = i;
        while cur != root {
            let next = self.id[cur].get() as usize;
            self.id[cur].set(root as i32);
            cur = next;
        }
        root
    }

    pub fn clear(&mut self) {
        self.count = self.id.len();
        self.id.fill(Cell::new(-1));
    }

    pub fn parts(&self) -> Vec<Vec<usize>> {
        let mut slot = vec![usize::MAX; self.len()];
        let mut res = Vec::<Vec<usize>>::with_capacity(self.count);
        for i in self.id.indices() {
            let root = self.find(i);
            if slot[root] == usize::MAX {
                slot[root] = res.len();
                res.push(Vec::with_capacity((-self.id[root].get()).to()));
            }
            res[slot[root]].push(i);
        }
        res
    }
}
