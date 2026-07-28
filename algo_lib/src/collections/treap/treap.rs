use crate::collections::payload::{OrdPayload, Payload};
use crate::misc::bump_alloc::bump_new;
use crate::misc::direction::Direction;
use crate::misc::extensions::replace_with::ReplaceWith;
use crate::misc::random::{RandomTrait, StaticRandom};
use std::cmp::Ordering;
use std::collections::Bound;
use std::marker::{PhantomData, PhantomPinned};
use std::mem::{swap, take, MaybeUninit};
use std::ops::{Deref, DerefMut, RangeBounds};
use std::ptr::NonNull;

pub struct Content<P> {
    payload: P,
    parent: TreapNode<P>,
    left: TreapNode<P>,
    right: TreapNode<P>,
}

impl<P: Payload> Content<P> {
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
                .update(self.left.payload(), self.right.payload());
        }
    }
}

pub struct Node<P> {
    priority: u32,
    size: u32,
    reversed: bool,
    content: Option<Content<P>>,
    _phantom_pinned: PhantomPinned,
}

impl<P> Node<P> {
    const NULL_NODE: Self = Self {
        priority: 0,
        size: 0,
        reversed: false,
        content: None,
        _phantom_pinned: PhantomPinned,
    };

    fn payload(&self) -> Option<&P> {
        self.content.as_ref().map(|c| &c.payload)
    }

    fn payload_mut(&mut self) -> &mut P {
        unsafe { &mut self.content.as_mut().unwrap_unchecked().payload }
    }

    fn reverse(&mut self) {
        if let Some(content) = &mut self.content {
            self.reversed ^= true;
            swap(&mut content.left, &mut content.right);
        }
    }

    fn replace_payload(&mut self, f: impl FnOnce(P) -> P) {
        let content = unsafe { self.content.take().unwrap_unchecked() };
        let Content {
            payload,
            parent,
            left,
            right,
        } = content;
        self.content = Some(Content {
            payload: f(payload),
            parent,
            left,
            right,
        });
    }
}

impl<P: Payload> Node<P> {
    fn update(&mut self) {
        assert!(!self.reversed);
        self.size = 1 + self.left.size + self.right.size;
        self.deref_mut().update();
    }

    fn push_down(&mut self) {
        if self.reversed {
            self.left.reverse();
            self.right.reverse();
            self.reversed = false;
        }
        self.deref_mut().push_down();
    }

    fn detach(&mut self) {
        if self.size != 0 {
            self.parent = TreapNode::default();
        }
    }

    fn detach_left(&mut self) -> TreapNode<P> {
        self.push_down();
        let mut left = take(&mut self.left);
        left.detach();
        left
    }

    fn detach_right(&mut self) -> TreapNode<P> {
        self.push_down();
        let mut right = take(&mut self.right);
        right.detach();
        right
    }

    fn attach_left(&mut self, left: TreapNode<P>) {
        assert_eq!(self.left.size, 0);
        if left.size != 0 {
            self.left = left;
            self.left.deref_mut().parent = TreapNode::new_ref(self);
        }
        self.update();
    }

    fn attach_right(&mut self, right: TreapNode<P>) {
        assert_eq!(self.right.size, 0);
        if right.size != 0 {
            self.right = right;
            self.right.deref_mut().parent = TreapNode::new_ref(self);
        }
        self.update();
    }

    fn heapify(&mut self) {
        if self.left.size != 0 {
            self.left.heapify();
            if self.left.priority > self.priority {
                let p = self.priority;
                self.priority = self.left.priority;
                self.left.priority = p;
            }
        }
        if self.right.size != 0 {
            self.right.heapify();
            if self.right.priority > self.priority {
                let p = self.priority;
                self.priority = self.right.priority;
                self.right.priority = p;
            }
        }
    }

    fn with_gen(n: usize, f: impl FnMut(usize) -> P) -> TreapNode<P> {
        let mut res = Self::build(f, 0, n).0;
        if res.size != 0 {
            res.heapify();
        }
        res
    }

