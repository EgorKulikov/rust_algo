use crate::collections::bounds::clamp;
use crate::misc::direction::Direction;
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::when;
use std::marker::PhantomData;
use std::ops::{Add, MulAssign, RangeBounds};

pub trait SegmentTreeNode {
    fn new(left: usize, right: usize) -> Self;
    fn join(&mut self, left_val: &Self, right_val: &Self);
    fn accumulate(&mut self, value: &Self);
    fn reset_delta(&mut self);
}

pub trait Pushable<T>: SegmentTreeNode {
    fn push(&mut self, delta: T);
}

impl<T: SegmentTreeNode> Pushable<&T> for T {
    fn push(&mut self, delta: &T) {
        self.accumulate(delta);
    }
}

impl<T: SegmentTreeNode> Pushable<T> for T {
    fn push(&mut self, delta: T) {
        *self = delta;
    }
}

pub trait QueryResult<Result, Args>: SegmentTreeNode {
    fn empty_result(args: &Args) -> Result;
    fn result(&self, args: &Args) -> Result;
    fn join_results(
        left_res: Result,
        right_res: Result,
        args: &Args,
        left: usize,
        mid: usize,
        right: usize,
    ) -> Result;
}

impl<T: SegmentTreeNode + Clone> QueryResult<T, ()> for T {
    fn empty_result(_: &()) -> T {
        Self::new(0, 0)
    }

    fn result(&self, _: &()) -> T {
        self.clone()
    }

    fn join_results(left_res: T, right_res: T, _: &(), left: usize, mid: usize, right: usize) -> T {
        when! {
            left == mid => right_res,
            right == mid => left_res,
            else => {
                let mut res = Self::new(left, right);
                res.join(&left_res, &right_res);
                res
            },
        }
    }
}

impl<
        Key: Add<Output = Key> + MulAssign<Delta> + Zero + Copy,
        Delta: MulAssign<Delta> + One + Copy,
    > SegmentTreeNode for (Key, Delta)
{
    fn new(_left: usize, _right: usize) -> Self {
        (Key::zero(), Delta::one())
    }

    fn join(&mut self, left_val: &Self, right_val: &Self) {
        self.0 = left_val.0 + right_val.0;
    }

    fn accumulate(&mut self, value: &Self) {
        self.0 *= value.1;
        self.1 *= value.1;
    }

    fn reset_delta(&mut self) {
        self.1 = Delta::one();
    }
}

#[derive(Clone)]
pub struct SegmentTree<Node> {
    n: usize,
    nodes: Vec<Node>,
}

impl<Node: SegmentTreeNode> SegmentTree<Node> {
    pub fn new(n: usize) -> Self {
        Self::from_generator(n, |left| Node::new(left, left + 1))
    }

    pub fn from_array(arr: Vec<Node>) -> Self {
        let n = arr.len();
        let mut iter = arr.into_iter();
        Self::from_generator(n, |_| iter.next().unwrap())
    }

    pub fn from_generator<F>(n: usize, gen: F) -> Self
    where
        F: FnMut(usize) -> Node,
    {
        if n == 0 {
            return Self {
                n,
                nodes: vec![Node::new(0, 0)],
            };
        }
        let mut res = Self {
            n,
            nodes: Vec::with_capacity(2 * n - 1),
        };
        res.init(gen);
        res
    }

    fn init<F>(&mut self, mut f: F)
    where
        F: FnMut(usize) -> Node,
    {
        self.init_impl(2 * self.n - 2, 0, self.n, &mut f);
    }

    fn init_impl<F>(&mut self, root: usize, left: usize, right: usize, f: &mut F)
    where
        F: FnMut(usize) -> Node,
    {
        if left + 1 == right {
            self.nodes.push(f(left));
        } else {
            let mid = left + ((right - left) >> 1);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            self.init_impl(left_child, left, mid, f);
            self.init_impl(right_child, mid, right, f);
            let mut node = Node::new(left, right);
            node.join(&self.nodes[left_child], &self.nodes[right_child]);
            self.nodes.push(node);
        }
    }

