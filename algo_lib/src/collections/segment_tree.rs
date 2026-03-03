use crate::collections::bounds::clamp;
use crate::misc::direction::Direction;
use crate::misc::value_delta::ValueDeltaTrait;
use crate::when;
use std::marker::PhantomData;
use std::ops::RangeBounds;

#[allow(unused_variables)]
pub trait SegmentTreeNode: Default {
    fn join(left: &Self, right: &Self) -> Self {
        let mut node = Self::default();
        node.update(left, right);
        node
    }
    fn update(&mut self, left_val: &Self, right_val: &Self) {}
    fn accumulate(&mut self, value: &Self) {}
    fn reset_delta(&mut self) {}
}

pub struct ValueDeltaNode<VDT: ValueDeltaTrait> {
    pub v: VDT::V,
    pub d: VDT::D,
    phantom: PhantomData<VDT>,
}

impl<VDT: ValueDeltaTrait> ValueDeltaNode<VDT>
where
    VDT::D: Default,
{
    pub fn new(v: VDT::V) -> Self {
        Self {
            v,
            d: VDT::D::default(),
            phantom: PhantomData,
        }
    }
}

impl<VDT: ValueDeltaTrait> Clone for ValueDeltaNode<VDT>
where
    VDT::V: Clone,
    VDT::D: Clone,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            d: self.d.clone(),
            phantom: PhantomData,
        }
    }
}

impl<VDT: ValueDeltaTrait> Copy for ValueDeltaNode<VDT>
where
    VDT::V: Copy,
    VDT::D: Copy,
{
}

impl<VDT: ValueDeltaTrait> Default for ValueDeltaNode<VDT>
where
    VDT::V: Default,
    VDT::D: Default,
{
    fn default() -> Self {
        Self::new(VDT::V::default())
    }
}

impl<VDT: ValueDeltaTrait> SegmentTreeNode for ValueDeltaNode<VDT>
where
    VDT::V: Copy + Default,
    VDT::D: Copy + Default,
{
    fn join(left: &Self, right: &Self) -> Self {
        Self::new(VDT::join(left.v, right.v))
    }

    fn update(&mut self, left_val: &Self, right_val: &Self) {
        self.v = VDT::join(left_val.v, right_val.v);
    }

    fn accumulate(&mut self, value: &Self) {
        self.d = VDT::accumulate(self.d, value.d);
        self.v = VDT::apply(self.v, value.d);
    }

    fn reset_delta(&mut self) {
        self.d = VDT::D::default();
    }
}

#[derive(Clone)]
pub struct SegmentTree<Node> {
    n: usize,
    nodes: Vec<Node>,
}

impl<Node: SegmentTreeNode> SegmentTree<Node> {
    pub fn new(n: usize) -> Self {
        Self::with_gen(n, |_| Node::default())
    }

    pub fn from_array(arr: Vec<Node>) -> Self {
        let n = arr.len();
        let mut iter = arr.into_iter();
        Self::with_gen(n, |_| iter.next().unwrap())
    }

    pub fn with_gen(n: usize, mut g: impl FnMut(usize) -> Node) -> Self {
        if n == 0 {
            return Self {
                n,
                nodes: vec![Node::default()],
            };
        }
        let mut res = Self {
            n,
            nodes: Vec::with_capacity(2 * n - 1),
        };
        res.init_impl(
            2 * n - 2,
            0,
            n,
            &mut |left, right, left_node, right_node| {
                if left + 1 == right {
                    g(left)
                } else {
                    Node::join(left_node.unwrap(), right_node.unwrap())
                }
            },
        );
        res
    }

    pub fn with_gen_full(n: usize, f: impl Fn(usize, usize) -> Node) -> Self {
        if n == 0 {
            return Self {
                n,
                nodes: vec![Node::default()],
            };
        }
        let mut res = Self {
            n,
            nodes: Vec::with_capacity(2 * n - 1),
        };
        res.init_impl(
            2 * n - 2,
            0,
            n,
            &mut |left, right, left_node, right_node| {
                let mut res = f(left, right);
                if let Some(left_node) = left_node {
                    res.update(left_node, right_node.unwrap());
                }
                res
            },
        );
        res
    }

