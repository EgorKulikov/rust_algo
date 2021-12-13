use crate::collections::direction::Direction;
use crate::collections::treap::{TreapNode, Updateable};
use std::ops::Index;

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
        self.root.merge(node);
        self.root.merge(right);
        res
    }

    pub fn remove(&mut self, key: &T) -> Option<V> {
        let (left, mut right) = self.root.split(key, false);
        let (node, right) = right.split(key, true);
        self.root = left;
        self.root.merge(right);
        <TreapNode<T, TreapValue<V>> as Into<Option<TreapValue<V>>>>::into(node)
            .map(|data| data.value)
    }

    pub fn index(&mut self, key: &T) -> Option<usize> {
        let (left, mut right) = self.root.split(key, false);
        let (node, right) = right.split(key, true);
        let res = node
            .data()
            .map(|_| left.data().map(|data| data.size as usize).unwrap_or(0));
        self.root = left;
        self.root.merge(node);
        self.root.merge(right);
        res
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

    pub fn keys(&self) -> impl Iterator<Item = &T> + '_ {
        self.root.iter().map(|node| node.key().unwrap())
    }

    pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
        self.root.iter().map(|node| &node.data().unwrap().value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &V)> + '_ {
        self.root
            .iter()
            .map(|node| (node.key().unwrap(), &node.data().unwrap().value))
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

pub type TreapSet<T> = TreapMap<T, ()>;

impl<T: Ord> TreapSet<T> {
    pub fn add(&mut self, key: T) -> bool {
        self.insert(key, ()).is_some()
    }
}
