//{"name":"B - A Reverse","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 7\nabcdefgh\n","output":"abgfedch\n"},{"input":"1 7\nreviver\n","output":"reviver\n"},{"input":"4 13\nmerrychristmas\n","output":"meramtsirhcyrs\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAReverse"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let l = input.read::<usize>() - 1;
    let r: usize = input.read();
    let mut s: Str = input.read();

    s[l..r].reverse();
    out_line!(s);
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
