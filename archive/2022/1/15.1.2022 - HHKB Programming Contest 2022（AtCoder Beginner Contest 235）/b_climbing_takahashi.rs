//{"name":"B - Climbing Takahashi","group":"AtCoder - HHKB Programming Contest 2022（AtCoder Beginner Contest 235）","url":"https://atcoder.jp/contests/abc235/tasks/abc235_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 5 10 4 2\n","output":"10\n"},{"input":"3\n100 1000 100000\n","output":"100000\n"},{"input":"4\n27 1828 1828 9242\n","output":"1828\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BClimbingTakahashi"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let h = input.read_unsigned_vec(n);

    let mut ans = None;
    for i in h {
        if !ans.maxim(i) {
            break;
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
