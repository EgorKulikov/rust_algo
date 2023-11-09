use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::numbers::num_traits::bit_ops::BitOps;
use std::ops::{BitAndAssign, BitOrAssign, Index};

const TRUE: bool = true;
const FALSE: bool = false;

#[derive(Clone, Eq, PartialEq)]
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
    
    pub fn from_slice(len: usize, set: &[usize]) -> Self {
        let mut res = Self::new(len);
        for &i in set {
            res.set(i);
        }
        res
    }

    pub fn set(&mut self, at: usize) {
        assert!(at < self.len);
        self.data[Self::index(at)].set_bit(at & 63);
    }

    pub fn unset(&mut self, at: usize) {
        assert!(at < self.len);
        self.data[Self::index(at)].unset_bit(at & 63);
    }

    pub fn change(&mut self, at: usize, value: bool) {
        if value {
            self.set(at);
        } else {
            self.unset(at);
        }
    }

    pub fn flip(&mut self, at: usize) {
        self.change(at, !self[at]);
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn fill(&mut self, value: bool) {
        // 1.43
        self.data.legacy_fill(if value { std::u64::MAX } else { 0 })
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        assert_eq!(self.len, other.len);
        for i in 0..self.data.len() {
            if self.data[i] & other.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        other.is_superset(self)
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.into_iter()
    }

    fn index(at: usize) -> usize {
        at >> 6
    }

    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|x| x.count_ones() as usize).sum()
    }
}

pub struct BitSetIter<'s> {
    at: usize,
    inside: usize,
    set: &'s BitSet,
}

impl<'s> Iterator for BitSetIter<'s> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.at < self.set.data.len()
            && (self.inside == 64 || (self.set.data[self.at] >> self.inside) == 0)
        {
            self.at += 1;
            self.inside = 0;
        }
        if self.at == self.set.data.len() {
            None
        } else {
            while !self.set.data[self.at].is_set(self.inside) {
                self.inside += 1;
            }
            let res = self.at * 64 + self.inside;
            if res < self.set.len {
                self.inside += 1;
                Some(res)
            } else {
                None
            }
        }
    }
}

impl<'a> IntoIterator for &'a BitSet {
    type Item = usize;
    type IntoIter = BitSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIter {
            at: 0,
            inside: 0,
            set: self,
        }
    }
}

impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i |= j;
        }
    }
}

impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i &= j;
        }
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

impl From<Vec<bool>> for BitSet {
    fn from(data: Vec<bool>) -> Self {
        let mut res = Self::new(data.len());
        for (i, &value) in data.iter().enumerate() {
            res.change(i, value);
        }
        res
    }
}
