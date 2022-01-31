//{"name":"J4 - Snowball Fight","group":"DMOJ - Mock CCC '22 Contest 1","url":"https://dmoj.ca/problem/mccc3j4","interactive":false,"timeLimit":600,"tests":[{"input":"4 3\n4 1 1 2\n","output":"3 1 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"J4SnowballFight"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out_line, zip};
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let t = input.read_usize();
    let start = input.read_usize_vec(n).dec_by_one();

    let mut qs = start.into_iter().map(|i| VecDeque::from([i])).collect_vec();
    let mut ans = vec![None; n];
    let mut next = vec![None; n];
    for _ in 0..t {
        for (v, q) in next.iter_mut().zip(qs.iter_mut()) {
            *v = q.pop_front();
        }
        for (a, &v, i) in zip!(ans.iter_mut(), next.iter(), 0..n) {
            if v.is_some() {
                *a = v;
                qs[v.unwrap()].push_back(i);
            }
        }
    }
    for v in &mut ans {
        *v = v.map(|i| i + 1);
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
