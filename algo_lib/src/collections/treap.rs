use crate::misc::direction::Direction;
use crate::misc::random::random;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::Bound;
use std::mem::swap;
use std::ops::{Deref, DerefMut, RangeBounds};

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

    pub fn range<'a, 's: 'a>(&'s self, r: impl RangeBounds<&'a Key>) -> Iter<'s, Key, Data> {
        Iter::new(self.borrow(), r)
    }

    pub fn leftmost(&self) -> Option<&Self> {
        self.as_ref()
            .map(|node| node.left.leftmost().unwrap_or(self))
    }

    pub fn rightmost(&self) -> Option<&Self> {
        self.as_ref()
            .map(|node| node.right.rightmost().unwrap_or(self))
    }

    pub fn lower(&self, key: &Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if &node.key < key {
                node.right.lower(key).or(Some(self))
            } else {
                node.left.lower(key)
            }
        })
    }

    pub fn floor(&self, key: &Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if &node.key <= key {
                node.right.floor(key).or(Some(self))
            } else {
                node.left.floor(key)
            }
        })
    }

    pub fn higher(&self, key: &Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if &node.key > key {
                node.left.higher(key).or(Some(self))
            } else {
                node.right.higher(key)
            }
        })
    }

    pub fn ceil(&self, key: &Key) -> Option<&Self> {
        self.as_ref().and_then(|node| {
            if &node.key >= key {
                node.left.ceil(key).or(Some(self))
            } else {
                node.right.ceil(key)
            }
        })
    }

    pub fn priority(&self) -> u64 {
        self.as_ref().map(|node| node.priority).unwrap_or(0)
    }

    pub fn merge(&mut self, right: Self) {
        if self.is_some() && right.is_some() {
            assert!(
                self.rightmost().unwrap().key().unwrap() < right.leftmost().unwrap().key().unwrap()
            );
        }
        unsafe {
            self.merge_unsafe(right);
        }
    }

    pub unsafe fn merge_unsafe(&mut self, mut right: Self) {
        if self.is_none() {
            *self = right;
        } else if right.is_some() {
            if self.priority() > right.priority() {
                let node = self.as_mut().unwrap();
                node.right.merge_unsafe(right);
                node.update();
            } else {
                let mut node = right.take().unwrap();
                self.merge_unsafe(Self(node.left.take()));
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

// TODO: get rid of log
pub struct Iter<'a, Key, Data> {
    root: &'a TreapNode<Key, Data>,
    from: Option<&'a TreapNode<Key, Data>>,
    to: Option<&'a TreapNode<Key, Data>>,
}

impl<'a, Key: Ord, Data: Updateable> Iter<'a, Key, Data> {
    pub fn new<'b>(root: &'a TreapNode<Key, Data>, r: impl RangeBounds<&'b Key>) -> Self
    where
        'a: 'b,
    {
        Self {
            root,
            from: match r.start_bound() {
                Bound::Included(key) => root.ceil(*key),
                Bound::Excluded(key) => root.higher(*key),
                Bound::Unbounded => root.leftmost(),
            },
            to: match r.end_bound() {
                Bound::Included(key) => root.floor(*key),
                Bound::Excluded(key) => root.lower(*key),
                Bound::Unbounded => root.rightmost(),
            },
        }
    }
}

impl<'a, Key: Ord, Data: Updateable> Iterator for Iter<'a, Key, Data> {
    type Item = &'a TreapNode<Key, Data>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from?.key().unwrap() <= self.to?.key().unwrap() {
            let res = self.from;
            self.from = self.root.higher(self.from.unwrap().key().unwrap());
            res
        } else {
            None
        }
    }
}

impl<'a, Key: Ord, Data: Updateable> DoubleEndedIterator for Iter<'a, Key, Data> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.from?.key().unwrap() <= self.to?.key().unwrap() {
            let res = self.to;
            self.to = self.root.lower(self.to.unwrap().key().unwrap());
            res
        } else {
            None
        }
    }
}
