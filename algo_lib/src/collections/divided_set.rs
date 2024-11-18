use crate::misc::direction::Direction;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct DividedSet<'s, T: Ord> {
    left: BinaryHeap<T>,
    right: BinaryHeap<Reverse<T>>,
    criteria: Box<dyn Fn(usize, usize) -> Option<Direction> + 's>,
}

impl<'s, T: Ord> DividedSet<'s, T> {
    pub fn new(criteria: impl Fn(usize, usize) -> Option<Direction> + 's) -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
            criteria: Box::new(criteria),
        }
    }

    pub fn left_tail(&self) -> Option<&T> {
        self.left.peek()
    }

    pub fn right_head(&self) -> Option<&T> {
        self.right.peek().map(|x| &x.0)
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
