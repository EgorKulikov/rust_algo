//{"name":"G. Skip-Solve","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 17\n6 3 7 2 3\n1 15\n10\n10 30\n10 9 8 7 6 5 4 3 2 1\n10 8\n10 9 8 7 6 5 4 3 2 1\n10 9\n10 9 8 7 6 5 4 3 2 1\n","output":"4\n2 4 5 1\n1\n1\n6\n2 4 6 8 10 9\n0\n\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSkipSolve"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut k = input.read_long();
    let a = input.read_long_vec(n);

    let mut heap = BinaryHeap::new();
    let mut ans = Vec::new();
    for i in 0..n {
        if 2 * i < n {
            heap.push((Reverse(a[2 * i]), 2 * i + 1));
        }
        if 2 * i + 1 < n {
            heap.push((Reverse(a[2 * i + 1]), 2 * i + 2));
        }
        let (Reverse(cur), id) = heap.pop().unwrap();
        if cur > k {
            break;
        }
        ans.push(id);
        k -= cur;
    }
    out_line!(ans.len());
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
