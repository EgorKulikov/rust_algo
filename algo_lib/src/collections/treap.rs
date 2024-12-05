use crate::misc::direction::Direction;
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::misc::random::random;
use std::collections::Bound;
use std::marker::PhantomPinned;
use std::mem::{swap, MaybeUninit};
use std::ops::{Deref, DerefMut, RangeBounds};
use std::pin::Pin;

pub trait Size: Payload {
    const ONE: Self;
    fn size(&self) -> usize;
}

impl Payload for () {}

impl Size for () {
    const ONE: Self = ();
    #[inline]
    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl Payload for u32 {
    const NEED_UPDATE: bool = true;

    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        *self = 1 + left.unwrap_or(&0) + right.unwrap_or(&0);
    }
}

impl Size for u32 {
    const ONE: Self = 1;
    #[inline]
    fn size(&self) -> usize {
        *self as usize
    }
}

pub trait Reverse: Default + Unpin {
    fn need_reverse(&self) -> bool;
    fn reset_reverse(&mut self);
    fn flip(&mut self);
}

impl Reverse for () {
    #[inline]
    fn need_reverse(&self) -> bool {
        false
    }

    #[inline]
    fn flip(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn reset_reverse(&mut self) {
        unimplemented!()
    }
}

impl Reverse for bool {
    #[inline]
    fn need_reverse(&self) -> bool {
        *self
    }

    #[inline]
    fn reset_reverse(&mut self) {
        *self = false;
    }

    #[inline]
    fn flip(&mut self) {
        *self ^= true;
    }
}

#[allow(unused_variables)]
pub trait Payload: Unpin {
    const NEED_UPDATE: bool = false;
    const NEED_PUSH_DOWN: bool = false;
    fn reset_delta(&mut self) {
        unimplemented!()
    }
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        unimplemented!()
    }
    fn push_delta(&mut self, delta: &Self) {
        unimplemented!()
    }

    fn need_push_down(&self) -> bool {
        unimplemented!()
    }
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

impl<P: Payload> Pushable<P> for P {
    #[inline]
    fn push(&mut self, delta: P) {
        *self = delta;
    }
}

pub struct PurePayload<T>(pub T);

impl<T: Unpin> Payload for PurePayload<T> {}

impl<T: Ord + Unpin> PayloadWithKey for PurePayload<T> {
    type Key = T;

    #[inline]
    fn key(&self) -> &Self::Key {
        &self.0
    }
}

impl<T: Ord + Unpin> Pushable<T> for PurePayload<T> {
    fn push(&mut self, delta: T) {
        self.0 = delta;
    }
}

pub struct Iter<'a, P, S, R> {
    stack: Vec<*mut TreapNode<P, S, R>>,
    _marker: std::marker::PhantomData<&'a P>,
}

impl<'a, P: Payload, S: Size, R: Reverse> Iter<'a, P, S, R> {
    fn new(root: &'a mut OptionTreapNode<P, S, R>) -> Self {
        let mut res = Self {
            stack: Vec::new(),
            _marker: std::marker::PhantomData,
        };
        res.add_left(root);
        res
    }

    fn add_left(&mut self, mut node: &mut OptionTreapNode<P, S, R>) {
        while let Some(n) = node.0.as_mut() {
            n.push_down();
            self.stack.push(n);
            node = &mut n.as_mut().this().left;
        }
    }
}

impl<'a, P: Payload, S: Size + 'a, R: Reverse + 'a> Iterator for Iter<'a, P, S, R> {
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        let node = unsafe { &mut *self.stack.pop()? };
        self.add_left(&mut node.as_mut().this().right);
        Some(&node.payload)
    }
}

#[allow(private_interfaces)]
pub enum Treap<P, S = (), R = ()> {
    Whole {
        root: OptionTreapNode<P, S, R>,
    },
    Split {
        left: OptionTreapNode<P, S, R>,
        mid: Box<Treap<P, S, R>>,
        right: OptionTreapNode<P, S, R>,
    },
}

impl<P: Payload> Treap<P> {
    pub fn new() -> Self {
        Treap::Whole {
            root: OptionTreapNode::NONE,
        }
    }
}

