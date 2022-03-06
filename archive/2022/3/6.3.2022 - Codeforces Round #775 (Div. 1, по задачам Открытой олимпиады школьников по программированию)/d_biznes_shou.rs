//{"name":"D. Бизнес-шоу","group":"Codeforces - Codeforces Round #775 (Div. 1, по задачам Открытой олимпиады школьников по программированию)","url":"https://codeforces.com/contest/1648/problem/D","interactive":false,"timeLimit":5000,"tests":[{"input":"4 3\n1 0 2 -1\n-3 1 9 2\n3 2 4 1\n1 2 5\n2 3 4\n1 4 14\n","output":"13\n"},{"input":"5 4\n-20 -10 -11 -10 1\n1 3 3 6 3\n14 -20 3 6 2\n1 5 13\n1 2 2\n3 5 3\n2 3 1\n","output":"-4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBiznesShou"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let a = input.read_table::<i64>(3, n);
    let mut offers: Vec<(usize, usize, i64)> = input.read_vec(q);

    offers.sort();
    let mut sb = 0;
    for i in 0..n {
        sb += a[(1, i)];
    }
    let mut switch_up = vec![0; n];
    switch_up[0] = a[(0, 0)];
    for i in 1..n {
        switch_up[i] = switch_up[i - 1] + a[(0, i)] - a[(1, i - 1)];
    }
    let mut switch_down = vec![0; n];
    switch_down[n - 1] = a[(2, n - 1)];
    for i in (0..n - 1).rev() {
        switch_down[i] = switch_down[i + 1] + a[(2, i)] - a[(1, i + 1)];
    }

    const INFTY: i64 = i64::MIN / 2;
    #[derive(Copy, Clone)]
    struct Node(i64);
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self(INFTY)
        }

        fn join(&mut self, left: &Self, right: &Self) {
            self.0 = left.0.max(right.0)
        }

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    let mut up_tree = SegmentTree::from_generator(n, |i| Node(switch_up[i]));
    let mut down_tree = SegmentTree::from_generator(n, |i| Node(switch_down[i]));

    let mut ans = None;
    let mut v = vec![INFTY; 4 * n];
    for &(mut l, r, k) in &offers {
        l -= 1;
        let mut query = RecursiveFunction3::new(|f, root, from, to| -> i64 {
            if from >= r || to <= l {
                INFTY
            } else if from >= l && to <= r {
                v[root]
            } else {
                let mid = (from + to) >> 1;
                let lv = f.call(2 * root + 1, from, mid);
                let rv = f.call(2 * root + 2, mid, to);
                lv.max(rv)
            }
        });
        let mut cv = query.call(0, 0, n) - k;
        // ans.maxim(cv + down_tree.query(l, r).0);
        if r < n {
            cv.maxim(up_tree.query(l, r).0 - k);
            switch_up[r].maxim(cv);
            let mut update = RecursiveFunction3::new(|f, root: usize, from, to| {
                v[root].maxim(cv);
                if from + 1 == to {
                    return;
                }
                let mid = (from + to) >> 1;
                if mid > r {
                    f.call(2 * root + 1, from, mid);
                } else {
                    f.call(2 * root + 2, mid, to);
                }
            });
            update.call(0, 0, n);
        }
    }

    let mut res = vec![INFTY; 4 * n];
    let mut up = vec![INFTY; 4 * n];
    let mut down = vec![INFTY; 4 * n];
    let mut init = RecursiveFunction3::new(|f, root: usize, from, to| {
        if from + 1 == to {
            res[root] = switch_up[from] + switch_down[from];
            up[root] = switch_up[from];
            down[root] = switch_down[from];
            return;
        }
        let mid = (from + to) >> 1;
        f.call(2 * root + 1, from, mid);
        f.call(2 * root + 2, mid, to);
        up[root] = up[2 * root + 1].max(up[2 * root + 2]);
        down[root] = down[2 * root + 1].max(down[2 * root + 2]);
        res[root] = res[2 * root + 1]
            .max(res[2 * root + 2])
            .max(up[2 * root + 1] + down[2 * root + 2]);
    });
    init.call(0, 0, n);
    for (mut l, r, k) in offers {
        l -= 1;
        let mut query = RecursiveFunction3::new(|f, root: usize, from, to| -> (i64, i64, i64) {
            if from >= r || to <= l {
                (INFTY, INFTY, INFTY)
            } else if from >= l && to <= r {
                (res[root], up[root], down[root])
            } else {
                let mid = (from + to) >> 1;
                let (lr, lu, ld) = f.call(2 * root + 1, from, mid);
                let (rr, ru, rd) = f.call(2 * root + 2, mid, to);
                (lr.max(rr).max(lu + rd), lu.max(ru), ld.max(rd))
            }
        });
        ans.maxim(query.call(0, 0, n).0 - k);
    }
    out_line!(ans.unwrap() + sb);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