    fn init_impl(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        f: &mut impl FnMut(usize, usize, Option<&Node>, Option<&Node>) -> Node,
    ) {
        if left + 1 == right {
            self.nodes.push(f(left, left + 1, None, None));
        } else {
            let mid = left + ((right - left) >> 1);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            self.init_impl(left_child, left, mid, f);
            self.init_impl(right_child, mid, right, f);
            self.nodes.push(f(
                left,
                right,
                Some(&self.nodes[left_child]),
                Some(&self.nodes[right_child]),
            ));
        }
    }

    pub fn point_query(&mut self, at: usize) -> &Node {
        assert!(at < self.n);
        self.binary_search_with_mid(
            |_, _, mid| {
                if at < mid {
                    Direction::Left
                } else {
                    Direction::Right
                }
            },
            |node, _| node,
        )
    }

    pub fn point_update(&mut self, at: usize, val: Node) {
        assert!(at < self.n);
        self.binary_search_mut_with_mid(
            |_, _, _, mid| {
                if at < mid {
                    Direction::Left
                } else {
                    Direction::Right
                }
            },
            |node, _| *node = val,
        );
    }

    pub fn point_through_update(&mut self, at: usize, f: impl FnMut(&mut Node)) {
        assert!(at < self.n);
        self.point_through_update_impl(at, 0, self.n, self.nodes.len() - 1, f);
    }

    fn point_through_update_impl(
        &mut self,
        at: usize,
        left: usize,
        right: usize,
        root: usize,
        mut f: impl FnMut(&mut Node),
    ) {
        if left + 1 == right {
            f(&mut self.nodes[root]);
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            f(&mut self.nodes[root]);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            if at < mid {
                self.point_through_update_impl(at, left, mid, left_child, f);
            } else {
                self.point_through_update_impl(at, mid, right, right_child, f);
            }
            self.join(root, mid, right);
        }
    }

    pub fn update(&mut self, range: impl RangeBounds<usize>, val: &Node) {
        self.for_each_mut(range, |_, node| node.accumulate(val));
    }

    pub fn binary_search<'a, R>(
        &'a mut self,
        mut wh: impl FnMut(&Node, &Node) -> Direction,
        calc: impl FnOnce(&'a Node, usize) -> R,
    ) -> R {
        self.binary_search_with_mid(|left, right, _| wh(left, right), calc)
    }

    pub fn binary_search_with_mid<'a, R>(
        &'a mut self,
        wh: impl FnMut(&Node, &Node, usize) -> Direction,
        calc: impl FnOnce(&'a Node, usize) -> R,
    ) -> R {
        self.do_binary_search(self.nodes.len() - 1, 0, self.n, wh, calc)
    }

    fn do_binary_search<'a, R>(
        &'a mut self,
        root: usize,
        left: usize,
        right: usize,
        mut wh: impl FnMut(&Node, &Node, usize) -> Direction,
        calc: impl FnOnce(&'a Node, usize) -> R,
    ) -> R {
        if left + 1 == right {
            calc(&self.nodes[root], left)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let direction = wh(&self.nodes[left_child], &self.nodes[right_child], mid);
            match direction {
                Direction::Left => self.do_binary_search(left_child, left, mid, wh, calc),
                Direction::Right => self.do_binary_search(right_child, mid, right, wh, calc),
            }
        }
    }

    pub fn binary_search_mut<R>(
        &mut self,
        mut wh: impl FnMut(&mut Node, &Node, &Node) -> Direction,
        calc: impl FnOnce(&mut Node, usize) -> R,
    ) -> R {
        self.binary_search_mut_with_mid(|node, left, right, _| wh(node, left, right), calc)
    }

