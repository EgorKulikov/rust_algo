use crate::collections::treap::{KeyPayload, Payload, PureDataPayload, SizePayload, TreapNode};
use crate::misc::direction::Direction;
use std::cmp::Ordering;
use std::mem::swap;
use std::ops::{Deref, DerefMut, Index, RangeBounds};

type TreapValue<T, V> = KeyPayload<T, SizePayload<PureDataPayload<V>>>;

pub struct TreapMap<T: Ord, V> {
    root: TreapNode<TreapValue<T, V>>,
}

impl<T: Ord, V> TreapMap<T, V> {
    pub fn new() -> Self {
        Self {
            root: TreapNode::NONE,
        }
    }

    pub unsafe fn from_sorted(iter: impl Iterator<Item = (T, V)>) -> Self {
        let mut res = Self::new();
        for (t, v) in iter {
            res.root = TreapNode::merge_unsafe(res.root, TreapNode::new((t, v).into()));
        }
        res
    }

    pub fn len(&self) -> usize {
        self.root.payload().map(|data| data.inner.size).unwrap_or(0) as usize
    }

    pub fn insert(&mut self, key: T, value: V) -> Option<V> {
        let mut root = TreapNode::NONE;
        swap(&mut self.root, &mut root);
        let (left, mut node, right) = root.split_range(&key..=&key);
        let res = if node.is_some() {
            Some(node.replace(value))
        } else {
            node = TreapNode::new((key, value).into());
            None
        };
        unsafe {
            self.root = TreapNode::merge_unsafe(left, TreapNode::merge_unsafe(node, right));
        }
        res
    }

    pub fn remove(&mut self, key: &T) -> Option<V> {
        let mut root = TreapNode::NONE;
        swap(&mut self.root, &mut root);
        let (left, node, right) = root.split_range(key..=key);
        unsafe {
            self.root = TreapNode::merge_unsafe(left, right);
        }
        node.into_payload().map(|data| data.inner.inner.data)
    }

    pub fn index(&self, key: &T) -> Option<usize> {
        let mut res = 0;
        let mut found = false;
        self.root
            .binary_search(|k, _, left_data, _| match key.cmp(k) {
                Ordering::Less => Some(Direction::Left),
                Ordering::Equal => {
                    if let Some(data) = left_data {
                        res += data.inner.size;
                    }
                    found = true;
                    None
                }
                Ordering::Greater => {
                    if let Some(data) = left_data {
                        res += data.inner.size + 1;
                    } else {
                        res += 1;
                    }
                    Some(Direction::Right)
                }
            });
        if found {
            Some(res as usize)
        } else {
            None
        }
    }

    pub fn lower_bound(&self, key: &T) -> usize {
        match self.ceil(key) {
            None => self.len(),
            Some((key, _)) => self.index(key).unwrap(),
        }
    }

    pub fn get_at(&mut self, mut at: usize) -> Option<(&T, &V)> {
        if at >= self.len() {
            None
        } else {
            let mut res = None;
            self.root.binary_search(|key, value, left, _| {
                let to_left = left.map(|data| data.inner.size as usize).unwrap_or(0);
                match to_left {
                    v if v > at => Some(Direction::Left),
                    v if v == at => {
                        res = Some((key, &value.inner.inner.data));
                        None
                    }
                    v if v < at => {
                        at -= to_left + 1;
                        Some(Direction::Right)
                    }
                    _ => panic!("unreachable"),
                }
            });
            res
        }
    }

    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> {
        self.iter().map(|(_, val)| val)
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&T, &V)> {
        self.range(..)
    }

    pub fn range<'a, 's: 'a>(
        &'s self,
        r: impl RangeBounds<&'a T>,
    ) -> impl DoubleEndedIterator<Item = (&'s T, &'s V)> {
        self.root.range(r).map(Self::node_to_pair)
    }

    pub fn first(&self) -> Option<(&T, &V)> {
        self.root.leftmost().map(Self::node_to_pair)
    }

    pub fn last(&self) -> Option<(&T, &V)> {
        self.root.rightmost().map(Self::node_to_pair)
    }

    pub fn lower(&self, key: &T) -> Option<(&T, &V)> {
        self.root.lower(key).map(Self::node_to_pair)
    }

    pub fn higher(&self, key: &T) -> Option<(&T, &V)> {
        self.root.higher(key).map(Self::node_to_pair)
    }

    pub fn floor(&self, key: &T) -> Option<(&T, &V)> {
        self.root.floor(key).map(Self::node_to_pair)
    }

    pub fn ceil(&self, key: &T) -> Option<(&T, &V)> {
        self.root.ceil(key).map(Self::node_to_pair)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.root = TreapNode::NONE
    }

    pub fn get(&self, key: &T) -> Option<&V> {
        self.root
            .find(key)
            .payload()
            .map(|data| &data.inner.inner.data)
    }

    pub fn contains(&self, key: &T) -> bool {
        self.root.find(key).is_some()
    }

    fn node_to_pair(node: &TreapNode<TreapValue<T, V>>) -> (&T, &V) {
        (
            node.payload().as_ref().unwrap().key(),
            &node.payload().as_ref().unwrap().inner.inner.data,
        )
    }
}

impl<T: Ord, V> Default for TreapMap<T, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord, V> Index<&T> for TreapMap<T, V> {
    type Output = V;

    fn index(&self, index: &T) -> &Self::Output {
        self.get(index).unwrap()
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

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + '_ {
        self.0.keys()
    }

    pub fn range<'a, 's: 'a>(
        &'s self,
        r: impl RangeBounds<&'a T>,
    ) -> impl DoubleEndedIterator<Item = &'s T> {
        self.0.range(r).map(Self::map_to_key)
    }

    pub fn first(&self) -> Option<&T> {
        self.0.first().map(Self::map_to_key)
    }

    pub fn last(&self) -> Option<&T> {
        self.0.last().map(Self::map_to_key)
    }

    pub fn lower(&self, key: &T) -> Option<&T> {
        self.0.lower(key).map(Self::map_to_key)
    }

    pub fn higher(&self, key: &T) -> Option<&T> {
        self.0.higher(key).map(Self::map_to_key)
    }

    pub fn more(&self, key: &T) -> usize {
        match self.0.higher(key) {
            Some((k, _)) => self.len() - self.0.index(k).unwrap(),
            None => 0,
        }
    }

    pub fn more_or_eq(&self, key: &T) -> usize {
        match self.0.ceil(key) {
            Some((k, _)) => self.len() - self.0.index(k).unwrap(),
            None => 0,
        }
    }

    pub fn less(&self, key: &T) -> usize {
        match self.0.lower(key) {
            Some((k, _)) => self.0.index(k).unwrap() + 1,
            None => 0,
        }
    }

    pub fn less_or_eq(&self, key: &T) -> usize {
        match self.0.floor(key) {
            Some((k, _)) => self.0.index(k).unwrap() + 1,
            None => 0,
        }
    }

    pub fn floor(&self, key: &T) -> Option<&T> {
        self.0.floor(key).map(Self::map_to_key)
    }

    pub fn ceil(&self, key: &T) -> Option<&T> {
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