    fn build<F: FnMut(usize) -> P>(mut f: F, from: usize, to: usize) -> (TreapNode<P>, F) {
        if from == to {
            (TreapNode::default(), f)
        } else {
            let mid = StaticRandom.gen_range(from..to);
            let mut node = TreapNode::new(f(mid));
            let (left, f) = Self::build(f, from, mid);
            let (right, f) = Self::build(f, mid + 1, to);
            node.attach_left(left);
            node.attach_right(right);
            (node, f)
        }
    }

    fn refs(&mut self, res: &mut Vec<NodeId<P>>) {
        if self.size != 0 {
            self.left.refs(res);
            res.push(NodeId(TreapNode::new_ref(self)));
            self.right.refs(res);
        }
    }

    fn first(&mut self) -> &Node<P> {
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

    fn last(&mut self) -> &Node<P> {
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
            let direction = f(&self.payload, self.left.payload(), self.right.payload());
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

    fn binary_search_with_size<
        F: FnMut(&P, Option<&P>, Option<&P>, usize, usize) -> Option<Direction>,
    >(
        &mut self,
        mut f: F,
    ) {
        if self.size != 0 {
            self.push_down();
            let direction = f(
                &self.payload,
                self.left.payload(),
                self.right.payload(),
                self.left.size(),
                self.right.size(),
            );
            match direction {
                Some(Direction::Left) => self.left.binary_search_with_size(f),
                Some(Direction::Right) => self.right.binary_search_with_size(f),
                None => {}
            }
        }
    }
}

impl<P> Deref for Node<P> {
    type Target = Content<P>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.content.as_ref().unwrap_unchecked() }
    }
}

impl<P> DerefMut for Node<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.content.as_mut().unwrap_unchecked() }
    }
}

pub struct NodeId<P>(TreapNode<P>);

impl<P: Payload> NodeId<P> {
    pub unsafe fn with_payload<R>(&self, f: impl FnOnce(&P) -> R) -> R {
        let node = &self.0;
        node.push_from_up(&mut Vec::new());
        f(node.payload().unwrap())
    }
}

impl<P> Clone for NodeId<P> {
    fn clone(&self) -> Self {
        NodeId(self.0.clone())
    }
}

pub struct TreapNode<P> {
    link: NonNull<Node<P>>,
}

impl<P: Payload> TreapNode<P> {
    fn new(payload: P) -> Self {
        let node = Node {
            priority: StaticRandom.gen_int(),
            size: 1,
            reversed: false,
            content: Some(Content {
                payload,
                parent: TreapNode::default(),
                left: TreapNode::default(),
                right: TreapNode::default(),
            }),
            _phantom_pinned: PhantomPinned,
        };
        TreapNode {
            link: bump_new(node),
        }
    }

    fn new_ref(node: &Node<P>) -> Self {
        TreapNode {
            link: NonNull::from(node),
        }
    }

    fn into_payload(mut self) -> P {
        assert_eq!(self.left.size, 0);
        assert_eq!(self.right.size, 0);
        assert_eq!(self.parent.size, 0);
        self.size = 0;
        self.content.take().unwrap().payload
    }

    fn merge(mut left: Self, mut right: Self) -> Self {
        match (left.size, right.size) {
            (0, _) => right,
            (_, 0) => left,
            _ => {
                if left.priority > right.priority {
                    let left_right = left.detach_right();
                    left.attach_right(Self::merge(left_right, right));
                    left
                } else {
                    let right_left = right.detach_left();
                    right.attach_left(Self::merge(left, right_left));
                    right
                }
            }
        }
    }

    fn split_by<F: FnMut(&P, Option<&P>, Option<&P>) -> Direction>(
        mut self,
        mut f: F,
    ) -> (Self, Self) {
        if self.size != 0 {
            let direction = f(&self.payload, self.left.payload(), self.right.payload());
            match direction {
                Direction::Left => {
                    let (left, right) = self.detach_left().split_by(f);
                    self.attach_left(right);
                    (left, self)
                }
                Direction::Right => {
                    let (left, right) = self.detach_right().split_by(f);
                    self.attach_right(left);
                    (self, right)
                }
            }
        } else {
            (TreapNode::default(), TreapNode::default())
        }
    }

