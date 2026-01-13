use crate::collections::payload::{OrdPayload, Payload};
use crate::misc::direction::Direction;
use crate::misc::random::{RandomTrait, StaticRandom};
use std::collections::Bound;
use std::marker::PhantomPinned;
use std::mem::{forget, swap, take};
use std::ops::{Deref, DerefMut, RangeBounds};
use std::ptr::NonNull;

pub struct PersistentContent<P> {
    payload: P,
    left: PersistentTreapNode<P>,
    right: PersistentTreapNode<P>,
}

impl<P: Payload + Clone> PersistentContent<P> {
    fn push_down(&mut self) {
        if P::NEED_ACCUMULATE && self.payload.need_push_down() {
            if self.left.size != 0 {
                self.left.payload_mut().accumulate(&self.payload);
            }
            if self.right.size != 0 {
                self.right.payload_mut().accumulate(&self.payload);
            }
            self.payload.reset_delta();
        }
    }

    fn update(&mut self) {
        if P::NEED_UPDATE {
            self.payload
                .update(self.left.payload_ref(), self.right.payload_ref());
        }
    }
}

pub struct PersistentNode<P> {
    priority: u64,
    size: u32,
    reversed: bool,
    content: Option<PersistentContent<P>>,
    _phantom_pinned: PhantomPinned,
}

impl<P: Clone> PersistentNode<P> {
    const NULL_NODE: Self = Self {
        priority: 0,
        size: 0,
        reversed: false,
        content: None,
        _phantom_pinned: PhantomPinned,
    };

    fn payload(&self) -> Option<P> {
        self.content.as_ref().map(|c| c.payload.clone())
    }

    fn payload_ref(&self) -> Option<&P> {
        self.content.as_ref().map(|c| &c.payload)
    }

    fn payload_mut(&mut self) -> &mut P {
        unsafe { &mut self.content.as_mut().unwrap_unchecked().payload }
    }

    fn unreverse(&mut self) {
        if self.reversed {
            if let Some(content) = &mut self.content {
                self.reversed = false;
                swap(&mut content.left, &mut content.right);
                content.left.reversed ^= true;
                content.right.reversed ^= true;
            }
        }
    }
}

impl<P: Payload + Clone> PersistentNode<P> {
    fn update(&mut self) {
        self.size = 1 + self.left.size + self.right.size;
        self.deref_mut().update();
    }

    fn push_down(&mut self) {
        self.unreverse();
        self.deref_mut().push_down();
    }

    fn detach_left(&mut self) -> PersistentTreapNode<P> {
        self.push_down();
        let left = take(&mut self.left);
        left
    }

    fn detach_right(&mut self) -> PersistentTreapNode<P> {
        self.push_down();
        let right = take(&mut self.right);
        right
    }

    fn attach_left(&mut self, left: PersistentTreapNode<P>) {
        assert_eq!(self.left.size, 0);
        if left.size != 0 {
            self.left = left;
        }
        self.update();
    }

    fn attach_right(&mut self, right: PersistentTreapNode<P>) {
        assert_eq!(self.right.size, 0);
        if right.size != 0 {
            self.right = right;
        }
        self.update();
    }

    fn heapify(&mut self) {
        if self.left.size != 0 {
            self.left.heapify();
            if self.left.priority > self.priority {
                let left_priority = self.left.priority;
                self.left.priority = self.priority;
                self.priority = left_priority;
            }
        }
        if self.right.size != 0 {
            self.right.heapify();
            if self.right.priority > self.priority {
                let right_priority = self.right.priority;
                self.right.priority = self.priority;
                self.priority = right_priority;
            }
        }
    }

    fn with_gen(n: usize, f: impl FnMut(usize) -> P) -> PersistentTreapNode<P> {
        let mut res = Self::build(f, 0, n).0;
        if res.size != 0 {
            res.heapify();
        }
        res
    }

