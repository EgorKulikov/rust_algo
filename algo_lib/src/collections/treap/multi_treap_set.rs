use crate::collections::treap::multi_payload::MultiPayload;
use crate::collections::treap::Tree;
use std::iter::repeat;
use std::ops::{Bound, RangeBounds};

pub struct MultiTreapSet<T: Unpin> {
    root: Tree<MultiPayload<T>>,
}

impl<T: Ord + Unpin> MultiTreapSet<T> {
    pub fn new() -> Self {
        Self { root: Tree::new() }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&mut self) -> usize {
        self.root.payload().map_or(0, |p| p.total_size)
    }

    pub fn insert(&mut self, key: T) {
        let view = self.root.range(&key..=&key);
        if view.payload().is_some() {
            view.push(1);
        } else {
            view.add_back(MultiPayload::new(key, ()));
        }
    }

    pub fn remove(&mut self, key: &T) -> bool {
        let node = self.root.range(&key..=&key);
        if node.payload().is_some() {
            node.push(-1);
            if node.payload().unwrap().self_size == 0 {
                node.detach();
            }
            true
        } else {
            false
        }
    }

    pub fn index(&mut self, key: &T) -> Option<usize> {
        let lb = self.lower_bound(key);
        self.root.get(key).map(|_| lb)
    }

    pub fn lower_bound(&mut self, key: &T) -> usize {
        self.root.range(..key).payload().map_or(0, |p| p.total_size)
    }

    pub fn upper_bound(&mut self, key: &T) -> usize {
        self.root
            .range(..=key)
            .payload()
            .map_or(0, |p| p.total_size)
    }

    pub fn keys(&mut self) -> impl Iterator<Item = &T> {
        self.root.iter().map(|node| &node.key)
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &T> {
        self.root
            .iter()
            .map(Self::node_to_pair)
            .flat_map(|(key, count)| repeat(key).take(count))
    }

    pub fn range<'a, 's: 'a>(
        &'s mut self,
        r: impl RangeBounds<&'a T>,
    ) -> impl Iterator<Item = &'s T> {
        self.root
            .range(r)
            .iter()
            .map(Self::node_to_pair)
            .flat_map(|(key, count)| repeat(key).take(count))
    }

    pub fn first(&mut self) -> Option<&T> {
        self.root.first().map(Self::node_to_key)
    }

    pub fn last(&mut self) -> Option<&T> {
        self.root.last().map(Self::node_to_key)
    }

    pub fn prev(&mut self, key: &T) -> Option<&T> {
        self.root.prev(key).map(Self::node_to_key)
    }

    pub fn next(&mut self, key: &T) -> Option<&T> {
        self.root.next(key).map(Self::node_to_key)
    }

    pub fn floor(&mut self, key: &T) -> Option<&T> {
        self.root.floor(key).map(Self::node_to_key)
    }

    pub fn ceil(&mut self, key: &T) -> Option<&T> {
        self.root.ceil(key).map(Self::node_to_key)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn clear(&mut self) {
        self.root.detach();
    }

    pub fn get(&mut self, key: &T) -> usize {
        self.root.get(key).map_or(0, |node| node.self_size)
    }

    pub fn contains(&mut self, key: &T) -> bool {
        self.root.get(key).is_some()
    }

    pub fn more(&mut self, key: &T) -> usize {
        self.root
            .range((Bound::Excluded(key), Bound::Unbounded))
            .payload()
            .map_or(0, |p| p.total_size)
    }

    pub fn more_or_eq(&mut self, key: &T) -> usize {
        self.root.range(key..).payload().map_or(0, |p| p.total_size)
    }

    pub fn less(&mut self, key: &T) -> usize {
        self.root.range(..key).payload().map_or(0, |p| p.total_size)
    }

    pub fn less_or_eq(&mut self, key: &T) -> usize {
        self.root
            .range(..=key)
            .payload()
            .map_or(0, |p| p.total_size)
    }

    fn node_to_pair(node: &MultiPayload<T>) -> (&T, usize) {
        (&node.key, node.total_size)
    }

    fn node_to_key(node: &MultiPayload<T>) -> &T {
        &node.key
    }
}

impl<T: Ord + Unpin> Default for MultiTreapSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