    pub fn point_query(&mut self, at: usize) -> &Node {
        assert!(at < self.n);
        self.do_point_query(self.nodes.len() - 1, 0, self.n, at)
    }

    fn do_point_query(&mut self, root: usize, left: usize, right: usize, at: usize) -> &Node {
        if left + 1 == right {
            &self.nodes[root]
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            if at < mid {
                self.do_point_query(left_child, left, mid, at)
            } else {
                self.do_point_query(right_child, mid, right, at)
            }
        }
    }

    pub fn point_update<T>(&mut self, at: usize, val: T)
    where
        Node: Pushable<T>,
    {
        assert!(at < self.n);
        self.do_point_update(self.nodes.len() - 1, 0, self.n, at, val);
    }

    fn do_point_update<T>(&mut self, root: usize, left: usize, right: usize, at: usize, val: T)
    where
        Node: Pushable<T>,
    {
        if left + 1 == right {
            self.nodes[root].push(val);
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            if at < mid {
                self.do_point_update(left_child, left, mid, at, val);
            } else {
                self.do_point_update(right_child, mid, right, at, val);
            }
            self.join(root, mid, right);
        }
    }

    pub fn point_through_update<'a, T>(&mut self, at: usize, val: &'a T)
    where
        Node: Pushable<&'a T>,
    {
        assert!(at < self.n);
        self.do_point_through_update(self.nodes.len() - 1, 0, self.n, at, val);
    }

    fn do_point_through_update<'a, T>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        at: usize,
        val: &'a T,
    ) where
        Node: Pushable<&'a T>,
    {
        self.nodes[root].push(val);
        if left + 1 != right {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            if at < mid {
                self.do_point_through_update(left_child, left, mid, at, val);
            } else {
                self.do_point_through_update(right_child, mid, right, at, val);
            }
        }
    }

    pub fn point_operation<Args, Res>(
        &mut self,
        op: &mut dyn PointOperation<Node, Args, Res>,
        args: Args,
    ) -> Res {
        assert_ne!(self.n, 0);
        self.do_point_operation(op, self.nodes.len() - 1, 0, self.n, args)
    }

    fn do_point_operation<Args, Res>(
        &mut self,
        op: &mut dyn PointOperation<Node, Args, Res>,
        root: usize,
        left: usize,
        right: usize,
        args: Args,
    ) -> Res {
        if left + 1 == right {
            op.adjust_leaf(&mut self.nodes[root], left, args)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let (l, r) = self.nodes.split_at_mut(root);
            let (l, m) = l.split_at_mut(right_child);
            let direction = op.select_branch(
                &mut r[0],
                &mut l[left_child],
                &mut m[0],
                &args,
                left,
                mid,
                right,
            );
            let res = match direction {
                Direction::Left => self.do_point_operation(op, left_child, left, mid, args),
                Direction::Right => self.do_point_operation(op, right_child, mid, right, args),
            };
            self.join(root, mid, right);
            res
        }
    }

    pub fn update<'a, T>(&mut self, range: impl RangeBounds<usize>, val: &'a T)
    where
        Node: Pushable<&'a T>,
    {
        let (from, to) = clamp(range, self.n);
        self.do_update(self.nodes.len() - 1, 0, self.n, from, to, val)
    }

    pub fn do_update<'a, T>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        val: &'a T,
    ) where
        Node: Pushable<&'a T>,
    {
        when! {
            left >= to || right <= from => {},
            left >= from && right <= to => self.nodes[root].push(val),
            else => {
                let mid = (left + right) >> 1;
                self.push_down(root, mid, right);
                let left_child = root - 2 * (right - mid);
                let right_child = root - 1;
                self.do_update(left_child, left, mid, from, to, val);
                self.do_update(right_child, mid, right, from, to, val);
                self.join(root, mid, right);
            },
        }
    }

    pub fn operation<Args, Res>(
        &mut self,
        range: impl RangeBounds<usize>,
        op: &mut dyn Operation<Node, Args, Res>,
        args: &Args,
    ) -> Res {
        let (from, to) = clamp(range, self.n);
        self.do_operation(self.nodes.len() - 1, 0, self.n, from, to, op, args)
    }

    pub fn do_operation<Args, Res>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        op: &mut dyn Operation<Node, Args, Res>,
        args: &Args,
    ) -> Res {
        when! {
            left >= to || right <= from => op.empty_result(left, right, args),
            left >= from && right <= to => op.process_result(&mut self.nodes[root], args),
            else => {
                let mid = (left + right) >> 1;
                self.push_down(root, mid, right);
                let left_child = root - 2 * (right - mid);
                let right_child = root - 1;
                let left_result = self.do_operation(left_child, left, mid, from, to, op, args);
                let right_result = self.do_operation(right_child, mid, right, from, to, op, args);
                self.join(root, mid, right);
                op.join_results(left_result, right_result, args)
            },
        }
    }

    pub fn binary_search<Res>(
        &mut self,
        wh: impl FnMut(&Node, &Node) -> Direction,
        calc: impl FnMut(&Node, usize) -> Res,
    ) -> Res {
        self.do_binary_search(self.nodes.len() - 1, 0, self.n, wh, calc)
    }

    fn do_binary_search<Res>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        mut wh: impl FnMut(&Node, &Node) -> Direction,
        mut calc: impl FnMut(&Node, usize) -> Res,
    ) -> Res {
        if left + 1 == right {
            calc(&self.nodes[root], left)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let direction = wh(&self.nodes[left_child], &self.nodes[right_child]);
            match direction {
                Direction::Left => self.do_binary_search(left_child, left, mid, wh, calc),
                Direction::Right => self.do_binary_search(right_child, mid, right, wh, calc),
            }
        }
    }

    fn join(&mut self, root: usize, mid: usize, right: usize) {
        let left_child = root - 2 * (right - mid);
        let right_child = root - 1;
        let (left_node, right_node) = self.nodes.split_at_mut(root);
        right_node[0].join(&left_node[left_child], &left_node[right_child]);
    }

    fn do_push_down(&mut self, parent: usize, to: usize) {
        let (left_nodes, right_nodes) = self.nodes.split_at_mut(parent);
        left_nodes[to].accumulate(&right_nodes[0]);
    }

    fn push_down(&mut self, root: usize, mid: usize, right: usize) {
        self.do_push_down(root, root - 2 * (right - mid));
        self.do_push_down(root, root - 1);
        self.nodes[root].reset_delta();
    }

    pub fn query<T>(&mut self, range: impl RangeBounds<usize>) -> T
    where
        Node: QueryResult<T, ()>,
    {
        let (from, to) = clamp(range, self.n);
        if from >= to {
            Node::empty_result(&())
        } else {
            self.do_query(self.nodes.len() - 1, 0, self.n, from, to, &())
        }
    }

    pub fn query_with_args<T, Args>(&mut self, range: impl RangeBounds<usize>, args: &Args) -> T
    where
        Node: QueryResult<T, Args>,
    {
        let (from, to) = clamp(range, self.n);
        if from >= to {
            Node::empty_result(args)
        } else {
            self.do_query(self.nodes.len() - 1, 0, self.n, from, to, args)
        }
    }

    fn do_query<T, Args>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        args: &Args,
    ) -> T
    where
        Node: QueryResult<T, Args>,
    {
        if left >= from && right <= to {
            self.nodes[root].result(args)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            when! {
                to <= mid => self.do_query(left_child, left, mid, from, to, args),
                from >= mid => self.do_query(right_child, mid, right, from, to, args),
                else => {
                    let left_result = self.do_query(left_child, left, mid, from, to, args);
                    let right_result = self.do_query(right_child, mid, right, from, to, args);
                    Node::join_results(left_result, right_result, args, left, mid, right)
                },
            }
        }
    }
}