    fn push_from_up(&self, directions: &mut Vec<Direction>) -> TreapNode<P> {
        if self.parent.size != 0 {
            if self.parent.left == *self {
                directions.push(Direction::Left);
            } else if self.parent.right == *self {
                directions.push(Direction::Right);
            } else {
                unreachable!();
            }
            self.parent.push_from_up(directions)
        } else {
            TreapNode { link: self.link }
        }
    }

    fn raise(self, link: &TreapNode<P>) -> (Self, Self, Self) {
        assert!(link.content.is_some());
        let mut directions = Vec::new();
        let expected_parent = link.push_from_up(&mut directions);
        assert!(expected_parent == self);
        self.split_by_dir(directions)
    }

    fn split_by_dir(mut self, mut directions: Vec<Direction>) -> (Self, Self, Self) {
        if let Some(dir) = directions.pop() {
            match dir {
                Direction::Left => {
                    let (left, mid, right) = self.detach_left().split_by_dir(directions);
                    self.attach_left(right);
                    (left, mid, self)
                }
                Direction::Right => {
                    let (left, mid, right) = self.detach_right().split_by_dir(directions);
                    self.attach_right(left);
                    (self, mid, right)
                }
            }
        } else {
            let left = self.detach_left();
            let right = self.detach_right();
            self.update();
            (left, self, right)
        }
    }

