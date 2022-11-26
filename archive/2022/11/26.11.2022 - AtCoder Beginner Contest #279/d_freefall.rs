//{"name":"D - Freefall","group":"AtCoder - TOYOTA SYSTEMS Programming Contest 2022(AtCoder Beginner Contest 279)","url":"https://atcoder.jp/contests/abc279/tasks/abc279_d","interactive":false,"timeLimit":2000,"tests":[{"input":"10 1\n","output":"7.7735026919\n"},{"input":"5 10\n","output":"5.0000000000\n"},{"input":"1000000000000000000 100\n","output":"8772053214538.5976562500\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DFreefall"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let a = input.read_long();
    let b = input.read_float();

    let mut left = 1;
    let mut right = a;

    let f = |x: f64| b * (x - 1.) + (a as f64) / x.sqrt();

    while left < right {
        let mid = (left + right) / 2;
        if f((mid + 1) as f64) >= f(mid as f64) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out_line!(f(left as f64));
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
