use crate::collections::legacy_fill::LegacyFill;
use crate::numbers::num_traits::bit_ops::BitOps;
use std::ops::Index;

static TRUE: bool = true;
static FALSE: bool = false;

#[derive(Clone)]
pub struct BitSet {
    data: Vec<u64>,
    len: usize,
}

impl BitSet {
    pub fn new(len: usize) -> Self {
        let data_len = if len == 0 {
            0
        } else {
            Self::index(len - 1) + 1
        };
        Self {
            data: vec![0; data_len],
            len,
        }
    }

    pub fn set(&mut self, at: usize, value: bool) {
        assert!(at < self.len);
        if value {
            self.data[Self::index(at)].set_bit(at & 63);
        } else {
            self.data[Self::index(at)].unset_bit(at & 63);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn fill(&mut self, value: bool) {
        // 1.43
        self.data.legacy_fill(if value { std::u64::MAX } else { 0 })
    }

    fn index(at: usize) -> usize {
        at >> 6
    }
}

impl Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, at: usize) -> &Self::Output {
        assert!(at < self.len);
        if self.data[Self::index(at)].is_set(at & 63) {
            &TRUE
        } else {
            &FALSE
        }
    }
}
