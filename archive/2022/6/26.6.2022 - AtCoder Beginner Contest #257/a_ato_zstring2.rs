//{"name":"A - A to Z String 2","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_a","interactive":false,"timeLimit":2000,"tests":[{"input":"1 3\n","output":"C\n"},{"input":"2 12\n","output":"F\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAToZString2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let x = input.read_usize() - 1;

    out_line!((b'A' + (x / n).into_u8()) as char);
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
