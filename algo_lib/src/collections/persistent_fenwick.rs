use crate::collections::min_max::MinimMaxim;
use crate::collections::slice_ext::bounds::Bounds;
use crate::collections::slice_ext::indices::Indices;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct PersistentFenwickTree<T, E> {
    base: E,
    value: Vec<Vec<Value<T, E>>>,
}

#[derive(Clone)]
struct Value<T, E> {
    epoch: E,
    value: T,
}

impl<T, E: PartialEq> PartialEq<Self> for Value<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.epoch.eq(&other.epoch)
    }
}

impl<T, E: PartialOrd> PartialOrd for Value<T, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.epoch.partial_cmp(&other.epoch)
    }
}

impl<T: AdditionMonoidWithSub + Copy, E: PartialOrd + Copy> PersistentFenwickTree<T, E> {
    pub fn new(size: usize, base: E) -> Self {
        Self {
            base,
            value: vec![
                vec![Value {
                    value: T::zero(),
                    epoch: base
                }];
                size
            ],
        }
    }

    pub fn get_to(&self, mut to: usize, epoch: E) -> T {
        if epoch < self.base {
            panic!("Queried epoch before base");
        }
        to.minim(self.value.len());
        let mut result = T::zero();
        while to > 0 {
            to -= 1;
            let pos = self.value[to].as_slice().upper_bound(&Value {
                epoch,
                value: T::zero(),
            }) - 1;
            result += self.value[to][pos].value;
            to &= to + 1;
        }
        result
    }

    pub fn get(&self, from: usize, to: usize, epoch: E) -> T {
        if from >= to {
            T::zero()
        } else {
            self.get_to(to, epoch) - self.get_to(from, epoch)
        }
    }

    pub fn add(&mut self, mut at: usize, v: T, epoch: E) {
        while at < self.value.len() {
            if self.value[at].last().unwrap().epoch == epoch {
                self.value[at].last_mut().unwrap().value += v;
            } else {
                let vat = self.value[at].last().unwrap().value + v;
                self.value[at].push(Value { value: vat, epoch });
            }
            at |= at + 1;
        }
    }

    pub fn iter(&self, epoch: E) -> impl Iterator<Item = T> + '_ {
        self.value.indices().map(move |i| self.get(i, i + 1, epoch))
    }
}