    pub fn binary_search_mut_with_mid<R>(
        &mut self,
        wh: impl FnMut(&mut Node, &Node, &Node, usize) -> Direction,
        calc: impl FnOnce(&mut Node, usize) -> R,
    ) -> R {
        self.do_binary_search_mut(self.nodes.len() - 1, 0, self.n, wh, calc)
    }

    fn do_binary_search_mut<Res>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        mut wh: impl FnMut(&mut Node, &Node, &Node, usize) -> Direction,
        calc: impl FnOnce(&mut Node, usize) -> Res,
    ) -> Res {
        if left + 1 == right {
            calc(&mut self.nodes[root], left)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let (head, tail) = self.nodes.split_at_mut(root);
            let direction = wh(&mut tail[0], &head[left_child], &head[right_child], mid);
            let res = match direction {
                Direction::Left => self.do_binary_search_mut(left_child, left, mid, wh, calc),
                Direction::Right => self.do_binary_search_mut(right_child, mid, right, wh, calc),
            };
            self.join(root, mid, right);
            res
        }
    }

    pub fn binary_search_in<R>(
        &mut self,
        range: impl RangeBounds<usize>,
        mut accept: impl FnMut(&Node) -> bool,
        calc: impl FnOnce(&Node, usize) -> R,
    ) -> Option<R> {
        let (from, to) = clamp(&range, self.n);
        if from < to {
            self.do_binary_search_in(
                self.nodes.len() - 1,
                0,
                self.n,
                from,
                to,
                &mut accept,
                calc,
                SearchDirection::LTR,
            )
            .ok()
        } else {
            None
        }
    }

    pub fn binary_search_in_rtl<R>(
        &mut self,
        range: impl RangeBounds<usize>,
        mut accept: impl FnMut(&Node) -> bool,
        calc: impl FnOnce(&Node, usize) -> R,
    ) -> Option<R> {
        let (from, to) = clamp(&range, self.n);
        if from < to {
            self.do_binary_search_in(
                self.nodes.len() - 1,
                0,
                self.n,
                from,
                to,
                &mut accept,
                calc,
                SearchDirection::RTL,
            )
            .ok()
        } else {
            None
        }
    }

    fn do_binary_search_in<R, C: FnOnce(&Node, usize) -> R>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        accept: &mut impl FnMut(&Node) -> bool,
        calc: C,
        direction: SearchDirection,
    ) -> Result<R, C> {
        if left >= from && right <= to {
            if accept(&self.nodes[root]) {
                Ok(self.do_binary_search(
                    root,
                    left,
                    right,
                    |left, right, _| match direction {
                        SearchDirection::LTR => {
                            if accept(left) {
                                Direction::Left
                            } else {
                                Direction::Right
                            }
                        }
                        SearchDirection::RTL => {
                            if accept(right) {
                                Direction::Right
                            } else {
                                Direction::Left
                            }
                        }
                    },
                    calc,
                ))
            } else {
                Err(calc)
            }
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            when! {
                to <= mid => self.do_binary_search_in(left_child, left, mid, from, to, accept, calc, direction),
                from >= mid => self.do_binary_search_in(right_child, mid, right, from, to, accept, calc, direction),
                else => {
                    let res = match direction {
                        SearchDirection::LTR => self.do_binary_search_in(left_child, left, mid, from, to, accept, calc, direction),
                        SearchDirection::RTL => self.do_binary_search_in(right_child, mid, right, from, to, accept, calc, direction),
                    };
                    match res {
                        res @ Ok(_) => res,
                        Err(calc) => {
                            match direction {
                                SearchDirection::LTR => self.do_binary_search_in(right_child, mid, right, from, to, accept, calc, direction),
                                SearchDirection::RTL => self.do_binary_search_in(left_child, left, mid, from, to, accept, calc, direction),
                            }
                        }
                    }
                },
            }
        }
    }

    pub fn binary_search_in_mut<R>(
        &mut self,
        range: impl RangeBounds<usize>,
        mut accept: impl FnMut(&Node) -> bool,
        calc: impl FnOnce(&mut Node, usize) -> R,
    ) -> Option<R> {
        let (from, to) = clamp(&range, self.n);
        if from < to {
            self.do_binary_search_in_mut(
                self.nodes.len() - 1,
                0,
                self.n,
                from,
                to,
                &mut accept,
                calc,
                SearchDirection::LTR,
            )
            .ok()
        } else {
            None
        }
    }

    pub fn binary_search_in_mut_rtl<R>(
        &mut self,
        range: impl RangeBounds<usize>,
        mut accept: impl FnMut(&Node) -> bool,
        calc: impl FnOnce(&mut Node, usize) -> R,
    ) -> Option<R> {
        let (from, to) = clamp(&range, self.n);
        if from < to {
            self.do_binary_search_in_mut(
                self.nodes.len() - 1,
                0,
                self.n,
                from,
                to,
                &mut accept,
                calc,
                SearchDirection::RTL,
            )
            .ok()
        } else {
            None
        }
    }

    fn do_binary_search_in_mut<R, C: FnOnce(&mut Node, usize) -> R>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        accept: &mut impl FnMut(&Node) -> bool,
        calc: C,
        direction: SearchDirection,
    ) -> Result<R, C> {
        if left >= from && right <= to {
            if accept(&self.nodes[root]) {
                Ok(self.do_binary_search_mut(
                    root,
                    left,
                    right,
                    |_, left, right, _| match direction {
                        SearchDirection::LTR => {
                            if accept(left) {
                                Direction::Left
                            } else {
                                Direction::Right
                            }
                        }
                        SearchDirection::RTL => {
                            if accept(right) {
                                Direction::Right
                            } else {
                                Direction::Left
                            }
                        }
                    },
                    calc,
                ))
            } else {
                Err(calc)
            }
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let res = when! {
                to <= mid => self.do_binary_search_in_mut(left_child, left, mid, from, to, accept, calc, direction),
                from >= mid => self.do_binary_search_in_mut(right_child, mid, right, from, to, accept, calc, direction),
                else => {
                    let res = match direction {
                        SearchDirection::LTR => self.do_binary_search_in_mut(left_child, left, mid, from, to, accept, calc, direction),
                        SearchDirection::RTL => self.do_binary_search_in_mut(right_child, mid, right, from, to, accept, calc, direction),
                    };
                    match res {
                        res @ Ok(_) => res,
                        Err(calc) => match direction {
                            SearchDirection::LTR => self.do_binary_search_in_mut(right_child, mid, right, from, to, accept, calc, direction),
                            SearchDirection::RTL => self.do_binary_search_in_mut(left_child, left, mid, from, to, accept, calc, direction),
                        }
                    }
                },
            };
            self.join(root, mid, right);
            res
        }
    }

    pub fn query(&mut self, range: impl RangeBounds<usize>) -> Node
    where
        Node: Clone,
    {
        self.for_each::<Option<Node>>(range, |res, node| {
            if let Some(res) = res {
                Some(Node::join(&res, node))
            } else {
                Some(node.clone())
            }
        })
        .unwrap_or_default()
    }

    pub fn for_each_mut<R: Default>(
        &mut self,
        range: impl RangeBounds<usize>,
        f: impl FnMut(R, &mut Node) -> R,
    ) -> R {
        self.for_each_mut_with_init(range, f, R::default())
    }

    pub fn for_each_mut_with_init<R>(
        &mut self,
        range: impl RangeBounds<usize>,
        mut f: impl FnMut(R, &mut Node) -> R,
        init: R,
    ) -> R {
        let (from, to) = clamp(&range, self.n);
        if from < to {
            self.do_for_each_mut(self.nodes.len() - 1, 0, self.n, from, to, &mut f, init)
        } else {
            init
        }
    }

    fn do_for_each_mut<R>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        f: &mut impl FnMut(R, &mut Node) -> R,
        res: R,
    ) -> R {
        if left >= from && right <= to {
            f(res, &mut self.nodes[root])
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let res = when! {
                to <= mid => self.do_for_each_mut(left_child, left, mid, from, to, f, res),
                from >= mid => self.do_for_each_mut(right_child, mid, right, from, to, f, res),
                else => {
                    let res = self.do_for_each_mut(left_child, left, mid, from, to, f, res);
                    self.do_for_each_mut(right_child, mid, right, from, to, f, res)
                },
            };
            self.join(root, mid, right);
            res
        }
    }

    pub fn for_each<R: Default>(
        &mut self,
        range: impl RangeBounds<usize>,
        f: impl FnMut(R, &Node) -> R,
    ) -> R {
        self.for_each_with_init(range, f, R::default())
    }

    pub fn for_each_with_init<R>(
        &mut self,
        range: impl RangeBounds<usize>,
        mut f: impl FnMut(R, &Node) -> R,
        init: R,
    ) -> R {
        let (from, to) = clamp(&range, self.n);
        if from < to {
            self.do_for_each(self.nodes.len() - 1, 0, self.n, from, to, &mut f, init)
        } else {
            init
        }
    }

    fn do_for_each<R>(
        &mut self,
        root: usize,
        left: usize,
        right: usize,
        from: usize,
        to: usize,
        f: &mut impl FnMut(R, &Node) -> R,
        res: R,
    ) -> R {
        if left >= from && right <= to {
            f(res, &self.nodes[root])
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            when! {
                to <= mid => self.do_for_each(left_child, left, mid, from, to, f, res),
                from >= mid => self.do_for_each(right_child, mid, right, from, to, f, res),
                else => {
                    let res = self.do_for_each(left_child, left, mid, from, to, f, res);
                    self.do_for_each(right_child, mid, right, from, to, f, res)
                },
            }
        }
    }

    fn join(&mut self, root: usize, mid: usize, right: usize) {
        let left_child = root - 2 * (right - mid);
        let right_child = root - 1;
        let (left_node, right_node) = self.nodes.split_at_mut(root);
        right_node[0].update(&left_node[left_child], &left_node[right_child]);
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
}

