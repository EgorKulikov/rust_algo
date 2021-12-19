//{"name":"A - QQ solver","group":"AtCoder - M-SOLUTIONS Programming Contest 2021(AtCoder Beginner Contest 232)","url":"https://atcoder.jp/contests/abc232/tasks/abc232_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3x7\n","output":"21\n"},{"input":"9x9\n","output":"81\n"},{"input":"1x1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AQQSolver"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let a = input.get().unwrap() - b'0';
    input.get();
    let b = input.get().unwrap() - b'0';

    out_line!(a * b);
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
