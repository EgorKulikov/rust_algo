use crate::collections::payload::{OrdPayload, Payload};
use crate::misc::bump_alloc::bump_new;
use crate::misc::direction::Direction;
use crate::misc::random::{RandomTrait, StaticRandom};
use std::cmp::Ordering;
use std::collections::Bound;
use std::marker::PhantomPinned;
use std::mem::take;
use std::ops::{Deref, DerefMut, RangeBounds};
use std::ptr::NonNull;

// Shared nodes are never mutated: mutating ops copy the path, push_down runs
// only on private copies and installs adjusted copies of the children, reads
// resolve reversal and pending deltas contextually on the way down.

pub struct PersistentContent<P> {
    payload: P,
    left: PersistentTreapNode<P>,
    right: PersistentTreapNode<P>,
}

impl<P: Payload + Clone> PersistentContent<P> {
    fn update(&mut self) {
        if P::NEED_UPDATE {
            self.payload
                .update(self.left.payload_ref(), self.right.payload_ref());
        }
    }
}

pub struct PersistentNode<P> {
    priority: u32,
    size: u32,
    reversed: bool,
    content: Option<PersistentContent<P>>,
    _phantom_pinned: PhantomPinned,
}

impl<P> PersistentNode<P> {
    const NULL_NODE: Self = Self {
        priority: 0,
        size: 0,
        reversed: false,
        content: None,
        _phantom_pinned: PhantomPinned,
    };

    fn payload_ref(&self) -> Option<&P> {
        self.content.as_ref().map(|c| &c.payload)
    }

    fn payload_mut(&mut self) -> &mut P {
        unsafe { &mut self.content.as_mut().unwrap_unchecked().payload }
    }

    fn size(&self) -> usize {
        self.size as usize
    }
}

impl<P: Payload + Clone> PersistentNode<P> {
    fn update(&mut self) {
        self.size = 1 + self.left.size + self.right.size;
        self.deref_mut().update();
    }

    // Only called on private (freshly copied) nodes: pending state moves into
    // adjusted copies of the children, the shared originals stay untouched.
    fn push_down(&mut self) {
        if self.size == 0 {
            return;
        }
        let need_acc = P::NEED_ACCUMULATE && self.payload.need_push_down();
        if !self.reversed && !need_acc {
            return;
        }
        let reversed = self.reversed;
        let delta = if need_acc {
            Some(self.payload.clone())
        } else {
            None
        };
        let (left, right) = if reversed {
            (self.right, self.left)
        } else {
            (self.left, self.right)
        };
        self.left = left.copy_adjusted(reversed, delta.as_ref());
        self.right = right.copy_adjusted(reversed, delta.as_ref());
        self.reversed = false;
        if need_acc {
            self.payload.reset_delta();
        }
    }

    fn detach_left(&mut self) -> PersistentTreapNode<P> {
        self.push_down();
        take(&mut self.left)
    }

    fn detach_right(&mut self) -> PersistentTreapNode<P> {
        self.push_down();
        take(&mut self.right)
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
        PersistentTreapNode {
            link: bump_new(node),
        }
    }

    fn copy(&self) -> Self {
        let node = PersistentNode {
            priority: self.priority,
            size: self.size,
            reversed: self.reversed,
            content: self.content.as_ref().map(|c| PersistentContent {
                payload: c.payload.clone(),
                left: c.left,
                right: c.right,
            }),
            _phantom_pinned: PhantomPinned,
        };
        PersistentTreapNode {
            link: bump_new(node),
        }
    }

    fn copy_adjusted(&self, toggle_reversed: bool, delta: Option<&P>) -> Self {
        if self.size == 0 {
            return *self;
        }
        let mut res = self.copy();
        if toggle_reversed {
            res.reversed ^= true;
        }
        if P::NEED_ACCUMULATE {
            if let Some(delta) = delta {
                res.payload_mut().accumulate(delta);
            }
        }
        res
    }

    // Payload as seen through pending ancestor deltas (`carrier`).
    fn adjusted(&self, carrier: Option<&P>) -> P {
        let mut payload = unsafe { self.payload_ref().unwrap_unchecked() }.clone();
        if P::NEED_ACCUMULATE {
            if let Some(carrier) = carrier {
                payload.accumulate(carrier);
            }
        }
        payload
    }

