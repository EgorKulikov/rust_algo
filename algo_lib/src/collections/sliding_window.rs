use std::cmp::Ordering;
use std::collections::VecDeque;

pub struct SlidingWindow<T, F> {
    size: usize,
    data: VecDeque<(usize, T)>,
    last: usize,
    cmp: F,
}

impl<T: Ord, F: FnMut(&T, &T) -> Ordering> SlidingWindow<T, F> {
    pub fn new(size: usize, cmp: F) -> Self {
        Self {
            size,
            data: VecDeque::new(),
            last: 0,
            cmp,
        }
    }

    pub fn push(&mut self, val: T) {
        while let Some((_, last)) = self.data.back() {
            if (self.cmp)(last, &val) <= Ordering::Equal {
                self.data.pop_back();
            } else {
                break;
            }
        }
        self.data.push_back((self.last, val));
        if let Some((id, _)) = self.data.front() {
            if *id + self.size == self.last {
                self.data.pop_front();
            }
        }
        self.last += 1;
    }

    pub fn get(&self) -> Option<&T> {
        self.data.front().map(|(_, val)| val)
    }
}