    fn build<F: FnMut(usize) -> P>(
        mut f: F,
        from: usize,
        to: usize,
    ) -> (PersistentTreapNode<P>, F) {
        if from == to {
            (PersistentTreapNode::default(), f)
        } else {
            let mid = StaticRandom.gen_range(from..to);
            let mut node = PersistentTreapNode::new(f(mid));
            let (left, f) = Self::build(f, from, mid);
            let (right, f) = Self::build(f, mid + 1, to);
            node.attach_left(left);
            node.attach_right(right);
            (node, f)
        }
    }

    fn first(&mut self) -> &PersistentNode<P> {
        if self.size == 0 {
            &Self::NULL_NODE
        } else {
            self.push_down();
            if self.left.size != 0 {
                self.left.first()
            } else {
                self
            }
        }
    }

    fn last(&mut self) -> &PersistentNode<P> {
        if self.size == 0 {
            &Self::NULL_NODE
        } else {
            self.push_down();
            if self.right.size != 0 {
                self.right.last()
            } else {
                self
            }
        }
    }

    fn binary_search<F: FnMut(&P, Option<&P>, Option<&P>) -> Option<Direction>>(
        &mut self,
        mut f: F,
    ) {
        if self.size != 0 {
            self.push_down();
            let direction = f(
                &self.payload,
                self.left.payload_ref(),
                self.right.payload_ref(),
            );
            match direction {
                Some(Direction::Left) => self.left.binary_search(f),
                Some(Direction::Right) => self.right.binary_search(f),
                None => {}
            }
        }
    }

    fn size(&self) -> usize {
        self.size as usize
    }

    fn binary_search_size<F: FnMut(usize, usize) -> Option<Direction>>(&mut self, mut f: F) {
        if self.size != 0 {
            self.push_down();
            let direction = f(self.left.size(), self.right.size());
            match direction {
                Some(Direction::Left) => self.left.binary_search_size(f),
                Some(Direction::Right) => self.right.binary_search_size(f),
                None => {}
            }
        }
    }
}

impl<P> Deref for PersistentNode<P> {
    type Target = PersistentContent<P>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.content.as_ref().unwrap_unchecked() }
    }
}

impl<P> DerefMut for PersistentNode<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.content.as_mut().unwrap_unchecked() }
    }
}

pub struct PersistentTreapNode<P> {
    link: NonNull<PersistentNode<P>>,
}

impl<P: Payload + Clone> PersistentTreapNode<P> {
    fn new(payload: P) -> Self {
        let node = PersistentNode {
            priority: StaticRandom.gen_int(),
            size: 1,
            reversed: false,
            content: Some(PersistentContent {
                payload,
                left: PersistentTreapNode::default(),
                right: PersistentTreapNode::default(),
            }),
            _phantom_pinned: PhantomPinned,
        };
        let mut pinned = Box::pin(node);
        let res = PersistentTreapNode {
            link: unsafe { NonNull::from(pinned.as_mut().get_unchecked_mut()) },
        };
        forget(pinned);
        res
    }

    fn copy(&self) -> Self {
        let mut self_ = self.clone();
        self_.push_down();
        let node = PersistentNode {
            priority: self.priority,
            size: self.size,
            reversed: self.reversed,
            content: self.content.as_ref().map(|c| PersistentContent {
                payload: c.payload.clone(),
                left: c.left.clone(),
                right: c.right.clone(),
            }),
            _phantom_pinned: PhantomPinned,
        };
        let mut pinned = Box::pin(node);
        let res = PersistentTreapNode {
            link: unsafe { NonNull::from(pinned.as_mut().get_unchecked_mut()) },
        };
        forget(pinned);
        res
    }

    fn merge(left: &Self, right: &Self) -> Self {
        match (left.size, right.size) {
            (0, _) => right.clone(),
            (_, 0) => left.clone(),
            _ => {
                if left.priority > right.priority {
                    let mut left = left.copy();
                    let left_right = left.detach_right();
                    left.attach_right(Self::merge(&left_right, right));
                    left
                } else {
                    let mut right = right.copy();
                    let right_left = right.detach_left();
                    right.attach_left(Self::merge(left, &right_left));
                    right
                }
            }
        }
    }

