//{"name":"A. Лента TubeTube","group":"Codeforces - Codeforces Round 867 (Div. 3)","url":"https://codeforces.com/contest/1822/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5 9\n1 5 7 6 6\n3 4 7 1 9\n4 4\n4 3 3 2\n1 2 3 4\n5 7\n5 5 5 5 5\n2 1 3 9 7\n4 33\n54 71 69 96\n42 24 99 1\n2 179\n55 66\n77 88\n","output":"3\n2\n3\n-1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALentaTubeTube"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let t = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_int_vec(n);

    let mut ans = None;
    let mut id = None;
    for (i, a) in a.into_iter().enumerate() {
        if i + a <= t && ans.maxim(b[i]) {
            id = Some(i + 1);
        }
    }
    out_line!(id);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
