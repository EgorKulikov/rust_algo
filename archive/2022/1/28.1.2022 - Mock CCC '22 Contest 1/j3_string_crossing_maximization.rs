//{"name":"J3 - String Crossing Maximization","group":"DMOJ - Mock CCC '22 Contest 1","url":"https://dmoj.ca/problem/mccc3j3","interactive":false,"timeLimit":600,"tests":[{"input":"10 7\nHELLOWORLD\nGOODBYE\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"J3StringCrossingMaximization"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.read_usize();
    let _m = input.read_usize();
    let s: Str = input.read();
    let t: Str = input.read();

    let mut q = [0i64; 26];
    for c in s {
        q[(c - b'A').into_usize()] += 1;
    }
    let max = *q.iter().max().unwrap();
    let mut min = None;
    let mut ans = 0;
    for c in t {
        let cur = q[(c - b'A').into_usize()];
        min.minim(cur);
        ans += cur;
    }
    ans += max - min.unwrap();
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