    fn split_by<F: FnMut(&P, Option<&P>, Option<&P>) -> Direction>(
        &self,
        mut f: F,
    ) -> (Self, Self) {
        if self.size != 0 {
            let mut self_ = self.copy();
            let direction = f(
                &self_.payload,
                self_.left.payload_ref(),
                self_.right.payload_ref(),
            );
            match direction {
                Direction::Left => {
                    let (left, right) = self_.detach_left().split_by(f);
                    self_.attach_left(right);
                    (left, self_)
                }
                Direction::Right => {
                    let (left, right) = self_.detach_right().split_by(f);
                    self_.attach_right(left);
                    (self_, right)
                }
            }
        } else {
            (
                PersistentTreapNode::default(),
                PersistentTreapNode::default(),
            )
        }
    }

    fn split_by_size<F: FnMut(usize, usize) -> Direction>(&self, mut f: F) -> (Self, Self) {
        if self.size != 0 {
            let mut self_ = self.copy();
            let direction = f(self_.left.size(), self_.right.size());
            match direction {
                Direction::Left => {
                    let (left, right) = self_.detach_left().split_by_size(f);
                    self_.attach_left(right);
                    (left, self_)
                }
                Direction::Right => {
                    let (left, right) = self_.detach_right().split_by_size(f);
                    self_.attach_right(left);
                    (self_, right)
                }
            }
        } else {
            (Self::default(), Self::default())
        }
    }

    fn split_at(&self, mut at: usize) -> (Self, Self) {
        self.split_by_size(|left_size, _| {
            if at <= left_size {
                Direction::Left
            } else {
                at -= left_size + 1;
                Direction::Right
            }
        })
    }
}

impl<P: OrdPayload + Clone> PersistentTreapNode<P> {
    fn split(&self, key: &P::Key) -> (Self, Self) {
        self.split_by(|payload, _left, _right| {
            if payload.key() < key {
                Direction::Right
            } else {
                Direction::Left
            }
        })
    }

    fn split_inclusive(&self, key: &P::Key) -> (Self, Self) {
        self.split_by(|payload, _left, _right| {
            if payload.key() <= key {
                Direction::Right
            } else {
                Direction::Left
            }
        })
    }

    fn union(a: &Self, b: &Self) -> Self {
        match (a.size, b.size) {
            (0, _) => b.clone(),
            (_, 0) => a.clone(),
            _ => {
                if a.priority < b.priority {
                    return Self::union(b, a);
                }
                let (b_left, b_right) = b.split(a.payload.key());
                let (same, b_right) = b_right.split_inclusive(a.payload.key());
                let mut a = a.copy();
                let left = a.detach_left();
                let right = a.detach_right();
                if same.size != 0 {
                    a = PersistentTreapNode::new(P::union(
                        a.payload().unwrap(),
                        same.payload().unwrap(),
                    ));
                }
                let left = Self::union(&left, &b_left);
                let right = Self::union(&right, &b_right);
                a.attach_left(left);
                a.attach_right(right);
                a
            }
        }
    }
}

impl<P> Clone for PersistentTreapNode<P> {
    fn clone(&self) -> Self {
        PersistentTreapNode { link: self.link }
    }
}

impl<P> Copy for PersistentTreapNode<P> {}

impl<P> PartialEq for PersistentTreapNode<P> {
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
    }
}

impl<P> Eq for PersistentTreapNode<P> {}

impl<P> Deref for PersistentTreapNode<P> {
    type Target = PersistentNode<P>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.link.as_ref() }
    }
}

impl<P> DerefMut for PersistentTreapNode<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.link.as_mut() }
    }
}

