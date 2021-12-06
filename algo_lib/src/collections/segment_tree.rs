use std::marker::PhantomData;

pub trait SegmentTreeLogic<V, D: Copy + Default + PartialEq> {
    fn join_val(left: &V, right: &V) -> V;
    fn join_delta(was: D, add: D) -> D;
    fn accumulate(value: &mut V, delta: D, left: usize, right: usize);

    fn join_val_reuse(value: &mut V, left: &V, right: &V) {
        *value = Self::join_val(left, right);
    }
}

pub struct SegmentTree<V, D: Copy + Default + PartialEq, Logic: SegmentTreeLogic<V, D>> {
    n: usize,
    val: Vec<V>,
    delta: Vec<D>,
    phantom: PhantomData<Logic>,
}

impl<V: Clone, D: Copy + Default + PartialEq, Logic: SegmentTreeLogic<V, D>>
    SegmentTree<V, D, Logic>
{
    pub fn new(n: usize, default_val: V) -> Self {
        Self::from_generator(n, |_| default_val.clone())
    }
}

impl<V: Copy, D: Copy + Default + PartialEq, Logic: SegmentTreeLogic<V, D>>
    SegmentTree<V, D, Logic>
{
    pub fn point_query(&mut self, at: usize) -> V {
        self.do_point_query(2 * self.n - 2, 0, self.n, at)
    }

    fn do_point_query(&mut self, root: usize, left: usize, right: usize, at: usize) -> V {
        if left + 1 == right {
            self.val[root]
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, left, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let res = if at < mid {
                self.do_point_query(left_child, left, mid, at)
            } else {
                self.do_point_query(right_child, mid, right, at)
            };
            self.do_join_val(root, mid, right);
            res
        }
    }
}

impl<V, D: Copy + Default + PartialEq, Logic: SegmentTreeLogic<V, D>> SegmentTree<V, D, Logic> {
    pub fn from_array(arr: Vec<V>) -> Self {
        let n = arr.len();
        let mut iter = arr.into_iter();
        Self::from_generator(n, |_| iter.next().unwrap())
    }

    pub fn from_generator<F>(n: usize, gen: F) -> Self
    where
        F: FnMut(usize) -> V,
    {
        if n == 0 {
            panic!("Empty seg trees not allowed");
        }
        let mut res = Self {
            n,
            val: Vec::with_capacity(2 * n - 1),
            delta: vec![Default::default(); 2 * n - 1],
            phantom: Default::default(),
        };
        res.init(gen);
        res
    }

    pub fn point_update(&mut self, at: usize, val: V) {
        self.do_point_update(2 * self.n - 2, 0, self.n, at, val);
    }

    fn do_point_update(&mut self, root: usize, left: usize, right: usize, at: usize, val: V) {
        if left + 1 == right {
            self.val[root] = val;
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, left, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            if at < mid {
                self.do_point_update(left_child, left, mid, at, val);
            } else {
                self.do_point_update(right_child, mid, right, at, val);
            }
            self.do_join_val(root, mid, right);
        }
    }

    pub fn point_operation<Args, Res>(
        &mut self,
        op: &mut PointOperation<V, Args, Res>,
        at: usize,
        args: Args,
    ) -> Res {
        self.do_point_operation(op, 2 * self.n - 2, 0, self.n, at, &args)
    }

    fn do_point_operation<Args, Res>(
        &mut self,
        op: &mut PointOperation<V, Args, Res>,
        root: usize,
        left: usize,
        right: usize,
        at: usize,
        args: &Args,
    ) -> Res {
        if left + 1 == right {
            (op.adjust_leaf)(&mut self.val[root], at, args)
        } else {
            let mid = (left + right) >> 1;
            self.push_down(root, left, mid, right);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            let res = if at < mid {
                let res = self.do_point_operation(op, left_child, left, mid, at, args);
                let (l, r) = self.val.split_at_mut(root);
                (op.combine_val)(
                    res,
                    &mut l[right_child],
                    &mut r[0],
                    Side::Left,
                    at,
                    left,
                    mid,
                    right,
                    args,
                )
            } else {
                let res = self.do_point_operation(op, right_child, mid, right, at, args);
                let (l, r) = self.val.split_at_mut(root);
                (op.combine_val)(
                    res,
                    &mut l[left_child],
                    &mut r[0],
                    Side::Right,
                    at,
                    left,
                    mid,
                    right,
                    args,
                )
            };
            self.do_join_val(root, mid, right);
            res
        }
    }

    fn init<F>(&mut self, mut f: F)
    where
        F: FnMut(usize) -> V,
    {
        self.init_impl(2 * self.n - 2, 0, self.n, &mut f);
    }

    fn init_impl<F>(&mut self, root: usize, left: usize, right: usize, f: &mut F)
    where
        F: FnMut(usize) -> V,
    {
        if left + 1 == right {
            self.val.push(f(left).into());
        } else {
            let mid = left + ((right - left) >> 1);
            let left_child = root - 2 * (right - mid);
            let right_child = root - 1;
            self.init_impl(left_child, left, mid, f);
            self.init_impl(right_child, mid, right, f);
            self.val.push(Logic::join_val(
                &self.val[left_child],
                &self.val[right_child],
            ));
        }
    }

    fn do_join_val(&mut self, root: usize, mid: usize, right: usize) {
        let left_child = root - 2 * (right - mid);
        let right_child = root - 1;
        let (left, right) = self.val.split_at_mut(root);
        Logic::join_val_reuse(&mut right[0], &left[left_child], &left[right_child]);
    }

    fn do_push_down(&mut self, delta: D, to: usize, left: usize, right: usize) {
        self.delta[to] = Logic::join_delta(self.delta[to], delta);
        Logic::accumulate(&mut self.val[to], delta, left, right);
    }

    fn push_down(&mut self, root: usize, left: usize, mid: usize, right: usize) {
        if self.delta[root] == Default::default() {
            return;
        }
        self.do_push_down(self.delta[root], root - 2 * (right - mid), left, mid);
        self.do_push_down(self.delta[root], root - 1, mid, right);
        self.delta[root] = Default::default();
    }
}

pub enum Side {
    Left,
    Right,
}

pub struct PointOperation<'s, V, Args, Res = V> {
    adjust_leaf: Box<dyn FnMut(&mut V, usize, &Args) -> Res + 's>,
    combine_val:
        Box<dyn FnMut(Res, &mut V, &mut V, Side, usize, usize, usize, usize, &Args) -> Res + 's>,
    phantom_v: PhantomData<V>,
    phantom_args: PhantomData<Args>,
    phantom_res: PhantomData<Res>,
}

impl<'s, V, Args, Res> PointOperation<'s, V, Args, Res> {
    pub fn new<F1, F2>(adjust_leaf: F1, combine_val: F2) -> Self
    where
        F1: FnMut(&mut V, usize, &Args) -> Res + 's,
        F2: FnMut(Res, &mut V, &mut V, Side, usize, usize, usize, usize, &Args) -> Res + 's,
    {
        Self {
            adjust_leaf: Box::new(adjust_leaf),
            combine_val: Box::new(combine_val),
            phantom_v: Default::default(),
            phantom_args: Default::default(),
            phantom_res: Default::default(),
        }
    }
}
