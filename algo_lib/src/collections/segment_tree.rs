pub struct SegmentTree<V: Clone, D: Clone, JoinVal, JoinDelta, Accumulate>
where
    JoinVal: Fn(&mut V, &V, &V),
    JoinDelta: Fn(&mut D, &D),
    Accumulate: Fn(&mut V, &D, usize, usize),
{
    n: usize,
    val: Vec<V>,
    delta: Vec<D>,
    empty_val: V,
    empty_delta: D,
    join_val: JoinVal,
    join_delta: JoinDelta,
    accumulate: Accumulate,
}

impl<V: Clone, D: Clone, JoinVal, JoinDelta, Accumulate>
    SegmentTree<V, D, JoinVal, JoinDelta, Accumulate>
where
    JoinVal: Fn(&mut V, &V, &V),
    JoinDelta: Fn(&mut D, &D),
    Accumulate: Fn(&mut V, &D, usize, usize),
{
    pub fn new(
        n: usize,
        join_val: JoinVal,
        join_delta: JoinDelta,
        accumulate: Accumulate,
        empty_val: V,
        empty_delta: D,
    ) -> Self {
        let size = Self::get_size(n);
        let mut res = Self {
            n,
            val: vec![empty_val.clone(); size],
            delta: vec![empty_delta.clone(); size],
            empty_val: empty_val.clone(),
            empty_delta,
            join_val,
            join_delta,
            accumulate,
        };
        res.init(|_| empty_val.clone());
        res
    }

    fn init<F>(&mut self, f: F)
    where
        F: Fn(usize) -> V,
    {
        self.init_impl(0, 0, self.n, &f);
    }

    fn init_impl<F>(&mut self, root: usize, left: usize, right: usize, f: &F)
    where
        F: Fn(usize) -> V,
    {
        if left + 1 == right {
            self.val[root] = f(left);
        } else {
            let mid = left + ((right - left) >> 1);
            self.init_impl(2 * root + 1, left, mid, f);
            self.init_impl(2 * root + 2, mid, right, f);
            let val_root = unsafe {
                let const_ptr = &self.val[root] as *const V;
                let mut_ptr = const_ptr as *mut V;
                &mut *mut_ptr
            };
            (self.join_val)(val_root, &self.val[2 * root + 1], &self.val[2 * root + 2]);
        }
    }

    //noinspection RsSelfConvention
    fn get_size(n: usize) -> usize {
        n.next_power_of_two() << 1
    }
}
