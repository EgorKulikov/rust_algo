//{"name":"P1 - Peanut Planning","group":"DMOJ - DMOPC '21 Contest 8","url":"https://dmoj.ca/problem/dmopc21c8p1","interactive":false,"timeLimit":2000,"tests":[{"input":"4 8\n2 6 10 1\n","output":"1 10 2 6\n"},{"input":"4 9\n2 6 10 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P1PeanutPlanning"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_int();
    let mut a = input.read_int_vec(n);

    a.sort_unstable();
    let mut ans = Vec::with_capacity(n);
    ans.push(a[0]);
    let mut set = BTreeMap::new();
    for i in a.into_iter().skip(1) {
        match set.get_mut(&i) {
            None => {
                set.insert(i, 1);
            }
            Some(v) => {
                *v += 1;
            }
        }
    }
    for _ in 1..n {
        let min = m - *ans.last().unwrap();
        if let Some((&v, q)) = set.range_mut(min..).next() {
            ans.push(v);
            if *q > 1 {
                *q -= 1;
            } else {
                set.remove(&v);
            }
        } else {
            out_line!(-1);
            return;
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
