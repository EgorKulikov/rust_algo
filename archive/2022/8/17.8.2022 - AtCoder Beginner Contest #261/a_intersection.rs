//{"name":"A - Intersection","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_a","interactive":false,"timeLimit":2000,"tests":[{"input":"0 3 1 5\n","output":"2\n"},{"input":"0 1 4 5\n","output":"0\n"},{"input":"0 3 3 7\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AIntersection"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let l1 = input.read_int();
    let r1 = input.read_int();
    let l2 = input.read_int();
    let r2 = input.read_int();

    out_line!((r2.min(r1) - l2.max(l1)).max(0));
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
