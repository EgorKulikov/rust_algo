//{"name":"C. Caught in Candy","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"1 2 3\n2 4\n","output":"2.000000000000\n"},{"input":"4 0 0\n-3 4\n5 0\n1 1\n-4 -3\n","output":"10.000000000000\n"},{"input":"2 1 1\n0 0\n2 0\n","output":"2.828427124746\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCaughtInCandy"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let h = input.read_float();
    let k = input.read_float();
    let pts: Vec<(f64, f64)> = input.read_vec(n);

    let mut ans = 0.;
    for (x, y) in pts {
        ans.maxim(f64::hypot(x - h, y - k));
    }
    out_line!(2. * ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