#[derive(Clone, Copy)]
pub enum SearchDirection {
    LTR,
    RTL,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::misc::value_delta::ValueDeltaTrait;

    // Range-add, range-sum segment tree
    struct SumAdd;
    impl ValueDeltaTrait for SumAdd {
        type V = (i64, i64); // (sum, count)
        type D = i64; // additive delta
        fn join(v1: Self::V, v2: Self::V) -> Self::V {
            (v1.0 + v2.0, v1.1 + v2.1)
        }
        fn accumulate(d1: Self::D, d2: Self::D) -> Self::D {
            d1 + d2
        }
        fn apply(v: Self::V, d: Self::D) -> Self::V {
            (v.0 + d * v.1, v.1)
        }
    }

    type Node = ValueDeltaNode<SumAdd>;

    fn make_tree(vals: &[i64]) -> SegmentTree<Node> {
        SegmentTree::with_gen(vals.len(), |i| Node::new((vals[i], 1)))
    }

    fn query_sum(tree: &mut SegmentTree<Node>, l: usize, r: usize) -> i64 {
        tree.query(l..r).v.0
    }

    fn range_add(tree: &mut SegmentTree<Node>, l: usize, r: usize, val: i64) {
        let delta = Node {
            v: (0, 0),
            d: val,
            phantom: std::marker::PhantomData,
        };
        tree.update(l..r, &delta);
    }