    fn edge(mut node: Self, to_first: bool) -> Option<P> {
        let mut rev = false;
        let mut carrier: Option<P> = None;
        while node.size != 0 {
            let r = rev ^ node.reversed;
            let payload = node.adjusted(carrier.as_ref());
            let (eff_left, eff_right) = if r {
                (node.right, node.left)
            } else {
                (node.left, node.right)
            };
            let target = if to_first { eff_left } else { eff_right };
            if target.size == 0 {
                return Some(payload);
            }
            carrier = Some(payload);
            rev = r;
            node = target;
        }
        None
    }

    fn merge(left: &Self, right: &Self) -> Self {
        Self::merge_ctx(*left, false, None, *right, false, None)
    }

    // Applies pending context (reversal + delta) to a shared node, copying
    // only when there is something to apply.
    fn materialize(node: Self, rev: bool, delta: Option<&P>) -> Self {
        if rev || delta.is_some() {
            node.copy_adjusted(rev, delta)
        } else {
            node
        }
    }

    // Copies a node composing the pending context into it, then extracts the
    // combined pending state (cleared on the copy) to pass to the children.
    fn copy_with(node: &Self, rev: bool, delta: Option<&P>) -> (Self, bool, Option<P>) {
        let mut cur = node.copy();
        if P::NEED_ACCUMULATE {
            if let Some(delta) = delta {
                cur.payload_mut().accumulate(delta);
            }
        }
        let pending_rev = cur.reversed ^ rev;
        cur.reversed = false;
        let pending_delta = if P::NEED_ACCUMULATE && cur.payload.need_push_down() {
            let delta = cur.payload.clone();
            cur.payload_mut().reset_delta();
            Some(delta)
        } else {
            None
        };
        (cur, pending_rev, pending_delta)
    }

    fn eff_children(cur: &Self, rev: bool) -> (Self, Self) {
        if rev {
            (cur.right, cur.left)
        } else {
            (cur.left, cur.right)
        }
    }

    fn set_children(mut cur: Self, left: Self, right: Self) -> Self {
        {
            let content = cur.deref_mut().deref_mut();
            content.left = Self::default();
            content.right = Self::default();
        }
        cur.attach_left(left);
        cur.attach_right(right);
        cur
    }

    fn merge_ctx(
        a: Self,
        arev: bool,
        adelta: Option<P>,
        b: Self,
        brev: bool,
        bdelta: Option<P>,
    ) -> Self {
        if a.size == 0 {
            return Self::materialize(b, brev, bdelta.as_ref());
        }
        if b.size == 0 {
            return Self::materialize(a, arev, adelta.as_ref());
        }
        if a.priority > b.priority {
            let (cur, rev, delta) = Self::copy_with(&a, arev, adelta.as_ref());
            let (eff_left, eff_right) = Self::eff_children(&cur, rev);
            let left = Self::materialize(eff_left, rev, delta.as_ref());
            let right = Self::merge_ctx(eff_right, rev, delta, b, brev, bdelta);
            Self::set_children(cur, left, right)
        } else {
            let (cur, rev, delta) = Self::copy_with(&b, brev, bdelta.as_ref());
            let (eff_left, eff_right) = Self::eff_children(&cur, rev);
            let right = Self::materialize(eff_right, rev, delta.as_ref());
            let left = Self::merge_ctx(a, arev, adelta, eff_left, rev, delta);
            Self::set_children(cur, left, right)
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
            (Self::default(), Self::default())
        }
    }

    fn split_at(&self, at: usize) -> (Self, Self) {
        Self::split_at_ctx(*self, at, false, None)
    }

    fn split_at_ctx(node: Self, mut at: usize, rev: bool, delta: Option<P>) -> (Self, Self) {
        if node.size == 0 {
            return (Self::default(), Self::default());
        }
        if at == 0 {
            return (
                Self::default(),
                Self::materialize(node, rev, delta.as_ref()),
            );
        }
        if at >= node.size() {
            return (
                Self::materialize(node, rev, delta.as_ref()),
                Self::default(),
            );
        }
        let (cur, rev, delta) = Self::copy_with(&node, rev, delta.as_ref());
        let (eff_left, eff_right) = Self::eff_children(&cur, rev);
        let left_size = eff_left.size();
        if at <= left_size {
            let (a, b) = Self::split_at_ctx(eff_left, at, rev, delta.clone());
            let right = Self::materialize(eff_right, rev, delta.as_ref());
            (a, Self::set_children(cur, b, right))
        } else {
            at -= left_size + 1;
            let (a, b) = Self::split_at_ctx(eff_right, at, rev, delta.clone());
            let left = Self::materialize(eff_left, rev, delta.as_ref());
            (Self::set_children(cur, left, a), b)
        }
    }

