use crate::collections::min_max::MinimMaxim;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::zero_one::ZeroOne;

#[derive(Clone)]
pub struct FastClearFenwickTree<T> {
    value: Vec<(u32, T)>,
    epoch: u32,
}

impl<T: AddSub + ZeroOne> FastClearFenwickTree<T> {
    pub fn new(size: usize) -> Self {
        Self {
            value: vec![(0, T::zero()); size],
            epoch: 0,
        }
    }

    pub fn get_to(&self, mut to: usize) -> T {
        to.minim(self.value.len());
        let mut result = T::zero();
        while to > 0 {
            to -= 1;
            if self.value[to].0 == self.epoch {
                result += self.value[to].1;
            }
            to &= to + 1;
        }
        result
    }

    pub fn get(&self, from: usize, to: usize) -> T {
        if from >= to {
            T::zero()
        } else {
            self.get_to(to) - self.get_to(from)
        }
    }

    pub fn add(&mut self, mut at: usize, v: T) {
        while at < self.value.len() {
            if self.value[at].0 != self.epoch {
                self.value[at].0 = self.epoch;
                self.value[at].1 = T::zero();
            }
            self.value[at].1 += v;
            at |= at + 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.value
            .iter()
            .enumerate()
            // edition 2021
            .map(move |(i, _)| self.get(i, i + 1))
    }

    pub fn clear(&mut self) {
        self.epoch += 1;
    }
}
