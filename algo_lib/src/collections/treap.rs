use crate::misc::direction::Direction;
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::misc::extensions::with::With;
use crate::misc::random::random;
use crate::misc::recursive_function::{Callable, RecursiveFunction};
use std::cmp::Ordering;
use std::collections::Bound;
use std::mem::{swap, MaybeUninit};
use std::ops::{Deref, DerefMut, RangeBounds};

pub trait Payload {
    fn reset_delta(&mut self);
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>);
    fn push_delta(&mut self, delta: &Self);
}

pub trait PayloadWithKey: Payload {
    type Key: Ord;
    fn key(&self) -> &Self::Key;
}

pub trait Pushable<Delta>: Payload {
    fn push(&mut self, delta: Delta);
}

impl<P: Payload> Pushable<&P> for P {
    #[inline]
    fn push(&mut self, delta: &P) {
        self.push_delta(delta);
    }
}

pub struct PurePayload<T>(pub T);

impl<T> Payload for PurePayload<T> {
    #[inline]
    fn reset_delta(&mut self) {}

    #[inline]
    fn update(&mut self, _left: Option<&Self>, _right: Option<&Self>) {}

    #[inline]
    fn push_delta(&mut self, _delta: &Self) {}
}

impl<T: Ord> PayloadWithKey for PurePayload<T> {
    type Key = T;

    #[inline]
    fn key(&self) -> &Self::Key {
        &self.0
    }
}

impl<T> Pushable<T> for PurePayload<T> {
    #[inline]
    fn push(&mut self, delta: T) {
        self.0 = delta;
    }
}

pub struct Iter<'a, P> {
    stack: Vec<*mut TreapNode<P>>,
    _marker: std::marker::PhantomData<&'a P>,
}

impl<'a, P: Payload> Iter<'a, P> {
    fn new(root: &'a mut OptionTreapNode<P>) -> Self {
        let mut res = Self {
            stack: Vec::new(),
            _marker: std::marker::PhantomData,
        };
        res.add_left(root);
        res
    }

    fn add_left(&mut self, mut node: &'a mut OptionTreapNode<P>) {
        while let Some(n) = node.0.as_deref_mut() {
            n.push_down();
            self.stack.push(n);
            node = &mut n.left;
        }
    }
}

impl<'a, P: Payload> Iterator for Iter<'a, P> {
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        let node = unsafe { &mut *self.stack.pop()? };
        self.add_left(&mut node.right);
        Some(&node.payload)
    }
}

#[allow(private_interfaces)]
pub enum Treap<P> {
    Whole {
        root: OptionTreapNode<P>,
    },
    Split {
        left: OptionTreapNode<P>,
        mid: Box<Treap<P>>,
        right: OptionTreapNode<P>,
    },
}

impl<P> Treap<P> {
    pub fn new() -> Self {
        Treap::Whole {
            root: OptionTreapNode::NONE,
        }
    }

    pub fn single(p: P) -> Self {
        Treap::Whole {
            root: OptionTreapNode::new(TreapNode::new(p)),
        }
    }
}

