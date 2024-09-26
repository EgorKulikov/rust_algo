use crate::collections::treap::{Payload, PayloadWithKey, Treap};
use crate::misc::extensions::with::With;
use std::ops::{Bound, Deref, DerefMut, RangeBounds};

struct MapPayload<T, V> {
    key: T,
    value: V,
}

impl<T, V> MapPayload<T, V> {
    fn new(key: T, value: V) -> Self {
        Self { key, value }
    }
}

impl<T, V> Payload for MapPayload<T, V> {
    #[inline]
    fn reset_delta(&mut self) {}

    #[inline]
    fn update(&mut self, _left: Option<&Self>, _right: Option<&Self>) {}

    #[inline]
    fn push_delta(&mut self, _delta: &Self) {}
}

impl<T: Ord, V> PayloadWithKey for MapPayload<T, V> {
    type Key = T;

    fn key(&self) -> &Self::Key {
        &self.key
    }
}

pub struct TreapMap<T, V> {
    root: Treap<MapPayload<T, V>>,
}

impl<T: Ord, V> TreapMap<T, V> {
    pub fn new() -> Self {
        Self { root: Treap::new() }
    }

    pub unsafe fn from_sorted(iter: impl Iterator<Item = (T, V)>) -> Self {
        let mut res = Self::new();
        for (t, v) in iter {
            res.root = Treap::merge(res.root, Treap::single(MapPayload::new(t, v)));
        }
        res
    }

    pub fn len(&mut self) -> usize {
        self.root.size()
    }

    pub fn insert(&mut self, key: T, value: V) -> Option<V> {
        self.root
            .insert(MapPayload::new(key, value))
            .map(|payload| payload.value)
    }

    pub fn remove(&mut self, key: &T) -> Option<V> {
        self.root.remove(key).map(|payload| payload.value)
    }

    pub fn index(&mut self, key: &T) -> Option<usize> {
        self.root.index(key)
    }

    pub fn lower_bound(&mut self, key: &T) -> usize {
        self.root.range(..key).with(|node| node.size())
    }

    pub fn upper_bound(&mut self, key: &T) -> usize {
        self.root.range(..=key).with(|node| node.size())
    }

    pub fn get_at(&mut self, at: usize) -> Option<(&T, &V)> {
        self.root.get_at(at).payload().map(Self::node_to_pair)
    }

    pub fn keys(&mut self) -> impl Iterator<Item = &T> {
        self.iter().map(|(key, _)| key)
    }

    pub fn values(&mut self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, val)| val)
    }

    pub fn iter(&mut self) -> impl Iterator<Item = (&T, &V)> {
        self.root.iter().map(Self::node_to_pair)
    }

    pub fn range<'a, 's: 'a>(
        &'s mut self,
        r: impl RangeBounds<&'a T>,
    ) -> impl Iterator<Item = (&'s T, &'s V)> {
        self.root.range(r).iter().map(Self::node_to_pair)
    }

    pub fn first(&mut self) -> Option<(&T, &V)> {
        self.root.first().map(Self::node_to_pair)
    }

    pub fn last(&mut self) -> Option<(&T, &V)> {
        self.root.last().map(Self::node_to_pair)
    }

    pub fn lower(&mut self, key: &T) -> Option<(&T, &V)> {
        self.root.lower(key).map(Self::node_to_pair)
    }

    pub fn higher(&mut self, key: &T) -> Option<(&T, &V)> {
        self.root.higher(key).map(Self::node_to_pair)
    }

    pub fn floor(&mut self, key: &T) -> Option<(&T, &V)> {
        self.root.floor(key).map(Self::node_to_pair)
    }

    pub fn ceil(&mut self, key: &T) -> Option<(&T, &V)> {
        self.root.ceil(key).map(Self::node_to_pair)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_empty(&mut self) -> bool {
        self.root.is_empty()
    }

    pub fn clear(&mut self) {
        self.root = Treap::new()
    }

    pub fn get(&mut self, key: &T) -> Option<&V> {
        self.root
            .get(key)
            .map(|node| &node.payload().as_ref().unwrap().value)
    }

    pub fn contains(&mut self, key: &T) -> bool {
        self.root.get(key).is_some()
    }

    pub fn more(&mut self, key: &T) -> usize {
        self.root
            .range((Bound::Excluded(key), Bound::Unbounded))
            .size()
    }

    pub fn more_or_eq(&mut self, key: &T) -> usize {
        self.root.range(key..).size()
    }

    pub fn less(&mut self, key: &T) -> usize {
        self.root.range(..key).size()
    }

    pub fn less_or_eq(&mut self, key: &T) -> usize {
        self.root.range(..=key).size()
    }

    fn node_to_pair(node: &MapPayload<T, V>) -> (&T, &V) {
        (&node.key, &node.value)
    }
}

impl<T: Ord, V> Default for TreapMap<T, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct TreapSet<T: Ord>(TreapMap<T, ()>);

impl<T: Ord> TreapSet<T> {
    pub fn new() -> Self {
        Self(TreapMap::new())
    }

    pub unsafe fn from_sorted(iter: impl Iterator<Item = T>) -> Self {
        Self(TreapMap::from_sorted(iter.map(|t| (t, ()))))
    }

    pub fn insert(&mut self, key: T) -> bool {
        self.0.insert(key, ()).is_some()
    }

    pub fn remove(&mut self, key: &T) -> bool {
        self.0.remove(key).is_some()
    }

    pub fn get_at(&mut self, idx: usize) -> Option<&T> {
        self.0.get_at(idx).map(Self::map_to_key)
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &T> + '_ {
        self.0.keys()
    }

    pub fn range<'a, 's: 'a>(
        &'s mut self,
        r: impl RangeBounds<&'a T>,
    ) -> impl Iterator<Item = &'s T> {
        self.0.range(r).map(Self::map_to_key)
    }

    pub fn first(&mut self) -> Option<&T> {
        self.0.first().map(Self::map_to_key)
    }

    pub fn last(&mut self) -> Option<&T> {
        self.0.last().map(Self::map_to_key)
    }

    pub fn lower(&mut self, key: &T) -> Option<&T> {
        self.0.lower(key).map(Self::map_to_key)
    }

    pub fn higher(&mut self, key: &T) -> Option<&T> {
        self.0.higher(key).map(Self::map_to_key)
    }

    pub fn floor(&mut self, key: &T) -> Option<&T> {
        self.0.floor(key).map(Self::map_to_key)
    }

    pub fn ceil(&mut self, key: &T) -> Option<&T> {
        self.0.ceil(key).map(Self::map_to_key)
    }

    fn map_to_key<'a>((key, _): (&'a T, &'a ())) -> &'a T {
        key
    }
}

impl<T: Ord> Deref for TreapSet<T> {
    type Target = TreapMap<T, ()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> DerefMut for TreapSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
