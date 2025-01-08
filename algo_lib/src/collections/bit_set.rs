use crate::collections::iter_ext::iter_copied::ItersCopied;
use crate::numbers::num_traits::bit_ops::BitOps;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, Index, ShlAssign, ShrAssign};

const TRUE: bool = true;
const FALSE: bool = false;

#[derive(Clone, Eq, PartialEq, Hash)]
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
        self.data.fill(if value { std::u64::MAX } else { 0 });
        if value {
            self.fix_last();
        }
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        assert_eq!(self.len, other.len);
        for (we, them) in self.data.copy_zip(&other.data) {
            if (we & them) != them {
                return false;
            }
        }
        true
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        other.is_superset(self)
    }

    pub fn iter(&self) -> BitSetIter<'_> {
        self.into_iter()
    }

    fn index(at: usize) -> usize {
        at >> 6
    }

    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|x| x.count_ones() as usize).sum()
    }

    pub fn shift_or(&mut self, rhs: usize) {
        if rhs == 0 || rhs >= self.len {
            return;
        }
        let small_shift = rhs & 63;
        let big_shift = rhs >> 6;
        for i in (0..self.data.len() - big_shift).rev() {
            if small_shift != 0 && i + 1 + big_shift < self.data.len() {
                let big = self.data[i] >> (64 - small_shift);
                self.data[i + 1 + big_shift] |= big;
            }
            let small = self.data[i] << small_shift;
            self.data[i + big_shift] |= small;
        }
        self.fix_last();
    }

    fn fix_last(&mut self) {
        if self.len & 63 != 0 {
            let mask = (1 << (self.len & 63)) - 1;
            *self.data.last_mut().unwrap() &= mask;
        }
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
            self.inside += (self.set.data[self.at] >> self.inside).trailing_zeros() as usize;
            let res = self.at * 64 + self.inside;
            self.inside += 1;
            Some(res)
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

impl BitXorAssign<&BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i ^= j;
        }
    }
}

impl ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        if rhs >= self.len {
            self.fill(false);
            return;
        }
        let small_shift = rhs & 63;
        if small_shift != 0 {
            let mut carry = 0;
            for data in self.data.iter_mut() {
                let new_carry = (*data) >> (64 - small_shift);
                *data <<= small_shift;
                *data |= carry;
                carry = new_carry;
            }
        }
        let big_shift = rhs >> 6;
        if big_shift != 0 {
            self.data.rotate_right(big_shift);
            self.data[..big_shift].fill(0);
        }
        self.fix_last();
    }
}

impl ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        if rhs >= self.len {
            self.fill(false);
            return;
        }
        let small_shift = rhs & 63;
        if small_shift != 0 {
            let mut carry = 0;
            for data in self.data.iter_mut().rev() {
                let new_carry = (*data) << (64 - small_shift);
                *data >>= small_shift;
                *data |= carry;
                carry = new_carry;
            }
        }
        let big_shift = rhs >> 6;
        if big_shift != 0 {
            self.data.rotate_left(big_shift);
            let from = self.data.len() - big_shift;
            self.data[from..].fill(0);
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