    // Aggregate payload of positions [from, to); read-only, no allocations.
    fn range_payload(
        node: Self,
        from: usize,
        to: usize,
        rev: bool,
        carrier: Option<&P>,
    ) -> Option<P> {
        if node.size == 0 || from >= to {
            return None;
        }
        let payload = node.adjusted(carrier);
        if from == 0 && to == node.size() {
            return Some(payload);
        }
        let r = rev ^ node.reversed;
        let (eff_left, eff_right) = Self::eff_children(&node, r);
        let left_size = eff_left.size();
        if to <= left_size {
            return Self::range_payload(eff_left, from, to, r, Some(&payload));
        }
        if from > left_size {
            return Self::range_payload(
                eff_right,
                from - left_size - 1,
                to - left_size - 1,
                r,
                Some(&payload),
            );
        }
        let left_part = Self::range_payload(eff_left, from, left_size, r, Some(&payload));
        let right_part = Self::range_payload(eff_right, 0, to - left_size - 1, r, Some(&payload));
        let mut res = payload;
        res.update(left_part.as_ref(), right_part.as_ref());
        Some(res)
    }

    // Applies `apply` to positions [from, to), copying only the boundary
    // paths and the covered subtree roots.
    fn push_range(
        node: Self,
        from: usize,
        to: usize,
        apply: &P,
        rev: bool,
        ctx: Option<P>,
    ) -> Self {
        if node.size == 0 || from >= to {
            return Self::materialize(node, rev, ctx.as_ref());
        }
        if from == 0 && to == node.size() {
            let mut cur = node.copy_adjusted(rev, ctx.as_ref());
            cur.payload_mut().accumulate(apply);
            return cur;
        }
        let (mut cur, rev, delta) = Self::copy_with(&node, rev, ctx.as_ref());
        let (eff_left, eff_right) = Self::eff_children(&cur, rev);
        let left_size = eff_left.size();
        let left = if from < left_size {
            Self::push_range(eff_left, from, to.min(left_size), apply, rev, delta.clone())
        } else {
            Self::materialize(eff_left, rev, delta.as_ref())
        };
        let right = if to > left_size + 1 {
            Self::push_range(
                eff_right,
                from.max(left_size + 1) - left_size - 1,
                to - left_size - 1,
                apply,
                rev,
                delta.clone(),
            )
        } else {
            Self::materialize(eff_right, rev, delta.as_ref())
        };
        if from <= left_size && left_size < to {
            cur.payload_mut().accumulate_self(apply);
        }
        Self::set_children(cur, left, right)
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

    fn find(mut node: Self, key: &P::Key) -> (Option<P>, usize) {
        let mut before = 0;
        let mut carrier: Option<P> = None;
        while node.size != 0 {
            let payload = node.adjusted(carrier.as_ref());
            match key.cmp(payload.key()) {
                Ordering::Less => {
                    carrier = Some(payload);
                    node = node.left;
                }
                Ordering::Greater => {
                    before += node.left.size() + 1;
                    carrier = Some(payload);
                    node = node.right;
                }
                Ordering::Equal => {
                    let index = before + node.left.size();
                    return (Some(payload), index);
                }
            }
        }
        (None, before)
    }

    // Monotone `go_right`; returns (last true payload, first false payload).
    fn descend(mut node: Self, mut go_right: impl FnMut(&P) -> bool) -> (Option<P>, Option<P>) {
        let mut last_true = None;
        let mut first_false = None;
        let mut carrier: Option<P> = None;
        while node.size != 0 {
            let payload = node.adjusted(carrier.as_ref());
            if P::NEED_ACCUMULATE {
                carrier = Some(payload.clone());
            }
            if go_right(&payload) {
                last_true = Some(payload);
                node = node.right;
            } else {
                first_false = Some(payload);
                node = node.left;
            }
        }
        (last_true, first_false)
    }

    fn union(a: &Self, b: &Self) -> Self {
        match (a.size, b.size) {
            (0, _) => *b,
            (_, 0) => *a,
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
                    a = PersistentTreapNode::new(P::union(a.payload.clone(), same.payload.clone()));
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
        *self
    }
}

impl<P> Copy for PersistentTreapNode<P> {}

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

impl<P> Default for PersistentTreapNode<P> {
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

impl<P> Clone for PersistentTree<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for PersistentTree<P> {}

impl<P: Payload + Clone> PersistentTree<P> {
    pub fn new() -> Self {
        Self {
            root: PersistentTreapNode::default(),
        }
    }

    pub fn with_gen(n: usize, f: impl FnMut(usize) -> P) -> Self {
        Self {
            root: PersistentNode::with_gen(n, f),
        }
    }

    pub fn single(p: P) -> Self {
        Self {
            root: PersistentTreapNode::new(p),
        }
    }

    fn do_split(
        self,
        f: impl FnOnce(PersistentTreapNode<P>) -> (PersistentTreapNode<P>, PersistentTreapNode<P>),
    ) -> (Self, Self) {
        let (left, right) = f(self.root);
        (Self { root: left }, Self { root: right })
    }

    pub fn split_by(self, f: impl FnMut(&P, Option<&P>, Option<&P>) -> Direction) -> (Self, Self) {
        let (left, right) = self.root.split_by(f);
        (Self { root: left }, Self { root: right })
    }

    pub fn push(self, delta: &P) -> Self {
        self.with_payload_mut(|p| p.accumulate(delta))
    }

    pub fn with_payload_mut(self, f: impl FnOnce(&mut P)) -> Self {
        if self.is_empty() {
            return self;
        }
        let mut root = self.root.copy();
        root.push_down();
        f(root.payload_mut());
        Self { root }
    }

    pub fn merge(left: Self, right: Self) -> Self {
        Self {
            root: PersistentTreapNode::merge(&left.root, &right.root),
        }
    }

    pub fn merge_three(left: Self, mid: Self, right: Self) -> Self {
        Self::merge(Self::merge(left, mid), right)
    }

    pub fn iter(&self) -> Iter<P> {
        Iter::new(self.root)
    }

    pub fn first(&self) -> Option<P> {
        PersistentTreapNode::edge(self.root, true)
    }

    pub fn last(&self) -> Option<P> {
        PersistentTreapNode::edge(self.root, false)
    }

    pub fn payload(&self) -> Option<P> {
        self.root.payload_ref().cloned()
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

    pub fn binary_search_size(&self, mut f: impl FnMut(usize, usize) -> Option<Direction>) {
        let mut node = self.root;
        let mut rev = false;
        while node.size != 0 {
            let r = rev ^ node.reversed;
            let (eff_left, eff_right) = if r {
                (node.right, node.left)
            } else {
                (node.left, node.right)
            };
            match f(eff_left.size(), eff_right.size()) {
                Some(Direction::Left) => node = eff_left,
                Some(Direction::Right) => node = eff_right,
                None => return,
            }
            rev = r;
        }
    }

    pub fn binary_search(
        &self,
        mut f: impl FnMut(&P, Option<&P>, Option<&P>) -> Option<Direction>,
    ) {
        let mut node = self.root;
        let mut rev = false;
        let mut carrier: Option<P> = None;
        while node.size != 0 {
            let r = rev ^ node.reversed;
            let payload = node.adjusted(carrier.as_ref());
            let (eff_left, eff_right) = if r {
                (node.right, node.left)
            } else {
                (node.left, node.right)
            };
            let left_payload = (eff_left.size != 0).then(|| eff_left.adjusted(Some(&payload)));
            let right_payload = (eff_right.size != 0).then(|| eff_right.adjusted(Some(&payload)));
            match f(&payload, left_payload.as_ref(), right_payload.as_ref()) {
                Some(Direction::Left) => node = eff_left,
                Some(Direction::Right) => node = eff_right,
                None => return,
            }
            carrier = Some(payload);
            rev = r;
        }
    }

    fn decode_bounds(&self, bounds: impl RangeBounds<usize>) -> (usize, usize) {
        let start = match bounds.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 0,
        };
        let end = match bounds.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => self.size(),
        };
        assert!(start <= end);
        (start, end)
    }

    pub fn split_range_index(self, bounds: impl RangeBounds<usize>) -> (Self, Self, Self) {
        let (start, end) = self.decode_bounds(bounds);
        let (left, mid_right) = self.root.split_at(start);
        let (mid, right) = mid_right.split_at(end - start);
        (
            Self { root: left },
            Self { root: mid },
            Self { root: right },
        )
    }

    // Aggregate payload over the range; read-only, allocation-free.
    pub fn range_payload(&self, bounds: impl RangeBounds<usize>) -> Option<P> {
        let (start, end) = self.decode_bounds(bounds);
        PersistentTreapNode::range_payload(self.root, start, end, false, None)
    }

    // New version with `delta` accumulated over the range; copies only the
    // boundary paths instead of splitting and merging.
    pub fn push_range(self, bounds: impl RangeBounds<usize>, delta: &P) -> Self {
        let (start, end) = self.decode_bounds(bounds);
        if start >= end {
            return self;
        }
        Self {
            root: PersistentTreapNode::push_range(self.root, start, end, delta, false, None),
        }
    }

    pub fn range_index(self, bounds: impl RangeBounds<usize>) -> Self {
        self.split_range_index(bounds).1
    }

    pub fn get_at(&self, at: usize) -> Self {
        self.range_index(at..=at)
    }

    pub fn reverse(self) -> Self {
        if self.is_empty() {
            return self;
        }
        let mut root = self.root.copy();
        root.reversed ^= true;
        Self { root }
    }
}

pub struct Iter<P> {
    stack: Vec<(PersistentTreapNode<P>, bool, P)>,
}

impl<P: Payload + Clone> Iter<P> {
    fn new(root: PersistentTreapNode<P>) -> Self {
        let mut res = Self { stack: Vec::new() };
        res.add_left(root, false, None);
        res
    }

    fn add_left(
        &mut self,
        mut node: PersistentTreapNode<P>,
        mut rev: bool,
        mut carrier: Option<P>,
    ) {
        while node.size != 0 {
            let r = rev ^ node.reversed;
            let payload = node.adjusted(carrier.as_ref());
            let next = if r { node.right } else { node.left };
            if P::NEED_ACCUMULATE {
                carrier = Some(payload.clone());
            }
            self.stack.push((node, r, payload));
            node = next;
            rev = r;
        }
    }
}

impl<P: Payload + Clone> Iterator for Iter<P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        let (node, rev, payload) = self.stack.pop()?;
        let right = if rev { node.left } else { node.right };
        let carrier = P::NEED_ACCUMULATE.then(|| payload.clone());
        self.add_left(right, rev, carrier);
        Some(payload)
    }
}

