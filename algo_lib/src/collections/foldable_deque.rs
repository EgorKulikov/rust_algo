//! Queue and deque that maintain the fold of all their elements under an
//! associative operation (the "sliding window aggregation" structure): two
//! stacks with cached prefix folds, amortized O(1) per operation.

/// Folds with an associative `op`; elements keep their queue order, so the
/// operation need not be commutative.
pub struct FoldableDeque<T, F> {
    /// Front stack: `front[i]` is nearer the front for larger `i`.
    front: Vec<(T, T)>,
    /// Back stack: `back[i]` is nearer the back for larger `i`.
    back: Vec<(T, T)>,
    op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> FoldableDeque<T, F> {
    pub fn new(op: F) -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
            op,
        }
    }

    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_back(&mut self, value: T) {
        let fold = match self.back.last() {
            Some((_, f)) => (self.op)(f, &value),
            None => value.clone(),
        };
        self.back.push((value, fold));
    }

    pub fn push_front(&mut self, value: T) {
        let fold = match self.front.last() {
            Some((_, f)) => (self.op)(&value, f),
            None => value.clone(),
        };
        self.front.push((value, fold));
    }

    /// Rebuilds both stacks from the elements in queue order, giving the
    /// front stack `to_front` of them.
    fn rebalance(&mut self, elements: Vec<T>, to_front: usize) {
        self.front.clear();
        self.back.clear();
        let mut it = elements.into_iter();
        let head: Vec<T> = it.by_ref().take(to_front).collect();
        for v in head.into_iter().rev() {
            self.push_front(v);
        }
        for v in it {
            self.push_back(v);
        }
    }

    fn drain_in_order(&mut self) -> Vec<T> {
        let mut all: Vec<T> = self.front.drain(..).rev().map(|(v, _)| v).collect();
        all.extend(self.back.drain(..).map(|(v, _)| v));
        all
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.front.is_empty() {
            let all = self.drain_in_order();
            let half = (all.len() + 1) / 2;
            self.rebalance(all, half);
        }
        self.front.pop().map(|(v, _)| v)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.back.is_empty() {
            let all = self.drain_in_order();
            let half = all.len() / 2;
            self.rebalance(all, half);
        }
        self.back.pop().map(|(v, _)| v)
    }

    pub fn front(&self) -> Option<&T> {
        self.front
            .last()
            .map(|(v, _)| v)
            .or_else(|| self.back.first().map(|(v, _)| v))
    }

    pub fn back(&self) -> Option<&T> {
        self.back
            .last()
            .map(|(v, _)| v)
            .or_else(|| self.front.first().map(|(v, _)| v))
    }

    /// Fold of all elements from front to back, or `None` when empty.
    pub fn fold(&self) -> Option<T> {
        match (self.front.last(), self.back.last()) {
            (Some((_, a)), Some((_, b))) => Some((self.op)(a, b)),
            (Some((_, a)), None) => Some(a.clone()),
            (None, Some((_, b))) => Some(b.clone()),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FoldableDeque;
    use crate::misc::random::{Random, RandomTrait};
    use std::collections::VecDeque;

    #[test]
    fn matches_vecdeque_with_a_non_commutative_fold() {
        const P: u64 = 998_244_353;
        // affine maps composed front to back
        let compose = |a: &(u64, u64), b: &(u64, u64)| (a.0 * b.0 % P, (a.1 * b.0 + b.1) % P);
        let mut rng = Random::new_with_seed(371);
        for _ in 0..50 {
            let mut dq = FoldableDeque::new(compose);
            let mut reference: VecDeque<(u64, u64)> = VecDeque::new();
            for _ in 0..400 {
                match rng.gen_range(0..6u32) {
                    0 | 1 => {
                        let v = (rng.gen_range(1..P), rng.gen_range(0..P));
                        dq.push_back(v);
                        reference.push_back(v);
                    }
                    2 => {
                        let v = (rng.gen_range(1..P), rng.gen_range(0..P));
                        dq.push_front(v);
                        reference.push_front(v);
                    }
                    3 => assert_eq!(dq.pop_front(), reference.pop_front()),
                    4 => assert_eq!(dq.pop_back(), reference.pop_back()),
                    _ => {}
                }
                assert_eq!(dq.len(), reference.len());
                assert_eq!(dq.front(), reference.front());
                assert_eq!(dq.back(), reference.back());
                let expected = reference.iter().copied().reduce(|a, b| compose(&a, &b));
                assert_eq!(dq.fold(), expected);
            }
        }
    }
}
