use std::marker::PhantomData;

pub trait SegmentTreeNode {
    fn new(left: usize, right: usize) -> Self;
    fn join(&mut self, left: &Self, right: &Self);
    fn accumulate(&mut self, value: &Self);
    fn reset_delta(&mut self);
}

pub struct SegmentTree<Node: SegmentTreeNode> {
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
            panic!("Empty seg trees not allowed");
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
            self.nodes.push(f(left).into());
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
        self.do_point_query(2 * self.n - 2, 0, self.n, at)
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

    pub fn point_update(&mut self, at: usize, val: &Node) {
        self.do_point_update(2 * self.n - 2, 0, self.n, at, val);
    }

    fn do_point_update(&mut self, root: usize, left: usize, right: usize, at: usize, val: &Node) {
        if left + 1 == right {
            self.nodes[root].accumulate(val);
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

    pub fn point_operation<Args, Res>(
        &mut self,
        op: &mut PointOperation<Node, Args, Res>,
        at: usize,
        args: Args,
    ) -> Res {
        self.do_point_operation(op, 2 * self.n - 2, 0, self.n, at, args)
    }

    fn do_point_operation<Args, Res>(
        &mut self,
        op: &mut PointOperation<Node, Args, Res>,
        root: usize,
        left: usize,
        right: usize,
        at: usize,
        args: Args,
    ) -> Res {
        if left + 1 == right {
            (op.adjust_leaf)(&mut self.nodes[root], at, args)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let res = if at < mid {
                let res = self.do_point_operation(op, left_child, left, mid, at, args);
                let (l, r) = self.nodes.split_at_mut(root);
                (op.combine_val)(res, &mut l[right_child], &mut r[0], Side::Left, at)
            } else {
                let res = self.do_point_operation(op, right_child, mid, right, at, args);
                let (l, r) = self.nodes.split_at_mut(root);
                (op.combine_val)(res, &mut l[left_child], &mut r[0], Side::Right, at)
            };
            self.join(root, mid, right);
            res
        }
    }

    pub fn update(&mut self, from: usize, to: usize, val: &Node) {
        self.do_update(2 * self.n - 2, 0, self.n, from, to, &val)
    }

    pub fn do_update(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        val: &Node,
    ) {
        if left >= to || right <= from {
            // Do nothing
        } else if left >= from && right <= to {
            self.nodes[root].accumulate(val);
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            self.do_update(left_child, left, mid, from, to, val);
            self.do_update(right_child, mid, right, from, to, val);
            self.join(root, mid, right);
        }
    }

    pub fn operation<Args, Res>(
        &mut self,
        from: usize,
        to: usize,
        op: &mut Operation<Node, Args, Res>,
        args: &Args,
    ) -> Res {
        self.do_operation(2 * self.n - 2, 0, self.n, from, to, op, args)
    }

    pub fn do_operation<Args, Res>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        op: &mut Operation<Node, Args, Res>,
        args: &Args,
    ) -> Res {
        if left >= to || right <= from {
            (op.empty_result)(left, right, args)
        } else if left >= from && right <= to {
            (op.process_result)(&mut self.nodes[root], args)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let left_result = self.do_operation(left_child, left, mid, from, to, op, args);
            let right_result = self.do_operation(right_child, mid, right, from, to, op, args);
            self.join(root, mid, right);
            (op.join_results)(left_result, right_result, args)
        }
    }

    fn join(&mut self, root: usize, mid: usize, right: usize) {
        let left_child = root - 2 * (right - mid);
        let right_child = root - 1;
        let (left, right) = self.nodes.split_at_mut(root);
        right[0].join(&left[left_child], &left[right_child]);
    }

    fn do_push_down(&mut self, parent: usize, to: usize) {
        let (left, right) = self.nodes.split_at_mut(parent);
        left[to].accumulate(&right[0]);
    }

    fn push_down(&mut self, root: usize, mid: usize, right: usize) {
        self.do_push_down(root, root - 2 * (right - mid));
        self.do_push_down(root, root - 1);
        self.nodes[root].reset_delta();
    }
}

impl<Node: SegmentTreeNode + Copy> SegmentTree<Node> {
    pub fn query(&mut self, from: usize, to: usize) -> Node {
        self.do_query(2 * self.n - 2, 0, self.n, from, to)
    }

    fn do_query(&mut self, root: usize, left: usize, right: usize, from: usize, to: usize) -> Node {
        if left >= to || right <= from {
            Node::new(left, right)
        } else if left >= from && right <= to {
            self.nodes[root]
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let left_res = self.do_query(left_child, left, mid, from, to);
            let right_res = self.do_query(right_child, mid, right, from, to);
            let mut res = Node::new(left, right);
            res.join(&left_res, &right_res);
            res
        }
    }
}

pub enum Side {
    Left,
    Right,
}

pub struct PointOperation<'s, Node: SegmentTreeNode, Args, Res = Node> {
    adjust_leaf: Box<dyn FnMut(&mut Node, usize, Args) -> Res + 's>,
    combine_val: Box<dyn FnMut(Res, &mut Node, &mut Node, Side, usize) -> Res + 's>,
    phantom_node: PhantomData<Node>,
    phantom_args: PhantomData<Args>,
    phantom_res: PhantomData<Res>,
}

impl<'s, Node: SegmentTreeNode, Args, Res> PointOperation<'s, Node, Args, Res> {
    pub fn new<F1, F2>(adjust_leaf: F1, combine_val: F2) -> Self
    where
        F1: FnMut(&mut Node, usize, Args) -> Res + 's,
        F2: FnMut(Res, &mut Node, &mut Node, Side, usize) -> Res + 's,
    {
        Self {
            adjust_leaf: Box::new(adjust_leaf),
            combine_val: Box::new(combine_val),
            phantom_node: Default::default(),
            phantom_args: Default::default(),
            phantom_res: Default::default(),
        }
    }
}

pub struct Operation<'s, Node: SegmentTreeNode, Args, Res = Node> {
    process_result: Box<dyn FnMut(&mut Node, &Args) -> Res + 's>,
    join_results: Box<dyn FnMut(Res, Res, &Args) -> Res + 's>,
    empty_result: Box<dyn FnMut(usize, usize, &Args) -> Res + 's>,
    phantom_node: PhantomData<Node>,
    phantom_args: PhantomData<Args>,
    phantom_res: PhantomData<Res>,
}

impl<'s, Node: SegmentTreeNode, Args, Res> Operation<'s, Node, Args, Res> {
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