    #[test]
    fn lazy_range_add_range_sum() {
        let mut tree = make_tree(&[1, 2, 3, 4, 5]);
        assert_eq!(query_sum(&mut tree, 0, 5), 15);
        assert_eq!(query_sum(&mut tree, 1, 4), 9);

        range_add(&mut tree, 1, 4, 10); // [1, 12, 13, 14, 5]
        assert_eq!(query_sum(&mut tree, 0, 5), 45);
        assert_eq!(query_sum(&mut tree, 1, 4), 39);
        assert_eq!(query_sum(&mut tree, 0, 1), 1);
        assert_eq!(query_sum(&mut tree, 4, 5), 5);
    }

    #[test]
    fn lazy_multiple_overlapping_updates() {
        let mut tree = make_tree(&[0; 8]);
        range_add(&mut tree, 0, 4, 1); // [1,1,1,1,0,0,0,0]
        range_add(&mut tree, 2, 6, 2); // [1,1,3,3,2,2,0,0]
        range_add(&mut tree, 4, 8, 3); // [1,1,3,3,5,5,3,3]

        assert_eq!(query_sum(&mut tree, 0, 8), 24);
        assert_eq!(query_sum(&mut tree, 0, 2), 2);
        assert_eq!(query_sum(&mut tree, 2, 4), 6);
        assert_eq!(query_sum(&mut tree, 4, 6), 10);
        assert_eq!(query_sum(&mut tree, 6, 8), 6);
    }

