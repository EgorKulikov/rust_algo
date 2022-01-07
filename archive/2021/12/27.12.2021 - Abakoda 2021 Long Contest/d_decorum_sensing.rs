//{"name":"D. Decorum Sensing","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 6 1 8 0 3 3 9 8 8\n","output":"5\n"},{"input":"4\n1 2 3 4\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDecorumSensing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_usize_vec(n);

    a.sort_unstable();
    for (i, a) in a.into_iter().enumerate() {
        if i < a {
            out_line!(i);
            return;
        }
    }
    out_line!(n);
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
