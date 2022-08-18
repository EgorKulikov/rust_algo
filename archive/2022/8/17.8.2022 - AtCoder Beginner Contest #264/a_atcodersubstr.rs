//{"name":"A - \"atcoder\".substr()","group":"AtCoder - freee Programming Contest 2022（AtCoder Beginner Contest 264）","url":"https://atcoder.jp/contests/abc264/tasks/abc264_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 6\n","output":"code\n"},{"input":"4 4\n","output":"o\n"},{"input":"1 7\n","output":"atcoder\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAtcodersubstr"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let l = input.read_usize() - 1;
    let r = input.read_usize();

    out_line!(&"atcoder"[l..r]);
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