    #[test]
    fn lazy_single_element_updates() {
        let mut tree = make_tree(&[0; 4]);
        for i in 0..4 {
            range_add(&mut tree, i, i + 1, (i + 1) as i64);
        }
        // [1, 2, 3, 4]
        assert_eq!(query_sum(&mut tree, 0, 4), 10);
        for i in 0..4 {
            assert_eq!(query_sum(&mut tree, i, i + 1), (i + 1) as i64);
        }
    }

    #[test]
    fn lazy_stress_vs_naive() {
        let n = 16;
        let mut arr = vec![0i64; n];
        let mut tree = make_tree(&arr);

        let ops: Vec<(usize, usize, i64)> = vec![
            (0, 8, 5),
            (4, 12, 3),
            (8, 16, 7),
            (0, 16, 1),
            (2, 5, -2),
            (10, 14, 4),
            (0, 1, 100),
            (15, 16, 50),
        ];

        for (l, r, v) in &ops {
            range_add(&mut tree, *l, *r, *v);
            for i in *l..*r {
                arr[i] += v;
            }
        }

        // Verify all possible subranges
        for l in 0..n {
            for r in l + 1..=n {
                let expected: i64 = arr[l..r].iter().sum();
                let actual = query_sum(&mut tree, l, r);
                assert_eq!(actual, expected, "mismatch for range [{}..{})", l, r);
            }
        }
    }

    // Range-assign, range-sum (non-commutative: assign overrides add)
    struct SumAssign;
    impl ValueDeltaTrait for SumAssign {
        type V = (i64, i64); // (sum, count)
        type D = Option<i64>; // assign delta (None = no-op)
        fn join(v1: Self::V, v2: Self::V) -> Self::V {
            (v1.0 + v2.0, v1.1 + v2.1)
        }
        fn accumulate(d1: Self::D, d2: Self::D) -> Self::D {
            // d2 (parent) overrides d1 (child) if present
            d2.or(d1)
        }
        fn apply(v: Self::V, d: Self::D) -> Self::V {
            match d {
                Some(val) => (val * v.1, v.1),
                None => v,
            }
        }
    }

    type AssignNode = ValueDeltaNode<SumAssign>;

    #[test]
    fn lazy_assign_then_query() {
        let mut tree: SegmentTree<AssignNode> =
            SegmentTree::with_gen(8, |_| AssignNode::new((0, 1)));

        // Assign 5 to [0..4]
        let delta = AssignNode {
            v: (0, 0),
            d: Some(5),
            phantom: std::marker::PhantomData,
        };
        tree.update(0..4, &delta);

        // Assign 3 to [2..6]
        let delta = AssignNode {
            v: (0, 0),
            d: Some(3),
            phantom: std::marker::PhantomData,
        };
        tree.update(2..6, &delta);

        // Expected: [5,5,3,3,3,3,0,0]
        assert_eq!(tree.query(0..8).v.0, 22);
        assert_eq!(tree.query(0..2).v.0, 10);
        assert_eq!(tree.query(2..6).v.0, 12);
        assert_eq!(tree.query(6..8).v.0, 0);
    }
}