impl<P: OrdPayload + Clone> PersistentTree<P> {
    pub fn split_range<'a>(self, bounds: impl RangeBounds<&'a P::Key>) -> (Self, Self, Self)
    where
        <P as OrdPayload>::Key: 'a,
    {
        let (left, mid_right) = match bounds.start_bound() {
            Bound::Included(key) => self.root.split(key),
            Bound::Excluded(key) => self.root.split_inclusive(key),
            Bound::Unbounded => (PersistentTreapNode::default(), self.root),
        };
        let (mid, right) = match bounds.end_bound() {
            Bound::Included(key) => mid_right.split_inclusive(key),
            Bound::Excluded(key) => mid_right.split(key),
            Bound::Unbounded => (mid_right, PersistentTreapNode::default()),
        };
        (
            Self { root: left },
            Self { root: mid },
            Self { root: right },
        )
    }

    pub fn range<'a>(self, bounds: impl RangeBounds<&'a P::Key>) -> Self
    where
        <P as OrdPayload>::Key: 'a,
    {
        self.split_range(bounds).1
    }

    pub fn insert_or_update(self, p: P) -> Self {
        let (left, right) = self.split(p.key());
        let (mid, right) = right.split_inclusive(p.key());
        let mid = if let Some(old) = mid.payload() {
            PersistentTree::single(P::union(old, p))
        } else {
            PersistentTree::single(p)
        };
        Self::merge_three(left, mid, right)
    }

    pub fn insert(self, p: P) -> (Self, Option<P>) {
        let (left, right) = self.split(p.key());
        let (mid, right) = right.split_inclusive(p.key());
        (
            Self::merge_three(left, PersistentTree::single(p), right),
            mid.payload(),
        )
    }

    pub fn remove(self, key: &P::Key) -> (Self, Option<P>) {
        let (left, right) = self.split(key);
        let (mid, right) = right.split_inclusive(key);
        (Self::merge(left, right), mid.payload())
    }

    pub fn split(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split(key))
    }

    pub fn split_inclusive(self, key: &P::Key) -> (Self, Self) {
        self.do_split(|root| root.split_inclusive(key))
    }

    pub fn get(&self, key: &P::Key) -> Option<P> {
        PersistentTreapNode::find(self.root, key).0
    }

    pub fn index(&self, key: &P::Key) -> Option<usize> {
        let (payload, index) = PersistentTreapNode::find(self.root, key);
        payload.map(|_| index)
    }

    pub fn floor(&self, key: &P::Key) -> Option<P> {
        PersistentTreapNode::descend(self.root, |p| p.key() <= key).0
    }

    pub fn ceil(&self, key: &P::Key) -> Option<P> {
        PersistentTreapNode::descend(self.root, |p| p.key() < key).1
    }

    pub fn prev(&self, key: &P::Key) -> Option<P> {
        PersistentTreapNode::descend(self.root, |p| p.key() < key).0
    }

    pub fn next(&self, key: &P::Key) -> Option<P> {
        PersistentTreapNode::descend(self.root, |p| p.key() <= key).1
    }

    pub fn union(a: Self, b: Self) -> Self {
        Self {
            root: PersistentTreapNode::union(&a.root, &b.root),
        }
    }
}

