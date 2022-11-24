//{"name":"A - Shift","group":"AtCoder - AtCoder Beginner Contest 278","url":"https://atcoder.jp/contests/abc278/tasks/abc278_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n2 7 8\n","output":"8 0 0\n"},{"input":"3 4\n9 9 9\n","output":"0 0 0\n"},{"input":"9 5\n1 2 3 4 5 6 7 8 9\n","output":"6 7 8 9 0 0 0 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AShift"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_int_vec(n);

    let ans = a.into_iter().skip(k).chain((0..k.min(n)).map(|_| 0));
    output().print_iter(ans);
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
