use crate::collections::fx_hash_map::FxHashMap;
use crate::collections::payload::Payload;
use crate::misc::direction::Direction;
use crate::misc::random::{RandomTrait, StaticRandom};
use crate::numbers::num_utils::UpperDiv;
use std::marker::PhantomPinned;
use std::mem::{forget, take};
use std::ops::{Deref, DerefMut};
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
    priority: u64,
    size: u32,
    content: Option<Content<P>>,
    _phantom_pinned: PhantomPinned,
}

impl<P> Node<P> {
    const NULL_NODE: Self = Self {
        priority: 0,
        size: 0,
        content: None,
        _phantom_pinned: PhantomPinned,
    };

    fn payload(&self) -> Option<&P> {
        self.content.as_ref().map(|c| &c.payload)
    }

    fn payload_mut(&mut self) -> &mut P {
        unsafe { &mut self.content.as_mut().unwrap_unchecked().payload }
    }
}

impl<P: Payload> Node<P> {
    fn update(&mut self) {
        self.size = 1 + self.left.size + self.right.size;
        self.deref_mut().update();
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

pub struct TreapNode<P> {
    link: NonNull<Node<P>>,
}

impl<P: Payload> TreapNode<P> {
    fn new(payload: P) -> Self {
        let node = Node {
            priority: StaticRandom.gen_int(),
            size: 1,
            content: Some(Content {
                payload: payload,
                parent: TreapNode::default(),
                left: TreapNode::default(),
                right: TreapNode::default(),
            }),
            _phantom_pinned: PhantomPinned,
        };
        let mut pinned = Box::pin(node);
        let mut res = TreapNode {
            link: unsafe { NonNull::from(pinned.as_mut().get_unchecked_mut()) },
        };
        forget(pinned);
        res.update();
        res
    }

    fn new_ref(node: &Node<P>) -> Self {
        TreapNode {
            link: NonNull::from(node),
        }
    }

    fn size(&self) -> usize {
        self.size as usize
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

    fn raise(self, link: TreapNode<P>) -> (Self, Self, Self) {
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

    fn drop_first(mut self) {
        if self.left.size == 0 {
            self.detach_right();
        } else if self.left.left.size == 0 {
            let mut left = self.detach_left();
            let right = left.detach_right();
            self.attach_left(right);
        } else {
            self.push_down();
            self.left.drop_first();
            self.update();
        }
    }

    fn root(mut self) -> Self {
        while self.parent.size != 0 {
            self = self.parent;
        }
        self
    }

    fn self_raise(self) -> (Self, Self, Self) {
        self.root().raise(self)
    }

    fn reroot_left(self) -> Self {
        let (left, mid, right) = self.self_raise();
        TreapNode::merge(mid, TreapNode::merge(right, left))
    }

    fn reroot_right(self) -> Self {
        let (left, mid, right) = self.self_raise();
        TreapNode::merge(TreapNode::merge(right, left), mid)
    }
}

impl<P> Clone for TreapNode<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for TreapNode<P> {}

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

pub struct EulerTourForest<P> {
    nodes: Vec<TreapNode<P>>,
    edges: FxHashMap<(usize, usize), TreapNode<P>>,
}

impl<P: Payload> EulerTourForest<P> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: FxHashMap::default(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_node(&mut self, payload: P) -> usize {
        let node = TreapNode::new(payload);
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, u: usize, v: usize, direct_payload: P, reverse_payload: P) {
        let left = self.nodes[u].reroot_left();
        let right = self.nodes[v].reroot_right();
        if left == right {
            panic!("Adding a loop");
        }
        let uv = TreapNode::new(direct_payload);
        let vu = TreapNode::new(reverse_payload);
        TreapNode::merge(TreapNode::merge(left, uv), TreapNode::merge(right, vu));
        self.edges.insert((u, v), uv);
        self.edges.insert((v, u), vu);
    }

    pub fn remove_edge(&mut self, u: usize, v: usize) {
        let node = self.edges.remove(&(u, v)).unwrap();
        node.reroot_left();
        node.root().drop_first();
        let node2 = self.edges.remove(&(v, u)).unwrap();
        node2.self_raise();
    }

    pub fn with_node<R>(&self, u: usize, f: impl FnOnce(&P) -> R) -> R {
        let (left, node, right) = self.nodes[u].self_raise();
        let res = f(&node.payload);
        TreapNode::merge(left, TreapNode::merge(node, right));
        res
    }

    pub fn with_node_mut<R>(&self, u: usize, f: impl FnOnce(&mut P) -> R) -> R {
        let (left, mut node, right) = self.nodes[u].self_raise();
        let res = f(&mut node.payload);
        TreapNode::merge(left, TreapNode::merge(node, right));
        res
    }

    pub fn with_component<R>(&self, u: usize, f: impl FnOnce(&P) -> R) -> R {
        let root = self.nodes[u].root();
        f(&root.payload)
    }

    pub fn with_component_mut<R>(&self, u: usize, f: impl FnOnce(&mut P) -> R) -> R {
        let mut root = self.nodes[u].root();
        let res = f(&mut root.payload);
        res
    }

    pub fn with_edge<R>(&self, u: usize, v: usize, f: impl FnOnce(&P) -> R) -> R {
        let node = self.edges[&(u, v)];
        let (left, node, right) = node.self_raise();
        let res = f(&node.payload);
        TreapNode::merge(left, TreapNode::merge(node, right));
        res
    }

    pub fn with_edge_mut<R>(&self, u: usize, v: usize, f: impl FnOnce(&mut P) -> R) -> R {
        let node = self.edges[&(u, v)];
        let (left, mut node, right) = node.self_raise();
        let res = f(&mut node.payload);
        TreapNode::merge(left, TreapNode::merge(node, right));
        res
    }

    pub fn with_subtree<R>(&self, vert: usize, parent: usize, f: impl FnOnce(&P) -> R) -> R {
        self.edges[&(parent, vert)].reroot_right();
        let (left, mid, right) = self.edges[&(vert, parent)].self_raise();
        let res = f(&left.payload);
        TreapNode::merge(TreapNode::merge(left, mid), right);
        res
    }

    pub fn with_subtree_mut<R>(
        &self,
        vert: usize,
        parent: usize,
        f: impl FnOnce(&mut P) -> R,
    ) -> R {
        self.edges[&(parent, vert)].reroot_right();
        let (mut left, mid, right) = self.edges[&(vert, parent)].self_raise();
        let res = f(&mut left.payload);
        TreapNode::merge(TreapNode::merge(left, mid), right);
        res
    }

    pub fn is_connected(&self, u: usize, v: usize) -> bool {
        self.nodes[u].root() == self.nodes[v].root()
    }

    pub fn component_size(&self, u: usize) -> usize {
        self.nodes[u].root().size().upper_div(3)
    }

    pub fn subtree_size(&self, vert: usize, parent: usize) -> usize {
        self.edges[&(parent, vert)].reroot_right();
        let (left, mid, right) = self.edges[&(vert, parent)].self_raise();
        let res = left.size();
        TreapNode::merge(TreapNode::merge(left, mid), right);
        res.upper_div(3)
    }
}

impl<P: Payload> Default for EulerTourForest<P> {
    fn default() -> Self {
        Self::new()
    }
}
