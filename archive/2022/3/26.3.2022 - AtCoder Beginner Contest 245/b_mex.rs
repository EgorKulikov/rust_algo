//{"name":"B - Mex","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_b","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n0 3 2 6 2 1 0 0\n","output":"4\n"},{"input":"3\n2000 2000 2000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMex"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut has = BitSet::new(2001);
    has.fill(true);
    for i in a {
        has.set(i, false);
    }
    out_line!(has.iter().next());
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
