//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::gcd::gcd;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long_vec(n);
    let q = input.read_usize();

    if n == 1 {
        for _ in 0..q {
            out_line!(1);
        }
        return;
    }

    let mut g = Vec::with_capacity(n - 1);
    for (&a, &b) in a.consecutive_iter() {
        g.push((b - a).abs());
    }

    #[derive(Copy, Clone)]
    struct Node(i64);
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self(0)
        }

        fn join(&mut self, left: &Self, right: &Self) {
            self.0 = gcd(left.0, right.0)
        }

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    let mut st = SegmentTree::from_generator(n - 1, |i| Node(g[i]));
    let mut end = Vec::with_capacity(n);
    let mut to = 0;
    for i in 0..n {
        if i > to {
            to = i;
        }
        while to < n - 1 && st.query(i, to + 1).0 != 1 {
            to += 1;
        }
        end.push(to);
    }

    let mut pos = vec![Vec::new(); 4 * n];
    let mut init = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
        if left + 1 < right {
            let mid = (left + right) >> 1;
            f.call(2 * root + 1, left, mid);
            f.call(2 * root + 2, mid, right);
        }
        let mut tar_len = 0;
        for i in left..right {
            if end[i] - i >= tar_len {
                pos[root].push(i);
                tar_len = end[i] - i + 1;
            }
        }
    });
    init.call(0, 0, n);

    let mut last_ans = 0usize;
    for _ in 0..q {
        let mut l = input.read_usize();
        let mut r = input.read_usize();
        l = (l + last_ans * k - 1) % n + 1;
        r = (r + last_ans * k - 1) % n + 1;
        if l > r {
            swap(&mut l, &mut r);
        }
        l -= 1;
        let mut query = RecursiveFunction3::new(
            |f, root: usize, left: usize, right: usize| -> (usize, usize) {
                if left >= r || right <= l {
                    (0, 0)
                } else if left >= l && right <= r {
                    let mut f = 0;
                    let mut t = pos[root].len() - 1;
                    while f < t {
                        let mid = (f + t + 1) >> 1;
                        if end[pos[root][mid]] < r {
                            f = mid;
                        } else {
                            t = mid - 1;
                        }
                    }
                    if end[pos[root][f]] >= r {
                        (r - pos[root][f], pos[root][f] + 1)
                    } else if f + 1 == pos[root].len() {
                        (end[pos[root][f]] - pos[root][f] + 1, pos[root][f] + 1)
                    } else {
                        (end[pos[root][f]] - pos[root][f] + 1, pos[root][f] + 1)
                            .max((r - pos[root][f + 1], pos[root][f + 1] + 1))
                    }
                } else {
                    let mid = (left + right) >> 1;
                    f.call(2 * root + 1, left, mid)
                        .max(f.call(2 * root + 2, mid, right))
                }
            },
        );
        let (ans, pos) = query.call(0, 0, n);
        last_ans = ans;
        out_line!(pos, pos + ans - 1);
    }
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