impl<P: Payload> Treap<P, u32> {
    pub fn sized() -> Self {
        Treap::Whole {
            root: OptionTreapNode::NONE,
        }
    }
}

impl<P: Payload> Treap<P, u32, bool> {
    pub fn reversible() -> Self {
        Treap::Whole {
            root: OptionTreapNode::NONE,
        }
    }
}

impl<P: Payload, S: Size, R: Reverse> Default for Treap<P, S, R> {
    fn default() -> Self {
        Treap::Whole {
            root: OptionTreapNode::NONE,
        }
    }
}

impl<P: Payload, S: Size, R: Reverse> Treap<P, S, R> {
    fn single(p: P) -> Self {
        Treap::Whole {
            root: OptionTreapNode::new(TreapNode::new(p)),
        }
    }

    #[inline]
    fn into_node(mut self) -> OptionTreapNode<P, S, R> {
        self.rebuild();
        match self {
            Treap::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    #[inline]
    unsafe fn as_node(&mut self) -> &mut OptionTreapNode<P, S, R> {
        match self {
            Treap::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    fn rebuild(&mut self) -> &mut OptionTreapNode<P, S, R> {
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
        mut self,
        f: impl FnOnce(OptionTreapNode<P, S, R>) -> (OptionTreapNode<P, S, R>, OptionTreapNode<P, S, R>),
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
            OptionTreapNode<P, S, R>,
        ) -> (
            OptionTreapNode<P, S, R>,
            OptionTreapNode<P, S, R>,
            OptionTreapNode<P, S, R>,
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

    pub fn push_front(&mut self, mut other: Self) -> &mut Self {
        self.replace_with(|mut root| {
            root.rebuild();
            other.rebuild();
            let root = root.into_node();
            Treap::Whole {
                root: OptionTreapNode::merge(other.into_node(), root),
            }
        });
        self
    }

    pub fn push_back(&mut self, mut other: Self) -> &mut Self {
        self.replace_with(|mut root| {
            root.rebuild();
            other.rebuild();
            let root = root.into_node();
            Treap::Whole {
                root: OptionTreapNode::merge(root, other.into_node()),
            }
        });
        self
    }

    pub fn detach(&mut self) -> Self {
        match self {
            Treap::Whole { root } => {
                let mut res = OptionTreapNode::NONE;
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
            res = Some(new_node.as_node().as_mut().unwrap().inner.as_mut().this() as *mut _);
            Self::merge(root, new_node)
        });
        NodeRef(res.unwrap())
    }

    pub fn add_front(&mut self, payload: P) -> NodeRef<P, S, R> {
        let mut res = None;
        self.replace_with(|root| unsafe {
            let mut new_node = Self::single(payload);
            res = Some(new_node.as_node().as_mut().unwrap().inner.as_mut().this() as *mut _);
            Self::merge(new_node, root)
        });
        NodeRef(res.unwrap())
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

pub struct NodeRef<P, S, R>(*mut TreapNodeInner<P, S, R>);

impl<P: Payload, R: Reverse> Treap<P, u32, R> {
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

    pub fn by_index(&mut self, bounds: impl RangeBounds<usize>) -> &mut Self {
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
            let (left, mid_right) = root.split_at(start);
            let (mid, right) = mid_right.split_at(end.max(start) - start);
            (left, mid, right)
        })
    }

    pub fn get_at(&mut self, at: usize) -> &mut Self {
        self.by_index(at..=at)
    }
}

impl<P: Payload, S: Size> Treap<P, S, bool> {
    pub fn reverse(&mut self) {
        self.rebuild().reverse();
    }
}

impl<P: PayloadWithKey, S: Size, R: Reverse> Treap<P, S, R> {
    pub fn range<'s: 'r, 'r>(&'s mut self, bounds: impl RangeBounds<&'r P::Key>) -> &mut Self {
        self.do_split_three(|root| {
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
                res = Some(mid.into_payload());
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
                res = Some(mid.into_payload());
            }
            Treap::Whole {
                root: OptionTreapNode::NONE,
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
}

impl<P: PayloadWithKey, R: Reverse> Treap<P, u32, R> {
    pub fn index(&mut self, key: &P::Key) -> Option<usize> {
        match self.range(key..=key) {
            Treap::Split { left, mid, .. } => mid.payload().map(|_| left.size()),
            _ => unreachable!(),
        }
    }
}

struct TreapNodeInner<P, S, R> {
    priority: u64,
    size: S,
    reversed: R,
    payload: P,
    left: OptionTreapNode<P, S, R>,
    right: OptionTreapNode<P, S, R>,
    parent: Option<*mut TreapNodeInner<P, S, R>>,
    _phantom_pinned: PhantomPinned,
}

impl<P, S, R> TreapNodeInner<P, S, R> {
    fn this(self: Pin<&mut Self>) -> &mut Self {
        unsafe { self.get_unchecked_mut() }
    }

    fn into_payload(self: Pin<Box<Self>>) -> P {
        unsafe { Pin::into_inner_unchecked(self).payload }
    }
}

struct TreapNode<P, S, R> {
    inner: Pin<Box<TreapNodeInner<P, S, R>>>,
}

impl<P, S, R> Deref for TreapNode<P, S, R> {
    type Target = Pin<Box<TreapNodeInner<P, S, R>>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P, S, R> DerefMut for TreapNode<P, S, R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

struct OptionTreapNode<P, S, R>(Option<TreapNode<P, S, R>>);

impl<P, S, R> Deref for OptionTreapNode<P, S, R> {
    type Target = Option<TreapNode<P, S, R>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P, S, R> DerefMut for OptionTreapNode<P, S, R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P: Payload, S: Size, R: Reverse> TreapNode<P, S, R> {
    #[inline]
    fn new(payload: P) -> Self {
        Self {
            inner: Box::pin(TreapNodeInner {
                priority: random().gen(),
                size: S::ONE,
                reversed: R::default(),
                payload,
                left: OptionTreapNode::NONE,
                right: OptionTreapNode::NONE,
                parent: None,
                _phantom_pinned: PhantomPinned,
            }),
        }
    }

    fn into_payload(self) -> P {
        assert!(self.parent.is_none());
        assert!(self.left.is_none());
        assert!(self.right.is_none());
        self.inner.into_payload()
    }

    #[inline]
    fn update(&mut self) {
        let this = self.as_mut().this();
        if S::NEED_UPDATE {
            this.size
                .update(this.left.size_opt(), this.right.size_opt());
        }
        if P::NEED_UPDATE {
            this.payload
                .update(this.left.payload(), this.right.payload());
        }
    }

    #[inline]
    fn push_down(&mut self) {
        let this = self.as_mut().this();
        if P::NEED_PUSH_DOWN && this.payload.need_push_down() {
            this.left.push(&this.payload);
            this.right.push(&this.payload);
            this.payload.reset_delta();
        }
        if this.reversed.need_reverse() {
            this.left.reverse();
            this.right.reverse();
            this.reversed.reset_reverse();
        }
    }

    fn detach_left(&mut self) -> OptionTreapNode<P, S, R> {
        self.push_down();
        let this = self.as_mut().this();
        let mut left = OptionTreapNode(this.left.take());
        self.update();
        left.detach();
        left
    }

    fn detach_right(&mut self) -> OptionTreapNode<P, S, R> {
        self.push_down();
        let this = self.as_mut().this();
        let mut right = OptionTreapNode(this.right.take());
        self.update();
        right.detach();
        right
    }

    fn attach_left(&mut self, mut left: OptionTreapNode<P, S, R>) {
        self.push_down();
        if let Some(left) = left.as_mut() {
            left.as_mut().this().parent = Some(self.as_inner_ptr());
        }
        let this = self.as_mut().this();
        this.left = left;
        self.update();
    }

    fn attach_right(&mut self, mut right: OptionTreapNode<P, S, R>) {
        self.push_down();
        if let Some(right) = right.as_mut() {
            right.as_mut().this().parent = Some(self.as_inner_ptr());
        }
        let this = self.as_mut().this();
        this.right = right;
        self.update();
    }

    #[inline]
    fn as_inner_ptr(&mut self) -> *mut TreapNodeInner<P, S, R> {
        self.as_mut().this()
    }
}

impl<P: Payload, S: Size, R: Reverse> OptionTreapNode<P, S, R> {
    const NONE: Self = Self(None);

    #[inline]
    fn new(node: TreapNode<P, S, R>) -> Self {
        Self(Some(node))
    }

    #[inline]
    fn detach(&mut self) {
        if let Some(node) = self.0.as_mut() {
            node.as_mut().this().parent = None;
        }
    }

    #[inline]
    fn size_opt(&self) -> Option<&S> {
        self.as_ref().map(|node| &node.size)
    }

    #[inline]
    fn payload(&self) -> Option<&P> {
        self.as_ref().map(|node| &node.payload)
    }

    #[inline]
    fn payload_mut(&mut self) -> Option<&mut P> {
        self.as_mut().map(|node| unsafe {
            Pin::into_inner(Pin::map_unchecked_mut(node.as_mut(), |node| {
                &mut node.payload
            }))
        })
    }

    #[inline]
    fn push<D>(&mut self, delta: D)
    where
        P: Pushable<D>,
    {
        if let Some(p) = self.payload_mut() {
            p.push(delta);
        }
    }

    #[inline]
    fn reverse(&mut self) {
        if let Some(node) = self.0.as_mut() {
            let this = node.as_mut().this();
            swap(&mut this.left, &mut this.right);
            this.reversed.flip();
        }
    }
}

impl<P: Payload, R: Reverse> OptionTreapNode<P, u32, R> {
    #[inline]
    fn size(&self) -> usize {
        self.as_ref().map(|node| node.size.size()).unwrap_or(0)
    }
}

impl<P: Payload, R: Reverse> OptionTreapNode<P, u32, R> {
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
            node.push_down();
            let direction = f(node.left.size(), node.right.size());
            match direction {
                Direction::Left => {
                    let (left, right) = node.detach_left().split_by_size(f);
                    node.attach_left(right);
                    node.update();
                    (left, Self(Some(node)))
                }
                Direction::Right => {
                    let (left, right) = node.detach_right().split_by_size(f);
                    node.attach_right(left);
                    node.update();
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
                Some(Direction::Left) => node.as_mut().this().left.binary_search_size(f),
                Some(Direction::Right) => node.as_mut().this().right.binary_search_size(f),
                None => {}
            }
        }
    }
}

impl<P: Payload, S: Size, R: Reverse> OptionTreapNode<P, S, R> {
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
                node.as_mut().this().left.first()
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
                node.as_mut().this().right.last()
            } else {
                self
            }
        } else {
            self
        }
    }

    fn split_by<F: FnMut(&P, Option<&P>, Option<&P>) -> Direction>(self, mut f: F) -> (Self, Self) {
        if let Some(mut node) = self.0 {
            node.push_down();
            let direction = f(&node.payload, node.left.payload(), node.right.payload());
            match direction {
                Direction::Left => {
                    let (left, right) = node.detach_left().split_by(f);
                    node.attach_left(right);
                    node.update();
                    (left, Self(Some(node)))
                }
                Direction::Right => {
                    let (left, right) = node.detach_right().split_by(f);
                    node.attach_right(left);
                    node.update();
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
                Some(Direction::Left) => node.as_mut().this().left.binary_search(f),
                Some(Direction::Right) => node.as_mut().this().right.binary_search(f),
                None => {}
            }
        }
    }

    fn push_from_up(
        node_ptr: *mut TreapNodeInner<P, S, R>,
        directions: &mut Vec<Direction>,
    ) -> *mut TreapNodeInner<P, S, R> {
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

    fn raise(mut self, node: *mut TreapNodeInner<P, S, R>) -> (Self, Self, Self) {
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
            (left, self, right)
        }
    }
}

impl<P: PayloadWithKey, S: Size, R: Reverse> OptionTreapNode<P, S, R> {
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
