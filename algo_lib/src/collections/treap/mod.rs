pub mod multi_payload;
pub mod multi_treap_set;
pub mod payload;
pub mod pure_payload;
pub mod treap_map;

use crate::collections::treap::payload::{OrdPayload, Payload, Pushable, Reverse, Size};
use crate::misc::direction::Direction;
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::misc::random::random;
use std::collections::Bound;
use std::marker::PhantomPinned;
use std::mem;
use std::mem::{swap, MaybeUninit};
use std::ops::{Deref, DerefMut, RangeBounds};
use std::pin::Pin;

pub struct Iter<'a, P, S, R> {
    stack: Vec<*mut Node<P, S, R>>,
    _marker: std::marker::PhantomData<&'a P>,
}

impl<'a, P: Payload + Unpin, S: Size, R: Reverse> Iter<'a, P, S, R> {
    fn new(root: &'a mut OptionNode<P, S, R>) -> Self {
        let mut res = Self {
            stack: Vec::new(),
            _marker: std::marker::PhantomData,
        };
        res.add_left(root);
        res
    }

    fn add_left(&mut self, mut node: &mut OptionNode<P, S, R>) {
        while let Some(n) = node.0.as_mut() {
            n.push_down();
            self.stack.push(n);
            node = &mut n.inner().left;
        }
    }
}

impl<'a, P: Payload + Unpin, S: Size + 'a, R: Reverse + 'a> Iterator for Iter<'a, P, S, R> {
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        let node = unsafe { &mut *self.stack.pop()? };
        self.add_left(&mut node.inner().right);
        Some(&node.payload)
    }
}

#[allow(private_interfaces)]
pub enum Treap<P, S = (), R = ()> {
    Whole {
        root: OptionNode<P, S, R>,
    },
    Split {
        left: OptionNode<P, S, R>,
        mid: Box<Treap<P, S, R>>,
        right: OptionNode<P, S, R>,
    },
}

impl<P: Payload + Unpin> Treap<P> {
    pub fn new() -> Self {
        Treap::Whole {
            root: OptionNode::NONE,
        }
    }

    pub fn gen(n: usize, f: impl Fn(usize) -> P) -> Self {
        Self::gen_impl(n, f)
    }
}

impl<P: Payload + Unpin> Treap<P, u32> {
    pub fn sized() -> Self {
        Treap::Whole {
            root: OptionNode::NONE,
        }
    }

    pub fn gen_sized(n: usize, f: impl Fn(usize) -> P) -> Self {
        Self::gen_impl(n, f)
    }
}

impl<P: Payload + Unpin> Treap<P, u32, bool> {
    pub fn reversible() -> Self {
        Treap::Whole {
            root: OptionNode::NONE,
        }
    }

    pub fn gen_reversible(n: usize, f: impl Fn(usize) -> P) -> Self {
        Self::gen_impl(n, f)
    }
}

impl<P: Payload + Unpin, S: Size, R: Reverse> Default for Treap<P, S, R> {
    fn default() -> Self {
        Treap::Whole {
            root: OptionNode::NONE,
        }
    }
}

impl<P: Payload + Unpin, S: Size, R: Reverse> Treap<P, S, R> {
    fn single(p: P) -> Self {
        Treap::Whole {
            root: OptionNode::new(Node::new(p)),
        }
    }

    fn gen_impl(n: usize, f: impl Fn(usize) -> P) -> Self {
        Treap::Whole {
            root: OptionNode::gen(n, f),
        }
    }

