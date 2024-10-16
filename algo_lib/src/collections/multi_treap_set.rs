use crate::collections::treap::{Payload, PayloadWithKey, Pushable, Treap};
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::misc::extensions::with::With;
use std::iter::repeat;
use std::ops::{Bound, RangeBounds};

struct MapPayload<T> {
    key: T,
    self_size: i32,
    total_size: i32,
}

impl<T> MapPayload<T> {
    fn new(key: T) -> Self {
        Self {
            key,
            self_size: 1,
            total_size: 1,
        }
    }
}

impl<T> Payload for MapPayload<T> {
    #[inline]
    fn reset_delta(&mut self) {}

    #[inline]
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.total_size =
            self.self_size + left.map_or(0, |l| l.total_size) + right.map_or(0, |r| r.total_size);
    }

    #[inline]
    fn push_delta(&mut self, _delta: &Self) {}
}

impl<T> Pushable<i32> for MapPayload<T> {
    fn push(&mut self, delta: i32) {
        self.self_size += delta;
        self.total_size += delta;
    }
}

impl<T: Ord> PayloadWithKey for MapPayload<T> {
    type Key = T;

    fn key(&self) -> &Self::Key {
        &self.key
    }
}

pub struct MultiTreapSet<T> {
    root: Treap<MapPayload<T>>,
}

impl<T: Ord> MultiTreapSet<T> {
    pub fn new() -> Self {
        Self { root: Treap::new() }
    }

    pub fn len(&mut self) -> usize {
        self.root.payload().map_or(0, |p| p.total_size as usize)
    }

    pub fn insert(&mut self, key: T) {
        self.root.range(&key..=&key).replace_with(|mut node| {
            if node.payload().is_some() {
                node.push(1);
                node
            } else {
                Treap::single(MapPayload::new(key))
            }
        });
    }

    pub fn remove(&mut self, key: &T) -> bool {
        let mut res = true;
        self.root.range(&key..=&key).replace_with(|mut node| {
            if node.payload().is_some() {
                node.push(-1);
                if node.payload().unwrap().self_size == 0 {
                    Treap::new()
                } else {
                    node
                }
            } else {
                res = false;
                node
            }
        });
        res
    }

    pub fn index(&mut self, key: &T) -> Option<usize> {
        let lb = self.lower_bound(key);
        self.root.get(key).map(|_| lb)
    }

    pub fn lower_bound(&mut self, key: &T) -> usize {
        self.root
            .range(..key)
            .with(|node| node.payload().map_or(0, |p| p.total_size) as usize)
    }

    pub fn upper_bound(&mut self, key: &T) -> usize {
        self.root
            .range(..=key)
            .with(|node| node.payload().map_or(0, |p| p.total_size) as usize)
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

    pub fn lower(&mut self, key: &T) -> Option<&T> {
        self.root.lower(key).map(Self::node_to_key)
    }

    pub fn higher(&mut self, key: &T) -> Option<&T> {
        self.root.higher(key).map(Self::node_to_key)
    }

    pub fn floor(&mut self, key: &T) -> Option<&T> {
        self.root.floor(key).map(Self::node_to_key)
    }

    pub fn ceil(&mut self, key: &T) -> Option<&T> {
        self.root.ceil(key).map(Self::node_to_key)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_empty(&mut self) -> bool {
        self.root.is_empty()
    }

    pub fn clear(&mut self) {
        self.root = Treap::new()
    }

    pub fn get(&mut self, key: &T) -> usize {
        self.root.get(key).map_or(0, |node| {
            node.payload().as_ref().unwrap().self_size as usize
        })
    }

    pub fn contains(&mut self, key: &T) -> bool {
        self.root.get(key).is_some()
    }

    pub fn more(&mut self, key: &T) -> usize {
        self.root
            .range((Bound::Excluded(key), Bound::Unbounded))
            .payload()
            .map_or(0, |p| p.total_size as usize)
    }

    pub fn more_or_eq(&mut self, key: &T) -> usize {
        self.root
            .range(key..)
            .payload()
            .map_or(0, |p| p.total_size as usize)
    }

    pub fn less(&mut self, key: &T) -> usize {
        self.root
            .range(..key)
            .payload()
            .map_or(0, |p| p.total_size as usize)
    }

    pub fn less_or_eq(&mut self, key: &T) -> usize {
        self.root
            .range(..=key)
            .payload()
            .map_or(0, |p| p.total_size as usize)
    }

    fn node_to_pair(node: &MapPayload<T>) -> (&T, usize) {
        (&node.key, node.total_size as usize)
    }

    fn node_to_key(node: &MapPayload<T>) -> &T {
        &node.key
    }
}

impl<T: Ord> Default for MultiTreapSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
