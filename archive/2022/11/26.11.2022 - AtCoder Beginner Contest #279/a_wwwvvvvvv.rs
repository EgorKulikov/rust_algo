//{"name":"A - wwwvvvvvv","group":"AtCoder - TOYOTA SYSTEMS Programming Contest 2022(AtCoder Beginner Contest 279)","url":"https://atcoder.jp/contests/abc279/tasks/abc279_a","interactive":false,"timeLimit":2000,"tests":[{"input":"vvwvw\n","output":"7\n"},{"input":"v\n","output":"1\n"},{"input":"wwwvvvvvv\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AWwwvvvvvv"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let s: Str = input.read();

    let mut ans = 0;
    for c in s {
        match c {
            b'v' => ans += 1,
            b'w' => ans += 2,
            _ => unreachable!(),
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
