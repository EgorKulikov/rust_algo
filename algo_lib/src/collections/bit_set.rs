use std::ops::Index;

static TRUE: bool = true;
static FALSE: bool = false;

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
            self.data[Self::index(at)] |= 1u64 << (at & 63);
        } else {
            self.data[Self::index(at)] &= !(1u64 << (at & 63));
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn fill(&mut self, value: bool) {
        self.data.fill(if value { u64::MAX } else { 0 })
    }

    fn index(at: usize) -> usize {
        at >> 6
    }
}

impl Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, at: usize) -> &Self::Output {
        assert!(at < self.len);
        if (self.data[Self::index(at)] >> (at & 63) & 1) == 1 {
            &TRUE
        } else {
            &FALSE
        }
    }
}
