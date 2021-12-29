//{"name":"A. Робот-пылесос","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"https://codeforces.com/contest/1623/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10 10 6 1 2 8\n10 10 9 9 1 1\n9 8 5 6 2 1\n6 9 2 2 5 8\n2 2 1 1 2 1\n","output":"7\n10\n9\n3\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARobotPilesos"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let rb = input.read_usize();
    let cb = input.read_usize();
    let rd = input.read_usize();
    let cd = input.read_usize();

    let mut ans = None;
    for (rb, rd, n) in [(rb, rd, n), (cb, cd, m)] {
        if rd >= rb {
            ans.minim(rd - rb);
        } else {
            ans.minim(2 * n - rd - rb);
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