impl<P: Clone> Default for PersistentTreapNode<P> {
    fn default() -> Self {
        PersistentTreapNode {
            link: unsafe {
                NonNull::new_unchecked(
                    &PersistentNode::NULL_NODE as *const PersistentNode<P>
                        as *mut PersistentNode<P>,
                )
            },
        }
    }
}

pub struct PersistentTree<P> {
    root: PersistentTreapNode<P>,
}

impl<P: Payload + Clone> Default for PersistentTree<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Payload + Clone> PersistentTree<P> {
    pub fn new() -> Self {
        Self {
            root: PersistentTreapNode::default(),
        }
    }

    pub fn with_gen(n: usize, f: impl FnMut(usize) -> P) -> Self {
        Self::gen_impl(n, f)
    }

    pub fn single(p: P) -> Self {
        Self {
            root: PersistentTreapNode::new(p),
        }
    }

    fn gen_impl(n: usize, f: impl FnMut(usize) -> P) -> Self {
        Self {
            root: PersistentNode::with_gen(n, f),
        }
    }

    fn do_split(
        self,
        f: impl FnOnce(PersistentTreapNode<P>) -> (PersistentTreapNode<P>, PersistentTreapNode<P>),
    ) -> (Self, Self) {
        let (left, right) = f(self.root);
        (Self { root: left }, Self { root: right })
    }

    fn do_split_three(
        self,
        f: impl FnOnce(
            PersistentTreapNode<P>,
        ) -> (
            PersistentTreapNode<P>,
            PersistentTreapNode<P>,
            PersistentTreapNode<P>,
        ),
    ) -> Self {
        Self {
            root: f(self.root).1,
        }
    }

    pub fn split_by(self, f: impl FnMut(&P, Option<&P>, Option<&P>) -> Direction) -> (Self, Self) {
        let (left, right) = self.root.split_by(f);
        (Self { root: left }, Self { root: right })
    }

    pub fn binary_search(mut self, f: impl FnMut(&P, Option<&P>, Option<&P>) -> Option<Direction>) {
        self.root.binary_search(f);
    }

    pub fn push(self, delta: &P) -> Self {
        self.with_payload_mut(|p| p.accumulate(delta))
    }

    pub fn with_payload_mut(self, f: impl FnOnce(&mut P)) -> Self {
        if self.is_empty() {
            return self;
        }
        let mut copy = Self {
            root: self.root.copy(),
        };
        f(copy.root.payload_mut());
        copy
    }

    pub fn merge(left: Self, right: Self) -> Self {
        Self {
            root: PersistentTreapNode::merge(&left.root, &right.root),
        }
    }

    pub fn merge_three(left: Self, mid: Self, right: Self) -> Self {
        Self::merge(Self::merge(left, mid), right)
    }

    pub fn iter(self) -> Iter<P> {
        Iter::new(&self.root)
    }

    pub fn first(mut self) -> Option<P> {
        self.root.first().payload()
    }

    pub fn last(mut self) -> Option<P> {
        self.root.last().payload()
    }

    pub fn payload(self) -> Option<P> {
        self.root.payload()
    }

    pub fn is_empty(&self) -> bool {
        self.root.size == 0
    }

    pub fn size(&self) -> usize {
        self.root.size as usize
    }

    pub fn split_at(self, at: usize) -> (Self, Self) {
        self.do_split(|root| root.split_at(at))
    }

    pub fn binary_search_size(mut self, f: impl FnMut(usize, usize) -> Option<Direction>) {
        self.root.binary_search_size(f);
    }

    pub fn range_index(self, bounds: impl RangeBounds<usize>) -> Self {
        self.do_split_three(|root| {
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
            assert!(start <= end);
            let (left, mid_right) = root.split_at(start);
            let (mid, right) = mid_right.split_at(end.max(start) - start);
            (left, mid, right)
        })
    }

    pub fn get_at(&mut self, at: usize) -> Self {
        self.range_index(at..=at)
    }

