//{"name":"A - Rotate","group":"AtCoder - HHKB Programming Contest 2022（AtCoder Beginner Contest 235）","url":"https://atcoder.jp/contests/abc235/tasks/abc235_a","interactive":false,"timeLimit":2000,"tests":[{"input":"123\n","output":"666\n"},{"input":"999\n","output":"2997\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARotate"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let a = (input.get().unwrap() - b'0').into_u32();
    let b = (input.get().unwrap() - b'0').into_u32();
    let c = (input.get().unwrap() - b'0').into_u32();
    out_line!((a + b + c) * 111);
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