#[cfg(test)]
mod test {
    use super::PersistentTree;
    use crate::collections::payload::PurePayload;
    use crate::misc::random::{Random, RandomTrait};
    use std::collections::BTreeSet;

    fn make_seq(n: usize) -> PersistentTree<PurePayload<i64>> {
        PersistentTree::with_gen(n, |i| PurePayload(i as i64))
    }

    fn collect(t: &PersistentTree<PurePayload<i64>>) -> Vec<i64> {
        t.iter().map(|p| p.0).collect()
    }

    #[test]
    fn basic_order() {
        let t = make_seq(7);
        assert_eq!(collect(&t), vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn reverse_gives_correct_order() {
        let t = make_seq(7);
        let r = t.reverse();
        assert_eq!(collect(&r), vec![6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn reverse_does_not_corrupt_original() {
        let t1 = make_seq(7);
        assert_eq!(collect(&t1), vec![0, 1, 2, 3, 4, 5, 6]);
        let t2 = t1.reverse();
        assert_eq!(collect(&t2), vec![6, 5, 4, 3, 2, 1, 0]);
        assert_eq!(
            collect(&t1),
            vec![0, 1, 2, 3, 4, 5, 6],
            "original tree was corrupted by iterating the reversed copy"
        );
    }

    #[test]
    fn double_reverse_is_identity() {
        let t1 = make_seq(5);
        let t2 = t1.reverse().reverse();
        assert_eq!(collect(&t2), vec![0, 1, 2, 3, 4]);
        assert_eq!(collect(&t1), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn split_does_not_corrupt_original() {
        let t1 = make_seq(6);
        let (left, right) = t1.split_at(3);
        assert_eq!(collect(&left), vec![0, 1, 2]);
        assert_eq!(collect(&right), vec![3, 4, 5]);
        assert_eq!(
            collect(&t1),
            vec![0, 1, 2, 3, 4, 5],
            "original tree corrupted after split"
        );
    }

    #[test]
    fn merge_does_not_corrupt_sources() {
        let t1 = make_seq(3);
        let t2 = make_seq(3);
        let merged = PersistentTree::merge(t1, t2);
        assert_eq!(collect(&merged), vec![0, 1, 2, 0, 1, 2]);
        assert_eq!(collect(&t1), vec![0, 1, 2], "t1 corrupted after merge");
        assert_eq!(collect(&t2), vec![0, 1, 2], "t2 corrupted after merge");
    }

    #[test]
    fn multiple_reverses_preserve_all() {
        let t1 = make_seq(5);
        let t2 = t1.reverse();
        let t3 = t1.reverse();
        assert_eq!(collect(&t2), vec![4, 3, 2, 1, 0]);
        assert_eq!(collect(&t3), vec![4, 3, 2, 1, 0]);
        assert_eq!(
            collect(&t1),
            vec![0, 1, 2, 3, 4],
            "original corrupted after iterating multiple reversed copies"
        );
    }

    #[test]
    fn iter_twice_gives_same_result() {
        let t = make_seq(5);
        assert_eq!(collect(&t), collect(&t));
        let r = make_seq(5).reverse();
        assert_eq!(collect(&r), collect(&r));
    }

    #[test]
    fn versions_stress() {
        let mut rng = Random::new_with_seed(777);
        for _ in 0..20 {
            let mut versions: Vec<(PersistentTree<PurePayload<i64>>, Vec<i64>)> = vec![(
                PersistentTree::with_gen(5, |i| PurePayload(i as i64)),
                (0..5).collect(),
            )];
            for _ in 0..60 {
                let idx = rng.gen_bound(versions.len());
                let (tree, reference) = versions[idx].clone();
                match rng.gen_bound(4u32) {
                    0 => {
                        let mut reversed = reference.clone();
                        reversed.reverse();
                        versions.push((tree.reverse(), reversed));
                    }
                    1 => {
                        let at = rng.gen_bound(reference.len() + 1);
                        let (left, right) = tree.split_at(at);
                        let mut rotated = reference[at..].to_vec();
                        rotated.extend_from_slice(&reference[..at]);
                        versions.push((PersistentTree::merge(right, left), rotated));
                    }
                    2 => {
                        let other_idx = rng.gen_bound(versions.len());
                        let (other, other_reference) = versions[other_idx].clone();
                        let mut concat = reference.clone();
                        concat.extend_from_slice(&other_reference);
                        if concat.len() <= 100 {
                            versions.push((PersistentTree::merge(tree, other), concat));
                        }
                    }
                    _ => {
                        if !reference.is_empty() {
                            let from = rng.gen_bound(reference.len());
                            let to = from + rng.gen_bound(reference.len() - from) + 1;
                            versions
                                .push((tree.range_index(from..to), reference[from..to].to_vec()));
                        }
                    }
                }
            }
            for (tree, reference) in &versions {
                assert_eq!(&collect(tree), reference);
                assert_eq!(tree.size(), reference.len());
                assert_eq!(tree.first().map(|p| p.0), reference.first().copied());
                assert_eq!(tree.last().map(|p| p.0), reference.last().copied());
            }
        }
    }

    #[test]
    fn ordered_versions_match_btree() {
        let mut rng = Random::new_with_seed(778);
        for _ in 0..20 {
            let mut versions: Vec<(PersistentTree<PurePayload<u32>>, BTreeSet<u32>)> =
                vec![(PersistentTree::new(), BTreeSet::new())];
            for _ in 0..100 {
                let idx = rng.gen_bound(versions.len());
                let (tree, reference) = versions[idx].clone();
                let key = rng.gen_bound(30u32);
                match rng.gen_bound(3u32) {
                    0 => {
                        let (new_tree, old) = tree.insert(PurePayload(key));
                        assert_eq!(old.is_some(), reference.contains(&key));
                        let mut new_reference = reference.clone();
                        new_reference.insert(key);
                        versions.push((new_tree, new_reference));
                    }
                    1 => {
                        let (new_tree, old) = tree.remove(&key);
                        assert_eq!(old.is_some(), reference.contains(&key));
                        let mut new_reference = reference.clone();
                        new_reference.remove(&key);
                        versions.push((new_tree, new_reference));
                    }
                    _ => {
                        assert_eq!(tree.get(&key).map(|p| p.0), reference.get(&key).copied());
                        assert_eq!(
                            tree.floor(&key).map(|p| p.0),
                            reference.range(..=key).next_back().copied()
                        );
                        assert_eq!(
                            tree.ceil(&key).map(|p| p.0),
                            reference.range(key..).next().copied()
                        );
                        assert_eq!(
                            tree.index(&key),
                            reference
                                .contains(&key)
                                .then(|| reference.range(..key).count())
                        );
                    }
                }
            }
            for (tree, reference) in &versions {
                let values: Vec<u32> = tree.iter().map(|p| p.0).collect();
                let expected: Vec<u32> = reference.iter().copied().collect();
                assert_eq!(values, expected);
            }
        }
    }
}

#[cfg(test)]
mod lazy_test {
    use super::PersistentTree;
    use crate::collections::payload::Payload;
    use crate::misc::random::{Random, RandomTrait};

    #[derive(Clone, Copy, Debug)]
    struct AddSum {
        value: i64,
        sum: i64,
        size: i64,
        delta: i64,
    }

    impl AddSum {
        fn new(value: i64) -> Self {
            Self {
                value,
                sum: value,
                size: 1,
                delta: 0,
            }
        }

        fn add(delta: i64) -> Self {
            Self {
                value: 0,
                sum: 0,
                size: 0,
                delta,
            }
        }
    }

    impl Payload for AddSum {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;

        fn reset_delta(&mut self) {
            self.delta = 0;
        }

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.size = 1;
            self.sum = self.value;
            if let Some(left) = left {
                self.size += left.size;
                self.sum += left.sum;
            }
            if let Some(right) = right {
                self.size += right.size;
                self.sum += right.sum;
            }
        }

        fn accumulate(&mut self, delta: &Self) {
            self.delta += delta.delta;
            self.value += delta.delta;
            self.sum += delta.delta * self.size;
        }

        fn accumulate_self(&mut self, delta: &Self) {
            self.value += delta.delta;
        }

        fn need_push_down(&self) -> bool {
            self.delta != 0
        }
    }

    #[test]
    fn lazy_add_versions_stress() {
        let mut rng = Random::new_with_seed(779);
        for _ in 0..20 {
            let n = 1 + rng.gen_bound(8usize);
            let base: Vec<i64> = (0..n).map(|_| rng.gen_bound(100u64) as i64).collect();
            let mut versions: Vec<(PersistentTree<AddSum>, Vec<i64>)> =
                vec![(PersistentTree::with_gen(n, |i| AddSum::new(base[i])), base)];
            for _ in 0..60 {
                let idx = rng.gen_bound(versions.len());
                let (tree, reference) = versions[idx].clone();
                let len = reference.len();
                match rng.gen_bound(4u32) {
                    0 => {
                        let from = rng.gen_bound(len);
                        let to = from + 1 + rng.gen_bound(len - from);
                        let x = rng.gen_bound(21u64) as i64 - 10;
                        let new_tree = tree.push_range(from..to, &AddSum::add(x));
                        let mut new_reference = reference.clone();
                        for value in &mut new_reference[from..to] {
                            *value += x;
                        }
                        versions.push((new_tree, new_reference));
                    }
                    1 => {
                        let mut reversed = reference.clone();
                        reversed.reverse();
                        versions.push((tree.reverse(), reversed));
                    }
                    2 => {
                        let at = rng.gen_bound(len + 1);
                        let (left, right) = tree.split_at(at);
                        let mut rotated = reference[at..].to_vec();
                        rotated.extend_from_slice(&reference[..at]);
                        versions.push((PersistentTree::merge(right, left), rotated));
                    }
                    _ => {
                        let from = rng.gen_bound(len);
                        let to = from + 1 + rng.gen_bound(len - from);
                        versions.push((tree.range_index(from..to), reference[from..to].to_vec()));
                    }
                }
            }
            for (tree, reference) in &versions {
                let values: Vec<i64> = tree.iter().map(|p| p.value).collect();
                assert_eq!(&values, reference);
                assert_eq!(
                    tree.payload().map_or(0, |p| p.sum),
                    reference.iter().sum::<i64>(),
                    "root sum mismatch"
                );
                assert_eq!(tree.first().map(|p| p.value), reference.first().copied());
                assert_eq!(tree.last().map(|p| p.value), reference.last().copied());
                let from = rng.gen_bound(reference.len());
                let to = from + 1 + rng.gen_bound(reference.len() - from);
                assert_eq!(
                    tree.range_payload(from..to).map_or(0, |p| p.sum),
                    reference[from..to].iter().sum::<i64>(),
                    "range sum mismatch"
                );
                assert_eq!(tree.range_payload(from..from).map(|p| p.sum), None);
            }
        }
    }
}
