use crate::misc::random::random;

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

    pub fn key(&self) -> Option<&Key> {
        self.0.as_ref().map(|node| &node.key)
    }

    pub fn data(&self) -> Option<&Data> {
        self.0.as_ref().map(|node| &node.data)
    }

    pub fn data_mut(&mut self) -> Option<&mut Data> {
        self.0.as_mut().map(|node| &mut node.data)
    }

    pub fn split(&mut self, split_key: &Key) -> (Self, Self) {
        match self.0.take() {
            Some(mut node) => {
                if &node.key < split_key {
                    let (left, right) = node.right.split(split_key);
                    node.right = left;
                    node.update();
                    (Self(Some(node)), right)
                } else {
                    let (left, right) = node.left.split(split_key);
                    node.left = right;
                    node.update();
                    (left, Self(Some(node)))
                }
            }
            None => (Self::NONE, Self::NONE),
        }
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

    pub fn priority(&self) -> u64 {
        self.0.as_ref().map(|node| node.priority).unwrap_or(0)
    }

    pub fn merge(&mut self, mut right: Self) {
        if self.0.is_none() {
            *self = right;
        } else if right.0.is_none() {
            return;
        } else {
            if self.priority() > right.priority() {
                let node = self.0.as_mut().unwrap();
                node.right.merge(right);
                node.update();
            } else {
                let mut node = right.0.take().unwrap();
                self.merge(Self(node.left.0.take()));
                node.left = Self(self.0.take());
                node.update();
                *self = Self(Some(node));
            }
        }
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
