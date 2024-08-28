//{"name":"coderun_433","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_433"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::value_ref;
use std::cell::Cell;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let storages = input.read_long_pair_vec(n);

    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| storages[i]);
    value_ref!(StoragesContainer STORAGES: Vec<(i64, i64)> = storages);

    #[derive(Default)]
    struct Node {
        to_right: Vec<(i64, usize)>,
        to_left: Vec<(i64, usize)>,
        left_index: Cell<usize>,
        right_index: Cell<usize>,
    }

    fn value(id: usize, at: i64) -> i64 {
        let (a, b) = StoragesContainer::val()[id];
        b + (a - at) * (a - at)
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            let mut left_at = 0;
            let mut right_at = 0;
            let mut pos = left_val.to_right[0].0;
            while pos < i32::MAX as i64 {
                let left_to = if left_at + 1 < left_val.to_right.len() {
                    left_val.to_right[left_at + 1].0
                } else {
                    i32::MAX as i64
                };
                let left_id = left_val.to_right[left_at].1;
                let right_to = if right_at + 1 < right_val.to_right.len() {
                    right_val.to_right[right_at + 1].0
                } else {
                    i32::MAX as i64
                };
                if pos < right_val.to_right[right_at].0 {
                    self.to_right.push((pos, left_id));
                    pos = left_to.min(right_val.to_right[right_at].0);
                    if left_to == pos {
                        left_at += 1;
                    }
                    continue;
                }
                let right_id = right_val.to_right[right_at].1;
                let l_value = value(left_id, pos);
                let r_value = value(right_id, pos);
                let next = left_to.min(right_to) - 1;
                if r_value <= l_value {
                    if self.to_right.is_empty() || self.to_right.backward()[0].1 != right_id {
                        self.to_right.push((pos, right_id));
                    }
                    pos = next + 1;
                    if left_to == pos {
                        left_at += 1;
                    }
                    if right_to == pos {
                        right_at += 1;
                    }
                    continue;
                }
                if value(left_id, next) < value(right_id, next) {
                    if self.to_right.is_empty() || self.to_right.backward()[0].1 != left_id {
                        self.to_right.push((pos, left_id));
                    }
                    pos = next + 1;
                    if left_to == pos {
                        left_at += 1;
                    }
                    if right_to == pos {
                        right_at += 1;
                    }
                    continue;
                }
                let mut left = pos;
                let mut right = next;
                while left < right {
                    let mid = (left + right) / 2;
                    if value(left_id, mid) >= value(right_id, mid) {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }
                if self.to_right.is_empty() || self.to_right.backward()[0].1 != left_id {
                    self.to_right.push((pos, left_id));
                }
                self.to_right.push((left, right_id));
                pos = next + 1;
                if left_to == pos {
                    left_at += 1;
                }
                if right_to == pos {
                    right_at += 1;
                }
            }
            let mut right_at = 0;
            let mut left_at = 0;
            let mut pos = right_val.to_left[0].0;
            while pos > (-i32::MAX / 2) as i64 {
                let right_to = if right_at + 1 < right_val.to_left.len() {
                    right_val.to_left[right_at + 1].0
                } else {
                    (-i32::MAX / 2) as i64
                };
                let right_id = right_val.to_left[right_at].1;
                let left_to = if left_at + 1 < left_val.to_left.len() {
                    left_val.to_left[left_at + 1].0
                } else {
                    (-i32::MAX / 2) as i64
                };
                if pos > left_val.to_left[left_at].0 {
                    self.to_left.push((pos, right_id));
                    pos = right_to.max(left_val.to_left[left_at].0);
                    if right_to == pos {
                        right_at += 1;
                    }
                    continue;
                }
                let left_id = left_val.to_left[left_at].1;
                let l_value = value(left_id, pos);
                let r_value = value(right_id, pos);
                let next = right_to.max(left_to) + 1;
                if l_value <= r_value {
                    if self.to_left.is_empty() || self.to_left.backward()[0].1 != left_id {
                        self.to_left.push((pos, left_id));
                    }
                    pos = next - 1;
                    if right_to == pos {
                        right_at += 1;
                    }
                    if left_to == pos {
                        left_at += 1;
                    }
                    continue;
                }
                if value(right_id, next) < value(left_id, next) {
                    if self.to_left.is_empty() || self.to_left.backward()[0].1 != right_id {
                        self.to_left.push((pos, right_id));
                    }
                    pos = next - 1;
                    if right_to == pos {
                        right_at += 1;
                    }
                    if left_to == pos {
                        left_at += 1;
                    }
                    continue;
                }
                let mut left = next;
                let mut right = pos;
                while left < right {
                    let mid = (left + right + 1) / 2;
                    if value(right_id, mid) >= value(left_id, mid) {
                        left = mid;
                    } else {
                        right = mid - 1;
                    }
                }
                if self.to_left.is_empty() || self.to_left.backward()[0].1 != right_id {
                    self.to_left.push((pos, right_id));
                }
                self.to_left.push((left, left_id));
                pos = next - 1;
                if right_to == pos {
                    right_at += 1;
                }
                if left_to == pos {
                    left_at += 1;
                }
            }
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl QueryResult<i64, (i64, bool)> for Node {
        fn empty_result(_args: &(i64, bool)) -> i64 {
            i64::MAX
        }

        fn result(&self, args: &(i64, bool)) -> i64 {
            let (pos, to_right) = *args;
            let mut res = i64::MAX;
            if to_right && pos >= self.to_right[0].0 {
                let mut id = self.right_index.get();
                while id + 1 < self.to_right.len() && self.to_right[id + 1].0 <= pos {
                    id += 1;
                }
                res.minim(value(self.to_right[id].1, pos));
                self.right_index.set(id);
            }
            if !to_right && pos <= self.to_left[0].0 {
                let mut id = self.left_index.get();
                while id + 1 < self.to_left.len() && self.to_left[id + 1].0 >= pos {
                    id += 1;
                }
                res.minim(value(self.to_left[id].1, pos));
                self.left_index.set(id);
            }
            res
        }

        fn join_results(
            left_res: i64,
            right_res: i64,
            _args: &(i64, bool),
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> i64 {
            left_res.min(right_res)
        }
    }

    let mut tree = SegmentTree::from_generator(n, |id| {
        let (pos, _) = StoragesContainer::val()[order[id]];
        Node {
            to_right: vec![(pos, order[id])],
            to_left: vec![(pos, order[id])],
            ..Default::default()
        }
    });

    let mut pos = vec![0; n];
    for i in 0..n {
        pos[order[i]] = i;
    }

    let mut queries = Vec::with_capacity(m);

    for _ in 0..m {
        let c = input.read_long();
        let k = input.read_size();
        let mut d = input.read_size_vec(k).dec();
        for i in d.iter_mut() {
            *i = pos[*i];
        }
        d.sort();
        queries.push((c, d));
    }

    let mut order = (0..m).collect_vec();
    order.sort_by_key(|&i| queries[i].0);
    let mut ans = vec![i64::MAX; m];

    for &i in &order {
        let (c, d) = &queries[i];
        let k = d.len();

        if k == 0 {
            ans[i].minim(tree.query_with_args(.., &(*c, true)));
        } else {
            ans[i].minim(tree.query_with_args(..d[0], &(*c, true)));
            for (&d1, &d2) in d.consecutive_iter() {
                ans[i].minim(tree.query_with_args(d1 + 1..d2, &(*c, true)));
            }
            ans[i].minim(tree.query_with_args(d[k - 1] + 1.., &(*c, true)));
        }
    }

    for i in order.into_iter().rev() {
        let (c, d) = &queries[i];
        let k = d.len();

        if k == 0 {
            ans[i].minim(tree.query_with_args(.., &(*c, false)));
        } else {
            ans[i].minim(tree.query_with_args(..d[0], &(*c, false)));
            for (&d1, &d2) in d.consecutive_iter() {
                ans[i].minim(tree.query_with_args(d1 + 1..d2, &(*c, false)));
            }
            ans[i].minim(tree.query_with_args(d[k - 1] + 1.., &(*c, false)));
        }
    }

    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