impl<P> Default for Treap<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Payload> Treap<P> {
    #[inline]
    fn into_node(mut self) -> OptionTreapNode<P> {
        self.rebuild();
        match self {
            Treap::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    #[inline]
    unsafe fn as_node(&mut self) -> &mut OptionTreapNode<P> {
        match self {
            Treap::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    #[inline]
    unsafe fn as_mid(&mut self) -> &mut Treap<P> {
        match self {
            Treap::Split { mid, .. } => mid,
            _ => unreachable!(),
        }
    }

    fn rebuild(&mut self) -> &mut OptionTreapNode<P> {
        self.replace_with(|self_| {
            if let Treap::Split { left, mid, right } = self_ {
                Treap::Whole {
                    root: OptionTreapNode::merge(
                        left,
                        OptionTreapNode::merge(mid.into_node(), right),
                    ),
                }
            } else {
                self_
            }
        });
        unsafe { self.as_node() }
    }

    fn do_split(
        &mut self,
        f: impl FnOnce(
            OptionTreapNode<P>,
        ) -> (OptionTreapNode<P>, OptionTreapNode<P>, OptionTreapNode<P>),
    ) -> &mut Self {
        self.replace_with(|self_| {
            let root = self_.into_node();
            let (left, mid, right) = f(root);
            Self::Split {
                left,
                mid: Box::new(Self::Whole { root: mid }),
                right,
            }
        });
        unsafe { self.as_mid() }
    }

    pub fn reverse(&mut self) {
        self.rebuild().reverse();
    }

    pub fn size(&mut self) -> usize {
        self.rebuild().size()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_empty(&mut self) -> bool {
        self.size() != 0
    }

    pub fn by_index(&mut self, bounds: impl RangeBounds<usize>) -> &mut Self {
        self.do_split(|root| {
            let size = root.size();
            let start = match bounds.start_bound() {
                Bound::Included(&s) => s,
                Bound::Excluded(&s) => s + 1,
                Bound::Unbounded => 0,
            };
            let end = match bounds.end_bound() {
                Bound::Included(&e) => e + 1,
                Bound::Excluded(&e) => e,
                Bound::Unbounded => size,
            };
            let (left, mid_right) = root.split_at(start);
            let (mid, right) = mid_right.split_at(end.max(start) - start);
            (left, mid, right)
        })
    }

    pub fn get_at(&mut self, at: usize) -> &mut Self {
        self.replace_with(|self_| {
            let root = self_.into_node();
            let (left, mid, right) = root.split_at_single(at);
            Self::Split {
                left,
                mid: Box::new(Self::Whole { root: mid }),
                right,
            }
        });
        unsafe { self.as_mid() }
    }

    pub fn split_at_single(self, at: usize) -> (Self, Self, Self) {
        let (left, mid, right) = self.into_node().split_at_single(at);
        (
            Treap::Whole { root: left },
            Treap::Whole { root: mid },
            Treap::Whole { root: right },
        )
    }

    pub fn binary_search(
        &mut self,
        f: impl FnMut(&P, Option<(&P, usize)>, Option<(&P, usize)>) -> Option<Direction>,
    ) {
        self.rebuild().binary_search(f);
    }

    pub fn push<D>(&mut self, delta: D)
    where
        P: Pushable<D>,
    {
        self.rebuild().push(delta);
    }

    pub fn split_at(mut self, at: usize) -> (Self, Self) {
        let mut right_res = MaybeUninit::uninit();
        self.rebuild().replace_with(|root| {
            let (left, right) = root.split_at(at);
            right_res.write(Treap::Whole { root: right });
            left
        });
        (self, unsafe { right_res.assume_init() })
    }

    pub fn merge(left: Self, right: Self) -> Self {
        match left {
            Treap::Whole { root: left_root } => Treap::Split {
                left: left_root,
                mid: Box::new(right),
                right: OptionTreapNode::NONE,
            },
            Treap::Split {
                left,
                mid,
                right: left_right,
            } => Treap::Split {
                left,
                mid,
                right: OptionTreapNode::merge(left_right, right.into_node()),
            },
        }
    }

    pub fn merge_three(left: Self, mid: Self, right: Self) -> Self {
        Self::Split {
            left: left.into_node(),
            mid: Box::new(mid),
            right: right.into_node(),
        }
    }

    pub fn iter(&mut self) -> Iter<P> {
        Iter::new(self.rebuild())
    }

    pub fn into_vec(self) -> Vec<P> {
        self.into_node().with(|root| {
            let mut res = Vec::with_capacity(root.size());
            let mut rec = RecursiveFunction::new(|rec, node: OptionTreapNode<P>| {
                if let Some(node) = node.0 {
                    let node = *node;
                    rec.call(node.left);
                    res.push(node.payload);
                    rec.call(node.right);
                }
            });
            rec.call(root);
            res
        })
    }

    pub fn first(&mut self) -> Option<&P> {
        self.rebuild().first().payload()
    }

    pub fn last(&mut self) -> Option<&P> {
        self.rebuild().last().payload()
    }

    pub fn payload(&mut self) -> Option<&P> {
        self.rebuild().payload()
    }

    pub fn add(&mut self, payload: P) {
        self.replace_with(|root| Self::merge(root, Self::single(payload)));
    }
}

impl<P: PayloadWithKey> Treap<P> {
    pub fn range<'s: 'r, 'r>(&'s mut self, bounds: impl RangeBounds<&'r P::Key>) -> &mut Self {
        self.do_split(|root| {
            let (left, mid_right) = match bounds.start_bound() {
                Bound::Included(key) => root.split(key),
                Bound::Excluded(key) => root.split_inclusive(key),
                Bound::Unbounded => (OptionTreapNode::NONE, root),
            };
            let (mid, right) = match bounds.end_bound() {
                Bound::Included(key) => mid_right.split_inclusive(key),
                Bound::Excluded(key) => mid_right.split(key),
                Bound::Unbounded => (mid_right, OptionTreapNode::NONE),
            };
            (left, mid, right)
        })
    }

    pub fn insert(&mut self, p: P) -> Option<P> {
        let mid = self.range(&p.key()..=&p.key());
        let mut res = None;
        mid.replace_with(|mid| {
            if let Some(mid) = mid.into_node().0 {
                res = Some(mid.payload);
            }
            Treap::Whole {
                root: OptionTreapNode::new(TreapNode::new(p)),
            }
        });
        res
    }

    pub fn remove(&mut self, key: &P::Key) -> Option<P> {
        let mid = self.range(key..=key);
        let mut res = None;
        mid.replace_with(|mid| {
            if let Some(mid) = mid.into_node().0 {
                res = Some(mid.payload);
            }
            Treap::Whole {
                root: OptionTreapNode::NONE,
            }
        });
        res
    }

    pub fn split(mut self, key: &P::Key) -> (Self, Self) {
        let mut right_res = MaybeUninit::uninit();
        self.rebuild().replace_with(|root| {
            let (left, right) = root.split(key);
            right_res.write(Treap::Whole { root: right });
            left
        });
        (self, unsafe { right_res.assume_init() })
    }

    pub fn get(&mut self, key: &P::Key) -> Option<&mut Self> {
        self.replace_with(|self_| {
            let mut root = self_.into_node();
            match root.split_single(key) {
                Ok((left, mid, right)) => Self::Split {
                    left,
                    mid: Box::new(Self::Whole { root: mid }),
                    right,
                },
                Err(_) => Self::Whole { root },
            }
        });
        match self {
            Treap::Whole { .. } => None,
            Treap::Split { mid, .. } => Some(mid),
        }
    }

    pub fn index(&mut self, key: &P::Key) -> Option<usize> {
        self.get(key);
        match self {
            Treap::Whole { .. } => None,
            Treap::Split { left, .. } => Some(left.size()),
        }
    }

    pub fn floor(&mut self, key: &P::Key) -> Option<&P> {
        self.range(..=key).last()
    }

    pub fn ceil(&mut self, key: &P::Key) -> Option<&P> {
        self.range(key..).first()
    }

    pub fn lower(&mut self, key: &P::Key) -> Option<&P> {
        self.range(..key).last()
    }

    pub fn higher(&mut self, key: &P::Key) -> Option<&P> {
        self.range((Bound::Excluded(key), Bound::Unbounded)).first()
    }
}

struct TreapNode<P> {
    priority: u64,
    size: u32,
    reversed: bool,
    payload: P,
    left: OptionTreapNode<P>,
    right: OptionTreapNode<P>,
}

struct OptionTreapNode<P>(Option<Box<TreapNode<P>>>);

impl<P> Deref for OptionTreapNode<P> {
    type Target = Option<Box<TreapNode<P>>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P> DerefMut for OptionTreapNode<P> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P> TreapNode<P> {
    #[inline]
    fn new(payload: P) -> Self {
        Self {
            priority: random().gen(),
            size: 1,
            reversed: false,
            payload,
            left: OptionTreapNode::NONE,
            right: OptionTreapNode::NONE,
        }
    }

    #[inline]
    fn payload_and_size(&self) -> (&P, usize) {
        (&self.payload, self.size as usize)
    }
}

impl<P: Payload> TreapNode<P> {
    #[inline]
    fn update(&mut self) {
        self.size = (1 + self.left.size() + self.right.size()) as u32;
        self.payload
            .update(self.left.payload(), self.right.payload());
    }

    #[inline]
    fn push_down(&mut self) {
        self.left.push(&self.payload);
        self.right.push(&self.payload);
        self.payload.reset_delta();
        if self.reversed {
            self.left.reverse();
            self.right.reverse();
            self.reversed = false;
        }
    }
}

impl<P> OptionTreapNode<P> {
    const NONE: Self = Self(None);

    #[inline]
    fn new(node: TreapNode<P>) -> Self {
        Self(Some(Box::new(node)))
    }

    #[inline]
    fn size(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.size as usize)
    }

    #[inline]
    fn payload(&self) -> Option<&P> {
        self.0.as_ref().map(|node| &node.payload)
    }

    #[inline]
    fn payload_mut(&mut self) -> Option<&mut P> {
        self.0.as_mut().map(|node| &mut node.payload)
    }

    #[inline]
    fn push<D>(&mut self, delta: D)
    where
        P: Pushable<D>,
    {
        if let Some(p) = self.payload_mut() {
            p.push(delta)
        }
    }

    #[inline]
    fn reverse(&mut self) {
        if let Some(node) = self.0.as_mut() {
            swap(&mut node.left, &mut node.right);
            node.reversed ^= true;
        }
    }
}

impl<P: Payload> OptionTreapNode<P> {
    fn merge(left: Self, right: Self) -> Self {
        match (left.0, right.0) {
            (None, right) => Self(right),
            (left, None) => Self(left),
            (Some(mut left), Some(mut right)) => {
                if left.priority > right.priority {
                    left.push_down();
                    left.right = Self::merge(left.right, Self(Some(right)));
                    left.update();
                    Self(Some(left))
                } else {
                    right.push_down();
                    right.left = Self::merge(Self(Some(left)), right.left);
                    right.update();
                    Self(Some(right))
                }
            }
        }
    }

    fn split_first(self) -> (Self, Self) {
        if let Some(mut node) = self.0 {
            node.push_down();
            if node.left.is_none() {
                let mut right = Self::NONE;
                swap(&mut node.right, &mut right);
                node.update();
                (Self(Some(node)), right)
            } else {
                let (left, right) = node.left.split_first();
                node.left = right;
                node.update();
                (left, Self(Some(node)))
            }
        } else {
            (Self::NONE, Self::NONE)
        }
    }

    fn split_last(self) -> (Self, Self) {
        if let Some(mut node) = self.0 {
            node.push_down();
            if node.right.is_none() {
                let mut left = Self::NONE;
                swap(&mut node.left, &mut left);
                node.update();
                (left, Self(Some(node)))
            } else {
                let (left, right) = node.right.split_last();
                node.right = left;
                node.update();
                (Self(Some(node)), right)
            }
        } else {
            (Self::NONE, Self::NONE)
        }
    }

    fn split_at_single(self, at: usize) -> (Self, Self, Self) {
        if self.is_none() {
            return (Self::NONE, Self::NONE, Self::NONE);
        }
        let size = self.size();
        if at >= size {
            return (self, Self::NONE, Self::NONE);
        }
        if at == 0 {
            let (mid, right) = self.split_first();
            (Self::NONE, mid, right)
        } else if at + 1 == size {
            let (left, mid) = self.split_last();
            (left, mid, Self::NONE)
        } else if let Some(mut node) = self.0 {
            node.push_down();
            let left_size = node.left.size();
            if at == left_size {
                let mut left = Self::NONE;
                swap(&mut node.left, &mut left);
                let mut right = Self::NONE;
                swap(&mut node.right, &mut right);
                node.update();
                (left, Self(Some(node)), right)
            } else if at < left_size {
                let (left, mid, right) = node.left.split_at_single(at);
                node.left = right;
                node.update();
                (left, mid, Self(Some(node)))
            } else {
                let (left, mid, right) = node.right.split_at_single(at - left_size - 1);
                node.right = left;
                node.update();
                (Self(Some(node)), mid, right)
            }
        } else {
            unreachable!();
        }
    }

    fn split_at(self, at: usize) -> (Self, Self) {
        if at == 0 {
            (Self::NONE, self)
        } else if at >= self.size() {
            (self, Self::NONE)
        } else if at == 1 {
            self.split_first()
        } else if at + 1 == self.size() {
            self.split_last()
        } else {
            let mut node = self.0.unwrap();
            node.push_down();
            let left_size = node.left.size();
            if at <= left_size {
                let (left, right) = node.left.split_at(at);
                node.left = right;
                node.update();
                (left, Self(Some(node)))
            } else {
                let (left, right) = node.right.split_at(at - left_size - 1);
                node.right = left;
                node.update();
                (Self(Some(node)), right)
            }
        }
    }

    fn split_by<F: FnMut(&P, Option<(&P, usize)>, Option<(&P, usize)>) -> Direction>(
        self,
        mut f: F,
    ) -> (Self, Self) {
        if let Some(mut node) = self.0 {
            node.push_down();
            let direction = f(
                &node.payload,
                node.left.as_ref().map(|left| left.payload_and_size()),
                node.right.as_ref().map(|right| right.payload_and_size()),
            );
            match direction {
                Direction::Left => {
                    let (left, right) = node.left.split_by(f);
                    node.left = right;
                    node.update();
                    (left, Self(Some(node)))
                }
                Direction::Right => {
                    let (left, right) = node.right.split_by(f);
                    node.right = left;
                    node.update();
                    (Self(Some(node)), right)
                }
            }
        } else {
            (Self::NONE, Self::NONE)
        }
    }

    fn binary_search<
        F: FnMut(&P, Option<(&P, usize)>, Option<(&P, usize)>) -> Option<Direction>,
    >(
        &mut self,
        mut f: F,
    ) {
        if let Some(node) = self.deref_mut() {
            node.push_down();
            let direction = f(
                &node.payload,
                node.left.as_ref().map(|left| left.payload_and_size()),
                node.right.as_ref().map(|right| right.payload_and_size()),
            );
            match direction {
                Some(Direction::Left) => node.left.binary_search(f),
                Some(Direction::Right) => node.right.binary_search(f),
                None => {}
            }
        }
    }

    fn first(&mut self) -> &mut Self {
        if let Some(node) = self.as_ref() {
            if node.left.0.is_some() {
                if let Some(node) = self.deref_mut() {
                    node.push_down();
                    node.left.first()
                } else {
                    unreachable!()
                }
            } else {
                self
            }
        } else {
            self
        }
    }

    fn last(&mut self) -> &mut Self {
        if let Some(node) = self.as_ref() {
            if node.right.0.is_some() {
                if let Some(node) = self.deref_mut() {
                    node.push_down();
                    node.right.last()
                } else {
                    unreachable!()
                }
            } else {
                self
            }
        } else {
            self
        }
    }
}

impl<P: PayloadWithKey> OptionTreapNode<P> {
    fn split_single(&mut self, key: &P::Key) -> Result<(Self, Self, Self), ()> {
        if let Some(node) = self.0.as_mut() {
            node.push_down();
            match node.payload.key().cmp(key) {
                Ordering::Less => {
                    let (left, mid, right) = node.right.split_single(key)?;
                    node.right = left;
                    node.update();
                    let mut left = Self::NONE;
                    swap(self, &mut left);
                    Ok((left, mid, right))
                }
                Ordering::Equal => {
                    let mut left = Self::NONE;
                    swap(&mut node.left, &mut left);
                    let mut right = Self::NONE;
                    swap(&mut node.right, &mut right);
                    node.update();
                    let mut mid = Self::NONE;
                    swap(self, &mut mid);
                    Ok((left, mid, right))
                }
                Ordering::Greater => {
                    let (left, mid, right) = node.left.split_single(key)?;
                    node.left = right;
                    node.update();
                    let mut right = Self::NONE;
                    swap(self, &mut right);
                    Ok((left, mid, right))
                }
            }
        } else {
            Err(())
        }
    }

    fn split(self, key: &P::Key) -> (Self, Self) {
        self.split_by(|payload, _left, _right| {
            if payload.key() < key {
                Direction::Right
            } else {
                Direction::Left
            }
        })
    }

    fn split_inclusive(self, key: &P::Key) -> (Self, Self) {
        self.split_by(|payload, _left, _right| {
            if payload.key() <= key {
                Direction::Right
            } else {
                Direction::Left
            }
        })
    }
}
