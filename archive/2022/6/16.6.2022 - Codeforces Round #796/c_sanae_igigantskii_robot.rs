//{"name":"C. Sanae и гигантский робот","group":"Codeforces - Codeforces Round #796 (Div. 1)","url":"https://codeforces.com/contest/1687/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 2\n1 5 4 2 3\n3 2 5 4 1\n1 3\n2 5\n5 2\n1 5 4 2 3\n3 2 4 5 1\n1 2\n2 4\n","output":"YES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSanaeIGigantskiiRobot"}}}

extern crate core;

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let segs = input.read_usize_pair_vec(m);

    let mut all_good = BitSet::new(4 * (n + 1));
    let mut good = BitSet::new(n + 1);
    let sa = a.as_slice().partial_sums();
    let sb = b.as_slice().partial_sums();
    let mut order = VecDeque::new();
    for i in 0..=n {
        if sa[i] == sb[i] {
            order.push_back(i);
            let at = i;
            let mut point_op =
                RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                    if left + 1 == right {
                        good.set(left, true);
                        all_good.set(root, true);
                        return;
                    }
                    let mid = (left + right) >> 1;
                    if at < mid {
                        f.call(2 * root + 1, left, mid);
                    } else {
                        f.call(2 * root + 2, mid, right);
                    }
                    if all_good[2 * root + 1] && all_good[2 * root + 2] {
                        all_good.set(root, true);
                    }
                });
            point_op.call(0, 0, n + 1);
        }
    }
    if sa[n] != sb[n] {
        out_line!(false);
        return;
    }
    let mut ends = vec![Vec::new(); n + 1];
    for &(i, j) in &segs {
        ends[i - 1].push(j);
        ends[j].push(i - 1);
    }
    while let Some(i) = order.pop_front() {
        for &j in &ends[i] {
            if good[j] {
                let from = j.min(i);
                let to = i.max(j);
                let mut segment_op =
                    RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                        if to <= left || right <= from {
                            return;
                        }
                        if all_good[root] {
                            return;
                        }
                        if left + 1 == right {
                            order.push_back(left);
                            good.set(left, true);
                            all_good.set(root, true);
                            return;
                        }
                        let mid = (left + right) >> 1;
                        // push down
                        f.call(2 * root + 1, left, mid);
                        f.call(2 * root + 2, mid, right);
                        // unite
                        if all_good[2 * root + 1] && all_good[2 * root + 2] {
                            all_good.set(root, true);
                        }
                    });
                segment_op.call(0, 0, n + 1);
            }
        }
    }
    out_line!(all_good[0]);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
