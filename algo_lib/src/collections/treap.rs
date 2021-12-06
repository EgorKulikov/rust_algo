use crate::misc::random::random;
use std::mem::swap;

pub struct TreapNode<Key: Ord, Data: Updateable>(Option<Box<TreapNodeInner<Key, Data>>>);

struct TreapNodeInner<Key: Ord, Data: Updateable> {
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
        let left_data = self.left.0.as_ref().map(|node| &node.data);
        let right_data = self.right.0.as_ref().map(|node| &node.data);
        self.data.update(left_data, right_data);
    }
}

impl<Key: Ord, Data: Updateable> TreapNode<Key, Data> {
    pub const NONE: Self = Self(None);

    pub fn new(key: Key, data: Data) -> Self {
        Self(Some(Box::new(TreapNodeInner::new(key, data))))
    }

    pub fn has_left(&self) -> bool {
        !self.0.is_none() && !self.0.as_ref().unwrap().left.0.is_none()
    }

    pub fn key(&self) -> Option<&Key> {
        self.0.as_ref().map(|node| &node.key)
    }

    pub fn data(&self) -> Option<&Data> {
        self.0.as_ref().map(|node| &node.data)
    }

    pub fn data_mut(&mut self) -> Option<&mut Data> {
        self.0.as_mut().map(|node| &mut node.data)
    }

    pub fn data_at(&self, at: Key) -> Option<&Data> {
        match self.0.as_ref() {
            None => None,
            Some(node) => {
                if node.key == at {
                    Some(&node.data)
                } else if node.key < at {
                    node.right.data_at(at)
                } else {
                    node.left.data_at(at)
                }
            }
        }
    }

    pub fn split(mut self, split_key: &Key) -> (Self, Self) {
        match self.0.as_mut() {
            Some(node) => {
                if &node.key < split_key {
                    let mut right = Self::NONE;
                    swap(&mut node.right, &mut right);
                    let (left, right) = right.split(split_key);
                    node.right = left;
                    node.update();
                    (self, right)
                } else {
                    let mut left = Self::NONE;
                    swap(&mut node.left, &mut left);
                    let (left, right) = left.split(split_key);
                    node.left = right;
                    node.update();
                    (left, self)
                }
            }
            None => (Self::NONE, Self::NONE),
        }
    }

    pub fn merge(&mut self, right: Self) {
        let mut left = Self::NONE;
        swap(self, &mut left);
        *self = Self::merge_impl(left, right);
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        match &self.0 {
            None => Box::new(std::iter::empty()),
            Some(node) => Box::new(
                node.left
                    .iter()
                    .chain(std::iter::once(self))
                    .chain(node.right.iter()),
            ),
        }
    }

    fn merge_impl(mut left: Self, mut right: Self) -> Self {
        if left.0.is_none() {
            right
        } else if right.0.is_none() {
            left
        } else {
            let mut l = left.0.as_mut().unwrap();
            let mut r = right.0.as_mut().unwrap();
            if l.priority > r.priority {
                let mut ll = Self::NONE;
                swap(&mut ll, &mut l.right);
                l.right = Self::merge_impl(ll, right);
                l.update();
                left
            } else {
                let mut rr = Self::NONE;
                swap(&mut rr, &mut r.left);
                r.left = Self::merge_impl(left, rr);
                r.update();
                right
            }
        }
    }
}

pub trait Updateable {
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>);
}

impl Updateable for () {
    fn update(&mut self, _: Option<&Self>, _: Option<&Self>) {}
}

pub trait DataProvider<Data: Updateable> {
    fn data(&self) -> Option<&Data>;
}

impl<Key: Ord, Data: Updateable> DataProvider<Data> for Option<Box<TreapNodeInner<Key, Data>>> {
    fn data(&self) -> Option<&Data> {
        match self {
            None => None,
            Some(node) => Some(&node.data),
        }
    }
}
