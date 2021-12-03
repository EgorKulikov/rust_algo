use crate::collections::min_max::MinimMaxim;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub struct FenwickTree<T: AddSub + ZeroOne> {
    value: Vec<T>,
}

impl<T: AddSub + ZeroOne> FenwickTree<T> {
    pub fn new(size: usize) -> Self {
        Self {
            value: vec![T::zero(); size],
        }
    }

    pub fn get_to(&self, mut to: usize) -> T {
        to.minim(self.value.len());
        to -= 1;
        let mut result = T::zero();
        loop {
            result += self.value[to];
            to &= to + 1;
            if to == 0 {
                break result;
            }
            to -= 1;
        }
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
            self.value[at] += v;
            at |= at + 1;
        }
    }

    pub fn clear(&mut self) {
        self.value.fill(T::zero());
    }
}