pub trait PointOperation<Node: SegmentTreeNode, Args, Res = Node> {
    fn adjust_leaf(&mut self, leaf: &mut Node, at: usize, args: Args) -> Res;
    fn select_branch(
        &mut self,
        root: &mut Node,
        left_child: &mut Node,
        right_child: &mut Node,
        args: &Args,
        left: usize,
        mid: usize,
        right: usize,
    ) -> Direction;
}

pub struct PointOperationClosure<'s, Node: SegmentTreeNode, Args, Res = Node> {
    adjust_leaf: Box<dyn FnMut(&mut Node, usize, Args) -> Res + 's>,
    select_branch: Box<
        dyn FnMut(&mut Node, &mut Node, &mut Node, &Args, usize, usize, usize) -> Direction + 's,
    >,
    phantom_node: PhantomData<Node>,
    phantom_args: PhantomData<Args>,
    phantom_res: PhantomData<Res>,
}

impl<'s, Node: SegmentTreeNode, Args, Res> PointOperationClosure<'s, Node, Args, Res> {
    pub fn new<F1, F2>(adjust_leaf: F1, select_branch: F2) -> Self
    where
        F1: FnMut(&mut Node, usize, Args) -> Res + 's,
        F2: FnMut(&mut Node, &mut Node, &mut Node, &Args, usize, usize, usize) -> Direction + 's,
    {
        Self {
            adjust_leaf: Box::new(adjust_leaf),
            select_branch: Box::new(select_branch),
            phantom_node: Default::default(),
            phantom_args: Default::default(),
            phantom_res: Default::default(),
        }
    }
}

