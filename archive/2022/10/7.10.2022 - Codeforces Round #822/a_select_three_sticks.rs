//{"name":"A. Select Three Sticks","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"http://codeforces.com/contest/1734/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 3\n4\n7 3 7 3\n5\n3 4 2 1 1\n8\n3 1 4 1 5 9 2 6\n","output":"2\n4\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASelectThreeSticks"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut sticks = input.read_usize_vec(n);

    sticks.sort();
    let mut ans = None;
    for i in 1..n - 1 {
        ans.minim(sticks[i + 1] - sticks[i - 1]);
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
