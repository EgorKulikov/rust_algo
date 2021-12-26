use crate::misc::direction::Direction;
use crate::misc::random::random;
use std::cmp::Ordering;
use std::mem::swap;
use std::ops::{Deref, DerefMut};

pub struct TreapNode<Key, Data>(Option<Box<TreapNodeInner<Key, Data>>>);

impl<Key, Data> Deref for TreapNode<Key, Data> {
    type Target = Option<Box<TreapNodeInner<Key, Data>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Key, Data> DerefMut for TreapNode<Key, Data> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct TreapNodeInner<Key, Data> {
    priority: u64,
    key: Key,
    data: Data,
    left: TreapNode<Key, Data>,
    right: TreapNode<Key, Data>,
}

impl<Key: Ord, Data: Updateable> TreapNodeInner<Key, Data> {
    fn new(key: Key, data: Data) -> Self {
        Self {
            priority: random().gen(),
            key,
            data,
            left: TreapNode::NONE,
            right: TreapNode::NONE,
        }
    }

    fn update(&mut self) {
        let left_data = self.left.as_ref().map(|node| &node.data);
        let right_data = self.right.as_ref().map(|node| &node.data);
        self.data.update(left_data, right_data);
    }
}

impl<Key: Ord, Data: Updateable> TreapNode<Key, Data> {
    pub const NONE: Self = Self(None);

    pub fn new(key: Key, data: Data) -> Self {
        Self(Some(Box::new(TreapNodeInner::new(key, data))))
    }

    pub fn key(&self) -> Option<&Key> {
        self.as_ref().map(|node| &node.key)
    }

    pub fn data(&self) -> Option<&Data> {
        self.as_ref().map(|node| &node.data)
    }

    pub fn replace_data(&mut self, key: Key, mut data: Data) -> Option<Data> {
        match self.as_mut() {
            None => {
                *self = TreapNode::new(key, data);
                None
            }
            Some(node) => {
                assert!(node.left.is_none() && node.right.is_none());
                swap(&mut node.data, &mut data);
                Some(data)
            }
        }
    }

    pub fn split(&mut self, split_key: &Key, to_left: bool) -> (Self, Self) {
        match self.take() {
            Some(mut node) => {
                let ordering = node.key.cmp(split_key);
                if ordering == Ordering::Less || to_left && ordering == Ordering::Equal {
                    let (left, right) = node.right.split(split_key, to_left);
                    node.right = left;
                    node.update();
                    (Self(Some(node)), right)
                } else {
                    let (left, right) = node.left.split(split_key, to_left);
                    node.left = right;
                    node.update();
                    (left, Self(Some(node)))
                }
            }
            None => (Self::NONE, Self::NONE),
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        match self.deref() {
            None => Box::new(std::iter::empty()),
            Some(node) => Box::new(
                node.left
                    .iter()
                    .chain(std::iter::once(self))
                    .chain(node.right.iter()),
            ),
        }
    }

    pub fn priority(&self) -> u64 {
        self.as_ref().map(|node| node.priority).unwrap_or(0)
    }

    pub fn merge(&mut self, mut right: Self) {
        if self.is_none() {
            *self = right;
        } else if right.is_some() {
            if self.priority() > right.priority() {
                let node = self.as_mut().unwrap();
                node.right.merge(right);
                node.update();
            } else {
                let mut node = right.take().unwrap();
                self.merge(Self(node.left.take()));
                node.left = Self(self.take());
                node.update();
                *self = Self(Some(node));
            }
        }
    }

    pub fn binary_search<'s, F>(&'s self, mut f: F)
    where
        F: FnMut(&'s Key, &'s Data, Option<&'s Data>, Option<&'s Data>) -> Option<Direction>,
    {
        if let Some(node) = self.deref() {
            let direction = f(&node.key, &node.data, node.left.data(), node.right.data());
            if let Some(direction) = direction {
                match direction {
                    Direction::Left => {
                        node.left.binary_search(f);
                    }
                    Direction::Right => {
                        node.right.binary_search(f);
                    }
                }
            }
        }
    }

    pub fn find(&self, key: &Key) -> &Self {
        match self.deref() {
            None => self,
            Some(node) => {
                let ordering = node.key.cmp(key);
                match ordering {
                    Ordering::Less => node.right.find(key),
                    Ordering::Equal => self,
                    Ordering::Greater => node.left.find(key),
                }
            }
        }
    }
}

impl<Key: Ord, Data: Updateable> From<TreapNode<Key, Data>> for Option<Data> {
    fn from(data: TreapNode<Key, Data>) -> Self {
        data.0.map(|node| node.data)
    }
}

impl<Key: Ord, Data: Updateable> Default for TreapNode<Key, Data> {
    fn default() -> Self {
        Self::NONE
    }
}

pub trait Updateable {
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>);
}

impl Updateable for u32 {
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        *self = 1 + *left.unwrap_or(&0) + *right.unwrap_or(&0)
    }
}
