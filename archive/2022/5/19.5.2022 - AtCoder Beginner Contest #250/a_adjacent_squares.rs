//{"name":"A - Adjacent Squares","group":"AtCoder - AtCoder Beginner Contest 250","url":"https://atcoder.jp/contests/abc250/tasks/abc250_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n2 2\n","output":"4\n"},{"input":"3 4\n1 3\n","output":"3\n"},{"input":"3 4\n3 4\n","output":"2\n"},{"input":"1 10\n1 5\n","output":"2\n"},{"input":"8 1\n8 1\n","output":"1\n"},{"input":"1 1\n1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAdjacentSquares"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let h = input.read_usize();
    let w = input.read_usize();
    let r = input.read_usize() - 1;
    let c = input.read_usize() - 1;

    out_line!(D4::iter(r, c, h, w).count());
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