    #[inline]
    fn into_node(mut self) -> OptionNode<P, S, R> {
        self.rebuild();
        match self {
            Treap::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    #[inline]
    unsafe fn as_node(&mut self) -> &mut OptionNode<P, S, R> {
        match self {
            Treap::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    fn rebuild(&mut self) -> &mut OptionNode<P, S, R> {
        self.replace_with(|self_| {
            if let Treap::Split { left, mid, right } = self_ {
                Treap::Whole {
                    root: OptionNode::merge(left, OptionNode::merge(mid.into_node(), right)),
                }
            } else {
                self_
            }
        });
        unsafe { self.as_node() }
    }

    fn do_split(
        mut self,
        f: impl FnOnce(OptionNode<P, S, R>) -> (OptionNode<P, S, R>, OptionNode<P, S, R>),
    ) -> (Self, Self) {
        let mut right = MaybeUninit::uninit();
        self.replace_with(|self_| {
            let root = self_.into_node();
            let (left, right_) = f(root);
            right.write(Treap::Whole { root: right_ });
            Treap::Whole { root: left }
        });
        (self, unsafe { right.assume_init() })
    }

    fn do_split_three(
        &mut self,
        f: impl FnOnce(
            OptionNode<P, S, R>,
        ) -> (
            OptionNode<P, S, R>,
            OptionNode<P, S, R>,
            OptionNode<P, S, R>,
        ),
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
        self.as_mid()
    }

    fn as_mid(&mut self) -> &mut Self {
        match self {
            Treap::Split { mid, .. } => mid,
            _ => unreachable!(),
        }
    }

    pub fn push_front(&mut self, other: Self) -> &mut Self {
        self.replace_with(|root| Treap::Whole {
            root: OptionNode::merge(other.into_node(), root.into_node()),
        });
        self
    }

    pub fn push_back(&mut self, other: Self) -> &mut Self {
        self.replace_with(|root| Treap::Whole {
            root: OptionNode::merge(root.into_node(), other.into_node()),
        });
        self
    }

    pub fn detach(&mut self) -> Self {
        match self {
            Treap::Whole { root } => {
                let mut res = OptionNode::NONE;
                swap(&mut res, root);
                Treap::Whole { root: res }
            }
            Treap::Split { mid, .. } => {
                let mut res = Self::default();
                swap(&mut res, mid);
                res
            }
        }
    }

    pub fn binary_search(
        &mut self,
        f: impl FnMut(&P, Option<&P>, Option<&P>) -> Option<Direction>,
    ) {
        self.rebuild().binary_search(f);
    }

    pub fn push<D>(&mut self, delta: D)
    where
        P: Pushable<D>,
    {
        self.rebuild().push(delta);
    }

    pub fn merge(left: Self, right: Self) -> Self {
        match left {
            Treap::Whole { root: left_root } => Treap::Split {
                left: left_root,
                mid: Box::new(right),
                right: OptionNode::NONE,
            },
            Treap::Split {
                left,
                mid,
                right: left_right,
            } => Treap::Split {
                left,
                mid,
                right: OptionNode::merge(left_right, right.into_node()),
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

    pub fn iter<'a>(&'a mut self) -> Iter<'a, P, S, R>
    where
        R: 'a,
        S: 'a,
    {
        Iter::new(self.rebuild())
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

    pub fn add_back(&mut self, payload: P) -> NodeRef<P, S, R> {
        let mut res = None;
        self.replace_with(|root| unsafe {
            let mut new_node = Self::single(payload);
            res = Some(new_node.as_node().as_mut().unwrap().inner() as *mut _);
            Self::merge(root, new_node)
        });
        NodeRef(res.unwrap())
    }

    pub fn add_front(&mut self, payload: P) -> NodeRef<P, S, R> {
        let mut res = None;
        self.replace_with(|root| unsafe {
            let mut new_node = Self::single(payload);
            res = Some(new_node.as_node().as_mut().unwrap().inner() as *mut _);
            Self::merge(new_node, root)
        });
        NodeRef(res.unwrap())
    }

    pub fn refs(&mut self) -> Vec<NodeRef<P, S, R>> {
        let mut res = Vec::new();
        self.rebuild().refs(&mut res);
        res
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Treap::Whole { root } => root.is_none(),
            Treap::Split { left, mid, right } => {
                left.is_none() && right.is_none() && mid.is_empty()
            }
        }
    }

    pub fn raise(&mut self, node_ref: &NodeRef<P, S, R>) -> &mut Self {
        self.do_split_three(|node| node.raise(node_ref.0))
    }
}

impl<P: Payload + Unpin, R: Reverse> Treap<P, u32, R> {
    pub fn index_ref(&mut self, node_ref: &NodeRef<P, u32, R>) -> usize {
        self.raise(node_ref);
        match self {
            Treap::Whole { .. } => unreachable!(),
            Treap::Split { left, .. } => left.size(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Treap::Whole { root } => root.size(),
            Treap::Split { left, mid, right } => left.size() + mid.size() + right.size(),
        }
    }

    pub fn split_at(self, at: usize) -> (Self, Self) {
        self.do_split(|root| root.split_at(at))
    }

    pub fn binary_search_size(&mut self, f: impl FnMut(usize, usize) -> Option<Direction>) {
        self.rebuild().binary_search_size(f);
    }

    pub fn range_index(&mut self, bounds: impl RangeBounds<usize>) -> &mut Self {
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

    pub fn get_at(&mut self, at: usize) -> &mut Self {
        self.range_index(at..=at)
    }
}

impl<P: Payload + Unpin, S: Size> Treap<P, S, bool> {
    pub fn reverse(&mut self) {
        self.rebuild().reverse();
    }
}

impl<P: OrdPayload + Unpin, S: Size, R: Reverse> Treap<P, S, R> {
    pub fn range<'s: 'r, 'r>(&'s mut self, bounds: impl RangeBounds<&'r P::Key>) -> &mut Self {
        self.do_split_three(|root| {
            let (left, mid_right) = match bounds.start_bound() {
                Bound::Included(key) => root.split(key),
                Bound::Excluded(key) => root.split_inclusive(key),
                Bound::Unbounded => (OptionNode::NONE, root),
            };
            let (mid, right) = match bounds.end_bound() {
                Bound::Included(key) => mid_right.split_inclusive(key),
                Bound::Excluded(key) => mid_right.split(key),
                Bound::Unbounded => (mid_right, OptionNode::NONE),
            };
            (left, mid, right)
        })
    }

    pub fn insert(&mut self, p: P) -> Option<P> {
        let mid = self.range(&p.key()..=&p.key());
        let mut res = None;
        mid.replace_with(|mid| {
            if let Some(mid) = mid.into_node().0 {
                res = Some(mid.into_payload());
            }
            Treap::Whole {
                root: OptionNode::new(Node::new(p)),
            }
        });
        res
    }

    pub fn remove(&mut self, key: &P::Key) -> Option<P> {
        let mid = self.range(key..=key);
        let mut res = None;
        mid.replace_with(|mid| {
            if let Some(mid) = mid.into_node().0 {
                res = Some(mid.into_payload());
            }
            Treap::Whole {
                root: OptionNode::NONE,
            }
        });
        res
    }

    pub fn split(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split(key))
    }

    pub fn split_inclusive(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split_inclusive(key))
    }

    pub fn get(&mut self, key: &P::Key) -> Option<&P> {
        self.range(key..=key).payload()
    }

    pub fn floor(&mut self, key: &P::Key) -> Option<&P> {
        self.range(..=key).last()
    }

    pub fn ceil(&mut self, key: &P::Key) -> Option<&P> {
        self.range(key..).first()
    }

    pub fn prev(&mut self, key: &P::Key) -> Option<&P> {
        self.range(..key).last()
    }

    pub fn next(&mut self, key: &P::Key) -> Option<&P> {
        self.range((Bound::Excluded(key), Bound::Unbounded)).first()
    }

    pub fn union(a: Self, b: Self) -> Self {
        let a = a.into_node();
        let b = b.into_node();
        Self::Whole {
            root: OptionNode::union(a, b),
        }
    }
}

impl<P: OrdPayload + Unpin, R: Reverse> Treap<P, u32, R> {
    pub fn index(&mut self, key: &P::Key) -> Option<usize> {
        match self.range(key..=key) {
            Treap::Split { left, mid, .. } => mid.payload().map(|_| left.size()),
            _ => unreachable!(),
        }
    }
}

pub struct NodeRef<P, S, R>(*mut NodeInner<P, S, R>);

struct NodeInner<P, S, R> {
    priority: u64,
    size: S,
    reversed: R,
    payload: P,
    left: OptionNode<P, S, R>,
    right: OptionNode<P, S, R>,
    parent: Option<*mut NodeInner<P, S, R>>,
    _phantom_pinned: PhantomPinned,
}

impl<P, S, R> NodeInner<P, S, R> {
    fn into_payload(self: Pin<Box<Self>>) -> P {
        unsafe { Pin::into_inner_unchecked(self).payload }
    }
}

struct Node<P, S, R> {
    inner: Pin<Box<NodeInner<P, S, R>>>,
}

impl<P, S, R> Deref for Node<P, S, R> {
    type Target = Pin<Box<NodeInner<P, S, R>>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P, S, R> DerefMut for Node<P, S, R> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<P: Payload + Unpin, S: Size, R: Reverse> Node<P, S, R> {
    #[inline(always)]
    fn new(payload: P) -> Self {
        Self {
            inner: Box::pin(NodeInner {
                priority: random().gen(),
                size: S::ONE,
                reversed: R::default(),
                payload,
                left: OptionNode::NONE,
                right: OptionNode::NONE,
                parent: None,
                _phantom_pinned: PhantomPinned,
            }),
        }
    }

    #[inline(always)]
    fn into_payload(self) -> P {
        assert!(self.parent.is_none());
        assert!(self.left.is_none());
        assert!(self.right.is_none());
        self.inner.into_payload()
    }

    #[inline(always)]
    fn inner(&mut self) -> &mut NodeInner<P, S, R> {
        unsafe { self.inner.as_mut().get_unchecked_mut() }
    }

    #[inline(always)]
    fn update(&mut self) {
        let this = self.inner();
        if S::NEED_UPDATE {
            this.size
                .update(this.left.size_opt(), this.right.size_opt());
        }
        if P::NEED_UPDATE {
            this.payload
                .update(this.left.payload(), this.right.payload());
        }
    }

    #[inline(always)]
    fn update_leaf(&mut self) {
        let this = self.inner();
        if S::NEED_UPDATE {
            this.size = S::ONE;
        }
        if P::NEED_UPDATE {
            this.payload.update(None, None);
        }
    }

    #[inline(always)]
    fn push_down(&mut self) {
        if P::NEED_PUSH_DOWN && self.payload.need_push_down() {
            let this = self.inner();
            this.left.push(&this.payload);
            this.right.push(&this.payload);
            this.payload.reset_delta();
        }
        if self.reversed.need_reverse() {
            let this = self.inner();
            this.left.reverse();
            this.right.reverse();
            this.reversed.reset_reverse();
        }
    }

    #[inline(always)]
    fn detach_left(&mut self) -> OptionNode<P, S, R> {
        self.push_down();
        let this = self.inner();
        let mut left = mem::replace(&mut this.left, OptionNode::NONE);
        left.detach();
        left
    }

    #[inline(always)]
    fn detach_right(&mut self) -> OptionNode<P, S, R> {
        self.push_down();
        let this = self.inner();
        let mut right = mem::replace(&mut this.right, OptionNode::NONE);
        right.detach();
        right
    }

    #[inline(always)]
    fn attach_left(&mut self, mut left: OptionNode<P, S, R>) {
        if let Some(left) = left.as_mut() {
            left.inner().parent = Some(self.as_inner_ptr());
        }
        let this = self.inner();
        this.left = left;
        self.update();
    }

    #[inline(always)]
    fn attach_right(&mut self, mut right: OptionNode<P, S, R>) {
        if let Some(right) = right.as_mut() {
            right.inner().parent = Some(self.as_inner_ptr());
        }
        let this = self.inner();
        this.right = right;
        self.update();
    }

    #[inline(always)]
    fn as_inner_ptr(&mut self) -> *mut NodeInner<P, S, R> {
        self.inner()
    }
}

struct OptionNode<P, S, R>(Option<Node<P, S, R>>);

impl<P, S, R> Deref for OptionNode<P, S, R> {
    type Target = Option<Node<P, S, R>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P, S, R> DerefMut for OptionNode<P, S, R> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P: Payload + Unpin, S: Size, R: Reverse> OptionNode<P, S, R> {
    const NONE: Self = Self(None);

    #[inline(always)]
    fn new(node: Node<P, S, R>) -> Self {
        Self(Some(node))
    }

    fn gen(n: usize, f: impl FnMut(usize) -> P) -> Self {
        let mut res = Self::build(f, 0, n).0;
        res.heapify();
        res
    }

    fn build<F: FnMut(usize) -> P>(mut f: F, from: usize, to: usize) -> (Self, F) {
        if from == to {
            (OptionNode::NONE, f)
        } else {
            let mid = random().next_bounds(from, to - 1);
            let mut node = Node::new(f(mid));
            let (left, f) = Self::build(f, from, mid);
            let (right, f) = Self::build(f, mid + 1, to);
            node.attach_left(left);
            node.attach_right(right);
            (OptionNode::new(node), f)
        }
    }

    fn heapify(&mut self) {
        if let Some(node) = self.as_mut() {
            let this = node.inner();
            this.left.heapify();
            this.right.heapify();
            let mut priority = this.priority;
            if let Some(left) = this.left.as_mut() {
                if left.priority > priority {
                    let was_priority = priority;
                    priority = left.priority;
                    left.inner().priority = was_priority;
                }
            }
            if let Some(right) = this.right.as_mut() {
                if right.priority > priority {
                    let was_priority = priority;
                    priority = right.priority;
                    right.inner().priority = was_priority;
                }
            }
            this.priority = priority;
        }
    }

    #[inline(always)]
    fn detach(&mut self) {
        if let Some(node) = self.0.as_mut() {
            node.inner().parent = None;
        }
    }

    #[inline(always)]
    fn size_opt(&self) -> Option<&S> {
        self.as_ref().map(|node| &node.size)
    }

    #[inline(always)]
    fn payload(&self) -> Option<&P> {
        self.as_ref().map(|node| &node.payload)
    }

    #[inline(always)]
    fn payload_mut(&mut self) -> Option<&mut P> {
        self.as_mut().map(|node| unsafe {
            Pin::into_inner(Pin::map_unchecked_mut(node.as_mut(), |node| {
                &mut node.payload
            }))
        })
    }

    #[inline(always)]
    fn push<D>(&mut self, delta: D)
    where
        P: Pushable<D>,
    {
        if let Some(p) = self.payload_mut() {
            p.push(delta);
        }
    }

    #[inline(always)]
    fn reverse(&mut self) {
        if let Some(node) = self.0.as_mut() {
            let this = node.inner();
            swap(&mut this.left, &mut this.right);
            this.reversed.flip();
        }
    }

    fn refs(&mut self, res: &mut Vec<NodeRef<P, S, R>>) {
        if let Some(node) = self.0.as_mut() {
            let this = node.inner();
            this.left.refs(res);
            res.push(NodeRef(this as *mut _));
            this.right.refs(res);
        }
    }

    fn merge(left: Self, right: Self) -> Self {
        match (left.0, right.0) {
            (None, right) => Self(right),
            (left, None) => Self(left),
            (Some(mut left), Some(mut right)) => {
                if left.priority > right.priority {
                    let left_right = left.detach_right();
                    left.attach_right(Self::merge(left_right, Self(Some(right))));
                    Self(Some(left))
                } else {
                    let right_left = right.detach_left();
                    right.attach_left(Self::merge(Self(Some(left)), right_left));
                    Self(Some(right))
                }
            }
        }
    }

    fn first(&mut self) -> &mut Self {
        if let Some(node) = self.as_ref() {
            if node.left.is_some() {
                let node = self.as_mut().unwrap();
                node.push_down();
                node.inner().left.first()
            } else {
                self
            }
        } else {
            self
        }
    }

    fn last(&mut self) -> &mut Self {
        if let Some(node) = self.as_ref() {
            if node.right.is_some() {
                let node = self.as_mut().unwrap();
                node.push_down();
                node.inner().right.last()
            } else {
                self
            }
        } else {
            self
        }
    }

    fn split_by<F: FnMut(&P, Option<&P>, Option<&P>) -> Direction>(self, mut f: F) -> (Self, Self) {
        if let Some(mut node) = self.0 {
            let direction = f(&node.payload, node.left.payload(), node.right.payload());
            match direction {
                Direction::Left => {
                    let (left, right) = node.detach_left().split_by(f);
                    node.attach_left(right);
                    (left, Self(Some(node)))
                }
                Direction::Right => {
                    let (left, right) = node.detach_right().split_by(f);
                    node.attach_right(left);
                    (Self(Some(node)), right)
                }
            }
        } else {
            (Self::NONE, Self::NONE)
        }
    }

    fn binary_search<F: FnMut(&P, Option<&P>, Option<&P>) -> Option<Direction>>(
        &mut self,
        mut f: F,
    ) {
        if let Some(node) = self.deref_mut() {
            node.push_down();
            let direction = f(&node.payload, node.left.payload(), node.right.payload());
            match direction {
                Some(Direction::Left) => node.inner().left.binary_search(f),
                Some(Direction::Right) => node.inner().right.binary_search(f),
                None => {}
            }
        }
    }

    fn push_from_up(
        node_ptr: *mut NodeInner<P, S, R>,
        directions: &mut Vec<Direction>,
    ) -> *mut NodeInner<P, S, R> {
        let node = unsafe { &mut *node_ptr };
        match node.parent {
            None => node_ptr,
            Some(parent_ptr) => {
                let parent = unsafe { &mut *parent_ptr };
                let mut found = false;
                if let Some(left) = parent.left.as_mut() {
                    if left.as_inner_ptr() == node_ptr {
                        directions.push(Direction::Left);
                        found = true;
                    }
                }
                if let Some(right) = parent.right.as_mut() {
                    if right.as_inner_ptr() == node_ptr {
                        directions.push(Direction::Right);
                        found = true;
                    }
                }
                assert!(found);
                Self::push_from_up(parent, directions)
            }
        }
    }

    fn raise(mut self, node: *mut NodeInner<P, S, R>) -> (Self, Self, Self) {
        let mut directions = Vec::new();
        let expected_ptr = Self::push_from_up(node, &mut directions);
        assert_eq!(expected_ptr, self.as_mut().unwrap().as_inner_ptr());
        self.split_by_dir(directions)
    }

    fn split_by_dir(mut self, mut directions: Vec<Direction>) -> (Self, Self, Self) {
        let node = self.as_mut().unwrap();
        if let Some(dir) = directions.pop() {
            match dir {
                Direction::Left => {
                    let (left, mid, right) = node.detach_left().split_by_dir(directions);
                    node.attach_left(right);
                    (left, mid, self)
                }
                Direction::Right => {
                    let (left, mid, right) = node.detach_right().split_by_dir(directions);
                    node.attach_right(left);
                    (self, mid, right)
                }
            }
        } else {
            let left = node.detach_left();
            let right = node.detach_right();
            node.update_leaf();
            (left, self, right)
        }
    }
}

impl<P: Payload + Unpin, R: Reverse> OptionNode<P, u32, R> {
    #[inline(always)]
    fn size(&self) -> usize {
        match self.as_ref() {
            None => 0,
            Some(node) => node.size as usize,
        }
    }

    fn split_at(self, mut at: usize) -> (Self, Self) {
        self.split_by_size(|left_size, _| {
            if at <= left_size {
                Direction::Left
            } else {
                at -= left_size + 1;
                Direction::Right
            }
        })
    }

    fn split_by_size<F: FnMut(usize, usize) -> Direction>(self, mut f: F) -> (Self, Self) {
        if let Some(mut node) = self.0 {
            let direction = f(node.left.size(), node.right.size());
            match direction {
                Direction::Left => {
                    let (left, right) = node.detach_left().split_by_size(f);
                    node.attach_left(right);
                    (left, Self(Some(node)))
                }
                Direction::Right => {
                    let (left, right) = node.detach_right().split_by_size(f);
                    node.attach_right(left);
                    (Self(Some(node)), right)
                }
            }
        } else {
            (Self::NONE, Self::NONE)
        }
    }

    fn binary_search_size<F: FnMut(usize, usize) -> Option<Direction>>(&mut self, mut f: F) {
        if let Some(node) = self.deref_mut() {
            node.push_down();
            let direction = f(node.left.size(), node.right.size());
            match direction {
                Some(Direction::Left) => node.inner().left.binary_search_size(f),
                Some(Direction::Right) => node.inner().right.binary_search_size(f),
                None => {}
            }
        }
    }
}

impl<P: OrdPayload + Unpin, S: Size, R: Reverse> OptionNode<P, S, R> {
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

    fn union(a: Self, b: Self) -> Self {
        match (a.0, b.0) {
            (None, b) => Self(b),
            (a, None) => Self(a),
            (Some(mut a), Some(mut b)) => {
                if a.priority < b.priority {
                    swap(&mut a, &mut b);
                }
                let (b_left, b_right) = Self(Some(b)).split(a.payload.key());
                let (same, b_right) = b_right.split(a.payload.key());
                let (left, right) = if let Some(same) = same.0 {
                    let left = a.detach_left();
                    let right = a.detach_right();
                    a = Node::new(P::union(a.into_payload(), same.into_payload()));
                    (left, right)
                } else {
                    (a.detach_left(), a.detach_right())
                };
                let left = Self::union(left, b_left);
                let right = Self::union(right, b_right);
                a.attach_left(left);
                a.attach_right(right);
                Self(Some(a))
            }
        }
    }
}