impl<'s, Node: SegmentTreeNode, Args, Res> PointOperation<Node, Args, Res>
    for PointOperationClosure<'s, Node, Args, Res>
{
    fn adjust_leaf(&mut self, leaf: &mut Node, at: usize, args: Args) -> Res {
        (self.adjust_leaf)(leaf, at, args)
    }

    fn select_branch(
        &mut self,
        root: &mut Node,
        left_child: &mut Node,
        right_child: &mut Node,
        args: &Args,
        left: usize,
        mid: usize,
        right: usize,
    ) -> Direction {
        (self.select_branch)(root, left_child, right_child, args, left, mid, right)
    }
}

pub trait Operation<Node: SegmentTreeNode, Args, Res = Node> {
    fn process_result(&mut self, node: &mut Node, args: &Args) -> Res;
    fn join_results(&mut self, left_res: Res, right_res: Res, args: &Args) -> Res;
    fn empty_result(&mut self, left: usize, right: usize, args: &Args) -> Res;
}

pub struct OperationClosure<'s, Node: SegmentTreeNode, Args, Res = Node> {
    process_result: Box<dyn FnMut(&mut Node, &Args) -> Res + 's>,
    join_results: Box<dyn FnMut(Res, Res, &Args) -> Res + 's>,
    empty_result: Box<dyn FnMut(usize, usize, &Args) -> Res + 's>,
    phantom_node: PhantomData<Node>,
    phantom_args: PhantomData<Args>,
    phantom_res: PhantomData<Res>,
}

impl<'s, Node: SegmentTreeNode, Args, Res> OperationClosure<'s, Node, Args, Res> {
    pub fn new<F1, F2, F3>(process_result: F1, join_results: F2, empty_result: F3) -> Self
    where
        F1: FnMut(&mut Node, &Args) -> Res + 's,
        F2: FnMut(Res, Res, &Args) -> Res + 's,
        F3: FnMut(usize, usize, &Args) -> Res + 's,
    {
        Self {
            process_result: Box::new(process_result),
            join_results: Box::new(join_results),
            empty_result: Box::new(empty_result),
            phantom_node: Default::default(),
            phantom_args: Default::default(),
            phantom_res: Default::default(),
        }
    }
}

impl<'s, Node: SegmentTreeNode, Args, Res> Operation<Node, Args, Res>
    for OperationClosure<'s, Node, Args, Res>
{
    fn process_result(&mut self, node: &mut Node, args: &Args) -> Res {
        (self.process_result)(node, args)
    }

    fn join_results(&mut self, left_res: Res, right_res: Res, args: &Args) -> Res {
        (self.join_results)(left_res, right_res, args)
    }

    fn empty_result(&mut self, left: usize, right: usize, args: &Args) -> Res {
        (self.empty_result)(left, right, args)
    }
}
