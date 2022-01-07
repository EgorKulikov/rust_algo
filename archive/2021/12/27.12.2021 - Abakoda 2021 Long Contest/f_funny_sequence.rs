//{"name":"F. Funny Sequence","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/F","interactive":false,"timeLimit":500,"tests":[{"input":"0\n","output":"2\n"},{"input":"1\n","output":"3\n"},{"input":"2\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFunnySequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();

    out_line!(n * n + 2);
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
