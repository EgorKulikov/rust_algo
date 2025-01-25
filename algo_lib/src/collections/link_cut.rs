use crate::collections::payload::Payload;
use crate::misc::direction::Direction;
use std::cell::Cell;
use std::marker::PhantomPinned;
use std::mem::{forget, take};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct Content<P> {
    payload: P,
    parent: LinkCutNode<P>,
    direction: Option<Direction>,
    tree_parent: LinkCutNode<P>,
    lc_parent: LinkCutNode<P>,
    left: LinkCutNode<P>,
    right: LinkCutNode<P>,
}

impl<P: Payload> Content<P> {
    fn update(&mut self) {
        if P::NEED_UPDATE {
            self.payload
                .update(self.left.deref().payload(), self.right.deref().payload());
        }
    }

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
}

pub struct Node<P> {
    size: u32,
    content: Option<Content<P>>,
    _phantom_pinned: PhantomPinned,
}

impl<P> Node<P> {
    const NULL_NODE: Self = Self {
        size: 0,
        content: None,
        _phantom_pinned: PhantomPinned,
    };

    fn payload(&self) -> Option<&P> {
        self.content.as_ref().map(|content| &content.payload)
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
            self.parent = LinkCutNode::default();
            self.direction = None;
        }
    }

    fn detach_left(&mut self) -> LinkCutNode<P> {
        let mut left = take(&mut self.left);
        left.detach();
        left
    }

    fn detach_right(&mut self) -> LinkCutNode<P> {
        let mut right = take(&mut self.right);
        right.detach();
        right
    }

    fn detach_parent(&mut self) -> LinkCutNode<P> {
        let mut parent = self.parent;
        if parent.size != 0 {
            parent.push_down();
        }
        match self.direction {
            None => {}
            Some(Direction::Left) => {
                parent.detach_left();
            }
            Some(Direction::Right) => {
                parent.detach_right();
            }
        }
        parent
    }

    fn attach_left(&mut self, left: LinkCutNode<P>) {
        assert_eq!(self.left.size, 0);
        if left.size != 0 {
            self.left = left;
            self.left.parent = LinkCutNode::new_ref(self);
            self.left.direction = Some(Direction::Left);
        }
        self.update();
    }

    fn attach_right(&mut self, right: LinkCutNode<P>) {
        assert_eq!(self.right.size, 0);
        if right.size != 0 {
            self.right = right;
            self.right.parent = LinkCutNode::new_ref(self);
            self.right.direction = Some(Direction::Right);
        }
        self.update();
    }

    fn attach_child(&mut self, direction: Option<Direction>, child: LinkCutNode<P>) {
        match direction {
            None => {}
            Some(Direction::Left) => {
                self.attach_left(child);
            }
            Some(Direction::Right) => {
                self.attach_right(child);
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

pub struct LinkCutNode<P> {
    link: NonNull<Node<P>>,
}

impl<P> Copy for LinkCutNode<P> {}

impl<P> Clone for LinkCutNode<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: Payload> LinkCutNode<P> {
    pub fn new(payload: P) -> Self {
        let node = Node {
            size: 1,
            content: Some(Content {
                payload,
                parent: LinkCutNode::default(),
                direction: None,
                tree_parent: LinkCutNode::default(),
                left: LinkCutNode::default(),
                right: LinkCutNode::default(),
                lc_parent: LinkCutNode::default(),
            }),
            _phantom_pinned: PhantomPinned,
        };
        let mut pinned = Box::pin(node);
        let res = LinkCutNode {
            link: unsafe { NonNull::from(pinned.as_mut().get_unchecked_mut()) },
        };
        forget(pinned);
        res
    }

    fn new_ref(node: &Node<P>) -> Self {
        LinkCutNode {
            link: NonNull::from(node),
        }
    }

    fn do_push_down(mut self) {
        if self.size != 0 {
            self.parent.do_push_down();
            self.push_down();
        }
    }

    fn splay(mut self) {
        self.do_push_down();
        loop {
            match self.direction {
                None => {
                    break;
                }
                Some(Direction::Left) => {
                    let mut parent = self.detach_parent();
                    match parent.direction {
                        None => {
                            let b = self.detach_right();
                            parent.attach_left(b);
                            self.attach_right(parent);
                            self.tree_parent = take(&mut parent.tree_parent);
                            break;
                        }
                        Some(Direction::Left) => {
                            let mut grand_parent = parent.detach_parent();
                            let b = self.detach_right();
                            let c = parent.detach_right();
                            let dir = grand_parent.direction;
                            let mut ggp = grand_parent.detach_parent();
                            grand_parent.attach_left(c);
                            parent.attach_right(grand_parent);
                            parent.attach_left(b);
                            self.attach_right(parent);
                            ggp.attach_child(dir, self);
                            self.tree_parent = take(&mut grand_parent.tree_parent);
                        }
                        Some(Direction::Right) => {
                            let mut grand_parent = parent.detach_parent();
                            let b = self.detach_left();
                            let c = self.detach_right();
                            let dir = grand_parent.direction;
                            let mut ggp = grand_parent.detach_parent();
                            grand_parent.attach_right(b);
                            parent.attach_left(c);
                            self.attach_left(grand_parent);
                            self.attach_right(parent);
                            ggp.attach_child(dir, self);
                            self.tree_parent = take(&mut grand_parent.tree_parent);
                        }
                    }
                }
                Some(Direction::Right) => {
                    let mut parent = self.detach_parent();
                    match parent.direction {
                        None => {
                            let b = self.detach_left();
                            parent.attach_right(b);
                            self.attach_left(parent);
                            self.tree_parent = take(&mut parent.tree_parent);
                            break;
                        }
                        Some(Direction::Right) => {
                            let mut grand_parent = parent.detach_parent();
                            let b = self.detach_left();
                            let c = parent.detach_left();
                            let dir = grand_parent.direction;
                            let mut ggp = grand_parent.detach_parent();
                            grand_parent.attach_right(c);
                            parent.attach_left(grand_parent);
                            parent.attach_right(b);
                            self.attach_left(parent);
                            ggp.attach_child(dir, self);
                            self.tree_parent = take(&mut grand_parent.tree_parent);
                        }
                        Some(Direction::Left) => {
                            let mut grand_parent = parent.detach_parent();
                            let b = self.detach_right();
                            let c = self.detach_left();
                            let dir = grand_parent.direction;
                            let mut ggp = grand_parent.detach_parent();
                            grand_parent.attach_left(b);
                            parent.attach_right(c);
                            self.attach_right(grand_parent);
                            self.attach_left(parent);
                            ggp.attach_child(dir, self);
                            self.tree_parent = take(&mut grand_parent.tree_parent);
                        }
                    }
                }
            }
        }
    }

    fn leftmost(mut self) -> Self {
        while self.left.size != 0 {
            self = self.left;
        }
        self
    }

    pub fn access(mut self) -> Self {
        Self::check_lock();
        self.splay();
        let mut right = self.detach_right();
        if right.size != 0 {
            right.tree_parent = self;
        }
        self.update();
        let mut tree_parent = self.tree_parent;
        let res = if tree_parent.size != 0 {
            let res = tree_parent.access();
            tree_parent.attach_right(self);
            self.splay();
            res
        } else {
            self
        };
        self.tree_parent = LinkCutNode::default();
        res
    }

    pub fn find_root(self) -> Self {
        self.access();
        let root = self.leftmost();
        root.splay();
        root
    }

    pub fn cut(mut self) {
        self.access();
        self.detach_left();
        self.update();
        self.lc_parent = LinkCutNode::default();
    }

    pub fn link(mut self, parent: Self) {
        self.access();
        assert!(self.tree_parent.size == 0);
        parent.access();
        self.attach_left(parent);
        self.lc_parent = parent;
    }

    pub fn dist_to_root(self) -> usize {
        self.access();
        self.size as usize - 1
    }

    pub fn with_payload<R>(self, f: impl FnOnce(&P) -> R) -> R {
        self.access();
        Self::lock();
        let res = unsafe { f(self.payload().unwrap_unchecked()) };
        Self::unlock();
        res
    }

    pub fn with_payload_mut<R>(mut self, f: impl FnOnce(&mut P) -> R) -> R {
        self.access();
        Self::lock();
        let res = f(self.payload_mut());
        Self::unlock();
        res
    }

    pub fn lca(u: Self, v: Self) -> Option<Self> {
        let r1 = u.find_root();
        let lca = v.access();
        let r2 = v.find_root();
        if r1 == r2 {
            Some(lca)
        } else {
            None
        }
    }

    pub fn parent(self) -> Option<Self> {
        if self.lc_parent.size != 0 {
            Some(self.lc_parent)
        } else {
            None
        }
    }

    fn lock() {
        LOCK.with(|lock| lock.set(true));
    }

    fn unlock() {
        LOCK.with(|lock| lock.set(false));
    }

    fn check_lock() {
        assert!(LOCK.with(|lock| !lock.get()), "LinkCutNode is locked");
    }
}

impl<P> PartialEq for LinkCutNode<P> {
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
    }
}

impl<P> Eq for LinkCutNode<P> {}

impl<P> Deref for LinkCutNode<P> {
    type Target = Node<P>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.link.as_ref() }
    }
}

impl<P> DerefMut for LinkCutNode<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.link.as_mut() }
    }
}

impl<P> Default for LinkCutNode<P> {
    fn default() -> Self {
        LinkCutNode {
            link: unsafe {
                NonNull::new_unchecked(&Node::NULL_NODE as *const Node<P> as *mut Node<P>)
            },
        }
    }
}

thread_local! {
    static LOCK: Cell<bool> = Cell::new(false);
}
