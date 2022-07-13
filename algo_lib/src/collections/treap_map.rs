use crate::collections::treap::{TreapNode, Updateable};
use crate::misc::direction::Direction;
use crate::numbers::num_traits::primitive::Primitive;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut, Index, RangeBounds};

struct TreapValue<V> {
    size: u32,
    value: V,
}

impl<V> Updateable for TreapValue<V> {
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.size =
            1 + left.map(|node| node.size).unwrap_or(0) + right.map(|node| node.size).unwrap_or(0)
    }
}

pub struct TreapMap<T: Ord, V> {
    root: TreapNode<T, TreapValue<V>>,
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
            res.root
                .merge_unsafe(TreapNode::new(t, TreapValue { size: 1, value: v }));
        }
        res
    }

    pub fn len(&self) -> usize {
        self.root.data().map(|data| data.size).unwrap_or(0) as usize
    }

    pub fn insert(&mut self, key: T, value: V) -> Option<V> {
        let (left, mut right) = self.root.split(&key, false);
        let (mut node, right) = right.split(&key, true);
        let res = node
            .replace_data(key, TreapValue { size: 1, value })
            .map(|data| data.value);
        self.root = left;
        unsafe {
            self.root.merge_unsafe(node);
            self.root.merge_unsafe(right);
        }
        res
    }

    pub fn remove(&mut self, key: &T) -> Option<V> {
        let (left, mut right) = self.root.split(key, false);
        let (node, right) = right.split(key, true);
        self.root = left;
        unsafe {
            self.root.merge_unsafe(right);
        }
        <TreapNode<T, TreapValue<V>> as Into<Option<TreapValue<V>>>>::into(node)
            .map(|data| data.value)
    }

    pub fn index(&self, key: &T) -> Option<usize> {
        let mut res = 0;
        let mut found = false;
        self.root
            .binary_search(|k, _, left_data, _| match key.cmp(k) {
                Ordering::Less => Some(Direction::Left),
                Ordering::Equal => {
                    if let Some(data) = left_data {
                        res += data.size;
                    }
                    found = true;
                    None
                }
                Ordering::Greater => {
                    if let Some(data) = left_data {
                        res += data.size + 1;
                    } else {
                        res += 1;
                    }
                    Some(Direction::Right)
                }
            });
        if found {
            Some(res.into_usize())
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
                let to_left = left.map(|data| data.size as usize).unwrap_or(0);
                match to_left {
                    v if v > at => Some(Direction::Left),
                    v if v == at => {
                        res = Some((key, &value.value));
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
        self.root.find(key).data().map(|data| &data.value)
    }

    pub fn contains(&self, key: &T) -> bool {
        self.root.find(key).is_some()
    }

    fn node_to_pair(node: &TreapNode<T, TreapValue<V>>) -> (&T, &V) {
        (node.key().unwrap(), &node.data().unwrap().value)
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