    pub fn reverse(self) -> Self {
        if self.is_empty() {
            return self;
        }
        let mut copy = Self {
            root: self.root.copy(),
        };
        copy.root.reversed ^= true;
        copy
    }
}

impl<P: Payload + Clone> Clone for PersistentTree<P> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
        }
    }
}

impl<P: Payload + Clone> Copy for PersistentTree<P> {}

pub struct Iter<P> {
    stack: Vec<PersistentTreapNode<P>>,
}

impl<P: Payload + Clone> Iter<P> {
    fn new(root: &PersistentTreapNode<P>) -> Self {
        let mut res = Self { stack: Vec::new() };
        res.add_left(root);
        res
    }

    fn add_left(&mut self, node: &PersistentTreapNode<P>) {
        let mut node = node.clone();
        while node.size != 0 {
            node.push_down();
            let cur_node = node;
            node = cur_node.left.clone();
            self.stack.push(cur_node);
        }
    }
}

impl<P: Payload + Clone> Iterator for Iter<P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.add_left(&node.right);
        Some(node.payload.clone())
    }
}

impl<P: OrdPayload + Copy> PersistentTree<P> {
    pub fn range<'a>(self, bounds: impl RangeBounds<&'a P::Key>) -> Self
    where
        <P as OrdPayload>::Key: 'a,
    {
        self.do_split_three(|root| {
            let (left, mid_right) = match bounds.start_bound() {
                Bound::Included(key) => root.split(key),
                Bound::Excluded(key) => root.split_inclusive(key),
                Bound::Unbounded => (PersistentTreapNode::default(), root),
            };
            let (mid, right) = match bounds.end_bound() {
                Bound::Included(key) => mid_right.split_inclusive(key),
                Bound::Excluded(key) => mid_right.split(key),
                Bound::Unbounded => (mid_right, PersistentTreapNode::default()),
            };
            (left, mid, right)
        })
    }

    pub fn insert_or_update(&mut self, p: P) -> Self {
        let (left, right) = self.split(&p.key());
        let (mut mid, right) = right.split_inclusive(&p.key());
        if mid.payload().is_some() {
            mid = mid.with_payload_mut(|payload| {
                *payload = P::union(*payload, p);
            });
        } else {
            mid = PersistentTree::single(p);
        }
        Self::merge_three(left, mid, right)
    }

    pub fn insert(self, p: P) -> (Self, Option<P>) {
        let (left, right) = self.split(&p.key());
        let (mid, right) = right.split_inclusive(&p.key());
        (
            Self::merge_three(left, PersistentTree::single(p), right),
            mid.payload(),
        )
    }

    pub fn remove(self, key: &P::Key) -> (Self, Option<P>) {
        let (left, right) = self.split(key);
        let (mid, right) = right.split_inclusive(key);
        (
            Self::merge_three(left, PersistentTree::new(), right),
            mid.payload(),
        )
    }

    pub fn split(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split(key))
    }

    pub fn split_inclusive(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split_inclusive(key))
    }

    pub fn get(&mut self, key: &P::Key) -> Option<P> {
        self.range(key..=key).payload()
    }

    pub fn floor(&mut self, key: &P::Key) -> Option<P> {
        self.range(..=key).last()
    }

    pub fn ceil(&mut self, key: &P::Key) -> Option<P> {
        self.range(key..).first()
    }

    pub fn prev(&mut self, key: &P::Key) -> Option<P> {
        self.range(..key).last()
    }

    pub fn next(&mut self, key: &P::Key) -> Option<P> {
        self.range((Bound::Excluded(key), Bound::Unbounded)).first()
    }

    pub fn union(a: Self, b: Self) -> Self {
        Self {
            root: PersistentTreapNode::union(&a.root, &b.root),
        }
    }

    pub fn index(self, key: &P::Key) -> Option<usize> {
        let (left, right) = self.split(key);
        let (mid, _) = right.split_inclusive(key);
        mid.payload().map(|_| left.size())
    }
}
