use crate::misc::direction::Direction;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct DividedSet<T: Ord, F: Fn(usize, usize) -> Option<Direction>> {
    left: BinaryHeap<T>,
    right: BinaryHeap<Reverse<T>>,
    criteria: F,
}

impl<T: Ord, F: Fn(usize, usize) -> Option<Direction>> DividedSet<T, F> {
    pub fn new(criteria: F) -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
            criteria,
        }
    }

    pub fn left_tail(&self) -> Option<&T> {
        self.left.peek()
    }

    pub fn right_head(&self) -> Option<&T> {
        self.right.peek().map(|x| &x.0)
    }

    pub fn pop_left_tail(&mut self) -> Option<T> {
        let res = self.left.pop();
        self.balance();
        res
    }

    pub fn pop_right_head(&mut self) -> Option<T> {
        let res = self.right.pop().map(|x| x.0);
        self.balance();
        res
    }

    pub fn insert(&mut self, value: T) {
        if let Some(left_tail) = self.left_tail() {
            if value < *left_tail {
                self.left.push(value)
            } else {
                self.right.push(Reverse(value))
            }
        } else if let Some(right_head) = self.right_head() {
            if value < *right_head {
                self.left.push(value)
            } else {
                self.right.push(Reverse(value))
            }
        } else {
            self.left.push(value)
        }
        self.balance();
    }

    fn balance(&mut self) {
        while let Some(direction) = (self.criteria)(self.left.len(), self.right.len()) {
            match direction {
                Direction::Right => {
                    let value = self.left.pop().unwrap();
                    self.right.push(Reverse(value));
                }
                Direction::Left => {
                    let Reverse(value) = self.right.pop().unwrap();
                    self.left.push(value);
                }
            }
        }
    }
}
