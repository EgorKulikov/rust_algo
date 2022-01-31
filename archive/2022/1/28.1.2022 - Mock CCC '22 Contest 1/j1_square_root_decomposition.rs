//{"name":"J1 - Square Root Decomposition","group":"DMOJ - Mock CCC '22 Contest 1","url":"https://dmoj.ca/problem/mccc3j1","interactive":false,"timeLimit":250,"tests":[{"input":"9\n3\n4\n","output":"1\n"},{"input":"16\n5\n3\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"J1SquareRootDecomposition"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_int();
    let i = input.read_int();
    let j = input.read_int();

    if (n - i * i).abs() < (n - j * j).abs() {
        out_line!(1);
    } else {
        out_line!(2);
    }
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
