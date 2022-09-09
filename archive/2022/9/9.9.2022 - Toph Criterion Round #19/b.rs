//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_int_vec(n);

    a.sort();
    a.dedup();
    let s = a.iter().cloned().collect::<HashSet<_>>();
    let mut ignored = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut ans = None;
    if s.contains(&(2 * a[0])) {
        heap.push((Reverse(2 * a[0]), a[0]));
        ignored.insert(2 * a[0]);
    } else {
        ans.minim(2 * a[0]);
    }
    let mut at = 1;
    while let Some((Reverse(x), y)) = heap.pop() {
        while at < a.len() && a[at] <= x {
            if !ignored.contains(&a[at]) {
                if s.contains(&(2 * a[at])) {
                    heap.push((Reverse(2 * a[at]), a[at]));
                    ignored.insert(2 * a[at]);
                } else {
                    ans.minim(2 * a[at]);
                }
            }
            at += 1;
        }
        if s.contains(&(x + y)) {
            heap.push((Reverse(x + y), y));
            ignored.insert(x + y);
        } else {
            ans.minim(x + y);
        }
    }
    out_line!(ans);
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
