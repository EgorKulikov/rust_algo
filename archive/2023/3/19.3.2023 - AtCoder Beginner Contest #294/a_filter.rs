//{"name":"A - Filter","group":"AtCoder - AtCoder Beginner Contest 294","url":"https://atcoder.jp/contests/abc294/tasks/abc294_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2 3 5 6\n","output":"2 6\n"},{"input":"5\n2 2 2 3 3\n","output":"2 2 2\n"},{"input":"10\n22 3 17 8 30 15 12 14 11 17\n","output":"22 8 30 12 14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AFilter"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    out_line!(a.into_iter().filter(|x| x % 2 == 0).collect_vec());
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