    fn split_by_size<F: FnMut(usize, usize) -> Direction>(mut self, mut f: F) -> (Self, Self) {
        if self.size != 0 {
            let direction = f(self.left.size(), self.right.size());
            match direction {
                Direction::Left => {
                    let (left, right) = self.detach_left().split_by_size(f);
                    self.attach_left(right);
                    (left, self)
                }
                Direction::Right => {
                    let (left, right) = self.detach_right().split_by_size(f);
                    self.attach_right(left);
                    (self, right)
                }
            }
        } else {
            (Self::default(), Self::default())
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

    // Read-only descends: no splits, no updates, only push_down on the way.
    fn find_at(mut node: Self, mut at: usize) -> Self {
        if at >= node.size() {
            return Self::default();
        }
        loop {
            node.push_down();
            let left_size = node.left.size();
            match at.cmp(&left_size) {
                Ordering::Less => node = node.left.clone(),
                Ordering::Equal => return node,
                Ordering::Greater => {
                    at -= left_size + 1;
                    node = node.right.clone();
                }
            }
        }
    }
}

impl<P: OrdPayload> TreapNode<P> {
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

    fn find(mut node: Self, key: &P::Key) -> (Self, usize) {
        let mut before = 0;
        while node.size != 0 {
            node.push_down();
            match key.cmp(node.payload.key()) {
                Ordering::Less => node = node.left.clone(),
                Ordering::Greater => {
                    before += node.left.size() + 1;
                    node = node.right.clone();
                }
                Ordering::Equal => {
                    let index = before + node.left.size();
                    return (node, index);
                }
            }
        }
        (Self::default(), before)
    }

    // Monotone `go_right`; returns (last true node, first false node, fold over right turns).
    fn descend<Acc>(
        mut node: Self,
        mut go_right: impl FnMut(&P) -> bool,
        mut acc: Acc,
        mut fold: impl FnMut(Acc, &Node<P>) -> Acc,
    ) -> (Self, Self, Acc) {
        let mut last_true = Self::default();
        let mut first_false = Self::default();
        while node.size != 0 {
            node.push_down();
            if go_right(&node.payload) {
                acc = fold(acc, &node);
                last_true = node.clone();
                node = node.right.clone();
            } else {
                first_false = node.clone();
                node = node.left.clone();
            }
        }
        (last_true, first_false, acc)
    }

    fn rotate_right(mut root: Self) -> Self {
        let mut left = root.detach_left();
        let left_right = left.detach_right();
        root.attach_left(left_right);
        left.attach_right(root);
        left
    }

    fn rotate_left(mut root: Self) -> Self {
        let mut right = root.detach_right();
        let right_left = right.detach_left();
        root.attach_right(right_left);
        right.attach_left(root);
        right
    }

    // One descend, expected O(1) rotations; `merge_dup(old, new)` resolves duplicates.
    fn insert_impl<F: FnOnce(P, P) -> P>(
        mut root: Self,
        p: P,
        merge_dup: F,
        link: &mut Self,
    ) -> Self {
        if root.size == 0 {
            let node = Self::new(p);
            *link = node.clone();
            return node;
        }
        root.push_down();
        match p.key().cmp(root.payload.key()) {
            Ordering::Equal => {
                root.replace_payload(|old| merge_dup(old, p));
                root.update();
                *link = root.clone();
                root
            }
            Ordering::Less => {
                let left = root.detach_left();
                root.attach_left(Self::insert_impl(left, p, merge_dup, link));
                if root.left.priority > root.priority {
                    Self::rotate_right(root)
                } else {
                    root
                }
            }
            Ordering::Greater => {
                let right = root.detach_right();
                root.attach_right(Self::insert_impl(right, p, merge_dup, link));
                if root.right.priority > root.priority {
                    Self::rotate_left(root)
                } else {
                    root
                }
            }
        }
    }

    // One descend; on the found node `decide` may mutate and returns whether to delete.
    fn remove_impl<F: FnOnce(&mut P) -> bool>(
        mut root: Self,
        key: &P::Key,
        decide: F,
        removed: &mut Option<P>,
        found: &mut bool,
    ) -> Self {
        if root.size == 0 {
            return root;
        }
        root.push_down();
        match key.cmp(root.payload.key()) {
            Ordering::Equal => {
                *found = true;
                if decide(root.payload_mut()) {
                    let left = root.detach_left();
                    let right = root.detach_right();
                    root.detach();
                    let res = Self::merge(left, right);
                    *removed = Some(root.into_payload());
                    res
                } else {
                    root.update();
                    root
                }
            }
            Ordering::Less => {
                let left = root.detach_left();
                root.attach_left(Self::remove_impl(left, key, decide, removed, found));
                root
            }
            Ordering::Greater => {
                let right = root.detach_right();
                root.attach_right(Self::remove_impl(right, key, decide, removed, found));
                root
            }
        }
    }

    fn union(mut a: Self, mut b: Self) -> Self {
        match (a.size, b.size) {
            (0, _) => b,
            (_, 0) => a,
            _ => {
                if a.priority < b.priority {
                    swap(&mut a, &mut b);
                }
                let (b_left, b_right) = b.split(a.payload.key());
                let (same, b_right) = b_right.split_inclusive(a.payload.key());
                let left = a.detach_left();
                let right = a.detach_right();
                if same.size != 0 {
                    a = TreapNode::new(P::union(a.into_payload(), same.into_payload()));
                }
                let left = Self::union(left, b_left);
                let right = Self::union(right, b_right);
                a.attach_left(left);
                a.attach_right(right);
                a
            }
        }
    }
}

impl<P> Clone for TreapNode<P> {
    fn clone(&self) -> Self {
        TreapNode { link: self.link }
    }
}

impl<P> PartialEq for TreapNode<P> {
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
    }
}

impl<P> Eq for TreapNode<P> {}

impl<P> Deref for TreapNode<P> {
    type Target = Node<P>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.link.as_ref() }
    }
}

impl<P> DerefMut for TreapNode<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.link.as_mut() }
    }
}

impl<P> Default for TreapNode<P> {
    fn default() -> Self {
        TreapNode {
            link: unsafe {
                NonNull::new_unchecked(&Node::NULL_NODE as *const Node<P> as *mut Node<P>)
            },
        }
    }
}

pub enum Tree<P> {
    Whole {
        root: TreapNode<P>,
    },
    Split {
        left: TreapNode<P>,
        mid: Box<Tree<P>>,
        right: TreapNode<P>,
    },
}

impl<P: Payload> Default for Tree<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Payload> Tree<P> {
    pub fn new() -> Self {
        Tree::Whole {
            root: TreapNode::default(),
        }
    }

