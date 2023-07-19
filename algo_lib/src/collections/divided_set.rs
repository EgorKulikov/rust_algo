use crate::misc::direction::Direction;
use std::collections::BTreeSet;

pub struct DividedSet<'s, T: Ord> {
    left: BTreeSet<T>,
    right: BTreeSet<T>,
    criteria: Box<dyn Fn(usize, usize) -> Option<Direction> + 's>,
}

impl<'s, T: Ord + Clone> DividedSet<'s, T> {
    pub fn new(criteria: impl Fn(usize, usize) -> Option<Direction> + 's) -> Self {
        Self {
            left: BTreeSet::new(),
            right: BTreeSet::new(),
            criteria: Box::new(criteria),
        }
    }

    pub fn left_tail(&self) -> Option<&T> {
        self.left.iter().next_back()
    }

    pub fn right_head(&self) -> Option<&T> {
        self.right.iter().next()
    }

    pub fn insert(&mut self, value: T) -> bool {
        let res = if let Some(left_tail) = self.left_tail() {
            if value < *left_tail {
                self.left.insert(value)
            } else {
                self.right.insert(value)
            }
        } else if let Some(right_head) = self.right_head() {
            if value < *right_head {
                self.left.insert(value)
            } else {
                self.right.insert(value)
            }
        } else {
            self.left.insert(value)
        };
        if res {
            self.balance();
        }
        res
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let res = self.left.remove(value) || self.right.remove(value);
        if res {
            self.balance();
        }
        res
    }

    fn balance(&mut self) {
        while let Some(direction) = (self.criteria)(self.left.len(), self.right.len()) {
            match direction {
                Direction::Right => {
                    let value = self.left.iter().next_back().unwrap().clone();
                    self.left.remove(&value);
                    self.right.insert(value);
                }
                Direction::Left => {
                    let value = self.right.iter().next().unwrap().clone();
                    self.right.remove(&value);
                    self.left.insert(value);
                }
            }
        }
    }
}
