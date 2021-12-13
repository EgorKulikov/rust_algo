//{"name":"CamelCase","group":"HackerRank - Algorithms - Strings","url":"https://www.hackerrank.com/challenges/camelcase/problem","interactive":false,"timeLimit":4000,"tests":[{"input":"saveChangesInTheEditor\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CamelCase"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let ans = 1 + s
        .into_iter()
        .filter(|c| (*c as char).is_uppercase())
        .count();
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
