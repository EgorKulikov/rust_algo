//{"name":"B - Perfect String","group":"AtCoder - Monoxer Programming Contest 2022（AtCoder Beginner Contest 249）","url":"https://atcoder.jp/contests/abc249/tasks/abc249_b","interactive":false,"timeLimit":2000,"tests":[{"input":"AtCoder\n","output":"Yes\n"},{"input":"Aa\n","output":"Yes\n"},{"input":"atcoder\n","output":"No\n"},{"input":"Perfect\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPerfectString"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let mut s: Str = input.read();

    output().bool_output = BoolOutput::YesNo;
    let upper = s.iter().filter(|&it| it.is_ascii_uppercase()).count() != 0;
    let lower = s.iter().filter(|&it| it.is_ascii_lowercase()).count() != 0;
    s.sort();
    for (s, t) in s.as_slice().consecutive_iter() {
        if s == t {
            out_line!(false);
            return;
        }
    }
    out_line!(upper && lower);
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