    pub fn split_by_tail<F: FnMut(&P, Option<&P>, Option<&P>) -> Direction>(
        &mut self,
        f: F,
    ) -> &mut Self {
        self.do_split_three(|root| {
            let (left, right) = root.split_by(f);
            (left, right, TreapNode::default())
        })
    }

    pub fn split_by_head<F: FnMut(&P, Option<&P>, Option<&P>) -> Direction>(
        &mut self,
        f: F,
    ) -> &mut Self {
        self.do_split_three(|root| {
            let (left, right) = root.split_by(f);
            (TreapNode::default(), left, right)
        })
    }

    pub fn with_gen(n: usize, f: impl FnMut(usize) -> P) -> Self {
        Tree::Whole {
            root: Node::with_gen(n, f),
        }
    }

    pub fn single(p: P) -> Self {
        Tree::Whole {
            root: TreapNode::new(p),
        }
    }

    fn into_node(mut self) -> TreapNode<P> {
        self.rebuild();
        match self {
            Tree::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    fn as_node(&mut self) -> &mut TreapNode<P> {
        match self {
            Tree::Whole { root } => root,
            _ => unreachable!(),
        }
    }

    pub fn rebuild(&mut self) -> &mut TreapNode<P> {
        self.replace_with(|self_| {
            if let Tree::Split { left, mid, right } = self_ {
                Tree::Whole {
                    root: TreapNode::merge(left, TreapNode::merge(mid.into_node(), right)),
                }
            } else {
                self_
            }
        });
        self.as_node()
    }

    fn do_split(
        mut self,
        f: impl FnOnce(TreapNode<P>) -> (TreapNode<P>, TreapNode<P>),
    ) -> (Self, Self) {
        let mut right = MaybeUninit::uninit();
        self.replace_with(|self_| {
            let root = self_.into_node();
            let (left, right_) = f(root);
            right.write(Tree::Whole { root: right_ });
            Tree::Whole { root: left }
        });
        (self, unsafe { right.assume_init() })
    }

    fn do_split_three(
        &mut self,
        f: impl FnOnce(TreapNode<P>) -> (TreapNode<P>, TreapNode<P>, TreapNode<P>),
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
            Tree::Split { mid, .. } => mid,
            _ => unreachable!(),
        }
    }

    pub fn merge_front(&mut self, other: Self) -> &mut Self {
        self.replace_with(|root| Tree::Whole {
            root: TreapNode::merge(other.into_node(), root.into_node()),
        });
        self
    }

    pub fn merge_back(&mut self, other: Self) -> &mut Self {
        self.replace_with(|root| Tree::Whole {
            root: TreapNode::merge(root.into_node(), other.into_node()),
        });
        self
    }

    pub fn detach(&mut self) -> Self {
        match self {
            Tree::Whole { root } => Tree::Whole { root: take(root) },
            Tree::Split { mid, .. } => take(mid),
        }
    }

    pub fn binary_search(
        &mut self,
        f: impl FnMut(&P, Option<&P>, Option<&P>) -> Option<Direction>,
    ) {
        self.rebuild().binary_search(f);
    }

    pub fn push(&mut self, delta: &P) {
        self.with_payload_mut(|p| p.accumulate(delta));
    }

    pub fn replace(&mut self, delta: P) {
        self.with_payload_mut(|p| *p = delta);
    }

    pub fn payload_mut(&mut self) -> Option<&mut P> {
        self.rebuild().content.as_mut().map(|c| &mut c.payload)
    }

    pub fn with_payload_mut<R: Default>(&mut self, f: impl FnOnce(&mut P) -> R) -> R {
        if let Some(payload) = self.payload_mut() {
            f(payload)
        } else {
            R::default()
        }
    }

    pub fn merge(left: Self, right: Self) -> Self {
        match left {
            Tree::Whole { root: left_root } => Tree::Split {
                left: left_root,
                mid: Box::new(right),
                right: TreapNode::default(),
            },
            Tree::Split {
                left,
                mid,
                right: left_right,
            } => Tree::Split {
                left,
                mid,
                right: TreapNode::merge(left_right, right.into_node()),
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

    pub fn iter(&mut self) -> Iter<'_, P> {
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

    pub fn add_back(&mut self, payload: P) -> NodeId<P> {
        let mut res = MaybeUninit::uninit();
        self.replace_with(|root| {
            let mut new_node = Self::single(payload);
            res.write(NodeId(new_node.as_node().clone()));
            Self::merge(root, new_node)
        });
        unsafe { res.assume_init() }
    }

    pub fn add_front(&mut self, payload: P) -> NodeId<P> {
        let mut res = MaybeUninit::uninit();
        self.replace_with(|root| {
            let mut new_node = Self::single(payload);
            res.write(NodeId(new_node.as_node().clone()));
            Self::merge(new_node, root)
        });
        unsafe { res.assume_init() }
    }

    pub fn refs(&mut self) -> Vec<NodeId<P>> {
        let mut res = Vec::new();
        self.rebuild().refs(&mut res);
        res
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Tree::Whole { root } => root.size == 0,
            Tree::Split { left, mid, right } => left.size == 0 && right.size == 0 && mid.is_empty(),
        }
    }

    pub fn raise(&mut self, node_ref: &NodeId<P>) -> &mut Self {
        self.do_split_three(|node| node.raise(&node_ref.0))
    }

    pub fn into_payload(self) -> P {
        self.into_node().into_payload()
    }

    pub fn index_ref(&mut self, node_ref: &NodeId<P>) -> usize {
        self.raise(node_ref);
        match self {
            Tree::Whole { .. } => unreachable!(),
            Tree::Split { left, .. } => left.size(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Tree::Whole { root } => root.size(),
            Tree::Split { left, mid, right } => left.size() + mid.size() + right.size(),
        }
    }

    pub fn split_at(self, at: usize) -> (Self, Self) {
        self.do_split(|root| root.split_at(at))
    }

    pub fn binary_search_size(&mut self, f: impl FnMut(usize, usize) -> Option<Direction>) {
        self.rebuild().binary_search_size(f);
    }

    pub fn binary_search_with_size(
        &mut self,
        f: impl FnMut(&P, Option<&P>, Option<&P>, usize, usize) -> Option<Direction>,
    ) {
        self.rebuild().binary_search_with_size(f);
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

    pub fn at(&mut self, at: usize) -> Option<&P> {
        let root = self.rebuild().clone();
        self.payload_of(TreapNode::find_at(root, at))
    }

    // Sound: nodes are never freed or moved.
    fn payload_of(&self, node: TreapNode<P>) -> Option<&P> {
        unsafe { node.link.as_ref() }.payload()
    }

    pub fn reverse(&mut self) {
        self.rebuild().reverse();
    }
}

pub struct Iter<'a, P> {
    stack: Vec<*mut TreapNode<P>>,
    _marker: PhantomData<&'a P>,
}

impl<'a, P: Payload> Iter<'a, P> {
    fn new(root: &'a mut TreapNode<P>) -> Self {
        let mut res = Self {
            stack: Vec::new(),
            _marker: PhantomData,
        };
        res.add_left(root);
        res
    }

    fn add_left(&mut self, mut node: &mut TreapNode<P>) {
        while node.size != 0 {
            node.push_down();
            self.stack.push(node);
            node = &mut node.left;
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

impl<P: OrdPayload> Tree<P> {
    pub fn range<'s: 'r, 'r>(&'s mut self, bounds: impl RangeBounds<&'r P::Key>) -> &'s mut Self {
        self.do_split_three(|root| {
            let (left, mid_right) = match bounds.start_bound() {
                Bound::Included(key) => root.split(key),
                Bound::Excluded(key) => root.split_inclusive(key),
                Bound::Unbounded => (TreapNode::default(), root),
            };
            let (mid, right) = match bounds.end_bound() {
                Bound::Included(key) => mid_right.split_inclusive(key),
                Bound::Excluded(key) => mid_right.split(key),
                Bound::Unbounded => (mid_right, TreapNode::default()),
            };
            (left, mid, right)
        })
    }

    pub fn insert_with_id(&mut self, p: P) -> (Option<P>, NodeId<P>) {
        let mut res = None;
        let mut link = TreapNode::default();
        self.replace_with(|tree| Tree::Whole {
            root: TreapNode::insert_impl(
                tree.into_node(),
                p,
                |old, new| {
                    res = Some(old);
                    new
                },
                &mut link,
            ),
        });
        (res, NodeId(link))
    }

    pub fn insert_or_update(&mut self, p: P) {
        let mut link = TreapNode::default();
        self.replace_with(|tree| Tree::Whole {
            root: TreapNode::insert_impl(tree.into_node(), p, P::union, &mut link),
        });
    }

    pub fn insert(&mut self, p: P) -> Option<P> {
        self.insert_with_id(p).0
    }

    pub fn remove(&mut self, key: &P::Key) -> Option<P> {
        self.remove_if(key, |_| true).1
    }

    // Returns (key was found, payload if the node was deleted).
    pub fn remove_if(&mut self, key: &P::Key, f: impl FnOnce(&mut P) -> bool) -> (bool, Option<P>) {
        let mut removed = None;
        let mut found = false;
        self.replace_with(|tree| Tree::Whole {
            root: TreapNode::remove_impl(tree.into_node(), key, f, &mut removed, &mut found),
        });
        (found, removed)
    }

    pub fn split(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split(key))
    }

    pub fn split_inclusive(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split_inclusive(key))
    }

    pub fn get(&mut self, key: &P::Key) -> Option<&P> {
        let root = self.rebuild().clone();
        self.payload_of(TreapNode::find(root, key).0)
    }

    fn descend(&mut self, go_right: impl FnMut(&P) -> bool) -> (TreapNode<P>, TreapNode<P>, usize) {
        let root = self.rebuild().clone();
        TreapNode::descend(root, go_right, 0, |acc, n| acc + n.left.size() + 1)
    }

    pub fn floor(&mut self, key: &P::Key) -> Option<&P> {
        let node = self.descend(|p| p.key() <= key).0;
        self.payload_of(node)
    }

    pub fn ceil(&mut self, key: &P::Key) -> Option<&P> {
        let node = self.descend(|p| p.key() < key).1;
        self.payload_of(node)
    }

    pub fn prev(&mut self, key: &P::Key) -> Option<&P> {
        let node = self.descend(|p| p.key() < key).0;
        self.payload_of(node)
    }

    pub fn next(&mut self, key: &P::Key) -> Option<&P> {
        let node = self.descend(|p| p.key() <= key).1;
        self.payload_of(node)
    }

    pub fn less(&mut self, key: &P::Key) -> usize {
        self.descend(|p| p.key() < key).2
    }

    pub fn less_or_eq(&mut self, key: &P::Key) -> usize {
        self.descend(|p| p.key() <= key).2
    }

    // `fold` gets each prefix node's payload and its left child payload.
    pub fn fold_prefix<Acc>(
        &mut self,
        key: &P::Key,
        inclusive: bool,
        init: Acc,
        mut fold: impl FnMut(Acc, &P, Option<&P>) -> Acc,
    ) -> Acc {
        let root = self.rebuild().clone();
        let adapted = |acc, n: &Node<P>| fold(acc, &n.payload, n.left.payload());
        if inclusive {
            TreapNode::descend(root, |p| p.key() <= key, init, adapted).2
        } else {
            TreapNode::descend(root, |p| p.key() < key, init, adapted).2
        }
    }

    pub fn union(a: Self, b: Self) -> Self {
        let a = a.into_node();
        let b = b.into_node();
        Self::Whole {
            root: TreapNode::union(a, b),
        }
    }

    pub fn index(&mut self, key: &P::Key) -> Option<usize> {
        let root = self.rebuild().clone();
        let (node, index) = TreapNode::find(root, key);
        node.payload().map(|_| index)
    }
}

#[cfg(test)]
mod test {
    use crate::collections::treap::multi_treap_set::MultiTreapSet;
    use crate::collections::treap::treap_map::TreapMap;
    use crate::misc::random::{Random, RandomTrait};
    use std::collections::BTreeMap;

    #[test]
    fn lookups_match_btree() {
        let mut rng = Random::new_with_seed(565);
        for _ in 0..100 {
            let mut map = TreapMap::new();
            let mut reference = BTreeMap::new();
            for _ in 0..300 {
                let key = rng.gen_bound(50u32);
                match rng.gen_bound(4u32) {
                    0 => {
                        assert_eq!(map.insert(key, key * 2), reference.insert(key, key * 2));
                    }
                    1 => {
                        assert_eq!(map.remove(&key), reference.remove(&key));
                    }
                    _ => {
                        assert_eq!(map.len(), reference.len());
                        assert_eq!(map.get(&key), reference.get(&key));
                        assert_eq!(map.contains(&key), reference.contains_key(&key));
                        assert_eq!(
                            map.floor(&key).map(|(k, _)| *k),
                            reference.range(..=key).next_back().map(|(k, _)| *k)
                        );
                        assert_eq!(
                            map.ceil(&key).map(|(k, _)| *k),
                            reference.range(key..).next().map(|(k, _)| *k)
                        );
                        assert_eq!(
                            map.prev(&key).map(|(k, _)| *k),
                            reference.range(..key).next_back().map(|(k, _)| *k)
                        );
                        assert_eq!(
                            map.next(&key).map(|(k, _)| *k),
                            reference
                                .range((std::ops::Bound::Excluded(key), std::ops::Bound::Unbounded))
                                .next()
                                .map(|(k, _)| *k)
                        );
                        let less = reference.range(..key).count();
                        let less_or_eq = reference.range(..=key).count();
                        assert_eq!(map.lower_bound(&key), less);
                        assert_eq!(map.upper_bound(&key), less_or_eq);
                        assert_eq!(map.more(&key), reference.len() - less_or_eq);
                        assert_eq!(map.more_or_eq(&key), reference.len() - less);
                        assert_eq!(
                            map.index(&key),
                            reference.contains_key(&key).then_some(less)
                        );
                        let hi = rng.gen_bound(50u32);
                        let (lo, hi) = (key.min(hi), key.max(hi));
                        assert_eq!(map.range_size(&lo..&hi), reference.range(lo..hi).count());
                        assert_eq!(map.range_size(&lo..=&hi), reference.range(lo..=hi).count());
                        if !reference.is_empty() {
                            let at = rng.gen_bound(reference.len());
                            assert_eq!(
                                map.get_at(at).map(|(k, _)| *k),
                                reference.keys().nth(at).copied()
                            );
                        }
                        assert_eq!(map.get_at(reference.len()), None);
                    }
                }
            }
        }
    }

    #[test]
    fn multi_set_counts_match_reference() {
        let mut rng = Random::new_with_seed(566);
        for _ in 0..100 {
            let mut set = MultiTreapSet::new();
            let mut reference: BTreeMap<u32, usize> = BTreeMap::new();
            for _ in 0..300 {
                let key = rng.gen_bound(20u32);
                match rng.gen_bound(4u32) {
                    0..=1 => {
                        set.insert(key);
                        *reference.entry(key).or_default() += 1;
                    }
                    2 => {
                        let expected = reference.contains_key(&key);
                        assert_eq!(set.remove(&key), expected);
                        if let Some(qty) = reference.get_mut(&key) {
                            *qty -= 1;
                            if *qty == 0 {
                                reference.remove(&key);
                            }
                        }
                    }
                    _ => {
                        let total: usize = reference.values().sum();
                        let less: usize = reference.range(..key).map(|(_, q)| q).sum();
                        let less_or_eq: usize = reference.range(..=key).map(|(_, q)| q).sum();
                        assert_eq!(set.len(), total);
                        assert_eq!(set.get(&key), reference.get(&key).copied().unwrap_or(0));
                        assert_eq!(set.lower_bound(&key), less);
                        assert_eq!(set.upper_bound(&key), less_or_eq);
                        assert_eq!(set.less(&key), less);
                        assert_eq!(set.less_or_eq(&key), less_or_eq);
                        assert_eq!(set.more(&key), total - less_or_eq);
                        assert_eq!(set.more_or_eq(&key), total - less);
                        assert_eq!(
                            set.index(&key),
                            reference.contains_key(&key).then_some(less)
                        );
                    }
                }
            }
        }
    }
}
