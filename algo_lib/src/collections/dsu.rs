use crate::collections::base_algo::create_order;
use std::ops::Index;

pub struct DSU {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            id: create_order(n),
            size: vec![n; 1],
            count: n,
        }
    }

    pub fn size(&self, i: usize) -> usize {
        self.size[self[i]]
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn join(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self[a];
        b = self[b];
        if a == b {
            false
        } else {
            self.size[a] += self.size[b];
            self.id[b] = a;
            self.count -= 1;
            true
        }
    }

    pub fn clear(&mut self) {
        self.count = self.id.len();
        self.size.fill(1);
        for i in 0..self.count {
            self.id[i] = i;
        }
    }
}

impl Index<usize> for DSU {
    type Output = usize;

    fn index(&self, i: usize) -> &Self::Output {
        if self.id[i] != i {
            let res = self[self.id[i]];
            unsafe {
                let const_ptr = self as *const Self;
                let mut_ptr = const_ptr as *mut Self;
                let mut_self = &mut *mut_ptr;
                mut_self.id[i] = res;
            }
        }
        &self.id[i]
    }
}
