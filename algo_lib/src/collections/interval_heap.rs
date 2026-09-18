//! Double-ended priority queue as an interval heap: a binary heap whose
//! nodes hold a `[min, max]` pair, giving O(log n) insertion and removal of
//! either extreme in one array.

pub struct IntervalHeap<T> {
    /// Node `k` is `data[2k]` (its lower end) and `data[2k + 1]` (upper end);
    /// the last node may hold a single element.
    data: Vec<T>,
}

impl<T: Ord> Default for IntervalHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> IntervalHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn min(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn max(&self) -> Option<&T> {
        match self.data.len() {
            0 => None,
            1 => self.data.first(),
            _ => self.data.get(1),
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let i = self.data.len() - 1;
        if i % 2 == 1 && self.data[i - 1] > self.data[i] {
            self.data.swap(i - 1, i);
            self.up_min(i - 1);
        } else if i >= 2 {
            let parent = (i / 2 - 1) / 2 * 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                self.up_min(parent);
            } else if self.data[i] > self.data[parent + 1] {
                self.data.swap(i, parent + 1);
                self.up_max(parent + 1);
            }
        }
    }

    /// Sifts the lower end at even index `i` towards the root.
    fn up_min(&mut self, mut i: usize) {
        while i >= 2 {
            let parent = (i / 2 - 1) / 2 * 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    /// Sifts the upper end at odd index `i` towards the root.
    fn up_max(&mut self, mut i: usize) {
        while i >= 2 {
            let parent = (i / 2 - 1) / 2 * 2 + 1;
            if self.data[i] > self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if self.data.len() <= 1 {
            return self.data.pop();
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let res = self.data.pop();
        let n = self.data.len();
        let mut i = 0;
        loop {
            // keep the node's own interval ordered
            if i + 1 < n && self.data[i] > self.data[i + 1] {
                self.data.swap(i, i + 1);
            }
            let (l, r) = (2 * i + 2, 2 * i + 4);
            let mut child = i;
            if l < n && self.data[l] < self.data[child] {
                child = l;
            }
            if r < n && self.data[r] < self.data[child] {
                child = r;
            }
            if child == i {
                break;
            }
            self.data.swap(i, child);
            i = child;
        }
        res
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if self.data.len() <= 2 {
            return self.data.pop();
        }
        let last = self.data.len() - 1;
        self.data.swap(1, last);
        let res = self.data.pop();
        let n = self.data.len();
        let mut i = 1;
        loop {
            if self.data[i - 1] > self.data[i] {
                self.data.swap(i - 1, i);
            }
            // upper ends of the children; a single-element last node counts too
            let upper = |k: usize| {
                if k < n {
                    Some(k)
                } else if k - 1 < n {
                    Some(k - 1)
                } else {
                    None
                }
            };
            let (l, r) = (upper(2 * i + 1), upper(2 * i + 3));
            let mut child = i;
            for c in [l, r].into_iter().flatten() {
                if self.data[c] > self.data[child] {
                    child = c;
                }
            }
            if child == i {
                break;
            }
            self.data.swap(i, child);
            if child % 2 == 0 {
                // moved into a single-element node: nothing below it
                break;
            }
            i = child;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::IntervalHeap;
    use crate::misc::random::{Random, RandomTrait};
    use std::collections::BTreeMap;

    #[test]
    fn matches_multiset() {
        let mut rng = Random::new_with_seed(381);
        for _ in 0..200 {
            let mut heap = IntervalHeap::new();
            let mut reference: BTreeMap<i32, usize> = BTreeMap::new();
            let range = rng.gen_range(1..=50i32);
            for _ in 0..300 {
                match rng.gen_range(0..4u32) {
                    0 | 1 => {
                        let v = rng.gen_range(-range..=range);
                        heap.push(v);
                        *reference.entry(v).or_default() += 1;
                    }
                    2 => {
                        let expected = reference.keys().next().copied();
                        assert_eq!(heap.pop_min(), expected);
                        if let Some(v) = expected {
                            let c = reference.get_mut(&v).unwrap();
                            *c -= 1;
                            if *c == 0 {
                                reference.remove(&v);
                            }
                        }
                    }
                    _ => {
                        let expected = reference.keys().next_back().copied();
                        assert_eq!(heap.pop_max(), expected);
                        if let Some(v) = expected {
                            let c = reference.get_mut(&v).unwrap();
                            *c -= 1;
                            if *c == 0 {
                                reference.remove(&v);
                            }
                        }
                    }
                }
                assert_eq!(heap.len(), reference.values().sum::<usize>());
                assert_eq!(heap.min(), reference.keys().next());
                assert_eq!(heap.max(), reference.keys().next_back());
            }
        }
    }
}
